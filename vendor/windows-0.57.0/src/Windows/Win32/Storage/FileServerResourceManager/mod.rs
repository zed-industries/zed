#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(DIFsrmClassificationEvents, DIFsrmClassificationEvents_Vtbl, 0x26942db0_dabf_41d8_bbdd_b129a9f70424);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for DIFsrmClassificationEvents {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(DIFsrmClassificationEvents, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl DIFsrmClassificationEvents {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DIFsrmClassificationEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmAccessDeniedRemediationClient, IFsrmAccessDeniedRemediationClient_Vtbl, 0x40002314_590b_45a5_8e1b_8c05da527e52);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmAccessDeniedRemediationClient {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmAccessDeniedRemediationClient, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmAccessDeniedRemediationClient {
    pub unsafe fn Show<P0, P1, P2>(&self, parentwnd: usize, accesspath: P0, errortype: AdrClientErrorType, flags: i32, windowtitle: P1, windowmessage: P2) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self), parentwnd, accesspath.param().abi(), errortype, flags, windowtitle.param().abi(), windowmessage.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmAccessDeniedRemediationClient_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, usize, core::mem::MaybeUninit<windows_core::BSTR>, AdrClientErrorType, i32, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmAction, IFsrmAction_Vtbl, 0x6cd6408a_ae60_463b_9ef1_e117534d69dc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmAction {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmAction, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmAction {
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ActionType(&self) -> windows_core::Result<FsrmActionType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ActionType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RunLimitInterval(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RunLimitInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRunLimitInterval)(windows_core::Interface::as_raw(self), minutes).ok()
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmAction_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ActionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmActionType) -> windows_core::HRESULT,
    pub RunLimitInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRunLimitInterval: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmActionCommand, IFsrmActionCommand_Vtbl, 0x12937789_e247_4917_9c20_f3ee9c7ee783);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmActionCommand {
    type Target = IFsrmAction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmActionCommand, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmAction);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmActionCommand {
    pub unsafe fn ExecutablePath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExecutablePath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetExecutablePath<P0>(&self, executablepath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetExecutablePath)(windows_core::Interface::as_raw(self), executablepath.param().abi()).ok()
    }
    pub unsafe fn Arguments(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Arguments)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetArguments<P0>(&self, arguments: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetArguments)(windows_core::Interface::as_raw(self), arguments.param().abi()).ok()
    }
    pub unsafe fn Account(&self) -> windows_core::Result<FsrmAccountType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Account)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAccount(&self, account: FsrmAccountType) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAccount)(windows_core::Interface::as_raw(self), account).ok()
    }
    pub unsafe fn WorkingDirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WorkingDirectory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetWorkingDirectory<P0>(&self, workingdirectory: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetWorkingDirectory)(windows_core::Interface::as_raw(self), workingdirectory.param().abi()).ok()
    }
    pub unsafe fn MonitorCommand(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MonitorCommand)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMonitorCommand<P0>(&self, monitorcommand: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetMonitorCommand)(windows_core::Interface::as_raw(self), monitorcommand.param().abi()).ok()
    }
    pub unsafe fn KillTimeOut(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KillTimeOut)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetKillTimeOut(&self, minutes: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetKillTimeOut)(windows_core::Interface::as_raw(self), minutes).ok()
    }
    pub unsafe fn LogResult(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LogResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLogResult<P0>(&self, logresults: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetLogResult)(windows_core::Interface::as_raw(self), logresults.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmActionCommand_Vtbl {
    pub base__: IFsrmAction_Vtbl,
    pub ExecutablePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetExecutablePath: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Arguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetArguments: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Account: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmAccountType) -> windows_core::HRESULT,
    pub SetAccount: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmAccountType) -> windows_core::HRESULT,
    pub WorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetWorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MonitorCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetMonitorCommand: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub KillTimeOut: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetKillTimeOut: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub LogResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetLogResult: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmActionEmail, IFsrmActionEmail_Vtbl, 0xd646567d_26ae_4caa_9f84_4e0aad207fca);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmActionEmail {
    type Target = IFsrmAction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmActionEmail, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmAction);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmActionEmail {
    pub unsafe fn MailFrom(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MailFrom)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMailFrom<P0>(&self, mailfrom: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMailFrom)(windows_core::Interface::as_raw(self), mailfrom.param().abi()).ok()
    }
    pub unsafe fn MailReplyTo(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MailReplyTo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMailReplyTo<P0>(&self, mailreplyto: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMailReplyTo)(windows_core::Interface::as_raw(self), mailreplyto.param().abi()).ok()
    }
    pub unsafe fn MailTo(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MailTo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMailTo<P0>(&self, mailto: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMailTo)(windows_core::Interface::as_raw(self), mailto.param().abi()).ok()
    }
    pub unsafe fn MailCc(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MailCc)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMailCc<P0>(&self, mailcc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMailCc)(windows_core::Interface::as_raw(self), mailcc.param().abi()).ok()
    }
    pub unsafe fn MailBcc(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MailBcc)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMailBcc<P0>(&self, mailbcc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMailBcc)(windows_core::Interface::as_raw(self), mailbcc.param().abi()).ok()
    }
    pub unsafe fn MailSubject(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MailSubject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMailSubject<P0>(&self, mailsubject: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMailSubject)(windows_core::Interface::as_raw(self), mailsubject.param().abi()).ok()
    }
    pub unsafe fn MessageText(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MessageText)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMessageText<P0>(&self, messagetext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMessageText)(windows_core::Interface::as_raw(self), messagetext.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmActionEmail_Vtbl {
    pub base__: IFsrmAction_Vtbl,
    pub MailFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetMailFrom: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MailReplyTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetMailReplyTo: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetMailTo: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MailCc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetMailCc: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MailBcc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetMailBcc: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MailSubject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetMailSubject: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MessageText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetMessageText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmActionEmail2, IFsrmActionEmail2_Vtbl, 0x8276702f_2532_4839_89bf_4872609a2ea4);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmActionEmail2 {
    type Target = IFsrmActionEmail;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmActionEmail2, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmAction, IFsrmActionEmail);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmActionEmail2 {
    pub unsafe fn AttachmentFileListSize(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AttachmentFileListSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAttachmentFileListSize(&self, attachmentfilelistsize: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAttachmentFileListSize)(windows_core::Interface::as_raw(self), attachmentfilelistsize).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmActionEmail2_Vtbl {
    pub base__: IFsrmActionEmail_Vtbl,
    pub AttachmentFileListSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAttachmentFileListSize: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmActionEventLog, IFsrmActionEventLog_Vtbl, 0x4c8f96c3_5d94_4f37_a4f4_f56ab463546f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmActionEventLog {
    type Target = IFsrmAction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmActionEventLog, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmAction);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmActionEventLog {
    pub unsafe fn EventType(&self) -> windows_core::Result<FsrmEventType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEventType(&self, eventtype: FsrmEventType) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEventType)(windows_core::Interface::as_raw(self), eventtype).ok()
    }
    pub unsafe fn MessageText(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MessageText)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMessageText<P0>(&self, messagetext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMessageText)(windows_core::Interface::as_raw(self), messagetext.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmActionEventLog_Vtbl {
    pub base__: IFsrmAction_Vtbl,
    pub EventType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmEventType) -> windows_core::HRESULT,
    pub SetEventType: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmEventType) -> windows_core::HRESULT,
    pub MessageText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetMessageText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmActionReport, IFsrmActionReport_Vtbl, 0x2dbe63c4_b340_48a0_a5b0_158e07fc567e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmActionReport {
    type Target = IFsrmAction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmActionReport, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmAction);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmActionReport {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReportTypes(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReportTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetReportTypes(&self, reporttypes: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetReportTypes)(windows_core::Interface::as_raw(self), reporttypes).ok()
    }
    pub unsafe fn MailTo(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MailTo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMailTo<P0>(&self, mailto: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMailTo)(windows_core::Interface::as_raw(self), mailto.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmActionReport_Vtbl {
    pub base__: IFsrmAction_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ReportTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReportTypes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetReportTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetReportTypes: usize,
    pub MailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetMailTo: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmAutoApplyQuota, IFsrmAutoApplyQuota_Vtbl, 0xf82e5729_6aba_4740_bfc7_c7f58f75fb7b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmAutoApplyQuota {
    type Target = IFsrmQuotaObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmAutoApplyQuota, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmQuotaBase, IFsrmQuotaObject);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmAutoApplyQuota {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExcludeFolders(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExcludeFolders)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetExcludeFolders(&self, folders: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetExcludeFolders)(windows_core::Interface::as_raw(self), folders).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> windows_core::Result<IFsrmDerivedObjectsResult> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CommitAndUpdateDerived)(windows_core::Interface::as_raw(self), commitoptions, applyoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmAutoApplyQuota_Vtbl {
    pub base__: IFsrmQuotaObject_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ExcludeFolders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExcludeFolders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetExcludeFolders: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetExcludeFolders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CommitAndUpdateDerived: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmCommitOptions, FsrmTemplateApplyOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommitAndUpdateDerived: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmClassificationManager, IFsrmClassificationManager_Vtbl, 0xd2dc89da_ee91_48a0_85d8_cc72a56f7d04);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmClassificationManager {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmClassificationManager, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmClassificationManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ClassificationReportFormats(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClassificationReportFormats)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetClassificationReportFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetClassificationReportFormats)(windows_core::Interface::as_raw(self), formats).ok()
    }
    pub unsafe fn Logging(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Logging)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLogging(&self, logging: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLogging)(windows_core::Interface::as_raw(self), logging).ok()
    }
    pub unsafe fn ClassificationReportMailTo(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClassificationReportMailTo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetClassificationReportMailTo<P0>(&self, mailto: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetClassificationReportMailTo)(windows_core::Interface::as_raw(self), mailto.param().abi()).ok()
    }
    pub unsafe fn ClassificationReportEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClassificationReportEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetClassificationReportEnabled<P0>(&self, reportenabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetClassificationReportEnabled)(windows_core::Interface::as_raw(self), reportenabled.param().abi()).ok()
    }
    pub unsafe fn ClassificationLastReportPathWithoutExtension(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClassificationLastReportPathWithoutExtension)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ClassificationLastError(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClassificationLastError)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ClassificationRunningStatus(&self) -> windows_core::Result<FsrmReportRunningStatus> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClassificationRunningStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumPropertyDefinitions(&self, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumPropertyDefinitions)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePropertyDefinition(&self) -> windows_core::Result<IFsrmPropertyDefinition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePropertyDefinition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPropertyDefinition<P0>(&self, propertyname: P0) -> windows_core::Result<IFsrmPropertyDefinition>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPropertyDefinition)(windows_core::Interface::as_raw(self), propertyname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumRules(&self, ruletype: FsrmRuleType, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumRules)(windows_core::Interface::as_raw(self), ruletype, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateRule(&self, ruletype: FsrmRuleType) -> windows_core::Result<IFsrmRule> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRule)(windows_core::Interface::as_raw(self), ruletype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRule<P0>(&self, rulename: P0, ruletype: FsrmRuleType) -> windows_core::Result<IFsrmRule>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRule)(windows_core::Interface::as_raw(self), rulename.param().abi(), ruletype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumModuleDefinitions(&self, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumModuleDefinitions)(windows_core::Interface::as_raw(self), moduletype, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateModuleDefinition(&self, moduletype: FsrmPipelineModuleType) -> windows_core::Result<IFsrmPipelineModuleDefinition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateModuleDefinition)(windows_core::Interface::as_raw(self), moduletype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetModuleDefinition<P0>(&self, modulename: P0, moduletype: FsrmPipelineModuleType) -> windows_core::Result<IFsrmPipelineModuleDefinition>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetModuleDefinition)(windows_core::Interface::as_raw(self), modulename.param().abi(), moduletype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RunClassification<P0>(&self, context: FsrmReportGenerationContext, reserved: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).RunClassification)(windows_core::Interface::as_raw(self), context, reserved.param().abi()).ok()
    }
    pub unsafe fn WaitForClassificationCompletion(&self, waitseconds: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WaitForClassificationCompletion)(windows_core::Interface::as_raw(self), waitseconds, &mut result__).map(|| result__)
    }
    pub unsafe fn CancelClassification(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CancelClassification)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumFileProperties<P0>(&self, filepath: P0, options: FsrmGetFilePropertyOptions) -> windows_core::Result<IFsrmCollection>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumFileProperties)(windows_core::Interface::as_raw(self), filepath.param().abi(), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFileProperty<P0, P1>(&self, filepath: P0, propertyname: P1, options: FsrmGetFilePropertyOptions) -> windows_core::Result<IFsrmProperty>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFileProperty)(windows_core::Interface::as_raw(self), filepath.param().abi(), propertyname.param().abi(), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFileProperty<P0, P1, P2>(&self, filepath: P0, propertyname: P1, propertyvalue: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetFileProperty)(windows_core::Interface::as_raw(self), filepath.param().abi(), propertyname.param().abi(), propertyvalue.param().abi()).ok()
    }
    pub unsafe fn ClearFileProperty<P0, P1>(&self, filepath: P0, property: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ClearFileProperty)(windows_core::Interface::as_raw(self), filepath.param().abi(), property.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmClassificationManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ClassificationReportFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ClassificationReportFormats: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetClassificationReportFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetClassificationReportFormats: usize,
    pub Logging: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetLogging: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ClassificationReportMailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetClassificationReportMailTo: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ClassificationReportEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetClassificationReportEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ClassificationLastReportPathWithoutExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ClassificationLastError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ClassificationRunningStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmReportRunningStatus) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumPropertyDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumPropertyDefinitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePropertyDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePropertyDefinition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPropertyDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPropertyDefinition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumRules: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmRuleType, FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumRules: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRule: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmRuleType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRule: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRule: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, FsrmRuleType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRule: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumModuleDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmPipelineModuleType, FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumModuleDefinitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateModuleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmPipelineModuleType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateModuleDefinition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetModuleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, FsrmPipelineModuleType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetModuleDefinition: usize,
    pub RunClassification: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmReportGenerationContext, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub WaitForClassificationCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub CancelClassification: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumFileProperties: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, FsrmGetFilePropertyOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumFileProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFileProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, FsrmGetFilePropertyOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFileProperty: usize,
    pub SetFileProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ClearFileProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmClassificationManager2, IFsrmClassificationManager2_Vtbl, 0x0004c1c9_127e_4765_ba07_6a3147bca112);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmClassificationManager2 {
    type Target = IFsrmClassificationManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmClassificationManager2, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmClassificationManager);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmClassificationManager2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ClassifyFiles(&self, filepaths: *const super::super::System::Com::SAFEARRAY, propertynames: *const super::super::System::Com::SAFEARRAY, propertyvalues: *const super::super::System::Com::SAFEARRAY, options: FsrmGetFilePropertyOptions) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClassifyFiles)(windows_core::Interface::as_raw(self), filepaths, propertynames, propertyvalues, options).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmClassificationManager2_Vtbl {
    pub base__: IFsrmClassificationManager_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ClassifyFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY, *const super::super::System::Com::SAFEARRAY, *const super::super::System::Com::SAFEARRAY, FsrmGetFilePropertyOptions) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ClassifyFiles: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmClassificationRule, IFsrmClassificationRule_Vtbl, 0xafc052c2_5315_45ab_841b_c6db0e120148);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmClassificationRule {
    type Target = IFsrmRule;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmClassificationRule, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmRule);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmClassificationRule {
    pub unsafe fn ExecutionOption(&self) -> windows_core::Result<FsrmExecutionOption> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExecutionOption)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetExecutionOption(&self, executionoption: FsrmExecutionOption) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetExecutionOption)(windows_core::Interface::as_raw(self), executionoption).ok()
    }
    pub unsafe fn PropertyAffected(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PropertyAffected)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPropertyAffected<P0>(&self, property: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPropertyAffected)(windows_core::Interface::as_raw(self), property.param().abi()).ok()
    }
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
pub struct IFsrmClassificationRule_Vtbl {
    pub base__: IFsrmRule_Vtbl,
    pub ExecutionOption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmExecutionOption) -> windows_core::HRESULT,
    pub SetExecutionOption: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmExecutionOption) -> windows_core::HRESULT,
    pub PropertyAffected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPropertyAffected: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmClassifierModuleDefinition, IFsrmClassifierModuleDefinition_Vtbl, 0xbb36ea26_6318_4b8c_8592_f72dd602e7a5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmClassifierModuleDefinition {
    type Target = IFsrmPipelineModuleDefinition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmClassifierModuleDefinition, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmPipelineModuleDefinition);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmClassifierModuleDefinition {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PropertiesAffected(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PropertiesAffected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPropertiesAffected(&self, propertiesaffected: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPropertiesAffected)(windows_core::Interface::as_raw(self), propertiesaffected).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PropertiesUsed(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PropertiesUsed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPropertiesUsed(&self, propertiesused: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPropertiesUsed)(windows_core::Interface::as_raw(self), propertiesused).ok()
    }
    pub unsafe fn NeedsExplicitValue(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NeedsExplicitValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetNeedsExplicitValue<P0>(&self, needsexplicitvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetNeedsExplicitValue)(windows_core::Interface::as_raw(self), needsexplicitvalue.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmClassifierModuleDefinition_Vtbl {
    pub base__: IFsrmPipelineModuleDefinition_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub PropertiesAffected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PropertiesAffected: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPropertiesAffected: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPropertiesAffected: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PropertiesUsed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PropertiesUsed: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPropertiesUsed: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPropertiesUsed: usize,
    pub NeedsExplicitValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetNeedsExplicitValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmClassifierModuleImplementation, IFsrmClassifierModuleImplementation_Vtbl, 0x4c968fc6_6edb_4051_9c18_73b7291ae106);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmClassifierModuleImplementation {
    type Target = IFsrmPipelineModuleImplementation;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmClassifierModuleImplementation, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmPipelineModuleImplementation);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmClassifierModuleImplementation {
    pub unsafe fn LastModified(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastModified)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UseRulesAndDefinitions<P0, P1>(&self, rules: P0, propertydefinitions: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFsrmCollection>,
        P1: windows_core::Param<IFsrmCollection>,
    {
        (windows_core::Interface::vtable(self).UseRulesAndDefinitions)(windows_core::Interface::as_raw(self), rules.param().abi(), propertydefinitions.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnBeginFile<P0>(&self, propertybag: P0, arrayruleids: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFsrmPropertyBag>,
    {
        (windows_core::Interface::vtable(self).OnBeginFile)(windows_core::Interface::as_raw(self), propertybag.param().abi(), arrayruleids).ok()
    }
    pub unsafe fn DoesPropertyValueApply<P0, P1>(&self, property: P0, value: P1, applyvalue: *mut super::super::Foundation::VARIANT_BOOL, idrule: windows_core::GUID, idpropdef: windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DoesPropertyValueApply)(windows_core::Interface::as_raw(self), property.param().abi(), value.param().abi(), applyvalue, core::mem::transmute(idrule), core::mem::transmute(idpropdef)).ok()
    }
    pub unsafe fn GetPropertyValueToApply<P0>(&self, property: P0, value: *mut windows_core::BSTR, idrule: windows_core::GUID, idpropdef: windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetPropertyValueToApply)(windows_core::Interface::as_raw(self), property.param().abi(), core::mem::transmute(value), core::mem::transmute(idrule), core::mem::transmute(idpropdef)).ok()
    }
    pub unsafe fn OnEndFile(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnEndFile)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmClassifierModuleImplementation_Vtbl {
    pub base__: IFsrmPipelineModuleImplementation_Vtbl,
    pub LastModified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub UseRulesAndDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UseRulesAndDefinitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnBeginFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnBeginFile: usize,
    pub DoesPropertyValueApply: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::VARIANT_BOOL, windows_core::GUID, windows_core::GUID) -> windows_core::HRESULT,
    pub GetPropertyValueToApply: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, windows_core::GUID, windows_core::GUID) -> windows_core::HRESULT,
    pub OnEndFile: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmCollection, IFsrmCollection_Vtbl, 0xf76fbf3b_8ddd_4b42_b05a_cb1c3ff1fee8);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmCollection {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmCollection, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmCollection {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn State(&self) -> windows_core::Result<FsrmCollectionState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WaitForCompletion)(windows_core::Interface::as_raw(self), waitseconds, &mut result__).map(|| result__)
    }
    pub unsafe fn GetById(&self, id: windows_core::GUID) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetById)(windows_core::Interface::as_raw(self), core::mem::transmute(id), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmCollectionState) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WaitForCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetById: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmCommittableCollection, IFsrmCommittableCollection_Vtbl, 0x96deb3b5_8b91_4a2a_9d93_80a35d8aa847);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmCommittableCollection {
    type Target = IFsrmMutableCollection;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmCommittableCollection, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmCollection, IFsrmMutableCollection);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmCommittableCollection {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Commit(&self, options: FsrmCommitOptions) -> windows_core::Result<IFsrmCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmCommittableCollection_Vtbl {
    pub base__: IFsrmMutableCollection_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmCommitOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Commit: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmDerivedObjectsResult, IFsrmDerivedObjectsResult_Vtbl, 0x39322a2d_38ee_4d0d_8095_421a80849a82);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmDerivedObjectsResult {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmDerivedObjectsResult, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmDerivedObjectsResult {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DerivedObjects(&self) -> windows_core::Result<IFsrmCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DerivedObjects)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Results(&self) -> windows_core::Result<IFsrmCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Results)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmDerivedObjectsResult_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub DerivedObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DerivedObjects: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Results: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Results: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmExportImport, IFsrmExportImport_Vtbl, 0xefcb0ab1_16c4_4a79_812c_725614c3306b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmExportImport {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmExportImport, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmExportImport {
    pub unsafe fn ExportFileGroups<P0, P1>(&self, filepath: P0, filegroupnamessafearray: *const windows_core::VARIANT, remotehost: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ExportFileGroups)(windows_core::Interface::as_raw(self), filepath.param().abi(), core::mem::transmute(filegroupnamessafearray), remotehost.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ImportFileGroups<P0, P1>(&self, filepath: P0, filegroupnamessafearray: *const windows_core::VARIANT, remotehost: P1) -> windows_core::Result<IFsrmCommittableCollection>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ImportFileGroups)(windows_core::Interface::as_raw(self), filepath.param().abi(), core::mem::transmute(filegroupnamessafearray), remotehost.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ExportFileScreenTemplates<P0, P1>(&self, filepath: P0, templatenamessafearray: *const windows_core::VARIANT, remotehost: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ExportFileScreenTemplates)(windows_core::Interface::as_raw(self), filepath.param().abi(), core::mem::transmute(templatenamessafearray), remotehost.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ImportFileScreenTemplates<P0, P1>(&self, filepath: P0, templatenamessafearray: *const windows_core::VARIANT, remotehost: P1) -> windows_core::Result<IFsrmCommittableCollection>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ImportFileScreenTemplates)(windows_core::Interface::as_raw(self), filepath.param().abi(), core::mem::transmute(templatenamessafearray), remotehost.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ExportQuotaTemplates<P0, P1>(&self, filepath: P0, templatenamessafearray: *const windows_core::VARIANT, remotehost: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ExportQuotaTemplates)(windows_core::Interface::as_raw(self), filepath.param().abi(), core::mem::transmute(templatenamessafearray), remotehost.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ImportQuotaTemplates<P0, P1>(&self, filepath: P0, templatenamessafearray: *const windows_core::VARIANT, remotehost: P1) -> windows_core::Result<IFsrmCommittableCollection>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ImportQuotaTemplates)(windows_core::Interface::as_raw(self), filepath.param().abi(), core::mem::transmute(templatenamessafearray), remotehost.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmExportImport_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ExportFileGroups: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ImportFileGroups: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ImportFileGroups: usize,
    pub ExportFileScreenTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ImportFileScreenTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ImportFileScreenTemplates: usize,
    pub ExportQuotaTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ImportQuotaTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ImportQuotaTemplates: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileCondition, IFsrmFileCondition_Vtbl, 0x70684ffc_691a_4a1a_b922_97752e138cc1);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileCondition {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileCondition, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileCondition {
    pub unsafe fn Type(&self) -> windows_core::Result<FsrmFileConditionType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileCondition_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmFileConditionType) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileConditionProperty, IFsrmFileConditionProperty_Vtbl, 0x81926775_b981_4479_988f_da171d627360);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileConditionProperty {
    type Target = IFsrmFileCondition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileConditionProperty, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmFileCondition);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileConditionProperty {
    pub unsafe fn PropertyName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PropertyName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPropertyName<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPropertyName)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
    pub unsafe fn PropertyId(&self) -> windows_core::Result<FsrmFileSystemPropertyId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PropertyId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPropertyId(&self, newval: FsrmFileSystemPropertyId) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPropertyId)(windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn Operator(&self) -> windows_core::Result<FsrmPropertyConditionType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Operator)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetOperator(&self, newval: FsrmPropertyConditionType) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOperator)(windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn ValueType(&self) -> windows_core::Result<FsrmPropertyValueType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ValueType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetValueType(&self, newval: FsrmPropertyValueType) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetValueType)(windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetValue<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileConditionProperty_Vtbl {
    pub base__: IFsrmFileCondition_Vtbl,
    pub PropertyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPropertyName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PropertyId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmFileSystemPropertyId) -> windows_core::HRESULT,
    pub SetPropertyId: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmFileSystemPropertyId) -> windows_core::HRESULT,
    pub Operator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmPropertyConditionType) -> windows_core::HRESULT,
    pub SetOperator: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmPropertyConditionType) -> windows_core::HRESULT,
    pub ValueType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmPropertyValueType) -> windows_core::HRESULT,
    pub SetValueType: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmPropertyValueType) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileGroup, IFsrmFileGroup_Vtbl, 0x8dd04909_0e34_4d55_afaa_89e1f1a1bbb9);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileGroup {
    type Target = IFsrmObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileGroup, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileGroup {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Members(&self) -> windows_core::Result<IFsrmMutableCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Members)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMembers<P0>(&self, members: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFsrmMutableCollection>,
    {
        (windows_core::Interface::vtable(self).SetMembers)(windows_core::Interface::as_raw(self), members.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NonMembers(&self) -> windows_core::Result<IFsrmMutableCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NonMembers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNonMembers<P0>(&self, nonmembers: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFsrmMutableCollection>,
    {
        (windows_core::Interface::vtable(self).SetNonMembers)(windows_core::Interface::as_raw(self), nonmembers.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileGroup_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Members: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Members: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetMembers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetMembers: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NonMembers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NonMembers: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetNonMembers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetNonMembers: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileGroupImported, IFsrmFileGroupImported_Vtbl, 0xad55f10b_5f11_4be7_94ef_d9ee2e470ded);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileGroupImported {
    type Target = IFsrmFileGroup;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileGroupImported, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmFileGroup);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileGroupImported {
    pub unsafe fn OverwriteOnCommit(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OverwriteOnCommit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetOverwriteOnCommit<P0>(&self, overwrite: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetOverwriteOnCommit)(windows_core::Interface::as_raw(self), overwrite.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileGroupImported_Vtbl {
    pub base__: IFsrmFileGroup_Vtbl,
    pub OverwriteOnCommit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetOverwriteOnCommit: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileGroupManager, IFsrmFileGroupManager_Vtbl, 0x426677d5_018c_485c_8a51_20b86d00bdc4);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileGroupManager {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileGroupManager, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileGroupManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateFileGroup(&self) -> windows_core::Result<IFsrmFileGroup> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFileGroup)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFileGroup<P0>(&self, name: P0) -> windows_core::Result<IFsrmFileGroup>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFileGroup)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumFileGroups(&self, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCommittableCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumFileGroups)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ExportFileGroups(&self, filegroupnamesarray: *const windows_core::VARIANT) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExportFileGroups)(windows_core::Interface::as_raw(self), core::mem::transmute(filegroupnamesarray), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ImportFileGroups<P0>(&self, serializedfilegroups: P0, filegroupnamesarray: *const windows_core::VARIANT) -> windows_core::Result<IFsrmCommittableCollection>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ImportFileGroups)(windows_core::Interface::as_raw(self), serializedfilegroups.param().abi(), core::mem::transmute(filegroupnamesarray), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileGroupManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFileGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFileGroup: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFileGroup: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFileGroup: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumFileGroups: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumFileGroups: usize,
    pub ExportFileGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ImportFileGroups: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ImportFileGroups: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileManagementJob, IFsrmFileManagementJob_Vtbl, 0x0770687e_9f36_4d6f_8778_599d188461c9);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileManagementJob {
    type Target = IFsrmObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileManagementJob, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileManagementJob {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NamespaceRoots(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NamespaceRoots)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNamespaceRoots)(windows_core::Interface::as_raw(self), namespaceroots).ok()
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
    pub unsafe fn OperationType(&self) -> windows_core::Result<FsrmFileManagementType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OperationType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetOperationType(&self, operationtype: FsrmFileManagementType) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOperationType)(windows_core::Interface::as_raw(self), operationtype).ok()
    }
    pub unsafe fn ExpirationDirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExpirationDirectory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetExpirationDirectory<P0>(&self, expirationdirectory: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetExpirationDirectory)(windows_core::Interface::as_raw(self), expirationdirectory.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CustomAction(&self) -> windows_core::Result<IFsrmActionCommand> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CustomAction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Notifications(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Notifications)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Logging(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Logging)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLogging(&self, loggingflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLogging)(windows_core::Interface::as_raw(self), loggingflags).ok()
    }
    pub unsafe fn ReportEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReportEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetReportEnabled<P0>(&self, reportenabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetReportEnabled)(windows_core::Interface::as_raw(self), reportenabled.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Formats(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Formats)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFormats)(windows_core::Interface::as_raw(self), formats).ok()
    }
    pub unsafe fn MailTo(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MailTo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMailTo<P0>(&self, mailto: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMailTo)(windows_core::Interface::as_raw(self), mailto.param().abi()).ok()
    }
    pub unsafe fn DaysSinceFileCreated(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DaysSinceFileCreated)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDaysSinceFileCreated(&self, dayssincecreation: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDaysSinceFileCreated)(windows_core::Interface::as_raw(self), dayssincecreation).ok()
    }
    pub unsafe fn DaysSinceFileLastAccessed(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DaysSinceFileLastAccessed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDaysSinceFileLastAccessed(&self, dayssinceaccess: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDaysSinceFileLastAccessed)(windows_core::Interface::as_raw(self), dayssinceaccess).ok()
    }
    pub unsafe fn DaysSinceFileLastModified(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DaysSinceFileLastModified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDaysSinceFileLastModified(&self, dayssincemodify: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDaysSinceFileLastModified)(windows_core::Interface::as_raw(self), dayssincemodify).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PropertyConditions(&self) -> windows_core::Result<IFsrmCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PropertyConditions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FromDate(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FromDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFromDate(&self, fromdate: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFromDate)(windows_core::Interface::as_raw(self), fromdate).ok()
    }
    pub unsafe fn Task(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Task)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTask<P0>(&self, taskname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTask)(windows_core::Interface::as_raw(self), taskname.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Parameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetParameters)(windows_core::Interface::as_raw(self), parameters).ok()
    }
    pub unsafe fn RunningStatus(&self) -> windows_core::Result<FsrmReportRunningStatus> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RunningStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LastError(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastError)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LastReportPathWithoutExtension(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastReportPathWithoutExtension)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LastRun(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastRun)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn FileNamePattern(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FileNamePattern)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFileNamePattern<P0>(&self, filenamepattern: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetFileNamePattern)(windows_core::Interface::as_raw(self), filenamepattern.param().abi()).ok()
    }
    pub unsafe fn Run(&self, context: FsrmReportGenerationContext) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Run)(windows_core::Interface::as_raw(self), context).ok()
    }
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WaitForCompletion)(windows_core::Interface::as_raw(self), waitseconds, &mut result__).map(|| result__)
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddNotification(&self, days: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddNotification)(windows_core::Interface::as_raw(self), days).ok()
    }
    pub unsafe fn DeleteNotification(&self, days: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteNotification)(windows_core::Interface::as_raw(self), days).ok()
    }
    pub unsafe fn ModifyNotification(&self, days: i32, newdays: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ModifyNotification)(windows_core::Interface::as_raw(self), days, newdays).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateNotificationAction(&self, days: i32, actiontype: FsrmActionType) -> windows_core::Result<IFsrmAction> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateNotificationAction)(windows_core::Interface::as_raw(self), days, actiontype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumNotificationActions(&self, days: i32) -> windows_core::Result<IFsrmCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumNotificationActions)(windows_core::Interface::as_raw(self), days, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePropertyCondition<P0>(&self, name: P0) -> windows_core::Result<IFsrmPropertyCondition>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePropertyCondition)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateCustomAction(&self) -> windows_core::Result<IFsrmActionCommand> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCustomAction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileManagementJob_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub NamespaceRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NamespaceRoots: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetNamespaceRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetNamespaceRoots: usize,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub OperationType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmFileManagementType) -> windows_core::HRESULT,
    pub SetOperationType: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmFileManagementType) -> windows_core::HRESULT,
    pub ExpirationDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetExpirationDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CustomAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CustomAction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Notifications: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Notifications: usize,
    pub Logging: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetLogging: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ReportEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetReportEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Formats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Formats: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetFormats: usize,
    pub MailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetMailTo: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DaysSinceFileCreated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDaysSinceFileCreated: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DaysSinceFileLastAccessed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDaysSinceFileLastAccessed: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DaysSinceFileLastModified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDaysSinceFileLastModified: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PropertyConditions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PropertyConditions: usize,
    pub FromDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetFromDate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Task: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTask: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parameters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetParameters: usize,
    pub RunningStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmReportRunningStatus) -> windows_core::HRESULT,
    pub LastError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LastReportPathWithoutExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LastRun: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub FileNamePattern: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetFileNamePattern: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Run: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmReportGenerationContext) -> windows_core::HRESULT,
    pub WaitForCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddNotification: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DeleteNotification: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ModifyNotification: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateNotificationAction: unsafe extern "system" fn(*mut core::ffi::c_void, i32, FsrmActionType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateNotificationAction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumNotificationActions: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumNotificationActions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePropertyCondition: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePropertyCondition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateCustomAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateCustomAction: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileManagementJobManager, IFsrmFileManagementJobManager_Vtbl, 0xee321ecb_d95e_48e9_907c_c7685a013235);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileManagementJobManager {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileManagementJobManager, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileManagementJobManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariables(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ActionVariables)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariableDescriptions(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ActionVariableDescriptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumFileManagementJobs(&self, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumFileManagementJobs)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateFileManagementJob(&self) -> windows_core::Result<IFsrmFileManagementJob> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFileManagementJob)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFileManagementJob<P0>(&self, name: P0) -> windows_core::Result<IFsrmFileManagementJob>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFileManagementJob)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileManagementJobManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ActionVariables: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActionVariables: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ActionVariableDescriptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActionVariableDescriptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumFileManagementJobs: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumFileManagementJobs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFileManagementJob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFileManagementJob: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFileManagementJob: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFileManagementJob: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileScreen, IFsrmFileScreen_Vtbl, 0x5f6325d3_ce88_4733_84c1_2d6aefc5ea07);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileScreen {
    type Target = IFsrmFileScreenBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileScreen, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmFileScreenBase);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreen {
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SourceTemplateName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SourceTemplateName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn MatchesSourceTemplate(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MatchesSourceTemplate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn UserSid(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserSid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UserAccount(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserAccount)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ApplyTemplate<P0>(&self, filescreentemplatename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ApplyTemplate)(windows_core::Interface::as_raw(self), filescreentemplatename.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileScreen_Vtbl {
    pub base__: IFsrmFileScreenBase_Vtbl,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SourceTemplateName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MatchesSourceTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub UserSid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UserAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ApplyTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileScreenBase, IFsrmFileScreenBase_Vtbl, 0xf3637e80_5b22_4a2b_a637_bbb642b41cfc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileScreenBase {
    type Target = IFsrmObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileScreenBase, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreenBase {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BlockedFileGroups(&self) -> windows_core::Result<IFsrmMutableCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BlockedFileGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBlockedFileGroups<P0>(&self, blocklist: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFsrmMutableCollection>,
    {
        (windows_core::Interface::vtable(self).SetBlockedFileGroups)(windows_core::Interface::as_raw(self), blocklist.param().abi()).ok()
    }
    pub unsafe fn FileScreenFlags(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FileScreenFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFileScreenFlags(&self, filescreenflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFileScreenFlags)(windows_core::Interface::as_raw(self), filescreenflags).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAction(&self, actiontype: FsrmActionType) -> windows_core::Result<IFsrmAction> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateAction)(windows_core::Interface::as_raw(self), actiontype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumActions(&self) -> windows_core::Result<IFsrmCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumActions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileScreenBase_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub BlockedFileGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BlockedFileGroups: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetBlockedFileGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetBlockedFileGroups: usize,
    pub FileScreenFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetFileScreenFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateAction: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmActionType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateAction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumActions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumActions: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileScreenException, IFsrmFileScreenException_Vtbl, 0xbee7ce02_df77_4515_9389_78f01c5afc1a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileScreenException {
    type Target = IFsrmObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileScreenException, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreenException {
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AllowedFileGroups(&self) -> windows_core::Result<IFsrmMutableCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllowedFileGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetAllowedFileGroups<P0>(&self, allowlist: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFsrmMutableCollection>,
    {
        (windows_core::Interface::vtable(self).SetAllowedFileGroups)(windows_core::Interface::as_raw(self), allowlist.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileScreenException_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AllowedFileGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AllowedFileGroups: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetAllowedFileGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetAllowedFileGroups: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileScreenManager, IFsrmFileScreenManager_Vtbl, 0xff4fa04e_5a94_4bda_a3a0_d5b4d3c52eba);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileScreenManager {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileScreenManager, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreenManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariables(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ActionVariables)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariableDescriptions(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ActionVariableDescriptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateFileScreen<P0>(&self, path: P0) -> windows_core::Result<IFsrmFileScreen>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFileScreen)(windows_core::Interface::as_raw(self), path.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFileScreen<P0>(&self, path: P0) -> windows_core::Result<IFsrmFileScreen>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFileScreen)(windows_core::Interface::as_raw(self), path.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumFileScreens<P0>(&self, path: P0, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCommittableCollection>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumFileScreens)(windows_core::Interface::as_raw(self), path.param().abi(), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateFileScreenException<P0>(&self, path: P0) -> windows_core::Result<IFsrmFileScreenException>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFileScreenException)(windows_core::Interface::as_raw(self), path.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFileScreenException<P0>(&self, path: P0) -> windows_core::Result<IFsrmFileScreenException>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFileScreenException)(windows_core::Interface::as_raw(self), path.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumFileScreenExceptions<P0>(&self, path: P0, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCommittableCollection>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumFileScreenExceptions)(windows_core::Interface::as_raw(self), path.param().abi(), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateFileScreenCollection(&self) -> windows_core::Result<IFsrmCommittableCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFileScreenCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileScreenManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ActionVariables: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActionVariables: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ActionVariableDescriptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActionVariableDescriptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFileScreen: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFileScreen: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFileScreen: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFileScreen: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumFileScreens: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumFileScreens: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFileScreenException: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFileScreenException: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFileScreenException: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFileScreenException: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumFileScreenExceptions: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumFileScreenExceptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFileScreenCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFileScreenCollection: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileScreenTemplate, IFsrmFileScreenTemplate_Vtbl, 0x205bebf8_dd93_452a_95a6_32b566b35828);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileScreenTemplate {
    type Target = IFsrmFileScreenBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileScreenTemplate, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmFileScreenBase);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreenTemplate {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    pub unsafe fn CopyTemplate<P0>(&self, filescreentemplatename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).CopyTemplate)(windows_core::Interface::as_raw(self), filescreentemplatename.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> windows_core::Result<IFsrmDerivedObjectsResult> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CommitAndUpdateDerived)(windows_core::Interface::as_raw(self), commitoptions, applyoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileScreenTemplate_Vtbl {
    pub base__: IFsrmFileScreenBase_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CopyTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CommitAndUpdateDerived: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmCommitOptions, FsrmTemplateApplyOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommitAndUpdateDerived: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileScreenTemplateImported, IFsrmFileScreenTemplateImported_Vtbl, 0xe1010359_3e5d_4ecd_9fe4_ef48622fdf30);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileScreenTemplateImported {
    type Target = IFsrmFileScreenTemplate;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileScreenTemplateImported, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmFileScreenBase, IFsrmFileScreenTemplate);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreenTemplateImported {
    pub unsafe fn OverwriteOnCommit(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OverwriteOnCommit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetOverwriteOnCommit<P0>(&self, overwrite: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetOverwriteOnCommit)(windows_core::Interface::as_raw(self), overwrite.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileScreenTemplateImported_Vtbl {
    pub base__: IFsrmFileScreenTemplate_Vtbl,
    pub OverwriteOnCommit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetOverwriteOnCommit: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmFileScreenTemplateManager, IFsrmFileScreenTemplateManager_Vtbl, 0xcfe36cba_1949_4e74_a14f_f1d580ceaf13);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmFileScreenTemplateManager {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmFileScreenTemplateManager, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreenTemplateManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateTemplate(&self) -> windows_core::Result<IFsrmFileScreenTemplate> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTemplate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTemplate<P0>(&self, name: P0) -> windows_core::Result<IFsrmFileScreenTemplate>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTemplate)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumTemplates(&self, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCommittableCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumTemplates)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ExportTemplates(&self, filescreentemplatenamesarray: *const windows_core::VARIANT) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExportTemplates)(windows_core::Interface::as_raw(self), core::mem::transmute(filescreentemplatenamesarray), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ImportTemplates<P0>(&self, serializedfilescreentemplates: P0, filescreentemplatenamesarray: *const windows_core::VARIANT) -> windows_core::Result<IFsrmCommittableCollection>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ImportTemplates)(windows_core::Interface::as_raw(self), serializedfilescreentemplates.param().abi(), core::mem::transmute(filescreentemplatenamesarray), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileScreenTemplateManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumTemplates: usize,
    pub ExportTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ImportTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ImportTemplates: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmMutableCollection, IFsrmMutableCollection_Vtbl, 0x1bb617b8_3886_49dc_af82_a6c90fa35dda);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmMutableCollection {
    type Target = IFsrmCollection;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmMutableCollection, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmCollection);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmMutableCollection {
    pub unsafe fn Add<P0>(&self, item: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), item.param().abi()).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn RemoveById(&self, id: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveById)(windows_core::Interface::as_raw(self), core::mem::transmute(id)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> windows_core::Result<IFsrmMutableCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmMutableCollection_Vtbl {
    pub base__: IFsrmCollection_Vtbl,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub RemoveById: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmObject, IFsrmObject_Vtbl, 0x22bcef93_4a3f_4183_89f9_2f8b8a628aee);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmObject {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmObject, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmObject {
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, description: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), description.param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmObject_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmPathMapper, IFsrmPathMapper_Vtbl, 0x6f4dbfff_6920_4821_a6c3_b7e94c1fd60c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmPathMapper {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmPathMapper, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPathMapper {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSharePathsForLocalPath<P0>(&self, localpath: P0) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSharePathsForLocalPath)(windows_core::Interface::as_raw(self), localpath.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmPathMapper_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSharePathsForLocalPath: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSharePathsForLocalPath: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmPipelineModuleConnector, IFsrmPipelineModuleConnector_Vtbl, 0xc16014f3_9aa1_46b3_b0a7_ab146eb205f2);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmPipelineModuleConnector {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmPipelineModuleConnector, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPipelineModuleConnector {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ModuleImplementation(&self) -> windows_core::Result<IFsrmPipelineModuleImplementation> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ModuleImplementation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ModuleName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ModuleName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn HostingUserAccount(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HostingUserAccount)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn HostingProcessPid(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HostingProcessPid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Bind<P0, P1>(&self, moduledefinition: P0, moduleimplementation: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFsrmPipelineModuleDefinition>,
        P1: windows_core::Param<IFsrmPipelineModuleImplementation>,
    {
        (windows_core::Interface::vtable(self).Bind)(windows_core::Interface::as_raw(self), moduledefinition.param().abi(), moduleimplementation.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmPipelineModuleConnector_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ModuleImplementation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ModuleImplementation: usize,
    pub ModuleName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub HostingUserAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub HostingProcessPid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Bind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Bind: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmPipelineModuleDefinition, IFsrmPipelineModuleDefinition_Vtbl, 0x515c1277_2c81_440e_8fcf_367921ed4f59);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmPipelineModuleDefinition {
    type Target = IFsrmObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmPipelineModuleDefinition, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPipelineModuleDefinition {
    pub unsafe fn ModuleClsid(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ModuleClsid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetModuleClsid<P0>(&self, moduleclsid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetModuleClsid)(windows_core::Interface::as_raw(self), moduleclsid.param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    pub unsafe fn Company(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Company)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCompany<P0>(&self, company: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetCompany)(windows_core::Interface::as_raw(self), company.param().abi()).ok()
    }
    pub unsafe fn Version(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Version)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetVersion<P0>(&self, version: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetVersion)(windows_core::Interface::as_raw(self), version.param().abi()).ok()
    }
    pub unsafe fn ModuleType(&self) -> windows_core::Result<FsrmPipelineModuleType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ModuleType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    pub unsafe fn NeedsFileContent(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NeedsFileContent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetNeedsFileContent<P0>(&self, needsfilecontent: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetNeedsFileContent)(windows_core::Interface::as_raw(self), needsfilecontent.param().abi()).ok()
    }
    pub unsafe fn Account(&self) -> windows_core::Result<FsrmAccountType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Account)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAccount(&self, retrievalaccount: FsrmAccountType) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAccount)(windows_core::Interface::as_raw(self), retrievalaccount).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedExtensions(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SupportedExtensions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSupportedExtensions(&self, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSupportedExtensions)(windows_core::Interface::as_raw(self), supportedextensions).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Parameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetParameters)(windows_core::Interface::as_raw(self), parameters).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmPipelineModuleDefinition_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub ModuleClsid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetModuleClsid: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Company: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetCompany: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ModuleType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmPipelineModuleType) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub NeedsFileContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetNeedsFileContent: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Account: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmAccountType) -> windows_core::HRESULT,
    pub SetAccount: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmAccountType) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedExtensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSupportedExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSupportedExtensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parameters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetParameters: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmPipelineModuleImplementation, IFsrmPipelineModuleImplementation_Vtbl, 0xb7907906_2b02_4cb5_84a9_fdf54613d6cd);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmPipelineModuleImplementation {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmPipelineModuleImplementation, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPipelineModuleImplementation {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnLoad<P0>(&self, moduledefinition: P0) -> windows_core::Result<IFsrmPipelineModuleConnector>
    where
        P0: windows_core::Param<IFsrmPipelineModuleDefinition>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnLoad)(windows_core::Interface::as_raw(self), moduledefinition.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn OnUnload(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnUnload)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmPipelineModuleImplementation_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnLoad: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnLoad: usize,
    pub OnUnload: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmProperty, IFsrmProperty_Vtbl, 0x4a73fee4_4102_4fcc_9ffb_38614f9ee768);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmProperty {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmProperty, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmProperty {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Sources(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Sources)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PropertyFlags(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PropertyFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmProperty_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Sources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Sources: usize,
    pub PropertyFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmPropertyBag, IFsrmPropertyBag_Vtbl, 0x774589d1_d300_4f7a_9a24_f7b766800250);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmPropertyBag {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmPropertyBag, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPropertyBag {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RelativePath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RelativePath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn VolumeName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VolumeName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RelativeNamespaceRoot(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RelativeNamespaceRoot)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn VolumeIndex(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VolumeIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn FileId(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FileId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ParentDirectoryId(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ParentDirectoryId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Size(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SizeAllocated(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SizeAllocated)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreationTime(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreationTime)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LastAccessTime(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastAccessTime)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LastModificationTime(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastModificationTime)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Attributes(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Attributes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn OwnerSid(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OwnerSid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FilePropertyNames(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FilePropertyNames)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Messages(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Messages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PropertyBagFlags(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PropertyBagFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFileProperty<P0>(&self, name: P0) -> windows_core::Result<IFsrmProperty>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFileProperty)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFileProperty<P0, P1>(&self, name: P0, value: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetFileProperty)(windows_core::Interface::as_raw(self), name.param().abi(), value.param().abi()).ok()
    }
    pub unsafe fn AddMessage<P0>(&self, message: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AddMessage)(windows_core::Interface::as_raw(self), message.param().abi()).ok()
    }
    pub unsafe fn GetFileStreamInterface(&self, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFileStreamInterface)(windows_core::Interface::as_raw(self), accessmode, interfacetype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmPropertyBag_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RelativePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub VolumeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RelativeNamespaceRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub VolumeIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FileId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub ParentDirectoryId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SizeAllocated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub CreationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub LastAccessTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub LastModificationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Attributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub OwnerSid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub FilePropertyNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FilePropertyNames: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Messages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Messages: usize,
    pub PropertyBagFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFileProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFileProperty: usize,
    pub SetFileProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AddMessage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetFileStreamInterface: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmFileStreamingMode, FsrmFileStreamingInterfaceType, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmPropertyBag2, IFsrmPropertyBag2_Vtbl, 0x0e46bdbd_2402_4fed_9c30_9266e6eb2cc9);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmPropertyBag2 {
    type Target = IFsrmPropertyBag;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmPropertyBag2, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmPropertyBag);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPropertyBag2 {
    pub unsafe fn GetFieldValue(&self, field: FsrmPropertyBagField) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFieldValue)(windows_core::Interface::as_raw(self), field, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUntrustedInFileProperties(&self) -> windows_core::Result<IFsrmCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUntrustedInFileProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmPropertyBag2_Vtbl {
    pub base__: IFsrmPropertyBag_Vtbl,
    pub GetFieldValue: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmPropertyBagField, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetUntrustedInFileProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetUntrustedInFileProperties: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmPropertyCondition, IFsrmPropertyCondition_Vtbl, 0x326af66f_2ac0_4f68_bf8c_4759f054fa29);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmPropertyCondition {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmPropertyCondition, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPropertyCondition {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    pub unsafe fn Type(&self) -> windows_core::Result<FsrmPropertyConditionType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetType(&self, r#type: FsrmPropertyConditionType) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetType)(windows_core::Interface::as_raw(self), r#type).ok()
    }
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmPropertyCondition_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmPropertyConditionType) -> windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmPropertyConditionType) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmPropertyDefinition, IFsrmPropertyDefinition_Vtbl, 0xede0150f_e9a3_419c_877c_01fe5d24c5d3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmPropertyDefinition {
    type Target = IFsrmObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmPropertyDefinition, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPropertyDefinition {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    pub unsafe fn Type(&self) -> windows_core::Result<FsrmPropertyDefinitionType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetType(&self, r#type: FsrmPropertyDefinitionType) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetType)(windows_core::Interface::as_raw(self), r#type).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PossibleValues(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PossibleValues)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPossibleValues(&self, possiblevalues: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPossibleValues)(windows_core::Interface::as_raw(self), possiblevalues).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ValueDescriptions(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ValueDescriptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetValueDescriptions(&self, valuedescriptions: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetValueDescriptions)(windows_core::Interface::as_raw(self), valuedescriptions).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Parameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetParameters)(windows_core::Interface::as_raw(self), parameters).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmPropertyDefinition_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmPropertyDefinitionType) -> windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmPropertyDefinitionType) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PossibleValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PossibleValues: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPossibleValues: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPossibleValues: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ValueDescriptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ValueDescriptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetValueDescriptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetValueDescriptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parameters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetParameters: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmPropertyDefinition2, IFsrmPropertyDefinition2_Vtbl, 0x47782152_d16c_4229_b4e1_0ddfe308b9f6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmPropertyDefinition2 {
    type Target = IFsrmPropertyDefinition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmPropertyDefinition2, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmPropertyDefinition);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPropertyDefinition2 {
    pub unsafe fn PropertyDefinitionFlags(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PropertyDefinitionFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDisplayName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    pub unsafe fn AppliesTo(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AppliesTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ValueDefinitions(&self) -> windows_core::Result<IFsrmCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ValueDefinitions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmPropertyDefinition2_Vtbl {
    pub base__: IFsrmPropertyDefinition_Vtbl,
    pub PropertyDefinitionFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AppliesTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ValueDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ValueDefinitions: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmPropertyDefinitionValue, IFsrmPropertyDefinitionValue_Vtbl, 0xe946d148_bd67_4178_8e22_1c44925ed710);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmPropertyDefinitionValue {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmPropertyDefinitionValue, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPropertyDefinitionValue {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UniqueID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UniqueID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmPropertyDefinitionValue_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UniqueID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmQuota, IFsrmQuota_Vtbl, 0x377f739d_9647_4b8e_97d2_5ffce6d759cd);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmQuota {
    type Target = IFsrmQuotaObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmQuota, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmQuotaBase, IFsrmQuotaObject);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuota {
    pub unsafe fn QuotaUsed(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QuotaUsed)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QuotaPeakUsage(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QuotaPeakUsage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QuotaPeakUsageTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QuotaPeakUsageTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ResetPeakUsage(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResetPeakUsage)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RefreshUsageProperties(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RefreshUsageProperties)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmQuota_Vtbl {
    pub base__: IFsrmQuotaObject_Vtbl,
    pub QuotaUsed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub QuotaPeakUsage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub QuotaPeakUsageTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ResetPeakUsage: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RefreshUsageProperties: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmQuotaBase, IFsrmQuotaBase_Vtbl, 0x1568a795_3924_4118_b74b_68d8f0fa5daf);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmQuotaBase {
    type Target = IFsrmObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmQuotaBase, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaBase {
    pub unsafe fn QuotaLimit(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QuotaLimit)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetQuotaLimit<P0>(&self, quotalimit: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetQuotaLimit)(windows_core::Interface::as_raw(self), quotalimit.param().abi()).ok()
    }
    pub unsafe fn QuotaFlags(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QuotaFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetQuotaFlags)(windows_core::Interface::as_raw(self), quotaflags).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Thresholds(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Thresholds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AddThreshold(&self, threshold: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddThreshold)(windows_core::Interface::as_raw(self), threshold).ok()
    }
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteThreshold)(windows_core::Interface::as_raw(self), threshold).ok()
    }
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ModifyThreshold)(windows_core::Interface::as_raw(self), threshold, newthreshold).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> windows_core::Result<IFsrmAction> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateThresholdAction)(windows_core::Interface::as_raw(self), threshold, actiontype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> windows_core::Result<IFsrmCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumThresholdActions)(windows_core::Interface::as_raw(self), threshold, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmQuotaBase_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub QuotaLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetQuotaLimit: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub QuotaFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetQuotaFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Thresholds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Thresholds: usize,
    pub AddThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DeleteThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ModifyThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateThresholdAction: unsafe extern "system" fn(*mut core::ffi::c_void, i32, FsrmActionType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateThresholdAction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumThresholdActions: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumThresholdActions: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmQuotaManager, IFsrmQuotaManager_Vtbl, 0x8bb68c7d_19d8_4ffb_809e_be4fc1734014);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmQuotaManager {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmQuotaManager, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariables(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ActionVariables)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariableDescriptions(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ActionVariableDescriptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateQuota<P0>(&self, path: P0) -> windows_core::Result<IFsrmQuota>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateQuota)(windows_core::Interface::as_raw(self), path.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAutoApplyQuota<P0, P1>(&self, quotatemplatename: P0, path: P1) -> windows_core::Result<IFsrmAutoApplyQuota>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateAutoApplyQuota)(windows_core::Interface::as_raw(self), quotatemplatename.param().abi(), path.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetQuota<P0>(&self, path: P0) -> windows_core::Result<IFsrmQuota>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetQuota)(windows_core::Interface::as_raw(self), path.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAutoApplyQuota<P0>(&self, path: P0) -> windows_core::Result<IFsrmAutoApplyQuota>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAutoApplyQuota)(windows_core::Interface::as_raw(self), path.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRestrictiveQuota<P0>(&self, path: P0) -> windows_core::Result<IFsrmQuota>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRestrictiveQuota)(windows_core::Interface::as_raw(self), path.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumQuotas<P0>(&self, path: P0, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCommittableCollection>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumQuotas)(windows_core::Interface::as_raw(self), path.param().abi(), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumAutoApplyQuotas<P0>(&self, path: P0, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCommittableCollection>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumAutoApplyQuotas)(windows_core::Interface::as_raw(self), path.param().abi(), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumEffectiveQuotas<P0>(&self, path: P0, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCommittableCollection>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumEffectiveQuotas)(windows_core::Interface::as_raw(self), path.param().abi(), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Scan<P0>(&self, strpath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Scan)(windows_core::Interface::as_raw(self), strpath.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateQuotaCollection(&self) -> windows_core::Result<IFsrmCommittableCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateQuotaCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmQuotaManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ActionVariables: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActionVariables: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ActionVariableDescriptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActionVariableDescriptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateQuota: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateQuota: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateAutoApplyQuota: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateAutoApplyQuota: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetQuota: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetQuota: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAutoApplyQuota: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAutoApplyQuota: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRestrictiveQuota: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRestrictiveQuota: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumQuotas: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumQuotas: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumAutoApplyQuotas: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumAutoApplyQuotas: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumEffectiveQuotas: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumEffectiveQuotas: usize,
    pub Scan: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateQuotaCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateQuotaCollection: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmQuotaManagerEx, IFsrmQuotaManagerEx_Vtbl, 0x4846cb01_d430_494f_abb4_b1054999fb09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmQuotaManagerEx {
    type Target = IFsrmQuotaManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmQuotaManagerEx, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmQuotaManager);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaManagerEx {
    pub unsafe fn IsAffectedByQuota<P0>(&self, path: P0, options: FsrmEnumOptions) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsAffectedByQuota)(windows_core::Interface::as_raw(self), path.param().abi(), options, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmQuotaManagerEx_Vtbl {
    pub base__: IFsrmQuotaManager_Vtbl,
    pub IsAffectedByQuota: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, FsrmEnumOptions, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmQuotaObject, IFsrmQuotaObject_Vtbl, 0x42dc3511_61d5_48ae_b6dc_59fc00c0a8d6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmQuotaObject {
    type Target = IFsrmQuotaBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmQuotaObject, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmQuotaBase);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaObject {
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UserSid(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserSid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UserAccount(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserAccount)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SourceTemplateName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SourceTemplateName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn MatchesSourceTemplate(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MatchesSourceTemplate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ApplyTemplate<P0>(&self, quotatemplatename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ApplyTemplate)(windows_core::Interface::as_raw(self), quotatemplatename.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmQuotaObject_Vtbl {
    pub base__: IFsrmQuotaBase_Vtbl,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UserSid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UserAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SourceTemplateName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MatchesSourceTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ApplyTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmQuotaTemplate, IFsrmQuotaTemplate_Vtbl, 0xa2efab31_295e_46bb_b976_e86d58b52e8b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmQuotaTemplate {
    type Target = IFsrmQuotaBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmQuotaTemplate, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmQuotaBase);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaTemplate {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    pub unsafe fn CopyTemplate<P0>(&self, quotatemplatename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).CopyTemplate)(windows_core::Interface::as_raw(self), quotatemplatename.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> windows_core::Result<IFsrmDerivedObjectsResult> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CommitAndUpdateDerived)(windows_core::Interface::as_raw(self), commitoptions, applyoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmQuotaTemplate_Vtbl {
    pub base__: IFsrmQuotaBase_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CopyTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CommitAndUpdateDerived: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmCommitOptions, FsrmTemplateApplyOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommitAndUpdateDerived: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmQuotaTemplateImported, IFsrmQuotaTemplateImported_Vtbl, 0x9a2bf113_a329_44cc_809a_5c00fce8da40);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmQuotaTemplateImported {
    type Target = IFsrmQuotaTemplate;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmQuotaTemplateImported, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmQuotaBase, IFsrmQuotaTemplate);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaTemplateImported {
    pub unsafe fn OverwriteOnCommit(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OverwriteOnCommit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetOverwriteOnCommit<P0>(&self, overwrite: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetOverwriteOnCommit)(windows_core::Interface::as_raw(self), overwrite.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmQuotaTemplateImported_Vtbl {
    pub base__: IFsrmQuotaTemplate_Vtbl,
    pub OverwriteOnCommit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetOverwriteOnCommit: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmQuotaTemplateManager, IFsrmQuotaTemplateManager_Vtbl, 0x4173ac41_172d_4d52_963c_fdc7e415f717);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmQuotaTemplateManager {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmQuotaTemplateManager, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaTemplateManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateTemplate(&self) -> windows_core::Result<IFsrmQuotaTemplate> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTemplate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTemplate<P0>(&self, name: P0) -> windows_core::Result<IFsrmQuotaTemplate>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTemplate)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumTemplates(&self, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCommittableCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumTemplates)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ExportTemplates(&self, quotatemplatenamesarray: *const windows_core::VARIANT) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExportTemplates)(windows_core::Interface::as_raw(self), core::mem::transmute(quotatemplatenamesarray), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ImportTemplates<P0>(&self, serializedquotatemplates: P0, quotatemplatenamesarray: *const windows_core::VARIANT) -> windows_core::Result<IFsrmCommittableCollection>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ImportTemplates)(windows_core::Interface::as_raw(self), serializedquotatemplates.param().abi(), core::mem::transmute(quotatemplatenamesarray), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmQuotaTemplateManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumTemplates: usize,
    pub ExportTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ImportTemplates: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ImportTemplates: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmReport, IFsrmReport_Vtbl, 0xd8cc81d9_46b8_4fa4_bfa5_4aa9dec9b638);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmReport {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmReport, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmReport {
    pub unsafe fn Type(&self) -> windows_core::Result<FsrmReportType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, description: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), description.param().abi()).ok()
    }
    pub unsafe fn LastGeneratedFileNamePrefix(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastGeneratedFileNamePrefix)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFilter(&self, filter: FsrmReportFilter) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFilter)(windows_core::Interface::as_raw(self), filter, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFilter<P0>(&self, filter: FsrmReportFilter, filtervalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetFilter)(windows_core::Interface::as_raw(self), filter, filtervalue.param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmReport_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmReportType) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LastGeneratedFileNamePrefix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetFilter: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmReportFilter, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetFilter: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmReportFilter, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmReportJob, IFsrmReportJob_Vtbl, 0x38e87280_715c_4c7d_a280_ea1651a19fef);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmReportJob {
    type Target = IFsrmObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmReportJob, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmReportJob {
    pub unsafe fn Task(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Task)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTask<P0>(&self, taskname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTask)(windows_core::Interface::as_raw(self), taskname.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NamespaceRoots(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NamespaceRoots)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNamespaceRoots)(windows_core::Interface::as_raw(self), namespaceroots).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Formats(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Formats)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFormats)(windows_core::Interface::as_raw(self), formats).ok()
    }
    pub unsafe fn MailTo(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MailTo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMailTo<P0>(&self, mailto: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMailTo)(windows_core::Interface::as_raw(self), mailto.param().abi()).ok()
    }
    pub unsafe fn RunningStatus(&self) -> windows_core::Result<FsrmReportRunningStatus> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RunningStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LastRun(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastRun)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LastError(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastError)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LastGeneratedInDirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastGeneratedInDirectory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumReports(&self) -> windows_core::Result<IFsrmCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumReports)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateReport(&self, reporttype: FsrmReportType) -> windows_core::Result<IFsrmReport> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateReport)(windows_core::Interface::as_raw(self), reporttype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Run(&self, context: FsrmReportGenerationContext) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Run)(windows_core::Interface::as_raw(self), context).ok()
    }
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WaitForCompletion)(windows_core::Interface::as_raw(self), waitseconds, &mut result__).map(|| result__)
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmReportJob_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub Task: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTask: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub NamespaceRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NamespaceRoots: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetNamespaceRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetNamespaceRoots: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Formats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Formats: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetFormats: usize,
    pub MailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetMailTo: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RunningStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmReportRunningStatus) -> windows_core::HRESULT,
    pub LastRun: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub LastError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LastGeneratedInDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumReports: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumReports: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateReport: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmReportType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateReport: usize,
    pub Run: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmReportGenerationContext) -> windows_core::HRESULT,
    pub WaitForCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmReportManager, IFsrmReportManager_Vtbl, 0x27b899fe_6ffa_4481_a184_d3daade8a02b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmReportManager {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmReportManager, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmReportManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumReportJobs(&self, options: FsrmEnumOptions) -> windows_core::Result<IFsrmCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumReportJobs)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateReportJob(&self) -> windows_core::Result<IFsrmReportJob> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateReportJob)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetReportJob<P0>(&self, taskname: P0) -> windows_core::Result<IFsrmReportJob>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetReportJob)(windows_core::Interface::as_raw(self), taskname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetOutputDirectory(&self, context: FsrmReportGenerationContext) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOutputDirectory)(windows_core::Interface::as_raw(self), context, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOutputDirectory<P0>(&self, context: FsrmReportGenerationContext, path: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetOutputDirectory)(windows_core::Interface::as_raw(self), context, path.param().abi()).ok()
    }
    pub unsafe fn IsFilterValidForReportType(&self, reporttype: FsrmReportType, filter: FsrmReportFilter) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsFilterValidForReportType)(windows_core::Interface::as_raw(self), reporttype, filter, &mut result__).map(|| result__)
    }
    pub unsafe fn GetDefaultFilter(&self, reporttype: FsrmReportType, filter: FsrmReportFilter) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDefaultFilter)(windows_core::Interface::as_raw(self), reporttype, filter, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDefaultFilter<P0>(&self, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetDefaultFilter)(windows_core::Interface::as_raw(self), reporttype, filter, filtervalue.param().abi()).ok()
    }
    pub unsafe fn GetReportSizeLimit(&self, limit: FsrmReportLimit) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetReportSizeLimit)(windows_core::Interface::as_raw(self), limit, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetReportSizeLimit<P0>(&self, limit: FsrmReportLimit, limitvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetReportSizeLimit)(windows_core::Interface::as_raw(self), limit, limitvalue.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmReportManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumReportJobs: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumReportJobs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateReportJob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateReportJob: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetReportJob: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetReportJob: usize,
    pub GetOutputDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmReportGenerationContext, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetOutputDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmReportGenerationContext, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsFilterValidForReportType: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmReportType, FsrmReportFilter, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetDefaultFilter: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmReportType, FsrmReportFilter, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetDefaultFilter: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmReportType, FsrmReportFilter, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetReportSizeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmReportLimit, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetReportSizeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmReportLimit, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmReportScheduler, IFsrmReportScheduler_Vtbl, 0x6879caf9_6617_4484_8719_71c3d8645f94);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmReportScheduler {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmReportScheduler, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmReportScheduler {
    pub unsafe fn VerifyNamespaces(&self, namespacessafearray: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).VerifyNamespaces)(windows_core::Interface::as_raw(self), core::mem::transmute(namespacessafearray)).ok()
    }
    pub unsafe fn CreateScheduleTask<P0, P1>(&self, taskname: P0, namespacessafearray: *const windows_core::VARIANT, serializedtask: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).CreateScheduleTask)(windows_core::Interface::as_raw(self), taskname.param().abi(), core::mem::transmute(namespacessafearray), serializedtask.param().abi()).ok()
    }
    pub unsafe fn ModifyScheduleTask<P0, P1>(&self, taskname: P0, namespacessafearray: *const windows_core::VARIANT, serializedtask: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ModifyScheduleTask)(windows_core::Interface::as_raw(self), taskname.param().abi(), core::mem::transmute(namespacessafearray), serializedtask.param().abi()).ok()
    }
    pub unsafe fn DeleteScheduleTask<P0>(&self, taskname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteScheduleTask)(windows_core::Interface::as_raw(self), taskname.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmReportScheduler_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub VerifyNamespaces: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub CreateScheduleTask: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ModifyScheduleTask: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DeleteScheduleTask: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmRule, IFsrmRule_Vtbl, 0xcb0df960_16f5_4495_9079_3f9360d831df);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmRule {
    type Target = IFsrmObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmRule, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmRule {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    pub unsafe fn RuleType(&self) -> windows_core::Result<FsrmRuleType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RuleType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ModuleDefinitionName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ModuleDefinitionName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetModuleDefinitionName<P0>(&self, moduledefinitionname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetModuleDefinitionName)(windows_core::Interface::as_raw(self), moduledefinitionname.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NamespaceRoots(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NamespaceRoots)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNamespaceRoots)(windows_core::Interface::as_raw(self), namespaceroots).ok()
    }
    pub unsafe fn RuleFlags(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RuleFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetRuleFlags(&self, ruleflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRuleFlags)(windows_core::Interface::as_raw(self), ruleflags).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Parameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetParameters)(windows_core::Interface::as_raw(self), parameters).ok()
    }
    pub unsafe fn LastModified(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastModified)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmRule_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RuleType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmRuleType) -> windows_core::HRESULT,
    pub ModuleDefinitionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetModuleDefinitionName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub NamespaceRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NamespaceRoots: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetNamespaceRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetNamespaceRoots: usize,
    pub RuleFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRuleFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parameters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetParameters: usize,
    pub LastModified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmSetting, IFsrmSetting_Vtbl, 0xf411d4fd_14be_4260_8c40_03b7c95e608a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmSetting {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmSetting, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmSetting {
    pub unsafe fn SmtpServer(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SmtpServer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSmtpServer<P0>(&self, smtpserver: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSmtpServer)(windows_core::Interface::as_raw(self), smtpserver.param().abi()).ok()
    }
    pub unsafe fn MailFrom(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MailFrom)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMailFrom<P0>(&self, mailfrom: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMailFrom)(windows_core::Interface::as_raw(self), mailfrom.param().abi()).ok()
    }
    pub unsafe fn AdminEmail(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AdminEmail)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetAdminEmail<P0>(&self, adminemail: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetAdminEmail)(windows_core::Interface::as_raw(self), adminemail.param().abi()).ok()
    }
    pub unsafe fn DisableCommandLine(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisableCommandLine)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDisableCommandLine<P0>(&self, disablecommandline: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetDisableCommandLine)(windows_core::Interface::as_raw(self), disablecommandline.param().abi()).ok()
    }
    pub unsafe fn EnableScreeningAudit(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnableScreeningAudit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnableScreeningAudit<P0>(&self, enablescreeningaudit: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnableScreeningAudit)(windows_core::Interface::as_raw(self), enablescreeningaudit.param().abi()).ok()
    }
    pub unsafe fn EmailTest<P0>(&self, mailto: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).EmailTest)(windows_core::Interface::as_raw(self), mailto.param().abi()).ok()
    }
    pub unsafe fn SetActionRunLimitInterval(&self, actiontype: FsrmActionType, delaytimeminutes: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetActionRunLimitInterval)(windows_core::Interface::as_raw(self), actiontype, delaytimeminutes).ok()
    }
    pub unsafe fn GetActionRunLimitInterval(&self, actiontype: FsrmActionType) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetActionRunLimitInterval)(windows_core::Interface::as_raw(self), actiontype, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmSetting_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SmtpServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSmtpServer: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MailFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetMailFrom: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AdminEmail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetAdminEmail: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DisableCommandLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetDisableCommandLine: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub EnableScreeningAudit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnableScreeningAudit: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub EmailTest: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetActionRunLimitInterval: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmActionType, i32) -> windows_core::HRESULT,
    pub GetActionRunLimitInterval: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmActionType, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmStorageModuleDefinition, IFsrmStorageModuleDefinition_Vtbl, 0x15a81350_497d_4aba_80e9_d4dbcc5521fe);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmStorageModuleDefinition {
    type Target = IFsrmPipelineModuleDefinition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmStorageModuleDefinition, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmPipelineModuleDefinition);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmStorageModuleDefinition {
    pub unsafe fn Capabilities(&self) -> windows_core::Result<FsrmStorageModuleCaps> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Capabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCapabilities(&self, capabilities: FsrmStorageModuleCaps) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCapabilities)(windows_core::Interface::as_raw(self), capabilities).ok()
    }
    pub unsafe fn StorageType(&self) -> windows_core::Result<FsrmStorageModuleType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StorageType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStorageType(&self, storagetype: FsrmStorageModuleType) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStorageType)(windows_core::Interface::as_raw(self), storagetype).ok()
    }
    pub unsafe fn UpdatesFileContent(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UpdatesFileContent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetUpdatesFileContent<P0>(&self, updatesfilecontent: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetUpdatesFileContent)(windows_core::Interface::as_raw(self), updatesfilecontent.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmStorageModuleDefinition_Vtbl {
    pub base__: IFsrmPipelineModuleDefinition_Vtbl,
    pub Capabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmStorageModuleCaps) -> windows_core::HRESULT,
    pub SetCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmStorageModuleCaps) -> windows_core::HRESULT,
    pub StorageType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsrmStorageModuleType) -> windows_core::HRESULT,
    pub SetStorageType: unsafe extern "system" fn(*mut core::ffi::c_void, FsrmStorageModuleType) -> windows_core::HRESULT,
    pub UpdatesFileContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetUpdatesFileContent: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFsrmStorageModuleImplementation, IFsrmStorageModuleImplementation_Vtbl, 0x0af4a0da_895a_4e50_8712_a96724bcec64);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFsrmStorageModuleImplementation {
    type Target = IFsrmPipelineModuleImplementation;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFsrmStorageModuleImplementation, windows_core::IUnknown, super::super::System::Com::IDispatch, IFsrmPipelineModuleImplementation);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmStorageModuleImplementation {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UseDefinitions<P0>(&self, propertydefinitions: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFsrmCollection>,
    {
        (windows_core::Interface::vtable(self).UseDefinitions)(windows_core::Interface::as_raw(self), propertydefinitions.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadProperties<P0>(&self, propertybag: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFsrmPropertyBag>,
    {
        (windows_core::Interface::vtable(self).LoadProperties)(windows_core::Interface::as_raw(self), propertybag.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SaveProperties<P0>(&self, propertybag: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFsrmPropertyBag>,
    {
        (windows_core::Interface::vtable(self).SaveProperties)(windows_core::Interface::as_raw(self), propertybag.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmStorageModuleImplementation_Vtbl {
    pub base__: IFsrmPipelineModuleImplementation_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub UseDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UseDefinitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub LoadProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoadProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SaveProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SaveProperties: usize,
}
pub const AdrClientDisplayFlags_AllowEmailRequests: AdrClientDisplayFlags = AdrClientDisplayFlags(1i32);
pub const AdrClientDisplayFlags_ShowDeviceTroubleshooting: AdrClientDisplayFlags = AdrClientDisplayFlags(2i32);
pub const AdrClientErrorType_AccessDenied: AdrClientErrorType = AdrClientErrorType(1i32);
pub const AdrClientErrorType_FileNotFound: AdrClientErrorType = AdrClientErrorType(2i32);
pub const AdrClientErrorType_Unknown: AdrClientErrorType = AdrClientErrorType(0i32);
pub const AdrClientFlags_FailForLocalPaths: AdrClientFlags = AdrClientFlags(1i32);
pub const AdrClientFlags_FailIfNotDomainJoined: AdrClientFlags = AdrClientFlags(4i32);
pub const AdrClientFlags_FailIfNotSupportedByServer: AdrClientFlags = AdrClientFlags(2i32);
pub const AdrClientFlags_None: AdrClientFlags = AdrClientFlags(0i32);
pub const AdrEmailFlags_GenerateEventLog: AdrEmailFlags = AdrEmailFlags(16i32);
pub const AdrEmailFlags_IncludeDeviceClaims: AdrEmailFlags = AdrEmailFlags(4i32);
pub const AdrEmailFlags_IncludeUserInfo: AdrEmailFlags = AdrEmailFlags(8i32);
pub const AdrEmailFlags_PutAdminOnToLine: AdrEmailFlags = AdrEmailFlags(2i32);
pub const AdrEmailFlags_PutDataOwnerOnToLine: AdrEmailFlags = AdrEmailFlags(1i32);
pub const FSRM_DISPID_FEATURE_CLASSIFICATION: u32 = 83886080u32;
pub const FSRM_DISPID_FEATURE_FILESCREEN: u32 = 50331648u32;
pub const FSRM_DISPID_FEATURE_GENERAL: u32 = 16777216u32;
pub const FSRM_DISPID_FEATURE_MASK: u32 = 251658240u32;
pub const FSRM_DISPID_FEATURE_PIPELINE: u32 = 100663296u32;
pub const FSRM_DISPID_FEATURE_QUOTA: u32 = 33554432u32;
pub const FSRM_DISPID_FEATURE_REPORTS: u32 = 67108864u32;
pub const FSRM_DISPID_INTERFACE_A_MASK: u32 = 15728640u32;
pub const FSRM_DISPID_INTERFACE_B_MASK: u32 = 983040u32;
pub const FSRM_DISPID_INTERFACE_C_MASK: u32 = 61440u32;
pub const FSRM_DISPID_INTERFACE_D_MASK: u32 = 3840u32;
pub const FSRM_DISPID_IS_PROPERTY: u32 = 128u32;
pub const FSRM_DISPID_METHOD_NUM_MASK: u32 = 127u32;
pub const FSRM_E_ADR_MAX_EMAILS_SENT: windows_core::HRESULT = windows_core::HRESULT(0x8004537E_u32 as _);
pub const FSRM_E_ADR_NOT_DOMAIN_JOINED: windows_core::HRESULT = windows_core::HRESULT(0x80045392_u32 as _);
pub const FSRM_E_ADR_PATH_IS_LOCAL: windows_core::HRESULT = windows_core::HRESULT(0x80045391_u32 as _);
pub const FSRM_E_ADR_SRV_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80045390_u32 as _);
pub const FSRM_E_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x80045303_u32 as _);
pub const FSRM_E_AUTO_QUOTA: windows_core::HRESULT = windows_core::HRESULT(0x4531B_u32 as _);
pub const FSRM_E_CACHE_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80045345_u32 as _);
pub const FSRM_E_CACHE_MODULE_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x80045346_u32 as _);
pub const FSRM_E_CANNOT_AGGREGATE: windows_core::HRESULT = windows_core::HRESULT(0x80045337_u32 as _);
pub const FSRM_E_CANNOT_ALLOW_REPARSE_POINT_TAG: windows_core::HRESULT = windows_core::HRESULT(0x80045356_u32 as _);
pub const FSRM_E_CANNOT_CHANGE_PROPERTY_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x8004533B_u32 as _);
pub const FSRM_E_CANNOT_CREATE_TEMP_COPY: windows_core::HRESULT = windows_core::HRESULT(0x8004537C_u32 as _);
pub const FSRM_E_CANNOT_DELETE_SYSTEM_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x80045379_u32 as _);
pub const FSRM_E_CANNOT_REMOVE_READONLY: windows_core::HRESULT = windows_core::HRESULT(0x80045393_u32 as _);
pub const FSRM_E_CANNOT_RENAME_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x8004533A_u32 as _);
pub const FSRM_E_CANNOT_STORE_PROPERTIES: windows_core::HRESULT = windows_core::HRESULT(0x80045355_u32 as _);
pub const FSRM_E_CANNOT_USE_DELETED_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x80045371_u32 as _);
pub const FSRM_E_CANNOT_USE_DEPRECATED_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x8004536F_u32 as _);
pub const FSRM_E_CLASSIFICATION_ALREADY_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x8004533D_u32 as _);
pub const FSRM_E_CLASSIFICATION_CANCELED: windows_core::HRESULT = windows_core::HRESULT(0x80045373_u32 as _);
pub const FSRM_E_CLASSIFICATION_NOT_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x8004533E_u32 as _);
pub const FSRM_E_CLASSIFICATION_PARTIAL_BATCH: windows_core::HRESULT = windows_core::HRESULT(0x80045378_u32 as _);
pub const FSRM_E_CLASSIFICATION_SCAN_FAIL: windows_core::HRESULT = windows_core::HRESULT(0x8004536C_u32 as _);
pub const FSRM_E_CLASSIFICATION_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80045377_u32 as _);
pub const FSRM_E_CLUSTER_NOT_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x8004532E_u32 as _);
pub const FSRM_E_CSC_PATH_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80045396_u32 as _);
pub const FSRM_E_DIFFERENT_CLUSTER_GROUP: windows_core::HRESULT = windows_core::HRESULT(0x80045331_u32 as _);
pub const FSRM_E_DRIVER_NOT_READY: windows_core::HRESULT = windows_core::HRESULT(0x80045313_u32 as _);
pub const FSRM_E_DUPLICATE_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80045310_u32 as _);
pub const FSRM_E_EMAIL_NOT_SENT: windows_core::HRESULT = windows_core::HRESULT(0x8004531C_u32 as _);
pub const FSRM_E_ENUM_PROPERTIES_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80045353_u32 as _);
pub const FSRM_E_ERROR_NOT_ENABLED: windows_core::HRESULT = windows_core::HRESULT(0x8004537B_u32 as _);
pub const FSRM_E_EXPIRATION_PATH_NOT_WRITEABLE: windows_core::HRESULT = windows_core::HRESULT(0x80045397_u32 as _);
pub const FSRM_E_EXPIRATION_PATH_TOO_LONG: windows_core::HRESULT = windows_core::HRESULT(0x80045398_u32 as _);
pub const FSRM_E_EXPIRATION_VOLUME_NOT_NTFS: windows_core::HRESULT = windows_core::HRESULT(0x80045399_u32 as _);
pub const FSRM_E_FAIL_BATCH: windows_core::HRESULT = windows_core::HRESULT(0x80045309_u32 as _);
pub const FSRM_E_FILE_ENCRYPTED: windows_core::HRESULT = windows_core::HRESULT(0x80045364_u32 as _);
pub const FSRM_E_FILE_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x8004537A_u32 as _);
pub const FSRM_E_FILE_MANAGEMENT_ACTION_GET_EXITCODE_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80045368_u32 as _);
pub const FSRM_E_FILE_MANAGEMENT_ACTION_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80045367_u32 as _);
pub const FSRM_E_FILE_MANAGEMENT_EXPIRATION_DIR_IN_SCOPE: windows_core::HRESULT = windows_core::HRESULT(0x80045347_u32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x80045348_u32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_ALREADY_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x8004533F_u32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_CUSTOM: windows_core::HRESULT = windows_core::HRESULT(0x80045341_u32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_DEPRECATED: windows_core::HRESULT = windows_core::HRESULT(0x8004539A_u32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_EXPIRATION: windows_core::HRESULT = windows_core::HRESULT(0x80045340_u32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_INVALID_CONTINUOUS_CONFIG: windows_core::HRESULT = windows_core::HRESULT(0x80045394_u32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_MAX_FILE_CONDITIONS: windows_core::HRESULT = windows_core::HRESULT(0x8004536E_u32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_NOTIFICATION: windows_core::HRESULT = windows_core::HRESULT(0x80045342_u32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_NOT_LEGACY_ACCESSIBLE: windows_core::HRESULT = windows_core::HRESULT(0x8004536D_u32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_RMS: windows_core::HRESULT = windows_core::HRESULT(0x80045388_u32 as _);
pub const FSRM_E_FILE_OPEN_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80045343_u32 as _);
pub const FSRM_E_FILE_SYSTEM_CORRUPT: windows_core::HRESULT = windows_core::HRESULT(0x8004531F_u32 as _);
pub const FSRM_E_INCOMPATIBLE_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x80045363_u32 as _);
pub const FSRM_E_INPROC_MODULE_BLOCKED: windows_core::HRESULT = windows_core::HRESULT(0x80045352_u32 as _);
pub const FSRM_E_INSECURE_PATH: windows_core::HRESULT = windows_core::HRESULT(0x80045317_u32 as _);
pub const FSRM_E_INSUFFICIENT_DISK: windows_core::HRESULT = windows_core::HRESULT(0x80045314_u32 as _);
pub const FSRM_E_INVALID_AD_CLAIM: windows_core::HRESULT = windows_core::HRESULT(0x80045372_u32 as _);
pub const FSRM_E_INVALID_COMBINATION: windows_core::HRESULT = windows_core::HRESULT(0x8004530F_u32 as _);
pub const FSRM_E_INVALID_DATASCREEN_DEFINITION: windows_core::HRESULT = windows_core::HRESULT(0x80045324_u32 as _);
pub const FSRM_E_INVALID_EMAIL_ADDRESS: windows_core::HRESULT = windows_core::HRESULT(0x8004531E_u32 as _);
pub const FSRM_E_INVALID_FILEGROUP_DEFINITION: windows_core::HRESULT = windows_core::HRESULT(0x80045321_u32 as _);
pub const FSRM_E_INVALID_FILENAME: windows_core::HRESULT = windows_core::HRESULT(0x8004532A_u32 as _);
pub const FSRM_E_INVALID_FOLDER_PROPERTY_STORE: windows_core::HRESULT = windows_core::HRESULT(0x80045374_u32 as _);
pub const FSRM_E_INVALID_IMPORT_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x8004530B_u32 as _);
pub const FSRM_E_INVALID_LIMIT: windows_core::HRESULT = windows_core::HRESULT(0x80045307_u32 as _);
pub const FSRM_E_INVALID_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80045308_u32 as _);
pub const FSRM_E_INVALID_PATH: windows_core::HRESULT = windows_core::HRESULT(0x80045306_u32 as _);
pub const FSRM_E_INVALID_REPORT_DESC: windows_core::HRESULT = windows_core::HRESULT(0x80045329_u32 as _);
pub const FSRM_E_INVALID_REPORT_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x80045328_u32 as _);
pub const FSRM_E_INVALID_SCHEDULER_ARGUMENT: windows_core::HRESULT = windows_core::HRESULT(0x80045302_u32 as _);
pub const FSRM_E_INVALID_SMTP_SERVER: windows_core::HRESULT = windows_core::HRESULT(0x80045318_u32 as _);
pub const FSRM_E_INVALID_TEXT: windows_core::HRESULT = windows_core::HRESULT(0x8004530A_u32 as _);
pub const FSRM_E_INVALID_USER: windows_core::HRESULT = windows_core::HRESULT(0x80045305_u32 as _);
pub const FSRM_E_LAST_ACCESS_UPDATE_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80045350_u32 as _);
pub const FSRM_E_LEGACY_SCHEDULE: windows_core::HRESULT = windows_core::HRESULT(0x80045395_u32 as _);
pub const FSRM_E_LOADING_DISABLED_MODULE: windows_core::HRESULT = windows_core::HRESULT(0x80045336_u32 as _);
pub const FSRM_E_LONG_CMDLINE: windows_core::HRESULT = windows_core::HRESULT(0x80045320_u32 as _);
pub const FSRM_E_MAX_PROPERTY_DEFINITIONS: windows_core::HRESULT = windows_core::HRESULT(0x8004533C_u32 as _);
pub const FSRM_E_MESSAGE_LIMIT_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x80045338_u32 as _);
pub const FSRM_E_MODULE_INITIALIZATION: windows_core::HRESULT = windows_core::HRESULT(0x8004536A_u32 as _);
pub const FSRM_E_MODULE_INVALID_PARAM: windows_core::HRESULT = windows_core::HRESULT(0x80045369_u32 as _);
pub const FSRM_E_MODULE_SESSION_INITIALIZATION: windows_core::HRESULT = windows_core::HRESULT(0x8004536B_u32 as _);
pub const FSRM_E_MODULE_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x8004539B_u32 as _);
pub const FSRM_E_NOT_CLUSTER_VOLUME: windows_core::HRESULT = windows_core::HRESULT(0x80045330_u32 as _);
pub const FSRM_E_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80045301_u32 as _);
pub const FSRM_E_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80045311_u32 as _);
pub const FSRM_E_NO_EMAIL_ADDRESS: windows_core::HRESULT = windows_core::HRESULT(0x8004537D_u32 as _);
pub const FSRM_E_NO_PROPERTY_VALUE: windows_core::HRESULT = windows_core::HRESULT(0x80045351_u32 as _);
pub const FSRM_E_OBJECT_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x80045339_u32 as _);
pub const FSRM_E_OUT_OF_RANGE: windows_core::HRESULT = windows_core::HRESULT(0x8004530D_u32 as _);
pub const FSRM_E_PARTIAL_CLASSIFICATION_PROPERTY_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80045357_u32 as _);
pub const FSRM_E_PATH_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80045304_u32 as _);
pub const FSRM_E_PATH_NOT_IN_NAMESPACE: windows_core::HRESULT = windows_core::HRESULT(0x8004537F_u32 as _);
pub const FSRM_E_PERSIST_PROPERTIES_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80045365_u32 as _);
pub const FSRM_E_PERSIST_PROPERTIES_FAILED_ENCRYPTED: windows_core::HRESULT = windows_core::HRESULT(0x8004535A_u32 as _);
pub const FSRM_E_PROPERTY_DELETED: windows_core::HRESULT = windows_core::HRESULT(0x80045349_u32 as _);
pub const FSRM_E_PROPERTY_MUST_APPLY_TO_FILES: windows_core::HRESULT = windows_core::HRESULT(0x80045376_u32 as _);
pub const FSRM_E_PROPERTY_MUST_APPLY_TO_FOLDERS: windows_core::HRESULT = windows_core::HRESULT(0x80045384_u32 as _);
pub const FSRM_E_PROPERTY_MUST_BE_GLOBAL: windows_core::HRESULT = windows_core::HRESULT(0x80045386_u32 as _);
pub const FSRM_E_PROPERTY_MUST_BE_SECURE: windows_core::HRESULT = windows_core::HRESULT(0x80045385_u32 as _);
pub const FSRM_E_REBUILDING_FODLER_TYPE_INDEX: windows_core::HRESULT = windows_core::HRESULT(0x80045375_u32 as _);
pub const FSRM_E_REPORT_GENERATION_ERR: windows_core::HRESULT = windows_core::HRESULT(0x80045334_u32 as _);
pub const FSRM_E_REPORT_JOB_ALREADY_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x80045333_u32 as _);
pub const FSRM_E_REPORT_TASK_TRIGGER: windows_core::HRESULT = windows_core::HRESULT(0x80045335_u32 as _);
pub const FSRM_E_REPORT_TYPE_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x80045332_u32 as _);
pub const FSRM_E_REQD_PARAM_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x8004530E_u32 as _);
pub const FSRM_E_RMS_NO_PROTECTORS_INSTALLED: windows_core::HRESULT = windows_core::HRESULT(0x80045382_u32 as _);
pub const FSRM_E_RMS_NO_PROTECTOR_INSTALLED_FOR_FILE: windows_core::HRESULT = windows_core::HRESULT(0x80045383_u32 as _);
pub const FSRM_E_RMS_TEMPLATE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80045380_u32 as _);
pub const FSRM_E_SECURE_PROPERTIES_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80045381_u32 as _);
pub const FSRM_E_SET_PROPERTY_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80045354_u32 as _);
pub const FSRM_E_SHADOW_COPY: windows_core::HRESULT = windows_core::HRESULT(0x8004532C_u32 as _);
pub const FSRM_E_STORE_NOT_INSTALLED: windows_core::HRESULT = windows_core::HRESULT(0x8004532F_u32 as _);
pub const FSRM_E_SYNC_TASK_HAD_ERRORS: windows_core::HRESULT = windows_core::HRESULT(0x80045389_u32 as _);
pub const FSRM_E_SYNC_TASK_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80045370_u32 as _);
pub const FSRM_E_TEXTREADER_FILENAME_TOO_LONG: windows_core::HRESULT = windows_core::HRESULT(0x80045362_u32 as _);
pub const FSRM_E_TEXTREADER_IFILTER_CLSID_MALFORMED: windows_core::HRESULT = windows_core::HRESULT(0x80045360_u32 as _);
pub const FSRM_E_TEXTREADER_IFILTER_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80045359_u32 as _);
pub const FSRM_E_TEXTREADER_NOT_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x80045358_u32 as _);
pub const FSRM_E_TEXTREADER_STREAM_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80045361_u32 as _);
pub const FSRM_E_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x80045316_u32 as _);
pub const FSRM_E_UNSECURE_LINK_TO_HOSTED_MODULE: windows_core::HRESULT = windows_core::HRESULT(0x80045344_u32 as _);
pub const FSRM_E_VOLUME_OFFLINE: windows_core::HRESULT = windows_core::HRESULT(0x80045366_u32 as _);
pub const FSRM_E_VOLUME_UNSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80045315_u32 as _);
pub const FSRM_E_WMI_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80045387_u32 as _);
pub const FSRM_E_XML_CORRUPTED: windows_core::HRESULT = windows_core::HRESULT(0x8004532D_u32 as _);
pub const FSRM_S_CLASSIFICATION_SCAN_FAILURES: windows_core::HRESULT = windows_core::HRESULT(0x45306_u32 as _);
pub const FSRM_S_PARTIAL_BATCH: windows_core::HRESULT = windows_core::HRESULT(0x45304_u32 as _);
pub const FSRM_S_PARTIAL_CLASSIFICATION: windows_core::HRESULT = windows_core::HRESULT(0x45305_u32 as _);
pub const FsrmAccountType_Automatic: FsrmAccountType = FsrmAccountType(500i32);
pub const FsrmAccountType_External: FsrmAccountType = FsrmAccountType(5i32);
pub const FsrmAccountType_InProc: FsrmAccountType = FsrmAccountType(4i32);
pub const FsrmAccountType_LocalService: FsrmAccountType = FsrmAccountType(2i32);
pub const FsrmAccountType_LocalSystem: FsrmAccountType = FsrmAccountType(3i32);
pub const FsrmAccountType_NetworkService: FsrmAccountType = FsrmAccountType(1i32);
pub const FsrmAccountType_Unknown: FsrmAccountType = FsrmAccountType(0i32);
pub const FsrmActionType_Command: FsrmActionType = FsrmActionType(3i32);
pub const FsrmActionType_Email: FsrmActionType = FsrmActionType(2i32);
pub const FsrmActionType_EventLog: FsrmActionType = FsrmActionType(1i32);
pub const FsrmActionType_Report: FsrmActionType = FsrmActionType(4i32);
pub const FsrmActionType_Unknown: FsrmActionType = FsrmActionType(0i32);
pub const FsrmClassificationLoggingFlags_ClassificationsInLogFile: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(1i32);
pub const FsrmClassificationLoggingFlags_ClassificationsInSystemLog: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(4i32);
pub const FsrmClassificationLoggingFlags_ErrorsInLogFile: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(2i32);
pub const FsrmClassificationLoggingFlags_ErrorsInSystemLog: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(8i32);
pub const FsrmClassificationLoggingFlags_None: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(0i32);
pub const FsrmCollectionState_Cancelled: FsrmCollectionState = FsrmCollectionState(4i32);
pub const FsrmCollectionState_Committing: FsrmCollectionState = FsrmCollectionState(2i32);
pub const FsrmCollectionState_Complete: FsrmCollectionState = FsrmCollectionState(3i32);
pub const FsrmCollectionState_Fetching: FsrmCollectionState = FsrmCollectionState(1i32);
pub const FsrmCommitOptions_Asynchronous: FsrmCommitOptions = FsrmCommitOptions(1i32);
pub const FsrmCommitOptions_None: FsrmCommitOptions = FsrmCommitOptions(0i32);
pub const FsrmDaysNotSpecified: i32 = -1i32;
pub const FsrmEnumOptions_Asynchronous: FsrmEnumOptions = FsrmEnumOptions(1i32);
pub const FsrmEnumOptions_CheckRecycleBin: FsrmEnumOptions = FsrmEnumOptions(2i32);
pub const FsrmEnumOptions_IncludeClusterNodes: FsrmEnumOptions = FsrmEnumOptions(4i32);
pub const FsrmEnumOptions_IncludeDeprecatedObjects: FsrmEnumOptions = FsrmEnumOptions(8i32);
pub const FsrmEnumOptions_None: FsrmEnumOptions = FsrmEnumOptions(0i32);
pub const FsrmEventType_Error: FsrmEventType = FsrmEventType(3i32);
pub const FsrmEventType_Information: FsrmEventType = FsrmEventType(1i32);
pub const FsrmEventType_Unknown: FsrmEventType = FsrmEventType(0i32);
pub const FsrmEventType_Warning: FsrmEventType = FsrmEventType(2i32);
pub const FsrmExecutionOption_EvaluateUnset: FsrmExecutionOption = FsrmExecutionOption(1i32);
pub const FsrmExecutionOption_ReEvaluate_ConsiderExistingValue: FsrmExecutionOption = FsrmExecutionOption(2i32);
pub const FsrmExecutionOption_ReEvaluate_IgnoreExistingValue: FsrmExecutionOption = FsrmExecutionOption(3i32);
pub const FsrmExecutionOption_Unknown: FsrmExecutionOption = FsrmExecutionOption(0i32);
pub const FsrmFileConditionType_Property: FsrmFileConditionType = FsrmFileConditionType(1i32);
pub const FsrmFileConditionType_Unknown: FsrmFileConditionType = FsrmFileConditionType(0i32);
pub const FsrmFileManagementLoggingFlags_Audit: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(4i32);
pub const FsrmFileManagementLoggingFlags_Error: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(1i32);
pub const FsrmFileManagementLoggingFlags_Information: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(2i32);
pub const FsrmFileManagementLoggingFlags_None: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(0i32);
pub const FsrmFileManagementType_Custom: FsrmFileManagementType = FsrmFileManagementType(2i32);
pub const FsrmFileManagementType_Expiration: FsrmFileManagementType = FsrmFileManagementType(1i32);
pub const FsrmFileManagementType_Rms: FsrmFileManagementType = FsrmFileManagementType(3i32);
pub const FsrmFileManagementType_Unknown: FsrmFileManagementType = FsrmFileManagementType(0i32);
pub const FsrmFileScreenFlags_Enforce: FsrmFileScreenFlags = FsrmFileScreenFlags(1i32);
pub const FsrmFileStreamingInterfaceType_ILockBytes: FsrmFileStreamingInterfaceType = FsrmFileStreamingInterfaceType(1i32);
pub const FsrmFileStreamingInterfaceType_IStream: FsrmFileStreamingInterfaceType = FsrmFileStreamingInterfaceType(2i32);
pub const FsrmFileStreamingInterfaceType_Unknown: FsrmFileStreamingInterfaceType = FsrmFileStreamingInterfaceType(0i32);
pub const FsrmFileStreamingMode_Read: FsrmFileStreamingMode = FsrmFileStreamingMode(1i32);
pub const FsrmFileStreamingMode_Unknown: FsrmFileStreamingMode = FsrmFileStreamingMode(0i32);
pub const FsrmFileStreamingMode_Write: FsrmFileStreamingMode = FsrmFileStreamingMode(2i32);
pub const FsrmFileSystemPropertyId_DateCreated: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(2i32);
pub const FsrmFileSystemPropertyId_DateLastAccessed: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(3i32);
pub const FsrmFileSystemPropertyId_DateLastModified: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(4i32);
pub const FsrmFileSystemPropertyId_DateNow: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(5i32);
pub const FsrmFileSystemPropertyId_FileName: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(1i32);
pub const FsrmFileSystemPropertyId_Undefined: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(0i32);
pub const FsrmGetFilePropertyOptions_FailOnPersistErrors: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(4i32);
pub const FsrmGetFilePropertyOptions_NoRuleEvaluation: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(1i32);
pub const FsrmGetFilePropertyOptions_None: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(0i32);
pub const FsrmGetFilePropertyOptions_Persistent: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(2i32);
pub const FsrmGetFilePropertyOptions_SkipOrphaned: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(8i32);
pub const FsrmMaxExcludeFolders: u32 = 32u32;
pub const FsrmMaxNumberPropertyDefinitions: u32 = 100u32;
pub const FsrmMaxNumberThresholds: u32 = 16u32;
pub const FsrmMaxThresholdValue: u32 = 250u32;
pub const FsrmMinQuotaLimit: u32 = 1024u32;
pub const FsrmMinThresholdValue: u32 = 1u32;
pub const FsrmPipelineModuleType_Classifier: FsrmPipelineModuleType = FsrmPipelineModuleType(2i32);
pub const FsrmPipelineModuleType_Storage: FsrmPipelineModuleType = FsrmPipelineModuleType(1i32);
pub const FsrmPipelineModuleType_Unknown: FsrmPipelineModuleType = FsrmPipelineModuleType(0i32);
pub const FsrmPropertyBagField_AccessVolume: FsrmPropertyBagField = FsrmPropertyBagField(0i32);
pub const FsrmPropertyBagField_VolumeGuidName: FsrmPropertyBagField = FsrmPropertyBagField(1i32);
pub const FsrmPropertyBagFlags_FailedClassifyingProperties: FsrmPropertyBagFlags = FsrmPropertyBagFlags(8i32);
pub const FsrmPropertyBagFlags_FailedLoadingProperties: FsrmPropertyBagFlags = FsrmPropertyBagFlags(2i32);
pub const FsrmPropertyBagFlags_FailedSavingProperties: FsrmPropertyBagFlags = FsrmPropertyBagFlags(4i32);
pub const FsrmPropertyBagFlags_UpdatedByClassifier: FsrmPropertyBagFlags = FsrmPropertyBagFlags(1i32);
pub const FsrmPropertyConditionType_Contain: FsrmPropertyConditionType = FsrmPropertyConditionType(5i32);
pub const FsrmPropertyConditionType_ContainedIn: FsrmPropertyConditionType = FsrmPropertyConditionType(10i32);
pub const FsrmPropertyConditionType_EndWith: FsrmPropertyConditionType = FsrmPropertyConditionType(9i32);
pub const FsrmPropertyConditionType_Equal: FsrmPropertyConditionType = FsrmPropertyConditionType(1i32);
pub const FsrmPropertyConditionType_Exist: FsrmPropertyConditionType = FsrmPropertyConditionType(6i32);
pub const FsrmPropertyConditionType_GreaterThan: FsrmPropertyConditionType = FsrmPropertyConditionType(3i32);
pub const FsrmPropertyConditionType_LessThan: FsrmPropertyConditionType = FsrmPropertyConditionType(4i32);
pub const FsrmPropertyConditionType_MatchesPattern: FsrmPropertyConditionType = FsrmPropertyConditionType(13i32);
pub const FsrmPropertyConditionType_NotEqual: FsrmPropertyConditionType = FsrmPropertyConditionType(2i32);
pub const FsrmPropertyConditionType_NotExist: FsrmPropertyConditionType = FsrmPropertyConditionType(7i32);
pub const FsrmPropertyConditionType_PrefixOf: FsrmPropertyConditionType = FsrmPropertyConditionType(11i32);
pub const FsrmPropertyConditionType_StartWith: FsrmPropertyConditionType = FsrmPropertyConditionType(8i32);
pub const FsrmPropertyConditionType_SuffixOf: FsrmPropertyConditionType = FsrmPropertyConditionType(12i32);
pub const FsrmPropertyConditionType_Unknown: FsrmPropertyConditionType = FsrmPropertyConditionType(0i32);
pub const FsrmPropertyDefinitionAppliesTo_Files: FsrmPropertyDefinitionAppliesTo = FsrmPropertyDefinitionAppliesTo(1i32);
pub const FsrmPropertyDefinitionAppliesTo_Folders: FsrmPropertyDefinitionAppliesTo = FsrmPropertyDefinitionAppliesTo(2i32);
pub const FsrmPropertyDefinitionFlags_Deprecated: FsrmPropertyDefinitionFlags = FsrmPropertyDefinitionFlags(2i32);
pub const FsrmPropertyDefinitionFlags_Global: FsrmPropertyDefinitionFlags = FsrmPropertyDefinitionFlags(1i32);
pub const FsrmPropertyDefinitionFlags_Secure: FsrmPropertyDefinitionFlags = FsrmPropertyDefinitionFlags(4i32);
pub const FsrmPropertyDefinitionType_Bool: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(7i32);
pub const FsrmPropertyDefinitionType_Date: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(8i32);
pub const FsrmPropertyDefinitionType_Int: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(6i32);
pub const FsrmPropertyDefinitionType_MultiChoiceList: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(2i32);
pub const FsrmPropertyDefinitionType_MultiString: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(5i32);
pub const FsrmPropertyDefinitionType_OrderedList: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(1i32);
pub const FsrmPropertyDefinitionType_SingleChoiceList: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(3i32);
pub const FsrmPropertyDefinitionType_String: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(4i32);
pub const FsrmPropertyDefinitionType_Unknown: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(0i32);
pub const FsrmPropertyFlags_AggregationFailed: FsrmPropertyFlags = FsrmPropertyFlags(64i32);
pub const FsrmPropertyFlags_Deleted: FsrmPropertyFlags = FsrmPropertyFlags(16i32);
pub const FsrmPropertyFlags_Existing: FsrmPropertyFlags = FsrmPropertyFlags(128i32);
pub const FsrmPropertyFlags_ExplicitValueDeleted: FsrmPropertyFlags = FsrmPropertyFlags(32768i32);
pub const FsrmPropertyFlags_FailedClassifyingProperties: FsrmPropertyFlags = FsrmPropertyFlags(512i32);
pub const FsrmPropertyFlags_FailedLoadingProperties: FsrmPropertyFlags = FsrmPropertyFlags(256i32);
pub const FsrmPropertyFlags_FailedSavingProperties: FsrmPropertyFlags = FsrmPropertyFlags(1024i32);
pub const FsrmPropertyFlags_Inherited: FsrmPropertyFlags = FsrmPropertyFlags(8192i32);
pub const FsrmPropertyFlags_Manual: FsrmPropertyFlags = FsrmPropertyFlags(16384i32);
pub const FsrmPropertyFlags_None: FsrmPropertyFlags = FsrmPropertyFlags(0i32);
pub const FsrmPropertyFlags_Orphaned: FsrmPropertyFlags = FsrmPropertyFlags(1i32);
pub const FsrmPropertyFlags_PersistentMask: FsrmPropertyFlags = FsrmPropertyFlags(20480i32);
pub const FsrmPropertyFlags_PolicyDerived: FsrmPropertyFlags = FsrmPropertyFlags(4096i32);
pub const FsrmPropertyFlags_PropertyDeletedFromClear: FsrmPropertyFlags = FsrmPropertyFlags(65536i32);
pub const FsrmPropertyFlags_PropertySourceMask: FsrmPropertyFlags = FsrmPropertyFlags(14i32);
pub const FsrmPropertyFlags_Reclassified: FsrmPropertyFlags = FsrmPropertyFlags(32i32);
pub const FsrmPropertyFlags_RetrievedFromCache: FsrmPropertyFlags = FsrmPropertyFlags(2i32);
pub const FsrmPropertyFlags_RetrievedFromStorage: FsrmPropertyFlags = FsrmPropertyFlags(4i32);
pub const FsrmPropertyFlags_Secure: FsrmPropertyFlags = FsrmPropertyFlags(2048i32);
pub const FsrmPropertyFlags_SetByClassifier: FsrmPropertyFlags = FsrmPropertyFlags(8i32);
pub const FsrmPropertyValueType_DateOffset: FsrmPropertyValueType = FsrmPropertyValueType(2i32);
pub const FsrmPropertyValueType_Literal: FsrmPropertyValueType = FsrmPropertyValueType(1i32);
pub const FsrmPropertyValueType_Undefined: FsrmPropertyValueType = FsrmPropertyValueType(0i32);
pub const FsrmQuotaFlags_Disable: FsrmQuotaFlags = FsrmQuotaFlags(512i32);
pub const FsrmQuotaFlags_Enforce: FsrmQuotaFlags = FsrmQuotaFlags(256i32);
pub const FsrmQuotaFlags_StatusIncomplete: FsrmQuotaFlags = FsrmQuotaFlags(65536i32);
pub const FsrmQuotaFlags_StatusRebuilding: FsrmQuotaFlags = FsrmQuotaFlags(131072i32);
pub const FsrmReportFilter_FileGroups: FsrmReportFilter = FsrmReportFilter(5i32);
pub const FsrmReportFilter_MaxAgeDays: FsrmReportFilter = FsrmReportFilter(3i32);
pub const FsrmReportFilter_MinAgeDays: FsrmReportFilter = FsrmReportFilter(2i32);
pub const FsrmReportFilter_MinQuotaUsage: FsrmReportFilter = FsrmReportFilter(4i32);
pub const FsrmReportFilter_MinSize: FsrmReportFilter = FsrmReportFilter(1i32);
pub const FsrmReportFilter_NamePattern: FsrmReportFilter = FsrmReportFilter(7i32);
pub const FsrmReportFilter_Owners: FsrmReportFilter = FsrmReportFilter(6i32);
pub const FsrmReportFilter_Property: FsrmReportFilter = FsrmReportFilter(8i32);
pub const FsrmReportFormat_Csv: FsrmReportFormat = FsrmReportFormat(4i32);
pub const FsrmReportFormat_DHtml: FsrmReportFormat = FsrmReportFormat(1i32);
pub const FsrmReportFormat_Html: FsrmReportFormat = FsrmReportFormat(2i32);
pub const FsrmReportFormat_Txt: FsrmReportFormat = FsrmReportFormat(3i32);
pub const FsrmReportFormat_Unknown: FsrmReportFormat = FsrmReportFormat(0i32);
pub const FsrmReportFormat_Xml: FsrmReportFormat = FsrmReportFormat(5i32);
pub const FsrmReportGenerationContext_IncidentReport: FsrmReportGenerationContext = FsrmReportGenerationContext(4i32);
pub const FsrmReportGenerationContext_InteractiveReport: FsrmReportGenerationContext = FsrmReportGenerationContext(3i32);
pub const FsrmReportGenerationContext_ScheduledReport: FsrmReportGenerationContext = FsrmReportGenerationContext(2i32);
pub const FsrmReportGenerationContext_Undefined: FsrmReportGenerationContext = FsrmReportGenerationContext(1i32);
pub const FsrmReportLimit_MaxDuplicateGroups: FsrmReportLimit = FsrmReportLimit(7i32);
pub const FsrmReportLimit_MaxFileGroups: FsrmReportLimit = FsrmReportLimit(2i32);
pub const FsrmReportLimit_MaxFileScreenEvents: FsrmReportLimit = FsrmReportLimit(9i32);
pub const FsrmReportLimit_MaxFiles: FsrmReportLimit = FsrmReportLimit(1i32);
pub const FsrmReportLimit_MaxFilesPerDuplGroup: FsrmReportLimit = FsrmReportLimit(6i32);
pub const FsrmReportLimit_MaxFilesPerFileGroup: FsrmReportLimit = FsrmReportLimit(4i32);
pub const FsrmReportLimit_MaxFilesPerOwner: FsrmReportLimit = FsrmReportLimit(5i32);
pub const FsrmReportLimit_MaxFilesPerPropertyValue: FsrmReportLimit = FsrmReportLimit(11i32);
pub const FsrmReportLimit_MaxFolders: FsrmReportLimit = FsrmReportLimit(12i32);
pub const FsrmReportLimit_MaxOwners: FsrmReportLimit = FsrmReportLimit(3i32);
pub const FsrmReportLimit_MaxPropertyValues: FsrmReportLimit = FsrmReportLimit(10i32);
pub const FsrmReportLimit_MaxQuotas: FsrmReportLimit = FsrmReportLimit(8i32);
pub const FsrmReportRunningStatus_NotRunning: FsrmReportRunningStatus = FsrmReportRunningStatus(1i32);
pub const FsrmReportRunningStatus_Queued: FsrmReportRunningStatus = FsrmReportRunningStatus(2i32);
pub const FsrmReportRunningStatus_Running: FsrmReportRunningStatus = FsrmReportRunningStatus(3i32);
pub const FsrmReportRunningStatus_Unknown: FsrmReportRunningStatus = FsrmReportRunningStatus(0i32);
pub const FsrmReportType_AutomaticClassification: FsrmReportType = FsrmReportType(11i32);
pub const FsrmReportType_DuplicateFiles: FsrmReportType = FsrmReportType(8i32);
pub const FsrmReportType_Expiration: FsrmReportType = FsrmReportType(12i32);
pub const FsrmReportType_ExportReport: FsrmReportType = FsrmReportType(7i32);
pub const FsrmReportType_FileScreenAudit: FsrmReportType = FsrmReportType(9i32);
pub const FsrmReportType_FilesByOwner: FsrmReportType = FsrmReportType(6i32);
pub const FsrmReportType_FilesByProperty: FsrmReportType = FsrmReportType(10i32);
pub const FsrmReportType_FilesByType: FsrmReportType = FsrmReportType(2i32);
pub const FsrmReportType_FoldersByProperty: FsrmReportType = FsrmReportType(13i32);
pub const FsrmReportType_LargeFiles: FsrmReportType = FsrmReportType(1i32);
pub const FsrmReportType_LeastRecentlyAccessed: FsrmReportType = FsrmReportType(3i32);
pub const FsrmReportType_MostRecentlyAccessed: FsrmReportType = FsrmReportType(4i32);
pub const FsrmReportType_QuotaUsage: FsrmReportType = FsrmReportType(5i32);
pub const FsrmReportType_Unknown: FsrmReportType = FsrmReportType(0i32);
pub const FsrmRuleFlags_ClearAutomaticallyClassifiedProperty: FsrmRuleFlags = FsrmRuleFlags(1024i32);
pub const FsrmRuleFlags_ClearManuallyClassifiedProperty: FsrmRuleFlags = FsrmRuleFlags(2048i32);
pub const FsrmRuleFlags_Disabled: FsrmRuleFlags = FsrmRuleFlags(256i32);
pub const FsrmRuleFlags_Invalid: FsrmRuleFlags = FsrmRuleFlags(4096i32);
pub const FsrmRuleType_Classification: FsrmRuleType = FsrmRuleType(1i32);
pub const FsrmRuleType_Generic: FsrmRuleType = FsrmRuleType(2i32);
pub const FsrmRuleType_Unknown: FsrmRuleType = FsrmRuleType(0i32);
pub const FsrmStorageModuleCaps_CanGet: FsrmStorageModuleCaps = FsrmStorageModuleCaps(1i32);
pub const FsrmStorageModuleCaps_CanHandleDirectories: FsrmStorageModuleCaps = FsrmStorageModuleCaps(4i32);
pub const FsrmStorageModuleCaps_CanHandleFiles: FsrmStorageModuleCaps = FsrmStorageModuleCaps(8i32);
pub const FsrmStorageModuleCaps_CanSet: FsrmStorageModuleCaps = FsrmStorageModuleCaps(2i32);
pub const FsrmStorageModuleCaps_Unknown: FsrmStorageModuleCaps = FsrmStorageModuleCaps(0i32);
pub const FsrmStorageModuleType_Cache: FsrmStorageModuleType = FsrmStorageModuleType(1i32);
pub const FsrmStorageModuleType_Database: FsrmStorageModuleType = FsrmStorageModuleType(3i32);
pub const FsrmStorageModuleType_InFile: FsrmStorageModuleType = FsrmStorageModuleType(2i32);
pub const FsrmStorageModuleType_System: FsrmStorageModuleType = FsrmStorageModuleType(100i32);
pub const FsrmStorageModuleType_Unknown: FsrmStorageModuleType = FsrmStorageModuleType(0i32);
pub const FsrmTemplateApplyOptions_ApplyToDerivedAll: FsrmTemplateApplyOptions = FsrmTemplateApplyOptions(2i32);
pub const FsrmTemplateApplyOptions_ApplyToDerivedMatching: FsrmTemplateApplyOptions = FsrmTemplateApplyOptions(1i32);
pub const MessageSizeLimit: u32 = 4096u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AdrClientDisplayFlags(pub i32);
impl windows_core::TypeKind for AdrClientDisplayFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AdrClientDisplayFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AdrClientDisplayFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AdrClientErrorType(pub i32);
impl windows_core::TypeKind for AdrClientErrorType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AdrClientErrorType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AdrClientErrorType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AdrClientFlags(pub i32);
impl windows_core::TypeKind for AdrClientFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AdrClientFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AdrClientFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AdrEmailFlags(pub i32);
impl windows_core::TypeKind for AdrEmailFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AdrEmailFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AdrEmailFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmAccountType(pub i32);
impl windows_core::TypeKind for FsrmAccountType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmAccountType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmAccountType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmActionType(pub i32);
impl windows_core::TypeKind for FsrmActionType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmActionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmActionType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmClassificationLoggingFlags(pub i32);
impl windows_core::TypeKind for FsrmClassificationLoggingFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmClassificationLoggingFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmClassificationLoggingFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmCollectionState(pub i32);
impl windows_core::TypeKind for FsrmCollectionState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmCollectionState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmCollectionState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmCommitOptions(pub i32);
impl windows_core::TypeKind for FsrmCommitOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmCommitOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmCommitOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmEnumOptions(pub i32);
impl windows_core::TypeKind for FsrmEnumOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmEnumOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmEnumOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmEventType(pub i32);
impl windows_core::TypeKind for FsrmEventType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmEventType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmEventType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmExecutionOption(pub i32);
impl windows_core::TypeKind for FsrmExecutionOption {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmExecutionOption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmExecutionOption").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmFileConditionType(pub i32);
impl windows_core::TypeKind for FsrmFileConditionType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmFileConditionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmFileConditionType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmFileManagementLoggingFlags(pub i32);
impl windows_core::TypeKind for FsrmFileManagementLoggingFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmFileManagementLoggingFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmFileManagementLoggingFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmFileManagementType(pub i32);
impl windows_core::TypeKind for FsrmFileManagementType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmFileManagementType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmFileManagementType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmFileScreenFlags(pub i32);
impl windows_core::TypeKind for FsrmFileScreenFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmFileScreenFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmFileScreenFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmFileStreamingInterfaceType(pub i32);
impl windows_core::TypeKind for FsrmFileStreamingInterfaceType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmFileStreamingInterfaceType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmFileStreamingInterfaceType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmFileStreamingMode(pub i32);
impl windows_core::TypeKind for FsrmFileStreamingMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmFileStreamingMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmFileStreamingMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmFileSystemPropertyId(pub i32);
impl windows_core::TypeKind for FsrmFileSystemPropertyId {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmFileSystemPropertyId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmFileSystemPropertyId").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmGetFilePropertyOptions(pub i32);
impl windows_core::TypeKind for FsrmGetFilePropertyOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmGetFilePropertyOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmGetFilePropertyOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmPipelineModuleType(pub i32);
impl windows_core::TypeKind for FsrmPipelineModuleType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmPipelineModuleType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmPipelineModuleType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmPropertyBagField(pub i32);
impl windows_core::TypeKind for FsrmPropertyBagField {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmPropertyBagField {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmPropertyBagField").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmPropertyBagFlags(pub i32);
impl windows_core::TypeKind for FsrmPropertyBagFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmPropertyBagFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmPropertyBagFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmPropertyConditionType(pub i32);
impl windows_core::TypeKind for FsrmPropertyConditionType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmPropertyConditionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmPropertyConditionType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmPropertyDefinitionAppliesTo(pub i32);
impl windows_core::TypeKind for FsrmPropertyDefinitionAppliesTo {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmPropertyDefinitionAppliesTo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmPropertyDefinitionAppliesTo").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmPropertyDefinitionFlags(pub i32);
impl windows_core::TypeKind for FsrmPropertyDefinitionFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmPropertyDefinitionFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmPropertyDefinitionFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmPropertyDefinitionType(pub i32);
impl windows_core::TypeKind for FsrmPropertyDefinitionType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmPropertyDefinitionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmPropertyDefinitionType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmPropertyFlags(pub i32);
impl windows_core::TypeKind for FsrmPropertyFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmPropertyFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmPropertyFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmPropertyValueType(pub i32);
impl windows_core::TypeKind for FsrmPropertyValueType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmPropertyValueType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmPropertyValueType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmQuotaFlags(pub i32);
impl windows_core::TypeKind for FsrmQuotaFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmQuotaFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmQuotaFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmReportFilter(pub i32);
impl windows_core::TypeKind for FsrmReportFilter {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmReportFilter {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmReportFilter").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmReportFormat(pub i32);
impl windows_core::TypeKind for FsrmReportFormat {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmReportFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmReportFormat").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmReportGenerationContext(pub i32);
impl windows_core::TypeKind for FsrmReportGenerationContext {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmReportGenerationContext {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmReportGenerationContext").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmReportLimit(pub i32);
impl windows_core::TypeKind for FsrmReportLimit {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmReportLimit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmReportLimit").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmReportRunningStatus(pub i32);
impl windows_core::TypeKind for FsrmReportRunningStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmReportRunningStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmReportRunningStatus").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmReportType(pub i32);
impl windows_core::TypeKind for FsrmReportType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmReportType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmReportType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmRuleFlags(pub i32);
impl windows_core::TypeKind for FsrmRuleFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmRuleFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmRuleFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmRuleType(pub i32);
impl windows_core::TypeKind for FsrmRuleType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmRuleType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmRuleType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmStorageModuleCaps(pub i32);
impl windows_core::TypeKind for FsrmStorageModuleCaps {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmStorageModuleCaps {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmStorageModuleCaps").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmStorageModuleType(pub i32);
impl windows_core::TypeKind for FsrmStorageModuleType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmStorageModuleType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmStorageModuleType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FsrmTemplateApplyOptions(pub i32);
impl windows_core::TypeKind for FsrmTemplateApplyOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FsrmTemplateApplyOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FsrmTemplateApplyOptions").field(&self.0).finish()
    }
}
pub const AdSyncTask: windows_core::GUID = windows_core::GUID::from_u128(0x2ae64751_b728_4d6b_97a0_b2da2e7d2a3b);
pub const FsrmAccessDeniedRemediationClient: windows_core::GUID = windows_core::GUID::from_u128(0x100b4fc8_74c1_470f_b1b7_dd7b6bae79bd);
pub const FsrmClassificationManager: windows_core::GUID = windows_core::GUID::from_u128(0xb15c0e47_c391_45b9_95c8_eb596c853f3a);
pub const FsrmExportImport: windows_core::GUID = windows_core::GUID::from_u128(0x1482dc37_fae9_4787_9025_8ce4e024ab56);
pub const FsrmFileGroupManager: windows_core::GUID = windows_core::GUID::from_u128(0x8f1363f6_656f_4496_9226_13aecbd7718f);
pub const FsrmFileManagementJobManager: windows_core::GUID = windows_core::GUID::from_u128(0xeb18f9b2_4c3a_4321_b203_205120cff614);
pub const FsrmFileScreenManager: windows_core::GUID = windows_core::GUID::from_u128(0x95941183_db53_4c5f_b37b_7d0921cf9dc7);
pub const FsrmFileScreenTemplateManager: windows_core::GUID = windows_core::GUID::from_u128(0x243111df_e474_46aa_a054_eaa33edc292a);
pub const FsrmPathMapper: windows_core::GUID = windows_core::GUID::from_u128(0xf3be42bd_8ac2_409e_bbd8_faf9b6b41feb);
pub const FsrmPipelineModuleConnector: windows_core::GUID = windows_core::GUID::from_u128(0xc7643375_1eb5_44de_a062_623547d933bc);
pub const FsrmQuotaManager: windows_core::GUID = windows_core::GUID::from_u128(0x90dcab7f_347c_4bfc_b543_540326305fbe);
pub const FsrmQuotaTemplateManager: windows_core::GUID = windows_core::GUID::from_u128(0x97d3d443_251c_4337_81e7_b32e8f4ee65e);
pub const FsrmReportManager: windows_core::GUID = windows_core::GUID::from_u128(0x0058ef37_aa66_4c48_bd5b_2fce432ab0c8);
pub const FsrmReportScheduler: windows_core::GUID = windows_core::GUID::from_u128(0xea25f1b8_1b8d_4290_8ee8_e17c12c2fe20);
pub const FsrmSetting: windows_core::GUID = windows_core::GUID::from_u128(0xf556d708_6d4d_4594_9c61_7dbb0dae2a46);
#[cfg(feature = "implement")]
core::include!("impl.rs");
