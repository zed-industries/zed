windows_core::imp::define_interface!(IEnumSpObjectTokens, IEnumSpObjectTokens_Vtbl, 0x06b64f9e_7fda_11d2_b4f2_00c04f797396);
impl core::ops::Deref for IEnumSpObjectTokens {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumSpObjectTokens, windows_core::IUnknown);
impl IEnumSpObjectTokens {
    pub unsafe fn Next(&self, celt: u32, pelt: *mut Option<ISpObjectToken>, pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(pelt), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumSpObjectTokens> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Item(&self, index: u32) -> windows_core::Result<ISpObjectToken> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCount(&self, pcount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), pcount).ok()
    }
}
#[repr(C)]
pub struct IEnumSpObjectTokens_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpAudio, ISpAudio_Vtbl, 0xc05c768f_fae8_4ec2_8e07_338321c12452);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpAudio {
    type Target = ISpStreamFormat;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpAudio, windows_core::IUnknown, super::super::System::Com::ISequentialStream, super::super::System::Com::IStream, ISpStreamFormat);
#[cfg(feature = "Win32_System_Com")]
impl ISpAudio {
    pub unsafe fn SetState(&self, newstate: SPAUDIOSTATE, ullreserved: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetState)(windows_core::Interface::as_raw(self), newstate, ullreserved).ok()
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetFormat(&self, rguidfmtid: *const windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFormat)(windows_core::Interface::as_raw(self), rguidfmtid, pwaveformatex).ok()
    }
    pub unsafe fn GetStatus(&self, pstatus: *mut SPAUDIOSTATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), pstatus).ok()
    }
    pub unsafe fn SetBufferInfo(&self, pbuffinfo: *const SPAUDIOBUFFERINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBufferInfo)(windows_core::Interface::as_raw(self), pbuffinfo).ok()
    }
    pub unsafe fn GetBufferInfo(&self, pbuffinfo: *mut SPAUDIOBUFFERINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBufferInfo)(windows_core::Interface::as_raw(self), pbuffinfo).ok()
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetDefaultFormat(&self, pformatid: *mut windows_core::GUID, ppcomemwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDefaultFormat)(windows_core::Interface::as_raw(self), pformatid, ppcomemwaveformatex).ok()
    }
    pub unsafe fn EventHandle(&self) -> super::super::Foundation::HANDLE {
        (windows_core::Interface::vtable(self).EventHandle)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetVolumeLevel(&self, plevel: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetVolumeLevel)(windows_core::Interface::as_raw(self), plevel).ok()
    }
    pub unsafe fn SetVolumeLevel(&self, level: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetVolumeLevel)(windows_core::Interface::as_raw(self), level).ok()
    }
    pub unsafe fn GetBufferNotifySize(&self, pcbsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBufferNotifySize)(windows_core::Interface::as_raw(self), pcbsize).ok()
    }
    pub unsafe fn SetBufferNotifySize(&self, cbsize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBufferNotifySize)(windows_core::Interface::as_raw(self), cbsize).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpAudio_Vtbl {
    pub base__: ISpStreamFormat_Vtbl,
    pub SetState: unsafe extern "system" fn(*mut core::ffi::c_void, SPAUDIOSTATE, u64) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub SetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::Audio::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    SetFormat: usize,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPAUDIOSTATUS) -> windows_core::HRESULT,
    pub SetBufferInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPAUDIOBUFFERINFO) -> windows_core::HRESULT,
    pub GetBufferInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPAUDIOBUFFERINFO) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetDefaultFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut *mut super::Audio::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetDefaultFormat: usize,
    pub EventHandle: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::HANDLE,
    pub GetVolumeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetVolumeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetBufferNotifySize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetBufferNotifySize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpCFGInterpreter, ISpCFGInterpreter_Vtbl, 0xf3d3f926_11fc_11d3_bb97_00c04f8ee6c0);
impl core::ops::Deref for ISpCFGInterpreter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpCFGInterpreter, windows_core::IUnknown);
impl ISpCFGInterpreter {
    pub unsafe fn InitGrammar<P0>(&self, pszgrammarname: P0, pvgrammardata: *const *const core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).InitGrammar)(windows_core::Interface::as_raw(self), pszgrammarname.param().abi(), pvgrammardata).ok()
    }
    pub unsafe fn Interpret<P0, P1>(&self, pphrase: P0, ulfirstelement: u32, ulcountofelements: u32, psite: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpPhraseBuilder>,
        P1: windows_core::Param<ISpCFGInterpreterSite>,
    {
        (windows_core::Interface::vtable(self).Interpret)(windows_core::Interface::as_raw(self), pphrase.param().abi(), ulfirstelement, ulcountofelements, psite.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISpCFGInterpreter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitGrammar: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const *const core::ffi::c_void) -> windows_core::HRESULT,
    pub Interpret: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpCFGInterpreterSite, ISpCFGInterpreterSite_Vtbl, 0x6a6ffad8_78b6_473d_b844_98152e4fb16b);
impl core::ops::Deref for ISpCFGInterpreterSite {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpCFGInterpreterSite, windows_core::IUnknown);
impl ISpCFGInterpreterSite {
    pub unsafe fn AddTextReplacement(&self, preplace: *const SPPHRASEREPLACEMENT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddTextReplacement)(windows_core::Interface::as_raw(self), preplace).ok()
    }
    pub unsafe fn AddProperty(&self, pproperty: *const SPPHRASEPROPERTY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddProperty)(windows_core::Interface::as_raw(self), pproperty).ok()
    }
    pub unsafe fn GetResourceValue<P0>(&self, pszresourcename: P0) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetResourceValue)(windows_core::Interface::as_raw(self), pszresourcename.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISpCFGInterpreterSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddTextReplacement: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPPHRASEREPLACEMENT) -> windows_core::HRESULT,
    pub AddProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPPHRASEPROPERTY) -> windows_core::HRESULT,
    pub GetResourceValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpContainerLexicon, ISpContainerLexicon_Vtbl, 0x8565572f_c094_41cc_b56e_10bd9c3ff044);
impl core::ops::Deref for ISpContainerLexicon {
    type Target = ISpLexicon;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpContainerLexicon, windows_core::IUnknown, ISpLexicon);
impl ISpContainerLexicon {
    pub unsafe fn AddLexicon<P0>(&self, paddlexicon: P0, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpLexicon>,
    {
        (windows_core::Interface::vtable(self).AddLexicon)(windows_core::Interface::as_raw(self), paddlexicon.param().abi(), dwflags).ok()
    }
}
#[repr(C)]
pub struct ISpContainerLexicon_Vtbl {
    pub base__: ISpLexicon_Vtbl,
    pub AddLexicon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpDataKey, ISpDataKey_Vtbl, 0x14056581_e16c_11d2_bb90_00c04f8ee6c0);
impl core::ops::Deref for ISpDataKey {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpDataKey, windows_core::IUnknown);
impl ISpDataKey {
    pub unsafe fn SetData<P0>(&self, pszvaluename: P0, cbdata: u32, pdata: *const u8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetData)(windows_core::Interface::as_raw(self), pszvaluename.param().abi(), cbdata, pdata).ok()
    }
    pub unsafe fn GetData<P0>(&self, pszvaluename: P0, pcbdata: *mut u32, pdata: *mut u8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), pszvaluename.param().abi(), pcbdata, pdata).ok()
    }
    pub unsafe fn SetStringValue<P0, P1>(&self, pszvaluename: P0, pszvalue: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetStringValue)(windows_core::Interface::as_raw(self), pszvaluename.param().abi(), pszvalue.param().abi()).ok()
    }
    pub unsafe fn GetStringValue<P0>(&self, pszvaluename: P0) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStringValue)(windows_core::Interface::as_raw(self), pszvaluename.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDWORD<P0>(&self, pszvaluename: P0, dwvalue: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetDWORD)(windows_core::Interface::as_raw(self), pszvaluename.param().abi(), dwvalue).ok()
    }
    pub unsafe fn GetDWORD<P0>(&self, pszvaluename: P0, pdwvalue: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetDWORD)(windows_core::Interface::as_raw(self), pszvaluename.param().abi(), pdwvalue).ok()
    }
    pub unsafe fn OpenKey<P0>(&self, pszsubkeyname: P0) -> windows_core::Result<ISpDataKey>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenKey)(windows_core::Interface::as_raw(self), pszsubkeyname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateKey<P0>(&self, pszsubkey: P0) -> windows_core::Result<ISpDataKey>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateKey)(windows_core::Interface::as_raw(self), pszsubkey.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteKey<P0>(&self, pszsubkey: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteKey)(windows_core::Interface::as_raw(self), pszsubkey.param().abi()).ok()
    }
    pub unsafe fn DeleteValue<P0>(&self, pszvaluename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteValue)(windows_core::Interface::as_raw(self), pszvaluename.param().abi()).ok()
    }
    pub unsafe fn EnumKeys(&self, index: u32) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumKeys)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
    }
    pub unsafe fn EnumValues(&self, index: u32) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumValues)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISpDataKey_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *const u8) -> windows_core::HRESULT,
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut u8) -> windows_core::HRESULT,
    pub SetStringValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetStringValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetDWORD: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub GetDWORD: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub OpenKey: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateKey: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteKey: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub DeleteValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub EnumKeys: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub EnumValues: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpDisplayAlternates, ISpDisplayAlternates_Vtbl, 0xc8d7c7e2_0dde_44b7_afe3_b0c991fbeb5e);
impl core::ops::Deref for ISpDisplayAlternates {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpDisplayAlternates, windows_core::IUnknown);
impl ISpDisplayAlternates {
    pub unsafe fn GetDisplayAlternates(&self, pphrase: *const SPDISPLAYPHRASE, crequestcount: u32, ppcomemphrases: *mut *mut SPDISPLAYPHRASE, pcphrasesreturned: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDisplayAlternates)(windows_core::Interface::as_raw(self), pphrase, crequestcount, ppcomemphrases, pcphrasesreturned).ok()
    }
    pub unsafe fn SetFullStopTrailSpace(&self, ultrailspace: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFullStopTrailSpace)(windows_core::Interface::as_raw(self), ultrailspace).ok()
    }
}
#[repr(C)]
pub struct ISpDisplayAlternates_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDisplayAlternates: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPDISPLAYPHRASE, u32, *mut *mut SPDISPLAYPHRASE, *mut u32) -> windows_core::HRESULT,
    pub SetFullStopTrailSpace: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpEnginePronunciation, ISpEnginePronunciation_Vtbl, 0xc360ce4b_76d1_4214_ad68_52657d5083da);
impl core::ops::Deref for ISpEnginePronunciation {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpEnginePronunciation, windows_core::IUnknown);
impl ISpEnginePronunciation {
    pub unsafe fn Normalize<P0, P1, P2>(&self, pszword: P0, pszleftcontext: P1, pszrightcontext: P2, langid: u16, pnormalizationlist: *mut SPNORMALIZATIONLIST) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Normalize)(windows_core::Interface::as_raw(self), pszword.param().abi(), pszleftcontext.param().abi(), pszrightcontext.param().abi(), langid, pnormalizationlist).ok()
    }
    pub unsafe fn GetPronunciations<P0, P1, P2>(&self, pszword: P0, pszleftcontext: P1, pszrightcontext: P2, langid: u16, penginepronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetPronunciations)(windows_core::Interface::as_raw(self), pszword.param().abi(), pszleftcontext.param().abi(), pszrightcontext.param().abi(), langid, penginepronunciationlist).ok()
    }
}
#[repr(C)]
pub struct ISpEnginePronunciation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Normalize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, u16, *mut SPNORMALIZATIONLIST) -> windows_core::HRESULT,
    pub GetPronunciations: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, u16, *mut SPWORDPRONUNCIATIONLIST) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpErrorLog, ISpErrorLog_Vtbl, 0xf4711347_e608_11d2_a086_00c04f8ef9b5);
impl core::ops::Deref for ISpErrorLog {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpErrorLog, windows_core::IUnknown);
impl ISpErrorLog {
    pub unsafe fn AddError<P0, P1>(&self, llinenumber: i32, hr: windows_core::HRESULT, pszdescription: P0, pszhelpfile: P1, dwhelpcontext: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddError)(windows_core::Interface::as_raw(self), llinenumber, hr, pszdescription.param().abi(), pszhelpfile.param().abi(), dwhelpcontext).ok()
    }
}
#[repr(C)]
pub struct ISpErrorLog_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddError: unsafe extern "system" fn(*mut core::ffi::c_void, i32, windows_core::HRESULT, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpEventSink, ISpEventSink_Vtbl, 0xbe7a9cc9_5f9e_11d2_960f_00c04f8ee628);
impl core::ops::Deref for ISpEventSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpEventSink, windows_core::IUnknown);
impl ISpEventSink {
    pub unsafe fn AddEvents(&self, peventarray: *const SPEVENT, ulcount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddEvents)(windows_core::Interface::as_raw(self), peventarray, ulcount).ok()
    }
    pub unsafe fn GetEventInterest(&self, pulleventinterest: *mut u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetEventInterest)(windows_core::Interface::as_raw(self), pulleventinterest).ok()
    }
}
#[repr(C)]
pub struct ISpEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPEVENT, u32) -> windows_core::HRESULT,
    pub GetEventInterest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpEventSource, ISpEventSource_Vtbl, 0xbe7a9cce_5f9e_11d2_960f_00c04f8ee628);
impl core::ops::Deref for ISpEventSource {
    type Target = ISpNotifySource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpEventSource, windows_core::IUnknown, ISpNotifySource);
impl ISpEventSource {
    pub unsafe fn SetInterest(&self, ulleventinterest: u64, ullqueuedinterest: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInterest)(windows_core::Interface::as_raw(self), ulleventinterest, ullqueuedinterest).ok()
    }
    pub unsafe fn GetEvents(&self, ulcount: u32, peventarray: *mut SPEVENT, pulfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetEvents)(windows_core::Interface::as_raw(self), ulcount, peventarray, pulfetched).ok()
    }
    pub unsafe fn GetInfo(&self, pinfo: *mut SPEVENTSOURCEINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInfo)(windows_core::Interface::as_raw(self), pinfo).ok()
    }
}
#[repr(C)]
pub struct ISpEventSource_Vtbl {
    pub base__: ISpNotifySource_Vtbl,
    pub SetInterest: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64) -> windows_core::HRESULT,
    pub GetEvents: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut SPEVENT, *mut u32) -> windows_core::HRESULT,
    pub GetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPEVENTSOURCEINFO) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpEventSource2, ISpEventSource2_Vtbl, 0x2373a435_6a4b_429e_a6ac_d4231a61975b);
impl core::ops::Deref for ISpEventSource2 {
    type Target = ISpEventSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpEventSource2, windows_core::IUnknown, ISpNotifySource, ISpEventSource);
impl ISpEventSource2 {
    pub unsafe fn GetEventsEx(&self, ulcount: u32, peventarray: *mut SPEVENTEX, pulfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetEventsEx)(windows_core::Interface::as_raw(self), ulcount, peventarray, pulfetched).ok()
    }
}
#[repr(C)]
pub struct ISpEventSource2_Vtbl {
    pub base__: ISpEventSource_Vtbl,
    pub GetEventsEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut SPEVENTEX, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpGramCompBackend, ISpGramCompBackend_Vtbl, 0x3ddca27c_665c_4786_9f97_8c90c3488b61);
impl core::ops::Deref for ISpGramCompBackend {
    type Target = ISpGrammarBuilder;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpGramCompBackend, windows_core::IUnknown, ISpGrammarBuilder);
impl ISpGramCompBackend {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSaveObjects<P0, P1>(&self, pstream: P0, perrorlog: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<ISpErrorLog>,
    {
        (windows_core::Interface::vtable(self).SetSaveObjects)(windows_core::Interface::as_raw(self), pstream.param().abi(), perrorlog.param().abi()).ok()
    }
    pub unsafe fn InitFromBinaryGrammar(&self, pbinarydata: *const SPBINARYGRAMMAR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InitFromBinaryGrammar)(windows_core::Interface::as_raw(self), pbinarydata).ok()
    }
}
#[repr(C)]
pub struct ISpGramCompBackend_Vtbl {
    pub base__: ISpGrammarBuilder_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSaveObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSaveObjects: usize,
    pub InitFromBinaryGrammar: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPBINARYGRAMMAR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpGrammarBuilder, ISpGrammarBuilder_Vtbl, 0x8137828f_591a_4a42_be58_49ea7ebaac68);
impl core::ops::Deref for ISpGrammarBuilder {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpGrammarBuilder, windows_core::IUnknown);
impl ISpGrammarBuilder {
    pub unsafe fn ResetGrammar(&self, newlanguage: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResetGrammar)(windows_core::Interface::as_raw(self), newlanguage).ok()
    }
    pub unsafe fn GetRule<P0, P1>(&self, pszrulename: P0, dwruleid: u32, dwattributes: u32, fcreateifnotexist: P1, phinitialstate: *mut SPSTATEHANDLE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).GetRule)(windows_core::Interface::as_raw(self), pszrulename.param().abi(), dwruleid, dwattributes, fcreateifnotexist.param().abi(), phinitialstate).ok()
    }
    pub unsafe fn ClearRule<P0>(&self, hstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SPSTATEHANDLE>,
    {
        (windows_core::Interface::vtable(self).ClearRule)(windows_core::Interface::as_raw(self), hstate.param().abi()).ok()
    }
    pub unsafe fn CreateNewState<P0>(&self, hstate: P0, phstate: *mut SPSTATEHANDLE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SPSTATEHANDLE>,
    {
        (windows_core::Interface::vtable(self).CreateNewState)(windows_core::Interface::as_raw(self), hstate.param().abi(), phstate).ok()
    }
    pub unsafe fn AddWordTransition<P0, P1, P2, P3>(&self, hfromstate: P0, htostate: P1, psz: P2, pszseparators: P3, ewordtype: SPGRAMMARWORDTYPE, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SPSTATEHANDLE>,
        P1: windows_core::Param<SPSTATEHANDLE>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddWordTransition)(windows_core::Interface::as_raw(self), hfromstate.param().abi(), htostate.param().abi(), psz.param().abi(), pszseparators.param().abi(), ewordtype, weight, ppropinfo).ok()
    }
    pub unsafe fn AddRuleTransition<P0, P1, P2>(&self, hfromstate: P0, htostate: P1, hrule: P2, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SPSTATEHANDLE>,
        P1: windows_core::Param<SPSTATEHANDLE>,
        P2: windows_core::Param<SPSTATEHANDLE>,
    {
        (windows_core::Interface::vtable(self).AddRuleTransition)(windows_core::Interface::as_raw(self), hfromstate.param().abi(), htostate.param().abi(), hrule.param().abi(), weight, ppropinfo).ok()
    }
    pub unsafe fn AddResource<P0, P1, P2>(&self, hrulestate: P0, pszresourcename: P1, pszresourcevalue: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SPSTATEHANDLE>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddResource)(windows_core::Interface::as_raw(self), hrulestate.param().abi(), pszresourcename.param().abi(), pszresourcevalue.param().abi()).ok()
    }
    pub unsafe fn Commit(&self, dwreserved: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), dwreserved).ok()
    }
}
#[repr(C)]
pub struct ISpGrammarBuilder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ResetGrammar: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub GetRule: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, super::super::Foundation::BOOL, *mut SPSTATEHANDLE) -> windows_core::HRESULT,
    pub ClearRule: unsafe extern "system" fn(*mut core::ffi::c_void, SPSTATEHANDLE) -> windows_core::HRESULT,
    pub CreateNewState: unsafe extern "system" fn(*mut core::ffi::c_void, SPSTATEHANDLE, *mut SPSTATEHANDLE) -> windows_core::HRESULT,
    pub AddWordTransition: unsafe extern "system" fn(*mut core::ffi::c_void, SPSTATEHANDLE, SPSTATEHANDLE, windows_core::PCWSTR, windows_core::PCWSTR, SPGRAMMARWORDTYPE, f32, *const SPPROPERTYINFO) -> windows_core::HRESULT,
    pub AddRuleTransition: unsafe extern "system" fn(*mut core::ffi::c_void, SPSTATEHANDLE, SPSTATEHANDLE, SPSTATEHANDLE, f32, *const SPPROPERTYINFO) -> windows_core::HRESULT,
    pub AddResource: unsafe extern "system" fn(*mut core::ffi::c_void, SPSTATEHANDLE, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpGrammarBuilder2, ISpGrammarBuilder2_Vtbl, 0x8ab10026_20cc_4b20_8c22_a49c9ba78f60);
impl core::ops::Deref for ISpGrammarBuilder2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpGrammarBuilder2, windows_core::IUnknown);
impl ISpGrammarBuilder2 {
    pub unsafe fn AddTextSubset<P0, P1, P2>(&self, hfromstate: P0, htostate: P1, psz: P2, ematchmode: SPMATCHINGMODE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SPSTATEHANDLE>,
        P1: windows_core::Param<SPSTATEHANDLE>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddTextSubset)(windows_core::Interface::as_raw(self), hfromstate.param().abi(), htostate.param().abi(), psz.param().abi(), ematchmode).ok()
    }
    pub unsafe fn SetPhoneticAlphabet(&self, phoneticalphabet: PHONETICALPHABET) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPhoneticAlphabet)(windows_core::Interface::as_raw(self), phoneticalphabet).ok()
    }
}
#[repr(C)]
pub struct ISpGrammarBuilder2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddTextSubset: unsafe extern "system" fn(*mut core::ffi::c_void, SPSTATEHANDLE, SPSTATEHANDLE, windows_core::PCWSTR, SPMATCHINGMODE) -> windows_core::HRESULT,
    pub SetPhoneticAlphabet: unsafe extern "system" fn(*mut core::ffi::c_void, PHONETICALPHABET) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpGrammarCompiler, ISpGrammarCompiler_Vtbl, 0xb1e29d58_a675_11d2_8302_00c04f8ee6c0);
impl core::ops::Deref for ISpGrammarCompiler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpGrammarCompiler, windows_core::IUnknown);
impl ISpGrammarCompiler {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CompileStream<P0, P1, P2, P3, P4>(&self, psource: P0, pdest: P1, pheader: P2, preserved: P3, perrorlog: P4, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::System::Com::IStream>,
        P2: windows_core::Param<super::super::System::Com::IStream>,
        P3: windows_core::Param<windows_core::IUnknown>,
        P4: windows_core::Param<ISpErrorLog>,
    {
        (windows_core::Interface::vtable(self).CompileStream)(windows_core::Interface::as_raw(self), psource.param().abi(), pdest.param().abi(), pheader.param().abi(), preserved.param().abi(), perrorlog.param().abi(), dwflags).ok()
    }
}
#[repr(C)]
pub struct ISpGrammarCompiler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CompileStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CompileStream: usize,
}
windows_core::imp::define_interface!(ISpITNProcessor, ISpITNProcessor_Vtbl, 0x12d7360f_a1c9_11d3_bc90_00c04f72df9f);
impl core::ops::Deref for ISpITNProcessor {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpITNProcessor, windows_core::IUnknown);
impl ISpITNProcessor {
    pub unsafe fn LoadITNGrammar<P0>(&self, pszclsid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).LoadITNGrammar)(windows_core::Interface::as_raw(self), pszclsid.param().abi()).ok()
    }
    pub unsafe fn ITNPhrase<P0>(&self, pphrase: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpPhraseBuilder>,
    {
        (windows_core::Interface::vtable(self).ITNPhrase)(windows_core::Interface::as_raw(self), pphrase.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISpITNProcessor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LoadITNGrammar: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub ITNPhrase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpLexicon, ISpLexicon_Vtbl, 0xda41a7c2_5383_4db2_916b_6c1719e3db58);
impl core::ops::Deref for ISpLexicon {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpLexicon, windows_core::IUnknown);
impl ISpLexicon {
    pub unsafe fn GetPronunciations<P0>(&self, pszword: P0, langid: u16, dwflags: u32, pwordpronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetPronunciations)(windows_core::Interface::as_raw(self), pszword.param().abi(), langid, dwflags, pwordpronunciationlist).ok()
    }
    pub unsafe fn AddPronunciation<P0>(&self, pszword: P0, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: Option<*const u16>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddPronunciation)(windows_core::Interface::as_raw(self), pszword.param().abi(), langid, epartofspeech, core::mem::transmute(pszpronunciation.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn RemovePronunciation<P0>(&self, pszword: P0, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: Option<*const u16>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RemovePronunciation)(windows_core::Interface::as_raw(self), pszword.param().abi(), langid, epartofspeech, core::mem::transmute(pszpronunciation.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn GetGeneration(&self, pdwgeneration: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGeneration)(windows_core::Interface::as_raw(self), pdwgeneration).ok()
    }
    pub unsafe fn GetGenerationChange(&self, dwflags: u32, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGenerationChange)(windows_core::Interface::as_raw(self), dwflags, pdwgeneration, pwordlist).ok()
    }
    pub unsafe fn GetWords(&self, dwflags: u32, pdwgeneration: *mut u32, pdwcookie: Option<*mut u32>, pwordlist: *mut SPWORDLIST) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetWords)(windows_core::Interface::as_raw(self), dwflags, pdwgeneration, core::mem::transmute(pdwcookie.unwrap_or(std::ptr::null_mut())), pwordlist).ok()
    }
}
#[repr(C)]
pub struct ISpLexicon_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPronunciations: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u16, u32, *mut SPWORDPRONUNCIATIONLIST) -> windows_core::HRESULT,
    pub AddPronunciation: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u16, SPPARTOFSPEECH, *const u16) -> windows_core::HRESULT,
    pub RemovePronunciation: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u16, SPPARTOFSPEECH, *const u16) -> windows_core::HRESULT,
    pub GetGeneration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetGenerationChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut SPWORDLIST) -> windows_core::HRESULT,
    pub GetWords: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32, *mut SPWORDLIST) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpMMSysAudio, ISpMMSysAudio_Vtbl, 0x15806f6e_1d70_4b48_98e6_3b1a007509ab);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpMMSysAudio {
    type Target = ISpAudio;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpMMSysAudio, windows_core::IUnknown, super::super::System::Com::ISequentialStream, super::super::System::Com::IStream, ISpStreamFormat, ISpAudio);
#[cfg(feature = "Win32_System_Com")]
impl ISpMMSysAudio {
    pub unsafe fn GetDeviceId(&self, pudeviceid: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDeviceId)(windows_core::Interface::as_raw(self), pudeviceid).ok()
    }
    pub unsafe fn SetDeviceId(&self, udeviceid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDeviceId)(windows_core::Interface::as_raw(self), udeviceid).ok()
    }
    pub unsafe fn GetMMHandle(&self, phandle: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMMHandle)(windows_core::Interface::as_raw(self), phandle).ok()
    }
    pub unsafe fn GetLineId(&self, pulineid: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLineId)(windows_core::Interface::as_raw(self), pulineid).ok()
    }
    pub unsafe fn SetLineId(&self, ulineid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLineId)(windows_core::Interface::as_raw(self), ulineid).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpMMSysAudio_Vtbl {
    pub base__: ISpAudio_Vtbl,
    pub GetDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMMHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLineId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetLineId: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpNotifyCallback, ISpNotifyCallback_Vtbl);
impl ISpNotifyCallback {
    pub unsafe fn NotifyCallback<P0, P1>(&self, wparam: P0, lparam: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::WPARAM>,
        P1: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).NotifyCallback)(windows_core::Interface::as_raw(self), wparam.param().abi(), lparam.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISpNotifyCallback_Vtbl {
    pub NotifyCallback: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpNotifySink, ISpNotifySink_Vtbl, 0x259684dc_37c3_11d2_9603_00c04f8ee628);
impl core::ops::Deref for ISpNotifySink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpNotifySink, windows_core::IUnknown);
impl ISpNotifySink {
    pub unsafe fn Notify(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ISpNotifySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Notify: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpNotifySource, ISpNotifySource_Vtbl, 0x5eff4aef_8487_11d2_961c_00c04f8ee628);
impl core::ops::Deref for ISpNotifySource {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpNotifySource, windows_core::IUnknown);
impl ISpNotifySource {
    pub unsafe fn SetNotifySink<P0>(&self, pnotifysink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpNotifySink>,
    {
        (windows_core::Interface::vtable(self).SetNotifySink)(windows_core::Interface::as_raw(self), pnotifysink.param().abi()).ok()
    }
    pub unsafe fn SetNotifyWindowMessage<P0, P1, P2>(&self, hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<super::super::Foundation::WPARAM>,
        P2: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).SetNotifyWindowMessage)(windows_core::Interface::as_raw(self), hwnd.param().abi(), msg, wparam.param().abi(), lparam.param().abi()).ok()
    }
    pub unsafe fn SetNotifyCallbackFunction<P0, P1>(&self, pfncallback: *mut SPNOTIFYCALLBACK, wparam: P0, lparam: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::WPARAM>,
        P1: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).SetNotifyCallbackFunction)(windows_core::Interface::as_raw(self), pfncallback, wparam.param().abi(), lparam.param().abi()).ok()
    }
    pub unsafe fn SetNotifyCallbackInterface<P0, P1, P2>(&self, pspcallback: P0, wparam: P1, lparam: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpNotifyCallback>,
        P1: windows_core::Param<super::super::Foundation::WPARAM>,
        P2: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).SetNotifyCallbackInterface)(windows_core::Interface::as_raw(self), pspcallback.param().abi(), wparam.param().abi(), lparam.param().abi()).ok()
    }
    pub unsafe fn SetNotifyWin32Event(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNotifyWin32Event)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForNotifyEvent(&self, dwmilliseconds: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WaitForNotifyEvent)(windows_core::Interface::as_raw(self), dwmilliseconds).ok()
    }
    pub unsafe fn GetNotifyEventHandle(&self) -> super::super::Foundation::HANDLE {
        (windows_core::Interface::vtable(self).GetNotifyEventHandle)(windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
pub struct ISpNotifySource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetNotifySink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNotifyWindowMessage: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
    pub SetNotifyCallbackFunction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPNOTIFYCALLBACK, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
    pub SetNotifyCallbackInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
    pub SetNotifyWin32Event: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WaitForNotifyEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetNotifyEventHandle: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::HANDLE,
}
windows_core::imp::define_interface!(ISpNotifyTranslator, ISpNotifyTranslator_Vtbl, 0xaca16614_5d3d_11d2_960e_00c04f8ee628);
impl core::ops::Deref for ISpNotifyTranslator {
    type Target = ISpNotifySink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpNotifyTranslator, windows_core::IUnknown, ISpNotifySink);
impl ISpNotifyTranslator {
    pub unsafe fn InitWindowMessage<P0, P1, P2>(&self, hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<super::super::Foundation::WPARAM>,
        P2: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).InitWindowMessage)(windows_core::Interface::as_raw(self), hwnd.param().abi(), msg, wparam.param().abi(), lparam.param().abi()).ok()
    }
    pub unsafe fn InitCallback<P0, P1>(&self, pfncallback: *mut SPNOTIFYCALLBACK, wparam: P0, lparam: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::WPARAM>,
        P1: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).InitCallback)(windows_core::Interface::as_raw(self), pfncallback, wparam.param().abi(), lparam.param().abi()).ok()
    }
    pub unsafe fn InitSpNotifyCallback<P0, P1, P2>(&self, pspcallback: P0, wparam: P1, lparam: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpNotifyCallback>,
        P1: windows_core::Param<super::super::Foundation::WPARAM>,
        P2: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).InitSpNotifyCallback)(windows_core::Interface::as_raw(self), pspcallback.param().abi(), wparam.param().abi(), lparam.param().abi()).ok()
    }
    pub unsafe fn InitWin32Event<P0, P1>(&self, hevent: P0, fclosehandleonrelease: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).InitWin32Event)(windows_core::Interface::as_raw(self), hevent.param().abi(), fclosehandleonrelease.param().abi()).ok()
    }
    pub unsafe fn Wait(&self, dwmilliseconds: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Wait)(windows_core::Interface::as_raw(self), dwmilliseconds).ok()
    }
    pub unsafe fn GetEventHandle(&self) -> super::super::Foundation::HANDLE {
        (windows_core::Interface::vtable(self).GetEventHandle)(windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
pub struct ISpNotifyTranslator_Vtbl {
    pub base__: ISpNotifySink_Vtbl,
    pub InitWindowMessage: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
    pub InitCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPNOTIFYCALLBACK, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
    pub InitSpNotifyCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
    pub InitWin32Event: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Wait: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetEventHandle: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::HANDLE,
}
windows_core::imp::define_interface!(ISpObjectToken, ISpObjectToken_Vtbl, 0x14056589_e16c_11d2_bb90_00c04f8ee6c0);
impl core::ops::Deref for ISpObjectToken {
    type Target = ISpDataKey;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpObjectToken, windows_core::IUnknown, ISpDataKey);
impl ISpObjectToken {
    pub unsafe fn SetId<P0, P1, P2>(&self, pszcategoryid: P0, psztokenid: P1, fcreateifnotexist: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetId)(windows_core::Interface::as_raw(self), pszcategoryid.param().abi(), psztokenid.param().abi(), fcreateifnotexist.param().abi()).ok()
    }
    pub unsafe fn GetId(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCategory(&self) -> windows_core::Result<ISpObjectTokenCategory> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCategory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateInstance<P0>(&self, punkouter: P0, dwclscontext: u32, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), punkouter.param().abi(), dwclscontext, riid, ppvobject).ok()
    }
    pub unsafe fn GetStorageFileName<P0, P1>(&self, clsidcaller: *const windows_core::GUID, pszvaluename: P0, pszfilenamespecifier: P1, nfolder: u32) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStorageFileName)(windows_core::Interface::as_raw(self), clsidcaller, pszvaluename.param().abi(), pszfilenamespecifier.param().abi(), nfolder, &mut result__).map(|| result__)
    }
    pub unsafe fn RemoveStorageFileName<P0, P1>(&self, clsidcaller: *const windows_core::GUID, pszkeyname: P0, fdeletefile: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).RemoveStorageFileName)(windows_core::Interface::as_raw(self), clsidcaller, pszkeyname.param().abi(), fdeletefile.param().abi()).ok()
    }
    pub unsafe fn Remove(&self, pclsidcaller: Option<*const windows_core::GUID>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute(pclsidcaller.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn IsUISupported<P0, P1>(&self, psztypeofui: P0, pvextradata: *mut core::ffi::c_void, cbextradata: u32, punkobject: P1, pfsupported: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).IsUISupported)(windows_core::Interface::as_raw(self), psztypeofui.param().abi(), pvextradata, cbextradata, punkobject.param().abi(), pfsupported).ok()
    }
    pub unsafe fn DisplayUI<P0, P1, P2, P3>(&self, hwndparent: P0, psztitle: P1, psztypeofui: P2, pvextradata: *mut core::ffi::c_void, cbextradata: u32, punkobject: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).DisplayUI)(windows_core::Interface::as_raw(self), hwndparent.param().abi(), psztitle.param().abi(), psztypeofui.param().abi(), pvextradata, cbextradata, punkobject.param().abi()).ok()
    }
    pub unsafe fn MatchesAttributes<P0>(&self, pszattributes: P0, pfmatches: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).MatchesAttributes)(windows_core::Interface::as_raw(self), pszattributes.param().abi(), pfmatches).ok()
    }
}
#[repr(C)]
pub struct ISpObjectToken_Vtbl {
    pub base__: ISpDataKey_Vtbl,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStorageFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, windows_core::PCWSTR, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub RemoveStorageFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub IsUISupported: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub DisplayUI: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MatchesAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpObjectTokenCategory, ISpObjectTokenCategory_Vtbl, 0x2d3d3845_39af_4850_bbf9_40b49780011d);
impl core::ops::Deref for ISpObjectTokenCategory {
    type Target = ISpDataKey;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpObjectTokenCategory, windows_core::IUnknown, ISpDataKey);
impl ISpObjectTokenCategory {
    pub unsafe fn SetId<P0, P1>(&self, pszcategoryid: P0, fcreateifnotexist: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetId)(windows_core::Interface::as_raw(self), pszcategoryid.param().abi(), fcreateifnotexist.param().abi()).ok()
    }
    pub unsafe fn GetId(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDataKey(&self, spdkl: SPDATAKEYLOCATION) -> windows_core::Result<ISpDataKey> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDataKey)(windows_core::Interface::as_raw(self), spdkl, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumTokens<P0, P1>(&self, pzsreqattribs: P0, pszoptattribs: P1) -> windows_core::Result<IEnumSpObjectTokens>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumTokens)(windows_core::Interface::as_raw(self), pzsreqattribs.param().abi(), pszoptattribs.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDefaultTokenId<P0>(&self, psztokenid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetDefaultTokenId)(windows_core::Interface::as_raw(self), psztokenid.param().abi()).ok()
    }
    pub unsafe fn GetDefaultTokenId(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDefaultTokenId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISpObjectTokenCategory_Vtbl {
    pub base__: ISpDataKey_Vtbl,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetDataKey: unsafe extern "system" fn(*mut core::ffi::c_void, SPDATAKEYLOCATION, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumTokens: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDefaultTokenId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetDefaultTokenId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpObjectTokenEnumBuilder, ISpObjectTokenEnumBuilder_Vtbl, 0x06b64f9f_7fda_11d2_b4f2_00c04f797396);
impl core::ops::Deref for ISpObjectTokenEnumBuilder {
    type Target = IEnumSpObjectTokens;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpObjectTokenEnumBuilder, windows_core::IUnknown, IEnumSpObjectTokens);
impl ISpObjectTokenEnumBuilder {
    pub unsafe fn SetAttribs<P0, P1>(&self, pszreqattribs: P0, pszoptattribs: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetAttribs)(windows_core::Interface::as_raw(self), pszreqattribs.param().abi(), pszoptattribs.param().abi()).ok()
    }
    pub unsafe fn AddTokens(&self, ctokens: u32, ptoken: *const Option<ISpObjectToken>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddTokens)(windows_core::Interface::as_raw(self), ctokens, core::mem::transmute(ptoken)).ok()
    }
    pub unsafe fn AddTokensFromDataKey<P0, P1, P2>(&self, pdatakey: P0, pszsubkey: P1, pszcategoryid: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpDataKey>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddTokensFromDataKey)(windows_core::Interface::as_raw(self), pdatakey.param().abi(), pszsubkey.param().abi(), pszcategoryid.param().abi()).ok()
    }
    pub unsafe fn AddTokensFromTokenEnum<P0>(&self, ptokenenum: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IEnumSpObjectTokens>,
    {
        (windows_core::Interface::vtable(self).AddTokensFromTokenEnum)(windows_core::Interface::as_raw(self), ptokenenum.param().abi()).ok()
    }
    pub unsafe fn Sort<P0>(&self, psztokenidtolistfirst: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Sort)(windows_core::Interface::as_raw(self), psztokenidtolistfirst.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISpObjectTokenEnumBuilder_Vtbl {
    pub base__: IEnumSpObjectTokens_Vtbl,
    pub SetAttribs: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub AddTokens: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddTokensFromDataKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub AddTokensFromTokenEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Sort: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpObjectTokenInit, ISpObjectTokenInit_Vtbl, 0xb8aab0cf_346f_49d8_9499_c8b03f161d51);
impl core::ops::Deref for ISpObjectTokenInit {
    type Target = ISpObjectToken;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpObjectTokenInit, windows_core::IUnknown, ISpDataKey, ISpObjectToken);
impl ISpObjectTokenInit {
    pub unsafe fn InitFromDataKey<P0, P1, P2>(&self, pszcategoryid: P0, psztokenid: P1, pdatakey: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<ISpDataKey>,
    {
        (windows_core::Interface::vtable(self).InitFromDataKey)(windows_core::Interface::as_raw(self), pszcategoryid.param().abi(), psztokenid.param().abi(), pdatakey.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISpObjectTokenInit_Vtbl {
    pub base__: ISpObjectToken_Vtbl,
    pub InitFromDataKey: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpObjectWithToken, ISpObjectWithToken_Vtbl, 0x5b559f40_e952_11d2_bb91_00c04f8ee6c0);
impl core::ops::Deref for ISpObjectWithToken {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpObjectWithToken, windows_core::IUnknown);
impl ISpObjectWithToken {
    pub unsafe fn SetObjectToken<P0>(&self, ptoken: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpObjectToken>,
    {
        (windows_core::Interface::vtable(self).SetObjectToken)(windows_core::Interface::as_raw(self), ptoken.param().abi()).ok()
    }
    pub unsafe fn GetObjectToken(&self) -> windows_core::Result<ISpObjectToken> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetObjectToken)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISpObjectWithToken_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetObjectToken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObjectToken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpPhoneConverter, ISpPhoneConverter_Vtbl, 0x8445c581_0cac_4a38_abfe_9b2ce2826455);
impl core::ops::Deref for ISpPhoneConverter {
    type Target = ISpObjectWithToken;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpPhoneConverter, windows_core::IUnknown, ISpObjectWithToken);
impl ISpPhoneConverter {
    pub unsafe fn PhoneToId<P0>(&self, pszphone: P0) -> windows_core::Result<u16>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PhoneToId)(windows_core::Interface::as_raw(self), pszphone.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn IdToPhone(&self, pid: *const u16, pszphone: windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IdToPhone)(windows_core::Interface::as_raw(self), pid, core::mem::transmute(pszphone)).ok()
    }
}
#[repr(C)]
pub struct ISpPhoneConverter_Vtbl {
    pub base__: ISpObjectWithToken_Vtbl,
    pub PhoneToId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u16) -> windows_core::HRESULT,
    pub IdToPhone: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpPhoneticAlphabetConverter, ISpPhoneticAlphabetConverter_Vtbl, 0x133adcd4_19b4_4020_9fdc_842e78253b17);
impl core::ops::Deref for ISpPhoneticAlphabetConverter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpPhoneticAlphabetConverter, windows_core::IUnknown);
impl ISpPhoneticAlphabetConverter {
    pub unsafe fn GetLangId(&self) -> windows_core::Result<u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLangId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLangId(&self, langid: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLangId)(windows_core::Interface::as_raw(self), langid).ok()
    }
    pub unsafe fn SAPI2UPS(&self, pszsapiid: *const u16, pszupsid: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SAPI2UPS)(windows_core::Interface::as_raw(self), pszsapiid, core::mem::transmute(pszupsid.as_ptr()), pszupsid.len().try_into().unwrap()).ok()
    }
    pub unsafe fn UPS2SAPI(&self, pszupsid: *const u16, pszsapiid: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UPS2SAPI)(windows_core::Interface::as_raw(self), pszupsid, core::mem::transmute(pszsapiid.as_ptr()), pszsapiid.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetMaxConvertLength<P0>(&self, csrclength: u32, bsapi2ups: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxConvertLength)(windows_core::Interface::as_raw(self), csrclength, bsapi2ups.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISpPhoneticAlphabetConverter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLangId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub SetLangId: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub SAPI2UPS: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut u16, u32) -> windows_core::HRESULT,
    pub UPS2SAPI: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut u16, u32) -> windows_core::HRESULT,
    pub GetMaxConvertLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::BOOL, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpPhoneticAlphabetSelection, ISpPhoneticAlphabetSelection_Vtbl, 0xb2745efd_42ce_48ca_81f1_a96e02538a90);
impl core::ops::Deref for ISpPhoneticAlphabetSelection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpPhoneticAlphabetSelection, windows_core::IUnknown);
impl ISpPhoneticAlphabetSelection {
    pub unsafe fn IsAlphabetUPS(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsAlphabetUPS)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAlphabetToUPS<P0>(&self, fforceups: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAlphabetToUPS)(windows_core::Interface::as_raw(self), fforceups.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISpPhoneticAlphabetSelection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsAlphabetUPS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetAlphabetToUPS: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpPhrase, ISpPhrase_Vtbl, 0x1a5c0354_b621_4b5a_8791_d306ed379e53);
impl core::ops::Deref for ISpPhrase {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpPhrase, windows_core::IUnknown);
impl ISpPhrase {
    pub unsafe fn GetPhrase(&self) -> windows_core::Result<*mut SPPHRASE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPhrase)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSerializedPhrase(&self) -> windows_core::Result<*mut SPSERIALIZEDPHRASE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSerializedPhrase)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetText<P0>(&self, ulstart: u32, ulcount: u32, fusetextreplacements: P0, ppszcomemtext: *mut windows_core::PWSTR, pbdisplayattributes: Option<*mut u8>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), ulstart, ulcount, fusetextreplacements.param().abi(), ppszcomemtext, core::mem::transmute(pbdisplayattributes.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Discard(&self, dwvaluetypes: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Discard)(windows_core::Interface::as_raw(self), dwvaluetypes).ok()
    }
}
#[repr(C)]
pub struct ISpPhrase_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPhrase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut SPPHRASE) -> windows_core::HRESULT,
    pub GetSerializedPhrase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut SPSERIALIZEDPHRASE) -> windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::super::Foundation::BOOL, *mut windows_core::PWSTR, *mut u8) -> windows_core::HRESULT,
    pub Discard: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpPhrase2, ISpPhrase2_Vtbl, 0xf264da52_e457_4696_b856_a737b717af79);
impl core::ops::Deref for ISpPhrase2 {
    type Target = ISpPhrase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpPhrase2, windows_core::IUnknown, ISpPhrase);
impl ISpPhrase2 {
    pub unsafe fn GetXMLResult(&self, ppszcomemxmlresult: *mut windows_core::PWSTR, options: SPXMLRESULTOPTIONS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetXMLResult)(windows_core::Interface::as_raw(self), ppszcomemxmlresult, options).ok()
    }
    pub unsafe fn GetXMLErrorInfo(&self, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetXMLErrorInfo)(windows_core::Interface::as_raw(self), psemanticerrorinfo).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAudio(&self, ulstartelement: u32, celements: u32) -> windows_core::Result<ISpStreamFormat> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAudio)(windows_core::Interface::as_raw(self), ulstartelement, celements, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISpPhrase2_Vtbl {
    pub base__: ISpPhrase_Vtbl,
    pub GetXMLResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, SPXMLRESULTOPTIONS) -> windows_core::HRESULT,
    pub GetXMLErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPSEMANTICERRORINFO) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAudio: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAudio: usize,
}
windows_core::imp::define_interface!(ISpPhraseAlt, ISpPhraseAlt_Vtbl, 0x8fcebc98_4e49_4067_9c6c_d86a0e092e3d);
impl core::ops::Deref for ISpPhraseAlt {
    type Target = ISpPhrase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpPhraseAlt, windows_core::IUnknown, ISpPhrase);
impl ISpPhraseAlt {
    pub unsafe fn GetAltInfo(&self, ppparent: *mut Option<ISpPhrase>, pulstartelementinparent: *mut u32, pcelementsinparent: *mut u32, pcelementsinalt: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAltInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(ppparent), pulstartelementinparent, pcelementsinparent, pcelementsinalt).ok()
    }
    pub unsafe fn Commit(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ISpPhraseAlt_Vtbl {
    pub base__: ISpPhrase_Vtbl,
    pub GetAltInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpPhraseBuilder, ISpPhraseBuilder_Vtbl, 0x88a3342a_0bed_4834_922b_88d43173162f);
impl core::ops::Deref for ISpPhraseBuilder {
    type Target = ISpPhrase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpPhraseBuilder, windows_core::IUnknown, ISpPhrase);
impl ISpPhraseBuilder {
    pub unsafe fn InitFromPhrase(&self, pphrase: *const SPPHRASE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InitFromPhrase)(windows_core::Interface::as_raw(self), pphrase).ok()
    }
    pub unsafe fn InitFromSerializedPhrase(&self, pphrase: *const SPSERIALIZEDPHRASE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InitFromSerializedPhrase)(windows_core::Interface::as_raw(self), pphrase).ok()
    }
    pub unsafe fn AddElements(&self, celements: u32, pelement: *const SPPHRASEELEMENT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddElements)(windows_core::Interface::as_raw(self), celements, pelement).ok()
    }
    pub unsafe fn AddRules<P0>(&self, hparent: P0, prule: *const SPPHRASERULE) -> windows_core::Result<SPPHRASERULEHANDLE>
    where
        P0: windows_core::Param<SPPHRASERULEHANDLE>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddRules)(windows_core::Interface::as_raw(self), hparent.param().abi(), prule, &mut result__).map(|| result__)
    }
    pub unsafe fn AddProperties<P0>(&self, hparent: P0, pproperty: *const SPPHRASEPROPERTY) -> windows_core::Result<SPPHRASEPROPERTYHANDLE>
    where
        P0: windows_core::Param<SPPHRASEPROPERTYHANDLE>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddProperties)(windows_core::Interface::as_raw(self), hparent.param().abi(), pproperty, &mut result__).map(|| result__)
    }
    pub unsafe fn AddReplacements(&self, creplacements: u32, preplacements: *const SPPHRASEREPLACEMENT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddReplacements)(windows_core::Interface::as_raw(self), creplacements, preplacements).ok()
    }
}
#[repr(C)]
pub struct ISpPhraseBuilder_Vtbl {
    pub base__: ISpPhrase_Vtbl,
    pub InitFromPhrase: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPPHRASE) -> windows_core::HRESULT,
    pub InitFromSerializedPhrase: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPSERIALIZEDPHRASE) -> windows_core::HRESULT,
    pub AddElements: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const SPPHRASEELEMENT) -> windows_core::HRESULT,
    pub AddRules: unsafe extern "system" fn(*mut core::ffi::c_void, SPPHRASERULEHANDLE, *const SPPHRASERULE, *mut SPPHRASERULEHANDLE) -> windows_core::HRESULT,
    pub AddProperties: unsafe extern "system" fn(*mut core::ffi::c_void, SPPHRASEPROPERTYHANDLE, *const SPPHRASEPROPERTY, *mut SPPHRASEPROPERTYHANDLE) -> windows_core::HRESULT,
    pub AddReplacements: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const SPPHRASEREPLACEMENT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpPrivateEngineCallEx, ISpPrivateEngineCallEx_Vtbl, 0xdefd682a_fe0a_42b9_bfa1_56d3d6cecfaf);
impl core::ops::Deref for ISpPrivateEngineCallEx {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpPrivateEngineCallEx, windows_core::IUnknown);
impl ISpPrivateEngineCallEx {
    pub unsafe fn CallEngineSynchronize(&self, pinframe: *const core::ffi::c_void, ulinframesize: u32, ppcomemoutframe: *mut *mut core::ffi::c_void, puloutframesize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CallEngineSynchronize)(windows_core::Interface::as_raw(self), pinframe, ulinframesize, ppcomemoutframe, puloutframesize).ok()
    }
    pub unsafe fn CallEngineImmediate(&self, pinframe: *const core::ffi::c_void, ulinframesize: u32, ppcomemoutframe: *mut *mut core::ffi::c_void, puloutframesize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CallEngineImmediate)(windows_core::Interface::as_raw(self), pinframe, ulinframesize, ppcomemoutframe, puloutframesize).ok()
    }
}
#[repr(C)]
pub struct ISpPrivateEngineCallEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CallEngineSynchronize: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CallEngineImmediate: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpProperties, ISpProperties_Vtbl, 0x5b4fb971_b115_4de1_ad97_e482e3bf6ee4);
impl core::ops::Deref for ISpProperties {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpProperties, windows_core::IUnknown);
impl ISpProperties {
    pub unsafe fn SetPropertyNum<P0>(&self, pname: P0, lvalue: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetPropertyNum)(windows_core::Interface::as_raw(self), pname.param().abi(), lvalue).ok()
    }
    pub unsafe fn GetPropertyNum<P0>(&self, pname: P0, plvalue: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetPropertyNum)(windows_core::Interface::as_raw(self), pname.param().abi(), plvalue).ok()
    }
    pub unsafe fn SetPropertyString<P0, P1>(&self, pname: P0, pvalue: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetPropertyString)(windows_core::Interface::as_raw(self), pname.param().abi(), pvalue.param().abi()).ok()
    }
    pub unsafe fn GetPropertyString<P0>(&self, pname: P0) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPropertyString)(windows_core::Interface::as_raw(self), pname.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISpProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetPropertyNum: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32) -> windows_core::HRESULT,
    pub GetPropertyNum: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut i32) -> windows_core::HRESULT,
    pub SetPropertyString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetPropertyString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpRecoContext, ISpRecoContext_Vtbl, 0xf740a62f_7c15_489e_8234_940a33d9272d);
impl core::ops::Deref for ISpRecoContext {
    type Target = ISpEventSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpRecoContext, windows_core::IUnknown, ISpNotifySource, ISpEventSource);
impl ISpRecoContext {
    pub unsafe fn GetRecognizer(&self) -> windows_core::Result<ISpRecognizer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRecognizer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateGrammar(&self, ullgrammarid: u64) -> windows_core::Result<ISpRecoGrammar> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateGrammar)(windows_core::Interface::as_raw(self), ullgrammarid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetStatus(&self, pstatus: *mut SPRECOCONTEXTSTATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), pstatus).ok()
    }
    pub unsafe fn GetMaxAlternates(&self, pcalternates: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMaxAlternates)(windows_core::Interface::as_raw(self), pcalternates).ok()
    }
    pub unsafe fn SetMaxAlternates(&self, calternates: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxAlternates)(windows_core::Interface::as_raw(self), calternates).ok()
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAudioOptions(&self, options: SPAUDIOOPTIONS, paudioformatid: *const windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAudioOptions)(windows_core::Interface::as_raw(self), options, paudioformatid, pwaveformatex).ok()
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAudioOptions(&self, poptions: *mut SPAUDIOOPTIONS, paudioformatid: *mut windows_core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAudioOptions)(windows_core::Interface::as_raw(self), poptions, paudioformatid, ppcomemwfex).ok()
    }
    pub unsafe fn DeserializeResult(&self, pserializedresult: *const SPSERIALIZEDRESULT) -> windows_core::Result<ISpRecoResult> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DeserializeResult)(windows_core::Interface::as_raw(self), pserializedresult, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Bookmark<P0>(&self, options: SPBOOKMARKOPTIONS, ullstreamposition: u64, lparamevent: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).Bookmark)(windows_core::Interface::as_raw(self), options, ullstreamposition, lparamevent.param().abi()).ok()
    }
    pub unsafe fn SetAdaptationData<P0>(&self, padaptationdata: P0, cch: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetAdaptationData)(windows_core::Interface::as_raw(self), padaptationdata.param().abi(), cch).ok()
    }
    pub unsafe fn Pause(&self, dwreserved: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self), dwreserved).ok()
    }
    pub unsafe fn Resume(&self, dwreserved: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self), dwreserved).ok()
    }
    pub unsafe fn SetVoice<P0, P1>(&self, pvoice: P0, fallowformatchanges: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpVoice>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetVoice)(windows_core::Interface::as_raw(self), pvoice.param().abi(), fallowformatchanges.param().abi()).ok()
    }
    pub unsafe fn GetVoice(&self) -> windows_core::Result<ISpVoice> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVoice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetVoicePurgeEvent(&self, ulleventinterest: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetVoicePurgeEvent)(windows_core::Interface::as_raw(self), ulleventinterest).ok()
    }
    pub unsafe fn GetVoicePurgeEvent(&self, pulleventinterest: *mut u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetVoicePurgeEvent)(windows_core::Interface::as_raw(self), pulleventinterest).ok()
    }
    pub unsafe fn SetContextState(&self, econtextstate: SPCONTEXTSTATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetContextState)(windows_core::Interface::as_raw(self), econtextstate).ok()
    }
    pub unsafe fn GetContextState(&self, pecontextstate: *mut SPCONTEXTSTATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetContextState)(windows_core::Interface::as_raw(self), pecontextstate).ok()
    }
}
#[repr(C)]
pub struct ISpRecoContext_Vtbl {
    pub base__: ISpEventSource_Vtbl,
    pub GetRecognizer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateGrammar: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPRECOCONTEXTSTATUS) -> windows_core::HRESULT,
    pub GetMaxAlternates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaxAlternates: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub SetAudioOptions: unsafe extern "system" fn(*mut core::ffi::c_void, SPAUDIOOPTIONS, *const windows_core::GUID, *const super::Audio::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    SetAudioOptions: usize,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetAudioOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPAUDIOOPTIONS, *mut windows_core::GUID, *mut *mut super::Audio::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetAudioOptions: usize,
    pub DeserializeResult: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPSERIALIZEDRESULT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Bookmark: unsafe extern "system" fn(*mut core::ffi::c_void, SPBOOKMARKOPTIONS, u64, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
    pub SetAdaptationData: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetVoice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetVoice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetVoicePurgeEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub GetVoicePurgeEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub SetContextState: unsafe extern "system" fn(*mut core::ffi::c_void, SPCONTEXTSTATE) -> windows_core::HRESULT,
    pub GetContextState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPCONTEXTSTATE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpRecoContext2, ISpRecoContext2_Vtbl, 0xbead311c_52ff_437f_9464_6b21054ca73d);
impl core::ops::Deref for ISpRecoContext2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpRecoContext2, windows_core::IUnknown);
impl ISpRecoContext2 {
    pub unsafe fn SetGrammarOptions(&self, egrammaroptions: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGrammarOptions)(windows_core::Interface::as_raw(self), egrammaroptions).ok()
    }
    pub unsafe fn GetGrammarOptions(&self, pegrammaroptions: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGrammarOptions)(windows_core::Interface::as_raw(self), pegrammaroptions).ok()
    }
    pub unsafe fn SetAdaptationData2<P0, P1>(&self, padaptationdata: P0, cch: u32, ptopicname: P1, eadaptationsettings: u32, erelevance: SPADAPTATIONRELEVANCE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetAdaptationData2)(windows_core::Interface::as_raw(self), padaptationdata.param().abi(), cch, ptopicname.param().abi(), eadaptationsettings, erelevance).ok()
    }
}
#[repr(C)]
pub struct ISpRecoContext2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetGrammarOptions: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetGrammarOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetAdaptationData2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR, u32, SPADAPTATIONRELEVANCE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpRecoGrammar, ISpRecoGrammar_Vtbl, 0x2177db29_7f45_47d0_8554_067e91c80502);
impl core::ops::Deref for ISpRecoGrammar {
    type Target = ISpGrammarBuilder;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpRecoGrammar, windows_core::IUnknown, ISpGrammarBuilder);
impl ISpRecoGrammar {
    pub unsafe fn GetGrammarId(&self, pullgrammarid: *mut u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGrammarId)(windows_core::Interface::as_raw(self), pullgrammarid).ok()
    }
    pub unsafe fn GetRecoContext(&self) -> windows_core::Result<ISpRecoContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRecoContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LoadCmdFromFile<P0>(&self, pszfilename: P0, options: SPLOADOPTIONS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).LoadCmdFromFile)(windows_core::Interface::as_raw(self), pszfilename.param().abi(), options).ok()
    }
    pub unsafe fn LoadCmdFromObject<P0>(&self, rcid: *const windows_core::GUID, pszgrammarname: P0, options: SPLOADOPTIONS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).LoadCmdFromObject)(windows_core::Interface::as_raw(self), rcid, pszgrammarname.param().abi(), options).ok()
    }
    pub unsafe fn LoadCmdFromResource<P0, P1, P2>(&self, hmodule: P0, pszresourcename: P1, pszresourcetype: P2, wlanguage: u16, options: SPLOADOPTIONS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HMODULE>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).LoadCmdFromResource)(windows_core::Interface::as_raw(self), hmodule.param().abi(), pszresourcename.param().abi(), pszresourcetype.param().abi(), wlanguage, options).ok()
    }
    pub unsafe fn LoadCmdFromMemory(&self, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LoadCmdFromMemory)(windows_core::Interface::as_raw(self), pgrammar, options).ok()
    }
    pub unsafe fn LoadCmdFromProprietaryGrammar<P0>(&self, rguidparam: *const windows_core::GUID, pszstringparam: P0, pvdataprarm: *const core::ffi::c_void, cbdatasize: u32, options: SPLOADOPTIONS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).LoadCmdFromProprietaryGrammar)(windows_core::Interface::as_raw(self), rguidparam, pszstringparam.param().abi(), pvdataprarm, cbdatasize, options).ok()
    }
    pub unsafe fn SetRuleState<P0>(&self, pszname: P0, preserved: *mut core::ffi::c_void, newstate: SPRULESTATE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetRuleState)(windows_core::Interface::as_raw(self), pszname.param().abi(), preserved, newstate).ok()
    }
    pub unsafe fn SetRuleIdState(&self, ulruleid: u32, newstate: SPRULESTATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRuleIdState)(windows_core::Interface::as_raw(self), ulruleid, newstate).ok()
    }
    pub unsafe fn LoadDictation<P0>(&self, psztopicname: P0, options: SPLOADOPTIONS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).LoadDictation)(windows_core::Interface::as_raw(self), psztopicname.param().abi(), options).ok()
    }
    pub unsafe fn UnloadDictation(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnloadDictation)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetDictationState(&self, newstate: SPRULESTATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDictationState)(windows_core::Interface::as_raw(self), newstate).ok()
    }
    pub unsafe fn SetWordSequenceData(&self, ptext: Option<&[u16]>, pinfo: *const SPTEXTSELECTIONINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetWordSequenceData)(windows_core::Interface::as_raw(self), core::mem::transmute(ptext.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), ptext.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pinfo).ok()
    }
    pub unsafe fn SetTextSelection(&self, pinfo: *const SPTEXTSELECTIONINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTextSelection)(windows_core::Interface::as_raw(self), pinfo).ok()
    }
    pub unsafe fn IsPronounceable<P0>(&self, pszword: P0, pwordpronounceable: *mut SPWORDPRONOUNCEABLE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).IsPronounceable)(windows_core::Interface::as_raw(self), pszword.param().abi(), pwordpronounceable).ok()
    }
    pub unsafe fn SetGrammarState(&self, egrammarstate: SPGRAMMARSTATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGrammarState)(windows_core::Interface::as_raw(self), egrammarstate).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SaveCmd<P0>(&self, pstream: P0, ppszcomemerrortext: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).SaveCmd)(windows_core::Interface::as_raw(self), pstream.param().abi(), core::mem::transmute(ppszcomemerrortext.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetGrammarState(&self, pegrammarstate: *mut SPGRAMMARSTATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGrammarState)(windows_core::Interface::as_raw(self), pegrammarstate).ok()
    }
}
#[repr(C)]
pub struct ISpRecoGrammar_Vtbl {
    pub base__: ISpGrammarBuilder_Vtbl,
    pub GetGrammarId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetRecoContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoadCmdFromFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, SPLOADOPTIONS) -> windows_core::HRESULT,
    pub LoadCmdFromObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, SPLOADOPTIONS) -> windows_core::HRESULT,
    pub LoadCmdFromResource: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HMODULE, windows_core::PCWSTR, windows_core::PCWSTR, u16, SPLOADOPTIONS) -> windows_core::HRESULT,
    pub LoadCmdFromMemory: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPBINARYGRAMMAR, SPLOADOPTIONS) -> windows_core::HRESULT,
    pub LoadCmdFromProprietaryGrammar: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, *const core::ffi::c_void, u32, SPLOADOPTIONS) -> windows_core::HRESULT,
    pub SetRuleState: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, SPRULESTATE) -> windows_core::HRESULT,
    pub SetRuleIdState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, SPRULESTATE) -> windows_core::HRESULT,
    pub LoadDictation: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, SPLOADOPTIONS) -> windows_core::HRESULT,
    pub UnloadDictation: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDictationState: unsafe extern "system" fn(*mut core::ffi::c_void, SPRULESTATE) -> windows_core::HRESULT,
    pub SetWordSequenceData: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *const SPTEXTSELECTIONINFO) -> windows_core::HRESULT,
    pub SetTextSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPTEXTSELECTIONINFO) -> windows_core::HRESULT,
    pub IsPronounceable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut SPWORDPRONOUNCEABLE) -> windows_core::HRESULT,
    pub SetGrammarState: unsafe extern "system" fn(*mut core::ffi::c_void, SPGRAMMARSTATE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SaveCmd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SaveCmd: usize,
    pub GetGrammarState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPGRAMMARSTATE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpRecoGrammar2, ISpRecoGrammar2_Vtbl, 0x4b37bc9e_9ed6_44a3_93d3_18f022b79ec3);
impl core::ops::Deref for ISpRecoGrammar2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpRecoGrammar2, windows_core::IUnknown);
impl ISpRecoGrammar2 {
    pub unsafe fn GetRules(&self, ppcomemrules: *mut *mut SPRULE, punumrules: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRules)(windows_core::Interface::as_raw(self), ppcomemrules, punumrules).ok()
    }
    pub unsafe fn LoadCmdFromFile2<P0, P1, P2>(&self, pszfilename: P0, options: SPLOADOPTIONS, pszsharinguri: P1, pszbaseuri: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).LoadCmdFromFile2)(windows_core::Interface::as_raw(self), pszfilename.param().abi(), options, pszsharinguri.param().abi(), pszbaseuri.param().abi()).ok()
    }
    pub unsafe fn LoadCmdFromMemory2<P0, P1>(&self, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS, pszsharinguri: P0, pszbaseuri: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).LoadCmdFromMemory2)(windows_core::Interface::as_raw(self), pgrammar, options, pszsharinguri.param().abi(), pszbaseuri.param().abi()).ok()
    }
    pub unsafe fn SetRulePriority<P0>(&self, pszrulename: P0, ulruleid: u32, nrulepriority: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetRulePriority)(windows_core::Interface::as_raw(self), pszrulename.param().abi(), ulruleid, nrulepriority).ok()
    }
    pub unsafe fn SetRuleWeight<P0>(&self, pszrulename: P0, ulruleid: u32, flweight: f32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetRuleWeight)(windows_core::Interface::as_raw(self), pszrulename.param().abi(), ulruleid, flweight).ok()
    }
    pub unsafe fn SetDictationWeight(&self, flweight: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDictationWeight)(windows_core::Interface::as_raw(self), flweight).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetGrammarLoader<P0>(&self, ploader: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpeechResourceLoader>,
    {
        (windows_core::Interface::vtable(self).SetGrammarLoader)(windows_core::Interface::as_raw(self), ploader.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com_Urlmon")]
    pub unsafe fn SetSMLSecurityManager<P0>(&self, psmlsecuritymanager: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::Urlmon::IInternetSecurityManager>,
    {
        (windows_core::Interface::vtable(self).SetSMLSecurityManager)(windows_core::Interface::as_raw(self), psmlsecuritymanager.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISpRecoGrammar2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut SPRULE, *mut u32) -> windows_core::HRESULT,
    pub LoadCmdFromFile2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, SPLOADOPTIONS, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub LoadCmdFromMemory2: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPBINARYGRAMMAR, SPLOADOPTIONS, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetRulePriority: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, i32) -> windows_core::HRESULT,
    pub SetRuleWeight: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, f32) -> windows_core::HRESULT,
    pub SetDictationWeight: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetGrammarLoader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetGrammarLoader: usize,
    #[cfg(feature = "Win32_System_Com_Urlmon")]
    pub SetSMLSecurityManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_Urlmon"))]
    SetSMLSecurityManager: usize,
}
windows_core::imp::define_interface!(ISpRecoResult, ISpRecoResult_Vtbl, 0x20b053be_e235_43cd_9a2a_8d17a48b7842);
impl core::ops::Deref for ISpRecoResult {
    type Target = ISpPhrase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpRecoResult, windows_core::IUnknown, ISpPhrase);
impl ISpRecoResult {
    pub unsafe fn GetResultTimes(&self, ptimes: *mut SPRECORESULTTIMES) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetResultTimes)(windows_core::Interface::as_raw(self), ptimes).ok()
    }
    pub unsafe fn GetAlternates(&self, ulstartelement: u32, celements: u32, ppphrases: &mut [Option<ISpPhraseAlt>], pcphrasesreturned: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAlternates)(windows_core::Interface::as_raw(self), ulstartelement, celements, ppphrases.len().try_into().unwrap(), core::mem::transmute(ppphrases.as_ptr()), pcphrasesreturned).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAudio(&self, ulstartelement: u32, celements: u32) -> windows_core::Result<ISpStreamFormat> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAudio)(windows_core::Interface::as_raw(self), ulstartelement, celements, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SpeakAudio(&self, ulstartelement: u32, celements: u32, dwflags: u32, pulstreamnumber: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SpeakAudio)(windows_core::Interface::as_raw(self), ulstartelement, celements, dwflags, pulstreamnumber).ok()
    }
    pub unsafe fn Serialize(&self, ppcomemserializedresult: *mut *mut SPSERIALIZEDRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Serialize)(windows_core::Interface::as_raw(self), ppcomemserializedresult).ok()
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn ScaleAudio(&self, paudioformatid: *const windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ScaleAudio)(windows_core::Interface::as_raw(self), paudioformatid, pwaveformatex).ok()
    }
    pub unsafe fn GetRecoContext(&self) -> windows_core::Result<ISpRecoContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRecoContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISpRecoResult_Vtbl {
    pub base__: ISpPhrase_Vtbl,
    pub GetResultTimes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPRECORESULTTIMES) -> windows_core::HRESULT,
    pub GetAlternates: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAudio: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAudio: usize,
    pub SpeakAudio: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub Serialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut SPSERIALIZEDRESULT) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub ScaleAudio: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::Audio::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    ScaleAudio: usize,
    pub GetRecoContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpRecoResult2, ISpRecoResult2_Vtbl, 0x27cac6c4_88f2_41f2_8817_0c95e59f1e6e);
impl core::ops::Deref for ISpRecoResult2 {
    type Target = ISpRecoResult;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpRecoResult2, windows_core::IUnknown, ISpPhrase, ISpRecoResult);
impl ISpRecoResult2 {
    pub unsafe fn CommitAlternate<P0>(&self, pphrasealt: P0) -> windows_core::Result<ISpRecoResult>
    where
        P0: windows_core::Param<ISpPhraseAlt>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CommitAlternate)(windows_core::Interface::as_raw(self), pphrasealt.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CommitText<P0>(&self, ulstartelement: u32, celements: u32, pszcorrecteddata: P0, ecommitflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).CommitText)(windows_core::Interface::as_raw(self), ulstartelement, celements, pszcorrecteddata.param().abi(), ecommitflags).ok()
    }
    pub unsafe fn SetTextFeedback<P0, P1>(&self, pszfeedback: P0, fsuccessful: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetTextFeedback)(windows_core::Interface::as_raw(self), pszfeedback.param().abi(), fsuccessful.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISpRecoResult2_Vtbl {
    pub base__: ISpRecoResult_Vtbl,
    pub CommitAlternate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CommitText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub SetTextFeedback: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpRecognizer, ISpRecognizer_Vtbl, 0xc2b5f241_daa0_4507_9e16_5a1eaa2b7a5c);
impl core::ops::Deref for ISpRecognizer {
    type Target = ISpProperties;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpRecognizer, windows_core::IUnknown, ISpProperties);
impl ISpRecognizer {
    pub unsafe fn SetRecognizer<P0>(&self, precognizer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpObjectToken>,
    {
        (windows_core::Interface::vtable(self).SetRecognizer)(windows_core::Interface::as_raw(self), precognizer.param().abi()).ok()
    }
    pub unsafe fn GetRecognizer(&self) -> windows_core::Result<ISpObjectToken> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRecognizer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetInput<P0, P1>(&self, punkinput: P0, fallowformatchanges: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetInput)(windows_core::Interface::as_raw(self), punkinput.param().abi(), fallowformatchanges.param().abi()).ok()
    }
    pub unsafe fn GetInputObjectToken(&self) -> windows_core::Result<ISpObjectToken> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInputObjectToken)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInputStream(&self) -> windows_core::Result<ISpStreamFormat> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInputStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateRecoContext(&self) -> windows_core::Result<ISpRecoContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRecoContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetRecoProfile(&self) -> windows_core::Result<ISpObjectToken> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRecoProfile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRecoProfile<P0>(&self, ptoken: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpObjectToken>,
    {
        (windows_core::Interface::vtable(self).SetRecoProfile)(windows_core::Interface::as_raw(self), ptoken.param().abi()).ok()
    }
    pub unsafe fn IsSharedInstance(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsSharedInstance)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetRecoState(&self, pstate: *mut SPRECOSTATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRecoState)(windows_core::Interface::as_raw(self), pstate).ok()
    }
    pub unsafe fn SetRecoState(&self, newstate: SPRECOSTATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRecoState)(windows_core::Interface::as_raw(self), newstate).ok()
    }
    pub unsafe fn GetStatus(&self, pstatus: *mut SPRECOGNIZERSTATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), pstatus).ok()
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormat(&self, waveformattype: SPSTREAMFORMATTYPE, pformatid: *mut windows_core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFormat)(windows_core::Interface::as_raw(self), waveformattype, pformatid, ppcomemwfex).ok()
    }
    pub unsafe fn IsUISupported<P0>(&self, psztypeofui: P0, pvextradata: *mut core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).IsUISupported)(windows_core::Interface::as_raw(self), psztypeofui.param().abi(), pvextradata, cbextradata, pfsupported).ok()
    }
    pub unsafe fn DisplayUI<P0, P1, P2>(&self, hwndparent: P0, psztitle: P1, psztypeofui: P2, pvextradata: *mut core::ffi::c_void, cbextradata: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DisplayUI)(windows_core::Interface::as_raw(self), hwndparent.param().abi(), psztitle.param().abi(), psztypeofui.param().abi(), pvextradata, cbextradata).ok()
    }
    pub unsafe fn EmulateRecognition<P0>(&self, pphrase: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpPhrase>,
    {
        (windows_core::Interface::vtable(self).EmulateRecognition)(windows_core::Interface::as_raw(self), pphrase.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISpRecognizer_Vtbl {
    pub base__: ISpProperties_Vtbl,
    pub SetRecognizer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRecognizer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetInputObjectToken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetInputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetInputStream: usize,
    pub CreateRecoContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRecoProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRecoProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSharedInstance: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRecoState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPRECOSTATE) -> windows_core::HRESULT,
    pub SetRecoState: unsafe extern "system" fn(*mut core::ffi::c_void, SPRECOSTATE) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPRECOGNIZERSTATUS) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, SPSTREAMFORMATTYPE, *mut windows_core::GUID, *mut *mut super::Audio::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetFormat: usize,
    pub IsUISupported: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub DisplayUI: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EmulateRecognition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpRecognizer2, ISpRecognizer2_Vtbl, 0x8fc6d974_c81e_4098_93c5_0147f61ed4d3);
impl core::ops::Deref for ISpRecognizer2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpRecognizer2, windows_core::IUnknown);
impl ISpRecognizer2 {
    pub unsafe fn EmulateRecognitionEx<P0>(&self, pphrase: P0, dwcompareflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpPhrase>,
    {
        (windows_core::Interface::vtable(self).EmulateRecognitionEx)(windows_core::Interface::as_raw(self), pphrase.param().abi(), dwcompareflags).ok()
    }
    pub unsafe fn SetTrainingState<P0, P1>(&self, fdoingtraining: P0, fadaptfromtrainingdata: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetTrainingState)(windows_core::Interface::as_raw(self), fdoingtraining.param().abi(), fadaptfromtrainingdata.param().abi()).ok()
    }
    pub unsafe fn ResetAcousticModelAdaptation(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResetAcousticModelAdaptation)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ISpRecognizer2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EmulateRecognitionEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetTrainingState: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ResetAcousticModelAdaptation: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpRegDataKey, ISpRegDataKey_Vtbl, 0x92a66e2b_c830_4149_83df_6fc2ba1e7a5b);
impl core::ops::Deref for ISpRegDataKey {
    type Target = ISpDataKey;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpRegDataKey, windows_core::IUnknown, ISpDataKey);
impl ISpRegDataKey {
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn SetKey<P0, P1>(&self, hkey: P0, freadonly: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Registry::HKEY>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetKey)(windows_core::Interface::as_raw(self), hkey.param().abi(), freadonly.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISpRegDataKey_Vtbl {
    pub base__: ISpDataKey_Vtbl,
    #[cfg(feature = "Win32_System_Registry")]
    pub SetKey: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Registry::HKEY, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    SetKey: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpResourceManager, ISpResourceManager_Vtbl, 0x93384e18_5014_43d5_adbb_a78e055926bd);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpResourceManager {
    type Target = super::super::System::Com::IServiceProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpResourceManager, windows_core::IUnknown, super::super::System::Com::IServiceProvider);
#[cfg(feature = "Win32_System_Com")]
impl ISpResourceManager {
    pub unsafe fn SetObject<P0>(&self, guidserviceid: *const windows_core::GUID, punkobject: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetObject)(windows_core::Interface::as_raw(self), guidserviceid, punkobject.param().abi()).ok()
    }
    pub unsafe fn GetObject<P0>(&self, guidserviceid: *const windows_core::GUID, objectclsid: *const windows_core::GUID, objectiid: *const windows_core::GUID, freleasewhenlastexternalrefreleased: P0, ppobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), guidserviceid, objectclsid, objectiid, freleasewhenlastexternalrefreleased.param().abi(), ppobject).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpResourceManager_Vtbl {
    pub base__: super::super::System::Com::IServiceProvider_Vtbl,
    pub SetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *const windows_core::GUID, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpSRAlternates, ISpSRAlternates_Vtbl, 0xfece8294_2be1_408f_8e68_2de377092f0e);
impl core::ops::Deref for ISpSRAlternates {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpSRAlternates, windows_core::IUnknown);
impl ISpSRAlternates {
    pub unsafe fn GetAlternates(&self, paltrequest: *const SPPHRASEALTREQUEST, ppalts: *mut *mut SPPHRASEALT, pcalts: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAlternates)(windows_core::Interface::as_raw(self), paltrequest, ppalts, pcalts).ok()
    }
    pub unsafe fn Commit(&self, paltrequest: *const SPPHRASEALTREQUEST, palt: *const SPPHRASEALT, ppvresultextra: *mut *mut core::ffi::c_void, pcbresultextra: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), paltrequest, palt, ppvresultextra, pcbresultextra).ok()
    }
}
#[repr(C)]
pub struct ISpSRAlternates_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAlternates: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPPHRASEALTREQUEST, *mut *mut SPPHRASEALT, *mut u32) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPPHRASEALTREQUEST, *const SPPHRASEALT, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpSRAlternates2, ISpSRAlternates2_Vtbl, 0xf338f437_cb33_4020_9cab_c71ff9ce12d3);
impl core::ops::Deref for ISpSRAlternates2 {
    type Target = ISpSRAlternates;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpSRAlternates2, windows_core::IUnknown, ISpSRAlternates);
impl ISpSRAlternates2 {
    pub unsafe fn CommitText<P0>(&self, paltrequest: *const SPPHRASEALTREQUEST, pcsznewtext: P0, commitflags: SPCOMMITFLAGS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).CommitText)(windows_core::Interface::as_raw(self), paltrequest, pcsznewtext.param().abi(), commitflags).ok()
    }
}
#[repr(C)]
pub struct ISpSRAlternates2_Vtbl {
    pub base__: ISpSRAlternates_Vtbl,
    pub CommitText: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPPHRASEALTREQUEST, windows_core::PCWSTR, SPCOMMITFLAGS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpSREngine, ISpSREngine_Vtbl, 0x2f472991_854b_4465_b613_fbafb3ad8ed8);
impl core::ops::Deref for ISpSREngine {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpSREngine, windows_core::IUnknown);
impl ISpSREngine {
    pub unsafe fn SetSite<P0>(&self, psite: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpSREngineSite>,
    {
        (windows_core::Interface::vtable(self).SetSite)(windows_core::Interface::as_raw(self), psite.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetInputAudioFormat(&self, pguidsourceformatid: *const windows_core::GUID, psourcewaveformatex: *const super::Audio::WAVEFORMATEX, pguiddesiredformatid: *mut windows_core::GUID, ppcomemdesiredwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInputAudioFormat)(windows_core::Interface::as_raw(self), pguidsourceformatid, psourcewaveformatex, pguiddesiredformatid, ppcomemdesiredwaveformatex).ok()
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn RecognizeStream<P0, P1, P2, P3, P4, P5>(&self, rguidfmtid: *const windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX, hrequestsync: P0, hdataavailable: P1, hexit: P2, fnewaudiostream: P3, frealtimeaudio: P4, paudioobjecttoken: P5) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
        P1: windows_core::Param<super::super::Foundation::HANDLE>,
        P2: windows_core::Param<super::super::Foundation::HANDLE>,
        P3: windows_core::Param<super::super::Foundation::BOOL>,
        P4: windows_core::Param<super::super::Foundation::BOOL>,
        P5: windows_core::Param<ISpObjectToken>,
    {
        (windows_core::Interface::vtable(self).RecognizeStream)(windows_core::Interface::as_raw(self), rguidfmtid, pwaveformatex, hrequestsync.param().abi(), hdataavailable.param().abi(), hexit.param().abi(), fnewaudiostream.param().abi(), frealtimeaudio.param().abi(), paudioobjecttoken.param().abi()).ok()
    }
    pub unsafe fn SetRecoProfile<P0>(&self, pprofile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpObjectToken>,
    {
        (windows_core::Interface::vtable(self).SetRecoProfile)(windows_core::Interface::as_raw(self), pprofile.param().abi()).ok()
    }
    pub unsafe fn OnCreateGrammar<P0>(&self, pvenginerecocontext: *const core::ffi::c_void, hsapigrammar: P0, ppvenginegrammarcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SPGRAMMARHANDLE>,
    {
        (windows_core::Interface::vtable(self).OnCreateGrammar)(windows_core::Interface::as_raw(self), pvenginerecocontext, hsapigrammar.param().abi(), ppvenginegrammarcontext).ok()
    }
    pub unsafe fn OnDeleteGrammar(&self, pvenginegrammar: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnDeleteGrammar)(windows_core::Interface::as_raw(self), pvenginegrammar).ok()
    }
    pub unsafe fn LoadProprietaryGrammar<P0>(&self, pvenginegrammar: *const core::ffi::c_void, rguidparam: *const windows_core::GUID, pszstringparam: P0, pvdataparam: *const core::ffi::c_void, uldatasize: u32, options: SPLOADOPTIONS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).LoadProprietaryGrammar)(windows_core::Interface::as_raw(self), pvenginegrammar, rguidparam, pszstringparam.param().abi(), pvdataparam, uldatasize, options).ok()
    }
    pub unsafe fn UnloadProprietaryGrammar(&self, pvenginegrammar: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnloadProprietaryGrammar)(windows_core::Interface::as_raw(self), pvenginegrammar).ok()
    }
    pub unsafe fn SetProprietaryRuleState<P0>(&self, pvenginegrammar: *const core::ffi::c_void, pszname: P0, preserved: *const core::ffi::c_void, newstate: SPRULESTATE) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetProprietaryRuleState)(windows_core::Interface::as_raw(self), pvenginegrammar, pszname.param().abi(), preserved, newstate, &mut result__).map(|| result__)
    }
    pub unsafe fn SetProprietaryRuleIdState(&self, pvenginegrammar: *const core::ffi::c_void, dwruleid: u32, newstate: SPRULESTATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProprietaryRuleIdState)(windows_core::Interface::as_raw(self), pvenginegrammar, dwruleid, newstate).ok()
    }
    pub unsafe fn LoadSLM<P0>(&self, pvenginegrammar: *const core::ffi::c_void, psztopicname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).LoadSLM)(windows_core::Interface::as_raw(self), pvenginegrammar, psztopicname.param().abi()).ok()
    }
    pub unsafe fn UnloadSLM(&self, pvenginegrammar: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnloadSLM)(windows_core::Interface::as_raw(self), pvenginegrammar).ok()
    }
    pub unsafe fn SetSLMState(&self, pvenginegrammar: *const core::ffi::c_void, newstate: SPRULESTATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSLMState)(windows_core::Interface::as_raw(self), pvenginegrammar, newstate).ok()
    }
    pub unsafe fn SetWordSequenceData(&self, pvenginegrammar: *const core::ffi::c_void, ptext: &[u16], pinfo: *const SPTEXTSELECTIONINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetWordSequenceData)(windows_core::Interface::as_raw(self), pvenginegrammar, core::mem::transmute(ptext.as_ptr()), ptext.len().try_into().unwrap(), pinfo).ok()
    }
    pub unsafe fn SetTextSelection(&self, pvenginegrammar: *const core::ffi::c_void, pinfo: *const SPTEXTSELECTIONINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTextSelection)(windows_core::Interface::as_raw(self), pvenginegrammar, pinfo).ok()
    }
    pub unsafe fn IsPronounceable<P0>(&self, pvenginegrammar: *const core::ffi::c_void, pszword: P0) -> windows_core::Result<SPWORDPRONOUNCEABLE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsPronounceable)(windows_core::Interface::as_raw(self), pvenginegrammar, pszword.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn OnCreateRecoContext<P0>(&self, hsapirecocontext: P0, ppvenginecontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SPRECOCONTEXTHANDLE>,
    {
        (windows_core::Interface::vtable(self).OnCreateRecoContext)(windows_core::Interface::as_raw(self), hsapirecocontext.param().abi(), ppvenginecontext).ok()
    }
    pub unsafe fn OnDeleteRecoContext(&self, pvenginecontext: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnDeleteRecoContext)(windows_core::Interface::as_raw(self), pvenginecontext).ok()
    }
    pub unsafe fn PrivateCall(&self, pvenginecontext: *const core::ffi::c_void, pcallframe: *mut core::ffi::c_void, ulcallframesize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PrivateCall)(windows_core::Interface::as_raw(self), pvenginecontext, pcallframe, ulcallframesize).ok()
    }
    pub unsafe fn SetAdaptationData(&self, pvenginecontext: *const core::ffi::c_void, padaptationdata: &[u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAdaptationData)(windows_core::Interface::as_raw(self), pvenginecontext, core::mem::transmute(padaptationdata.as_ptr()), padaptationdata.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SetPropertyNum<P0>(&self, esrc: SPPROPSRC, pvsrcobj: *const core::ffi::c_void, pname: P0, lvalue: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetPropertyNum)(windows_core::Interface::as_raw(self), esrc, pvsrcobj, pname.param().abi(), lvalue).ok()
    }
    pub unsafe fn GetPropertyNum<P0>(&self, esrc: SPPROPSRC, pvsrcobj: *const core::ffi::c_void, pname: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPropertyNum)(windows_core::Interface::as_raw(self), esrc, pvsrcobj, pname.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPropertyString<P0, P1>(&self, esrc: SPPROPSRC, pvsrcobj: *const core::ffi::c_void, pname: P0, pvalue: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetPropertyString)(windows_core::Interface::as_raw(self), esrc, pvsrcobj, pname.param().abi(), pvalue.param().abi()).ok()
    }
    pub unsafe fn GetPropertyString<P0>(&self, esrc: SPPROPSRC, pvsrcobj: *const core::ffi::c_void, pname: P0) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPropertyString)(windows_core::Interface::as_raw(self), esrc, pvsrcobj, pname.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn SetGrammarState(&self, pvenginegrammar: *const core::ffi::c_void, egrammarstate: SPGRAMMARSTATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGrammarState)(windows_core::Interface::as_raw(self), pvenginegrammar, egrammarstate).ok()
    }
    pub unsafe fn WordNotify(&self, action: SPCFGNOTIFY, cwords: u32, pwords: *const SPWORDENTRY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WordNotify)(windows_core::Interface::as_raw(self), action, cwords, pwords).ok()
    }
    pub unsafe fn RuleNotify(&self, action: SPCFGNOTIFY, crules: u32, prules: *const SPRULEENTRY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RuleNotify)(windows_core::Interface::as_raw(self), action, crules, prules).ok()
    }
    pub unsafe fn PrivateCallEx(&self, pvenginecontext: *const core::ffi::c_void, pincallframe: *const core::ffi::c_void, ulincallframesize: u32, ppvcomemresponse: *mut *mut core::ffi::c_void, pulresponsesize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PrivateCallEx)(windows_core::Interface::as_raw(self), pvenginecontext, pincallframe, ulincallframesize, ppvcomemresponse, pulresponsesize).ok()
    }
    pub unsafe fn SetContextState(&self, pvenginecontext: *const core::ffi::c_void, econtextstate: SPCONTEXTSTATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetContextState)(windows_core::Interface::as_raw(self), pvenginecontext, econtextstate).ok()
    }
}
#[repr(C)]
pub struct ISpSREngine_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetInputAudioFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::Audio::WAVEFORMATEX, *mut windows_core::GUID, *mut *mut super::Audio::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetInputAudioFormat: usize,
    #[cfg(feature = "Win32_Media_Audio")]
    pub RecognizeStream: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::Audio::WAVEFORMATEX, super::super::Foundation::HANDLE, super::super::Foundation::HANDLE, super::super::Foundation::HANDLE, super::super::Foundation::BOOL, super::super::Foundation::BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    RecognizeStream: usize,
    pub SetRecoProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnCreateGrammar: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, SPGRAMMARHANDLE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnDeleteGrammar: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub LoadProprietaryGrammar: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, *const core::ffi::c_void, u32, SPLOADOPTIONS) -> windows_core::HRESULT,
    pub UnloadProprietaryGrammar: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub SetProprietaryRuleState: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::PCWSTR, *const core::ffi::c_void, SPRULESTATE, *mut u32) -> windows_core::HRESULT,
    pub SetProprietaryRuleIdState: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, SPRULESTATE) -> windows_core::HRESULT,
    pub LoadSLM: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub UnloadSLM: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSLMState: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, SPRULESTATE) -> windows_core::HRESULT,
    pub SetWordSequenceData: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::PCWSTR, u32, *const SPTEXTSELECTIONINFO) -> windows_core::HRESULT,
    pub SetTextSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *const SPTEXTSELECTIONINFO) -> windows_core::HRESULT,
    pub IsPronounceable: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::PCWSTR, *mut SPWORDPRONOUNCEABLE) -> windows_core::HRESULT,
    pub OnCreateRecoContext: unsafe extern "system" fn(*mut core::ffi::c_void, SPRECOCONTEXTHANDLE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnDeleteRecoContext: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub PrivateCall: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetAdaptationData: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub SetPropertyNum: unsafe extern "system" fn(*mut core::ffi::c_void, SPPROPSRC, *const core::ffi::c_void, windows_core::PCWSTR, i32) -> windows_core::HRESULT,
    pub GetPropertyNum: unsafe extern "system" fn(*mut core::ffi::c_void, SPPROPSRC, *const core::ffi::c_void, windows_core::PCWSTR, *mut i32) -> windows_core::HRESULT,
    pub SetPropertyString: unsafe extern "system" fn(*mut core::ffi::c_void, SPPROPSRC, *const core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetPropertyString: unsafe extern "system" fn(*mut core::ffi::c_void, SPPROPSRC, *const core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetGrammarState: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, SPGRAMMARSTATE) -> windows_core::HRESULT,
    pub WordNotify: unsafe extern "system" fn(*mut core::ffi::c_void, SPCFGNOTIFY, u32, *const SPWORDENTRY) -> windows_core::HRESULT,
    pub RuleNotify: unsafe extern "system" fn(*mut core::ffi::c_void, SPCFGNOTIFY, u32, *const SPRULEENTRY) -> windows_core::HRESULT,
    pub PrivateCallEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *const core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetContextState: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, SPCONTEXTSTATE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpSREngine2, ISpSREngine2_Vtbl, 0x7ba627d8_33f9_4375_90c5_9985aee5ede5);
impl core::ops::Deref for ISpSREngine2 {
    type Target = ISpSREngine;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpSREngine2, windows_core::IUnknown, ISpSREngine);
impl ISpSREngine2 {
    pub unsafe fn PrivateCallImmediate(&self, pvenginecontext: *const core::ffi::c_void, pincallframe: *const core::ffi::c_void, ulincallframesize: u32, ppvcomemresponse: *mut *mut core::ffi::c_void, pulresponsesize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PrivateCallImmediate)(windows_core::Interface::as_raw(self), pvenginecontext, pincallframe, ulincallframesize, ppvcomemresponse, pulresponsesize).ok()
    }
    pub unsafe fn SetAdaptationData2<P0>(&self, pvenginecontext: *const core::ffi::c_void, padaptationdata: &[u16], ptopicname: P0, esettings: SPADAPTATIONSETTINGS, erelevance: SPADAPTATIONRELEVANCE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetAdaptationData2)(windows_core::Interface::as_raw(self), pvenginecontext, core::mem::transmute(padaptationdata.as_ptr()), padaptationdata.len().try_into().unwrap(), ptopicname.param().abi(), esettings, erelevance).ok()
    }
    pub unsafe fn SetGrammarPrefix<P0, P1>(&self, pvenginegrammar: *const core::ffi::c_void, pszprefix: P0, fisprefixrequired: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetGrammarPrefix)(windows_core::Interface::as_raw(self), pvenginegrammar, pszprefix.param().abi(), fisprefixrequired.param().abi()).ok()
    }
    pub unsafe fn SetRulePriority<P0>(&self, hrule: P0, pvclientrulecontext: *const core::ffi::c_void, nrulepriority: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SPRULEHANDLE>,
    {
        (windows_core::Interface::vtable(self).SetRulePriority)(windows_core::Interface::as_raw(self), hrule.param().abi(), pvclientrulecontext, nrulepriority).ok()
    }
    pub unsafe fn EmulateRecognition<P0>(&self, pphrase: P0, dwcompareflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpPhrase>,
    {
        (windows_core::Interface::vtable(self).EmulateRecognition)(windows_core::Interface::as_raw(self), pphrase.param().abi(), dwcompareflags).ok()
    }
    pub unsafe fn SetSLMWeight(&self, pvenginegrammar: *const core::ffi::c_void, flweight: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSLMWeight)(windows_core::Interface::as_raw(self), pvenginegrammar, flweight).ok()
    }
    pub unsafe fn SetRuleWeight<P0>(&self, hrule: P0, pvclientrulecontext: *const core::ffi::c_void, flweight: f32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SPRULEHANDLE>,
    {
        (windows_core::Interface::vtable(self).SetRuleWeight)(windows_core::Interface::as_raw(self), hrule.param().abi(), pvclientrulecontext, flweight).ok()
    }
    pub unsafe fn SetTrainingState<P0, P1>(&self, fdoingtraining: P0, fadaptfromtrainingdata: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetTrainingState)(windows_core::Interface::as_raw(self), fdoingtraining.param().abi(), fadaptfromtrainingdata.param().abi()).ok()
    }
    pub unsafe fn ResetAcousticModelAdaptation(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResetAcousticModelAdaptation)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnLoadCFG(&self, pvenginegrammar: *const core::ffi::c_void, pgrammardata: *const SPBINARYGRAMMAR, ulgrammarid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnLoadCFG)(windows_core::Interface::as_raw(self), pvenginegrammar, pgrammardata, ulgrammarid).ok()
    }
    pub unsafe fn OnUnloadCFG(&self, pvenginegrammar: *const core::ffi::c_void, ulgrammarid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnUnloadCFG)(windows_core::Interface::as_raw(self), pvenginegrammar, ulgrammarid).ok()
    }
}
#[repr(C)]
pub struct ISpSREngine2_Vtbl {
    pub base__: ISpSREngine_Vtbl,
    pub PrivateCallImmediate: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *const core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetAdaptationData2: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR, SPADAPTATIONSETTINGS, SPADAPTATIONRELEVANCE) -> windows_core::HRESULT,
    pub SetGrammarPrefix: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::PCWSTR, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetRulePriority: unsafe extern "system" fn(*mut core::ffi::c_void, SPRULEHANDLE, *const core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EmulateRecognition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetSLMWeight: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetRuleWeight: unsafe extern "system" fn(*mut core::ffi::c_void, SPRULEHANDLE, *const core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetTrainingState: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ResetAcousticModelAdaptation: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnLoadCFG: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *const SPBINARYGRAMMAR, u32) -> windows_core::HRESULT,
    pub OnUnloadCFG: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpSREngineSite, ISpSREngineSite_Vtbl, 0x3b414aec_720c_4883_b9ef_178cd394fb3a);
impl core::ops::Deref for ISpSREngineSite {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpSREngineSite, windows_core::IUnknown);
impl ISpSREngineSite {
    pub unsafe fn Read(&self, pv: *const core::ffi::c_void, cb: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), pv, cb, &mut result__).map(|| result__)
    }
    pub unsafe fn DataAvailable(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DataAvailable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBufferNotifySize(&self, cbsize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBufferNotifySize)(windows_core::Interface::as_raw(self), cbsize).ok()
    }
    pub unsafe fn ParseFromTransitions(&self, pparseinfo: *const SPPARSEINFO) -> windows_core::Result<ISpPhraseBuilder> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ParseFromTransitions)(windows_core::Interface::as_raw(self), pparseinfo, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Recognition(&self, presultinfo: *const SPRECORESULTINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Recognition)(windows_core::Interface::as_raw(self), presultinfo).ok()
    }
    pub unsafe fn AddEvent<P0>(&self, pevent: *const SPEVENT, hsapirecocontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SPRECOCONTEXTHANDLE>,
    {
        (windows_core::Interface::vtable(self).AddEvent)(windows_core::Interface::as_raw(self), pevent, hsapirecocontext.param().abi()).ok()
    }
    pub unsafe fn Synchronize(&self, ullprocessedthrupos: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Synchronize)(windows_core::Interface::as_raw(self), ullprocessedthrupos).ok()
    }
    pub unsafe fn GetWordInfo(&self, pwordentry: *mut SPWORDENTRY, options: SPWORDINFOOPT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetWordInfo)(windows_core::Interface::as_raw(self), pwordentry, options).ok()
    }
    pub unsafe fn SetWordClientContext<P0>(&self, hword: P0, pvclientcontext: *const core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SPWORDHANDLE>,
    {
        (windows_core::Interface::vtable(self).SetWordClientContext)(windows_core::Interface::as_raw(self), hword.param().abi(), pvclientcontext).ok()
    }
    pub unsafe fn GetRuleInfo(&self, pruleentry: *mut SPRULEENTRY, options: SPRULEINFOOPT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRuleInfo)(windows_core::Interface::as_raw(self), pruleentry, options).ok()
    }
    pub unsafe fn SetRuleClientContext<P0>(&self, hrule: P0, pvclientcontext: *const core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SPRULEHANDLE>,
    {
        (windows_core::Interface::vtable(self).SetRuleClientContext)(windows_core::Interface::as_raw(self), hrule.param().abi(), pvclientcontext).ok()
    }
    pub unsafe fn GetStateInfo<P0>(&self, hstate: P0, pstateinfo: *mut SPSTATEINFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SPSTATEHANDLE>,
    {
        (windows_core::Interface::vtable(self).GetStateInfo)(windows_core::Interface::as_raw(self), hstate.param().abi(), pstateinfo).ok()
    }
    pub unsafe fn GetResource<P0, P1>(&self, hrule: P0, pszresourcename: P1) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<SPRULEHANDLE>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetResource)(windows_core::Interface::as_raw(self), hrule.param().abi(), pszresourcename.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetTransitionProperty<P0>(&self, id: P0) -> windows_core::Result<*mut SPTRANSITIONPROPERTY>
    where
        P0: windows_core::Param<SPTRANSITIONID>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTransitionProperty)(windows_core::Interface::as_raw(self), id.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn IsAlternate<P0, P1>(&self, hrule: P0, haltrule: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SPRULEHANDLE>,
        P1: windows_core::Param<SPRULEHANDLE>,
    {
        (windows_core::Interface::vtable(self).IsAlternate)(windows_core::Interface::as_raw(self), hrule.param().abi(), haltrule.param().abi()).ok()
    }
    pub unsafe fn GetMaxAlternates<P0>(&self, hrule: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<SPRULEHANDLE>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxAlternates)(windows_core::Interface::as_raw(self), hrule.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetContextMaxAlternates<P0>(&self, hcontext: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<SPRECOCONTEXTHANDLE>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContextMaxAlternates)(windows_core::Interface::as_raw(self), hcontext.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn UpdateRecoPos(&self, ullcurrentrecopos: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UpdateRecoPos)(windows_core::Interface::as_raw(self), ullcurrentrecopos).ok()
    }
}
#[repr(C)]
pub struct ISpSREngineSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub DataAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetBufferNotifySize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ParseFromTransitions: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPPARSEINFO, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Recognition: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPRECORESULTINFO) -> windows_core::HRESULT,
    pub AddEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPEVENT, SPRECOCONTEXTHANDLE) -> windows_core::HRESULT,
    pub Synchronize: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub GetWordInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPWORDENTRY, SPWORDINFOOPT) -> windows_core::HRESULT,
    pub SetWordClientContext: unsafe extern "system" fn(*mut core::ffi::c_void, SPWORDHANDLE, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRuleInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPRULEENTRY, SPRULEINFOOPT) -> windows_core::HRESULT,
    pub SetRuleClientContext: unsafe extern "system" fn(*mut core::ffi::c_void, SPRULEHANDLE, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStateInfo: unsafe extern "system" fn(*mut core::ffi::c_void, SPSTATEHANDLE, *mut SPSTATEINFO) -> windows_core::HRESULT,
    pub GetResource: unsafe extern "system" fn(*mut core::ffi::c_void, SPRULEHANDLE, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetTransitionProperty: unsafe extern "system" fn(*mut core::ffi::c_void, SPTRANSITIONID, *mut *mut SPTRANSITIONPROPERTY) -> windows_core::HRESULT,
    pub IsAlternate: unsafe extern "system" fn(*mut core::ffi::c_void, SPRULEHANDLE, SPRULEHANDLE) -> windows_core::HRESULT,
    pub GetMaxAlternates: unsafe extern "system" fn(*mut core::ffi::c_void, SPRULEHANDLE, *mut u32) -> windows_core::HRESULT,
    pub GetContextMaxAlternates: unsafe extern "system" fn(*mut core::ffi::c_void, SPRECOCONTEXTHANDLE, *mut u32) -> windows_core::HRESULT,
    pub UpdateRecoPos: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpSREngineSite2, ISpSREngineSite2_Vtbl, 0x7bc6e012_684a_493e_bdd4_2bf5fbf48cfe);
impl core::ops::Deref for ISpSREngineSite2 {
    type Target = ISpSREngineSite;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpSREngineSite2, windows_core::IUnknown, ISpSREngineSite);
impl ISpSREngineSite2 {
    pub unsafe fn AddEventEx<P0>(&self, pevent: *const SPEVENTEX, hsapirecocontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SPRECOCONTEXTHANDLE>,
    {
        (windows_core::Interface::vtable(self).AddEventEx)(windows_core::Interface::as_raw(self), pevent, hsapirecocontext.param().abi()).ok()
    }
    pub unsafe fn UpdateRecoPosEx(&self, ullcurrentrecopos: u64, ullcurrentrecotime: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UpdateRecoPosEx)(windows_core::Interface::as_raw(self), ullcurrentrecopos, ullcurrentrecotime).ok()
    }
    pub unsafe fn GetRuleTransition(&self, ulgrammarid: u32, ruleindex: u32, ptrans: *mut SPTRANSITIONENTRY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRuleTransition)(windows_core::Interface::as_raw(self), ulgrammarid, ruleindex, ptrans).ok()
    }
    pub unsafe fn RecognitionEx(&self, presultinfo: *const SPRECORESULTINFOEX) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RecognitionEx)(windows_core::Interface::as_raw(self), presultinfo).ok()
    }
}
#[repr(C)]
pub struct ISpSREngineSite2_Vtbl {
    pub base__: ISpSREngineSite_Vtbl,
    pub AddEventEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPEVENTEX, SPRECOCONTEXTHANDLE) -> windows_core::HRESULT,
    pub UpdateRecoPosEx: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64) -> windows_core::HRESULT,
    pub GetRuleTransition: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut SPTRANSITIONENTRY) -> windows_core::HRESULT,
    pub RecognitionEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPRECORESULTINFOEX) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpSerializeState, ISpSerializeState_Vtbl, 0x21b501a0_0ec7_46c9_92c3_a2bc784c54b9);
impl core::ops::Deref for ISpSerializeState {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpSerializeState, windows_core::IUnknown);
impl ISpSerializeState {
    pub unsafe fn GetSerializedState(&self, ppbdata: *mut *mut u8, pulsize: *mut u32, dwreserved: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSerializedState)(windows_core::Interface::as_raw(self), ppbdata, pulsize, dwreserved).ok()
    }
    pub unsafe fn SetSerializedState(&self, pbdata: *const u8, ulsize: u32, dwreserved: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSerializedState)(windows_core::Interface::as_raw(self), pbdata, ulsize, dwreserved).ok()
    }
}
#[repr(C)]
pub struct ISpSerializeState_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSerializedState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32, u32) -> windows_core::HRESULT,
    pub SetSerializedState: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpShortcut, ISpShortcut_Vtbl, 0x3df681e2_ea56_11d9_8bde_f66bad1e3f3a);
impl core::ops::Deref for ISpShortcut {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpShortcut, windows_core::IUnknown);
impl ISpShortcut {
    pub unsafe fn AddShortcut<P0, P1>(&self, pszdisplay: P0, langid: u16, pszspoken: P1, shtype: SPSHORTCUTTYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddShortcut)(windows_core::Interface::as_raw(self), pszdisplay.param().abi(), langid, pszspoken.param().abi(), shtype).ok()
    }
    pub unsafe fn RemoveShortcut<P0, P1>(&self, pszdisplay: P0, langid: u16, pszspoken: P1, shtype: SPSHORTCUTTYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RemoveShortcut)(windows_core::Interface::as_raw(self), pszdisplay.param().abi(), langid, pszspoken.param().abi(), shtype).ok()
    }
    pub unsafe fn GetShortcuts(&self, langid: u16, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetShortcuts)(windows_core::Interface::as_raw(self), langid, pshortcutpairlist).ok()
    }
    pub unsafe fn GetGeneration(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGeneration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetWordsFromGenerationChange(&self, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetWordsFromGenerationChange)(windows_core::Interface::as_raw(self), pdwgeneration, pwordlist).ok()
    }
    pub unsafe fn GetWords(&self, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetWords)(windows_core::Interface::as_raw(self), pdwgeneration, pdwcookie, pwordlist).ok()
    }
    pub unsafe fn GetShortcutsForGeneration(&self, pdwgeneration: *mut u32, pdwcookie: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetShortcutsForGeneration)(windows_core::Interface::as_raw(self), pdwgeneration, pdwcookie, pshortcutpairlist).ok()
    }
    pub unsafe fn GetGenerationChange(&self, pdwgeneration: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGenerationChange)(windows_core::Interface::as_raw(self), pdwgeneration, pshortcutpairlist).ok()
    }
}
#[repr(C)]
pub struct ISpShortcut_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddShortcut: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u16, windows_core::PCWSTR, SPSHORTCUTTYPE) -> windows_core::HRESULT,
    pub RemoveShortcut: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u16, windows_core::PCWSTR, SPSHORTCUTTYPE) -> windows_core::HRESULT,
    pub GetShortcuts: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut SPSHORTCUTPAIRLIST) -> windows_core::HRESULT,
    pub GetGeneration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetWordsFromGenerationChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut SPWORDLIST) -> windows_core::HRESULT,
    pub GetWords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut SPWORDLIST) -> windows_core::HRESULT,
    pub GetShortcutsForGeneration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut SPSHORTCUTPAIRLIST) -> windows_core::HRESULT,
    pub GetGenerationChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut SPSHORTCUTPAIRLIST) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpStream, ISpStream_Vtbl, 0x12e3cca9_7518_44c5_a5e7_ba5a79cb929e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpStream {
    type Target = ISpStreamFormat;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpStream, windows_core::IUnknown, super::super::System::Com::ISequentialStream, super::super::System::Com::IStream, ISpStreamFormat);
#[cfg(feature = "Win32_System_Com")]
impl ISpStream {
    #[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
    pub unsafe fn SetBaseStream<P0>(&self, pstream: P0, rguidformat: *const windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).SetBaseStream)(windows_core::Interface::as_raw(self), pstream.param().abi(), rguidformat, pwaveformatex).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetBaseStream(&self) -> windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBaseStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn BindToFile<P0>(&self, pszfilename: P0, emode: SPFILEMODE, pformatid: Option<*const windows_core::GUID>, pwaveformatex: Option<*const super::Audio::WAVEFORMATEX>, ulleventinterest: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).BindToFile)(windows_core::Interface::as_raw(self), pszfilename.param().abi(), emode, core::mem::transmute(pformatid.unwrap_or(std::ptr::null())), core::mem::transmute(pwaveformatex.unwrap_or(std::ptr::null())), ulleventinterest).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpStream_Vtbl {
    pub base__: ISpStreamFormat_Vtbl,
    #[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
    pub SetBaseStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *const super::Audio::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com")))]
    SetBaseStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetBaseStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetBaseStream: usize,
    #[cfg(feature = "Win32_Media_Audio")]
    pub BindToFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, SPFILEMODE, *const windows_core::GUID, *const super::Audio::WAVEFORMATEX, u64) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    BindToFile: usize,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpStreamFormat, ISpStreamFormat_Vtbl, 0xbed530be_2606_4f4d_a1c0_54c5cda5566f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpStreamFormat {
    type Target = super::super::System::Com::IStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpStreamFormat, windows_core::IUnknown, super::super::System::Com::ISequentialStream, super::super::System::Com::IStream);
#[cfg(feature = "Win32_System_Com")]
impl ISpStreamFormat {
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormat(&self, pguidformatid: *const windows_core::GUID) -> windows_core::Result<*mut super::Audio::WAVEFORMATEX> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFormat)(windows_core::Interface::as_raw(self), pguidformatid, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpStreamFormat_Vtbl {
    pub base__: super::super::System::Com::IStream_Vtbl,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut super::Audio::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetFormat: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpStreamFormatConverter, ISpStreamFormatConverter_Vtbl, 0x678a932c_ea71_4446_9b41_78fda6280a29);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpStreamFormatConverter {
    type Target = ISpStreamFormat;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpStreamFormatConverter, windows_core::IUnknown, super::super::System::Com::ISequentialStream, super::super::System::Com::IStream, ISpStreamFormat);
#[cfg(feature = "Win32_System_Com")]
impl ISpStreamFormatConverter {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBaseStream<P0, P1, P2>(&self, pstream: P0, fsetformattobasestreamformat: P1, fwritetobasestream: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpStreamFormat>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetBaseStream)(windows_core::Interface::as_raw(self), pstream.param().abi(), fsetformattobasestreamformat.param().abi(), fwritetobasestream.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetBaseStream(&self) -> windows_core::Result<ISpStreamFormat> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBaseStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetFormat(&self, rguidformatidofconvertedstream: *const windows_core::GUID, pwaveformatexofconvertedstream: *const super::Audio::WAVEFORMATEX) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFormat)(windows_core::Interface::as_raw(self), rguidformatidofconvertedstream, pwaveformatexofconvertedstream).ok()
    }
    pub unsafe fn ResetSeekPosition(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResetSeekPosition)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ScaleConvertedToBaseOffset(&self, ulloffsetconvertedstream: u64) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ScaleConvertedToBaseOffset)(windows_core::Interface::as_raw(self), ulloffsetconvertedstream, &mut result__).map(|| result__)
    }
    pub unsafe fn ScaleBaseToConvertedOffset(&self, ulloffsetbasestream: u64) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ScaleBaseToConvertedOffset)(windows_core::Interface::as_raw(self), ulloffsetbasestream, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpStreamFormatConverter_Vtbl {
    pub base__: ISpStreamFormat_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SetBaseStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetBaseStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetBaseStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetBaseStream: usize,
    #[cfg(feature = "Win32_Media_Audio")]
    pub SetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::Audio::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    SetFormat: usize,
    pub ResetSeekPosition: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ScaleConvertedToBaseOffset: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut u64) -> windows_core::HRESULT,
    pub ScaleBaseToConvertedOffset: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpTTSEngine, ISpTTSEngine_Vtbl, 0xa74d7c8e_4cc5_4f2f_a6eb_804dee18500e);
impl core::ops::Deref for ISpTTSEngine {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpTTSEngine, windows_core::IUnknown);
impl ISpTTSEngine {
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn Speak<P0>(&self, dwspeakflags: u32, rguidformatid: *const windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX, ptextfraglist: *const SPVTEXTFRAG, poutputsite: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpTTSEngineSite>,
    {
        (windows_core::Interface::vtable(self).Speak)(windows_core::Interface::as_raw(self), dwspeakflags, rguidformatid, pwaveformatex, ptextfraglist, poutputsite.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetOutputFormat(&self, ptargetfmtid: *const windows_core::GUID, ptargetwaveformatex: *const super::Audio::WAVEFORMATEX, poutputformatid: *mut windows_core::GUID, ppcomemoutputwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetOutputFormat)(windows_core::Interface::as_raw(self), ptargetfmtid, ptargetwaveformatex, poutputformatid, ppcomemoutputwaveformatex).ok()
    }
}
#[repr(C)]
pub struct ISpTTSEngine_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Media_Audio")]
    pub Speak: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *const super::Audio::WAVEFORMATEX, *const SPVTEXTFRAG, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    Speak: usize,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetOutputFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::Audio::WAVEFORMATEX, *mut windows_core::GUID, *mut *mut super::Audio::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetOutputFormat: usize,
}
windows_core::imp::define_interface!(ISpTTSEngineSite, ISpTTSEngineSite_Vtbl, 0x9880499b_cce9_11d2_b503_00c04f797396);
impl core::ops::Deref for ISpTTSEngineSite {
    type Target = ISpEventSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpTTSEngineSite, windows_core::IUnknown, ISpEventSink);
impl ISpTTSEngineSite {
    pub unsafe fn GetActions(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetActions)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Write(&self, pbuff: *const core::ffi::c_void, cb: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), pbuff, cb, &mut result__).map(|| result__)
    }
    pub unsafe fn GetRate(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetVolume(&self) -> windows_core::Result<u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVolume)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSkipInfo(&self, petype: *mut SPVSKIPTYPE, plnumitems: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSkipInfo)(windows_core::Interface::as_raw(self), petype, plnumitems).ok()
    }
    pub unsafe fn CompleteSkip(&self, ulnumskipped: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CompleteSkip)(windows_core::Interface::as_raw(self), ulnumskipped).ok()
    }
}
#[repr(C)]
pub struct ISpTTSEngineSite_Vtbl {
    pub base__: ISpEventSink_Vtbl,
    pub GetActions: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetSkipInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPVSKIPTYPE, *mut i32) -> windows_core::HRESULT,
    pub CompleteSkip: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpTask, ISpTask_Vtbl);
impl ISpTask {
    pub unsafe fn Execute(&self, pvtaskdata: *mut core::ffi::c_void, pfcontinueprocessing: *const i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Execute)(windows_core::Interface::as_raw(self), pvtaskdata, pfcontinueprocessing).ok()
    }
}
#[repr(C)]
pub struct ISpTask_Vtbl {
    pub Execute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpTaskManager, ISpTaskManager_Vtbl, 0x2baeef81_2ca3_4331_98f3_26ec5abefb03);
impl core::ops::Deref for ISpTaskManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpTaskManager, windows_core::IUnknown);
impl ISpTaskManager {
    pub unsafe fn SetThreadPoolInfo(&self, ppoolinfo: *const SPTMTHREADINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetThreadPoolInfo)(windows_core::Interface::as_raw(self), ppoolinfo).ok()
    }
    pub unsafe fn GetThreadPoolInfo(&self) -> windows_core::Result<SPTMTHREADINFO> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetThreadPoolInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn QueueTask<P0, P1>(&self, ptask: P0, pvtaskdata: *const core::ffi::c_void, hcompevent: P1, pdwgroupid: *mut u32, ptaskid: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpTask>,
        P1: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        (windows_core::Interface::vtable(self).QueueTask)(windows_core::Interface::as_raw(self), ptask.param().abi(), pvtaskdata, hcompevent.param().abi(), pdwgroupid, ptaskid).ok()
    }
    pub unsafe fn CreateReoccurringTask<P0, P1>(&self, ptask: P0, pvtaskdata: *const core::ffi::c_void, hcompevent: P1) -> windows_core::Result<ISpNotifySink>
    where
        P0: windows_core::Param<ISpTask>,
        P1: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateReoccurringTask)(windows_core::Interface::as_raw(self), ptask.param().abi(), pvtaskdata, hcompevent.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateThreadControl<P0>(&self, ptask: P0, pvtaskdata: *const core::ffi::c_void, npriority: i32) -> windows_core::Result<ISpThreadControl>
    where
        P0: windows_core::Param<ISpThreadTask>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateThreadControl)(windows_core::Interface::as_raw(self), ptask.param().abi(), pvtaskdata, npriority, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TerminateTask(&self, dwtaskid: u32, ulwaitperiod: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TerminateTask)(windows_core::Interface::as_raw(self), dwtaskid, ulwaitperiod).ok()
    }
    pub unsafe fn TerminateTaskGroup(&self, dwgroupid: u32, ulwaitperiod: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TerminateTaskGroup)(windows_core::Interface::as_raw(self), dwgroupid, ulwaitperiod).ok()
    }
}
#[repr(C)]
pub struct ISpTaskManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetThreadPoolInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const SPTMTHREADINFO) -> windows_core::HRESULT,
    pub GetThreadPoolInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPTMTHREADINFO) -> windows_core::HRESULT,
    pub QueueTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, super::super::Foundation::HANDLE, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub CreateReoccurringTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, super::super::Foundation::HANDLE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateThreadControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TerminateTask: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub TerminateTaskGroup: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpThreadControl, ISpThreadControl_Vtbl, 0xa6be4d73_4403_4358_b22d_0346e23b1764);
impl core::ops::Deref for ISpThreadControl {
    type Target = ISpNotifySink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpThreadControl, windows_core::IUnknown, ISpNotifySink);
impl ISpThreadControl {
    pub unsafe fn StartThread(&self, dwflags: u32) -> windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StartThread)(windows_core::Interface::as_raw(self), dwflags, &mut result__).map(|| result__)
    }
    pub unsafe fn WaitForThreadDone<P0>(&self, fforcestop: P0, phrthreadresult: *mut windows_core::HRESULT, mstimeout: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).WaitForThreadDone)(windows_core::Interface::as_raw(self), fforcestop.param().abi(), phrthreadresult, mstimeout).ok()
    }
    pub unsafe fn TerminateThread(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TerminateThread)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ThreadHandle(&self) -> super::super::Foundation::HANDLE {
        (windows_core::Interface::vtable(self).ThreadHandle)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn ThreadId(&self) -> u32 {
        (windows_core::Interface::vtable(self).ThreadId)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn NotifyEvent(&self) -> super::super::Foundation::HANDLE {
        (windows_core::Interface::vtable(self).NotifyEvent)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn WindowHandle(&self) -> super::super::Foundation::HWND {
        (windows_core::Interface::vtable(self).WindowHandle)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn ThreadCompleteEvent(&self) -> super::super::Foundation::HANDLE {
        (windows_core::Interface::vtable(self).ThreadCompleteEvent)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn ExitThreadEvent(&self) -> super::super::Foundation::HANDLE {
        (windows_core::Interface::vtable(self).ExitThreadEvent)(windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
pub struct ISpThreadControl_Vtbl {
    pub base__: ISpNotifySink_Vtbl,
    pub StartThread: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub WaitForThreadDone: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, *mut windows_core::HRESULT, u32) -> windows_core::HRESULT,
    pub TerminateThread: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ThreadHandle: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::HANDLE,
    pub ThreadId: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub NotifyEvent: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::HANDLE,
    pub WindowHandle: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::HWND,
    pub ThreadCompleteEvent: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::HANDLE,
    pub ExitThreadEvent: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::HANDLE,
}
windows_core::imp::define_interface!(ISpThreadTask, ISpThreadTask_Vtbl);
impl ISpThreadTask {
    pub unsafe fn InitThread<P0>(&self, pvtaskdata: *mut core::ffi::c_void, hwnd: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).InitThread)(windows_core::Interface::as_raw(self), pvtaskdata, hwnd.param().abi()).ok()
    }
    pub unsafe fn ThreadProc<P0, P1, P2>(&self, pvtaskdata: *mut core::ffi::c_void, hexitthreadevent: P0, hnotifyevent: P1, hwndworker: P2, pfcontinueprocessing: *const i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
        P1: windows_core::Param<super::super::Foundation::HANDLE>,
        P2: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).ThreadProc)(windows_core::Interface::as_raw(self), pvtaskdata, hexitthreadevent.param().abi(), hnotifyevent.param().abi(), hwndworker.param().abi(), pfcontinueprocessing).ok()
    }
    pub unsafe fn WindowMessage<P0, P1, P2>(&self, pvtaskdata: *mut core::ffi::c_void, hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> super::super::Foundation::LRESULT
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<super::super::Foundation::WPARAM>,
        P2: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).WindowMessage)(windows_core::Interface::as_raw(self), pvtaskdata, hwnd.param().abi(), msg, wparam.param().abi(), lparam.param().abi())
    }
}
#[repr(C)]
pub struct ISpThreadTask_Vtbl {
    pub InitThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub ThreadProc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::HANDLE, super::super::Foundation::HANDLE, super::super::Foundation::HWND, *const i32) -> windows_core::HRESULT,
    pub WindowMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::HWND, u32, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT,
}
windows_core::imp::define_interface!(ISpTokenUI, ISpTokenUI_Vtbl, 0xf8e690f0_39cb_4843_b8d7_c84696e1119d);
impl core::ops::Deref for ISpTokenUI {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpTokenUI, windows_core::IUnknown);
impl ISpTokenUI {
    pub unsafe fn IsUISupported<P0, P1>(&self, psztypeofui: P0, pvextradata: *const core::ffi::c_void, cbextradata: u32, punkobject: P1) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsUISupported)(windows_core::Interface::as_raw(self), psztypeofui.param().abi(), pvextradata, cbextradata, punkobject.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn DisplayUI<P0, P1, P2, P3, P4>(&self, hwndparent: P0, psztitle: P1, psztypeofui: P2, pvextradata: *const core::ffi::c_void, cbextradata: u32, ptoken: P3, punkobject: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<ISpObjectToken>,
        P4: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).DisplayUI)(windows_core::Interface::as_raw(self), hwndparent.param().abi(), psztitle.param().abi(), psztypeofui.param().abi(), pvextradata, cbextradata, ptoken.param().abi(), punkobject.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISpTokenUI_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsUISupported: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const core::ffi::c_void, u32, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub DisplayUI: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, windows_core::PCWSTR, windows_core::PCWSTR, *const core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpTranscript, ISpTranscript_Vtbl, 0x10f63bce_201a_11d3_ac70_00c04f8ee6c0);
impl core::ops::Deref for ISpTranscript {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpTranscript, windows_core::IUnknown);
impl ISpTranscript {
    pub unsafe fn GetTranscript(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTranscript)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AppendTranscript<P0>(&self, psztranscript: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AppendTranscript)(windows_core::Interface::as_raw(self), psztranscript.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISpTranscript_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTranscript: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub AppendTranscript: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpVoice, ISpVoice_Vtbl, 0x6c44df74_72b9_4992_a1ec_ef996e0422d4);
impl core::ops::Deref for ISpVoice {
    type Target = ISpEventSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpVoice, windows_core::IUnknown, ISpNotifySource, ISpEventSource);
impl ISpVoice {
    pub unsafe fn SetOutput<P0, P1>(&self, punkoutput: P0, fallowformatchanges: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetOutput)(windows_core::Interface::as_raw(self), punkoutput.param().abi(), fallowformatchanges.param().abi()).ok()
    }
    pub unsafe fn GetOutputObjectToken(&self) -> windows_core::Result<ISpObjectToken> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOutputObjectToken)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetOutputStream(&self) -> windows_core::Result<ISpStreamFormat> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOutputStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetVoice<P0>(&self, ptoken: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpObjectToken>,
    {
        (windows_core::Interface::vtable(self).SetVoice)(windows_core::Interface::as_raw(self), ptoken.param().abi()).ok()
    }
    pub unsafe fn GetVoice(&self) -> windows_core::Result<ISpObjectToken> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVoice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Speak<P0>(&self, pwcs: P0, dwflags: u32, pulstreamnumber: Option<*mut u32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Speak)(windows_core::Interface::as_raw(self), pwcs.param().abi(), dwflags, core::mem::transmute(pulstreamnumber.unwrap_or(std::ptr::null_mut()))).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SpeakStream<P0>(&self, pstream: P0, dwflags: u32, pulstreamnumber: Option<*mut u32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).SpeakStream)(windows_core::Interface::as_raw(self), pstream.param().abi(), dwflags, core::mem::transmute(pulstreamnumber.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetStatus(&self, pstatus: *mut SPVOICESTATUS, ppszlastbookmark: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), pstatus, ppszlastbookmark).ok()
    }
    pub unsafe fn Skip<P0>(&self, pitemtype: P0, lnumitems: i32, pulnumskipped: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), pitemtype.param().abi(), lnumitems, pulnumskipped).ok()
    }
    pub unsafe fn SetPriority(&self, epriority: SPVPRIORITY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), epriority).ok()
    }
    pub unsafe fn GetPriority(&self, pepriority: *mut SPVPRIORITY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self), pepriority).ok()
    }
    pub unsafe fn SetAlertBoundary(&self, eboundary: SPEVENTENUM) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAlertBoundary)(windows_core::Interface::as_raw(self), eboundary).ok()
    }
    pub unsafe fn GetAlertBoundary(&self, peboundary: *mut SPEVENTENUM) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAlertBoundary)(windows_core::Interface::as_raw(self), peboundary).ok()
    }
    pub unsafe fn SetRate(&self, rateadjust: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRate)(windows_core::Interface::as_raw(self), rateadjust).ok()
    }
    pub unsafe fn GetRate(&self, prateadjust: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRate)(windows_core::Interface::as_raw(self), prateadjust).ok()
    }
    pub unsafe fn SetVolume(&self, usvolume: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetVolume)(windows_core::Interface::as_raw(self), usvolume).ok()
    }
    pub unsafe fn GetVolume(&self, pusvolume: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetVolume)(windows_core::Interface::as_raw(self), pusvolume).ok()
    }
    pub unsafe fn WaitUntilDone(&self, mstimeout: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WaitUntilDone)(windows_core::Interface::as_raw(self), mstimeout).ok()
    }
    pub unsafe fn SetSyncSpeakTimeout(&self, mstimeout: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSyncSpeakTimeout)(windows_core::Interface::as_raw(self), mstimeout).ok()
    }
    pub unsafe fn GetSyncSpeakTimeout(&self, pmstimeout: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSyncSpeakTimeout)(windows_core::Interface::as_raw(self), pmstimeout).ok()
    }
    pub unsafe fn SpeakCompleteEvent(&self) -> super::super::Foundation::HANDLE {
        (windows_core::Interface::vtable(self).SpeakCompleteEvent)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn IsUISupported<P0>(&self, psztypeofui: P0, pvextradata: *mut core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).IsUISupported)(windows_core::Interface::as_raw(self), psztypeofui.param().abi(), pvextradata, cbextradata, pfsupported).ok()
    }
    pub unsafe fn DisplayUI<P0, P1, P2>(&self, hwndparent: P0, psztitle: P1, psztypeofui: P2, pvextradata: *mut core::ffi::c_void, cbextradata: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DisplayUI)(windows_core::Interface::as_raw(self), hwndparent.param().abi(), psztitle.param().abi(), psztypeofui.param().abi(), pvextradata, cbextradata).ok()
    }
}
#[repr(C)]
pub struct ISpVoice_Vtbl {
    pub base__: ISpEventSource_Vtbl,
    pub SetOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetOutputObjectToken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetOutputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetOutputStream: usize,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetVoice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVoice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Speak: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SpeakStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SpeakStream: usize,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPVOICESTATUS, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut u32) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, SPVPRIORITY) -> windows_core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPVPRIORITY) -> windows_core::HRESULT,
    pub SetAlertBoundary: unsafe extern "system" fn(*mut core::ffi::c_void, SPEVENTENUM) -> windows_core::HRESULT,
    pub GetAlertBoundary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPEVENTENUM) -> windows_core::HRESULT,
    pub SetRate: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub GetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub WaitUntilDone: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetSyncSpeakTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetSyncSpeakTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SpeakCompleteEvent: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::HANDLE,
    pub IsUISupported: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub DisplayUI: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpXMLRecoResult, ISpXMLRecoResult_Vtbl, 0xae39362b_45a8_4074_9b9e_ccf49aa2d0b6);
impl core::ops::Deref for ISpXMLRecoResult {
    type Target = ISpRecoResult;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpXMLRecoResult, windows_core::IUnknown, ISpPhrase, ISpRecoResult);
impl ISpXMLRecoResult {
    pub unsafe fn GetXMLResult(&self, ppszcomemxmlresult: *mut windows_core::PWSTR, options: SPXMLRESULTOPTIONS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetXMLResult)(windows_core::Interface::as_raw(self), ppszcomemxmlresult, options).ok()
    }
    pub unsafe fn GetXMLErrorInfo(&self, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetXMLErrorInfo)(windows_core::Interface::as_raw(self), psemanticerrorinfo).ok()
    }
}
#[repr(C)]
pub struct ISpXMLRecoResult_Vtbl {
    pub base__: ISpRecoResult_Vtbl,
    pub GetXMLResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, SPXMLRESULTOPTIONS) -> windows_core::HRESULT,
    pub GetXMLErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SPSEMANTICERRORINFO) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechAudio, ISpeechAudio_Vtbl, 0xcff8e175_019e_11d3_a08e_00c04f8ef9b5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechAudio {
    type Target = ISpeechBaseStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechAudio, windows_core::IUnknown, super::super::System::Com::IDispatch, ISpeechBaseStream);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechAudio {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Status(&self) -> windows_core::Result<ISpeechAudioStatus> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BufferInfo(&self) -> windows_core::Result<ISpeechAudioBufferInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BufferInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DefaultFormat(&self) -> windows_core::Result<ISpeechAudioFormat> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DefaultFormat)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Volume(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Volume)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetVolume(&self, volume: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetVolume)(windows_core::Interface::as_raw(self), volume).ok()
    }
    pub unsafe fn BufferNotifySize(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BufferNotifySize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBufferNotifySize(&self, buffernotifysize: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBufferNotifySize)(windows_core::Interface::as_raw(self), buffernotifysize).ok()
    }
    pub unsafe fn EventHandle(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetState(&self, state: SpeechAudioState) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetState)(windows_core::Interface::as_raw(self), state).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechAudio_Vtbl {
    pub base__: ISpeechBaseStream_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Status: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub BufferInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BufferInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DefaultFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DefaultFormat: usize,
    pub Volume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub BufferNotifySize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBufferNotifySize: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EventHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetState: unsafe extern "system" fn(*mut core::ffi::c_void, SpeechAudioState) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechAudioBufferInfo, ISpeechAudioBufferInfo_Vtbl, 0x11b103d8_1142_4edf_a093_82fb3915f8cc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechAudioBufferInfo {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechAudioBufferInfo, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechAudioBufferInfo {
    pub unsafe fn MinNotification(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MinNotification)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMinNotification(&self, minnotification: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMinNotification)(windows_core::Interface::as_raw(self), minnotification).ok()
    }
    pub unsafe fn BufferSize(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BufferSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBufferSize(&self, buffersize: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBufferSize)(windows_core::Interface::as_raw(self), buffersize).ok()
    }
    pub unsafe fn EventBias(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventBias)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEventBias(&self, eventbias: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEventBias)(windows_core::Interface::as_raw(self), eventbias).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechAudioBufferInfo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub MinNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMinNotification: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub BufferSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBufferSize: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EventBias: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEventBias: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechAudioFormat, ISpeechAudioFormat_Vtbl, 0xe6e9c590_3e18_40e3_8299_061f98bde7c7);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechAudioFormat {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechAudioFormat, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechAudioFormat {
    pub unsafe fn Type(&self) -> windows_core::Result<SpeechAudioFormatType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetType(&self, audioformat: SpeechAudioFormatType) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetType)(windows_core::Interface::as_raw(self), audioformat).ok()
    }
    pub unsafe fn Guid(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Guid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetGuid<P0>(&self, guid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetGuid)(windows_core::Interface::as_raw(self), guid.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWaveFormatEx(&self) -> windows_core::Result<ISpeechWaveFormatEx> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWaveFormatEx)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetWaveFormatEx<P0>(&self, speechwaveformatex: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpeechWaveFormatEx>,
    {
        (windows_core::Interface::vtable(self).SetWaveFormatEx)(windows_core::Interface::as_raw(self), speechwaveformatex.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechAudioFormat_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechAudioFormatType) -> windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, SpeechAudioFormatType) -> windows_core::HRESULT,
    pub Guid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetGuid: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWaveFormatEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWaveFormatEx: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetWaveFormatEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetWaveFormatEx: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechAudioStatus, ISpeechAudioStatus_Vtbl, 0xc62d9c91_7458_47f6_862d_1ef86fb0b278);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechAudioStatus {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechAudioStatus, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechAudioStatus {
    pub unsafe fn FreeBufferSpace(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FreeBufferSpace)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NonBlockingIO(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NonBlockingIO)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn State(&self) -> windows_core::Result<SpeechAudioState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentSeekPosition(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentSeekPosition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentDevicePosition(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentDevicePosition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechAudioStatus_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub FreeBufferSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NonBlockingIO: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechAudioState) -> windows_core::HRESULT,
    pub CurrentSeekPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub CurrentDevicePosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechBaseStream, ISpeechBaseStream_Vtbl, 0x6450336f_7d49_4ced_8097_49d6dee37294);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechBaseStream {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechBaseStream, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechBaseStream {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Format(&self) -> windows_core::Result<ISpeechAudioFormat> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Format)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_Format<P0>(&self, audioformat: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpeechAudioFormat>,
    {
        (windows_core::Interface::vtable(self).putref_Format)(windows_core::Interface::as_raw(self), audioformat.param().abi()).ok()
    }
    pub unsafe fn Read(&self, buffer: *mut windows_core::VARIANT, numberofbytes: i32, bytesread: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), core::mem::transmute(buffer), numberofbytes, bytesread).ok()
    }
    pub unsafe fn Write<P0>(&self, buffer: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), buffer.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Seek<P0>(&self, position: P0, origin: SpeechStreamSeekPositionType) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Seek)(windows_core::Interface::as_raw(self), position.param().abi(), origin, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechBaseStream_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Format: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Format: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Format: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Format: usize,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>, i32, *mut i32) -> windows_core::HRESULT,
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut i32) -> windows_core::HRESULT,
    pub Seek: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, SpeechStreamSeekPositionType, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechCustomStream, ISpeechCustomStream_Vtbl, 0x1a9e9f4f_104f_4db8_a115_efd7fd0c97ae);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechCustomStream {
    type Target = ISpeechBaseStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechCustomStream, windows_core::IUnknown, super::super::System::Com::IDispatch, ISpeechBaseStream);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechCustomStream {
    pub unsafe fn BaseStream(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BaseStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn putref_BaseStream<P0>(&self, punkstream: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).putref_BaseStream)(windows_core::Interface::as_raw(self), punkstream.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechCustomStream_Vtbl {
    pub base__: ISpeechBaseStream_Vtbl,
    pub BaseStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_BaseStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechDataKey, ISpeechDataKey_Vtbl, 0xce17c09b_4efa_44d5_a4c9_59d9585ab0cd);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechDataKey {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechDataKey, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechDataKey {
    pub unsafe fn SetBinaryValue<P0, P1>(&self, valuename: P0, value: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetBinaryValue)(windows_core::Interface::as_raw(self), valuename.param().abi(), value.param().abi()).ok()
    }
    pub unsafe fn GetBinaryValue<P0>(&self, valuename: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBinaryValue)(windows_core::Interface::as_raw(self), valuename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetStringValue<P0, P1>(&self, valuename: P0, value: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetStringValue)(windows_core::Interface::as_raw(self), valuename.param().abi(), value.param().abi()).ok()
    }
    pub unsafe fn GetStringValue<P0>(&self, valuename: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStringValue)(windows_core::Interface::as_raw(self), valuename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLongValue<P0>(&self, valuename: P0, value: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLongValue)(windows_core::Interface::as_raw(self), valuename.param().abi(), value).ok()
    }
    pub unsafe fn GetLongValue<P0>(&self, valuename: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLongValue)(windows_core::Interface::as_raw(self), valuename.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenKey<P0>(&self, subkeyname: P0) -> windows_core::Result<ISpeechDataKey>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenKey)(windows_core::Interface::as_raw(self), subkeyname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateKey<P0>(&self, subkeyname: P0) -> windows_core::Result<ISpeechDataKey>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateKey)(windows_core::Interface::as_raw(self), subkeyname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteKey<P0>(&self, subkeyname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteKey)(windows_core::Interface::as_raw(self), subkeyname.param().abi()).ok()
    }
    pub unsafe fn DeleteValue<P0>(&self, valuename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteValue)(windows_core::Interface::as_raw(self), valuename.param().abi()).ok()
    }
    pub unsafe fn EnumKeys(&self, index: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumKeys)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumValues(&self, index: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumValues)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechDataKey_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SetBinaryValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetBinaryValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetStringValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetStringValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLongValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
    pub GetLongValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenKey: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenKey: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateKey: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateKey: usize,
    pub DeleteKey: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DeleteValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EnumKeys: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EnumValues: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechFileStream, ISpeechFileStream_Vtbl, 0xaf67f125_ab39_4e93_b4a2_cc2e66e182a7);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechFileStream {
    type Target = ISpeechBaseStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechFileStream, windows_core::IUnknown, super::super::System::Com::IDispatch, ISpeechBaseStream);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechFileStream {
    pub unsafe fn Open<P0, P1>(&self, filename: P0, filemode: SpeechStreamFileMode, doevents: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), filename.param().abi(), filemode, doevents.param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechFileStream_Vtbl {
    pub base__: ISpeechBaseStream_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, SpeechStreamFileMode, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechGrammarRule, ISpeechGrammarRule_Vtbl, 0xafe719cf_5dd1_44f2_999c_7a399f1cfccc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechGrammarRule {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechGrammarRule, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechGrammarRule {
    pub unsafe fn Attributes(&self) -> windows_core::Result<SpeechRuleAttributes> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Attributes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitialState(&self) -> windows_core::Result<ISpeechGrammarRuleState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InitialState)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Id(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddResource<P0, P1>(&self, resourcename: P0, resourcevalue: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AddResource)(windows_core::Interface::as_raw(self), resourcename.param().abi(), resourcevalue.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddState(&self) -> windows_core::Result<ISpeechGrammarRuleState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddState)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechGrammarRule_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Attributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechRuleAttributes) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InitialState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitialState: usize,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddResource: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddState: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechGrammarRuleState, ISpeechGrammarRuleState_Vtbl, 0xd4286f2c_ee67_45ae_b928_28d695362eda);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechGrammarRuleState {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechGrammarRuleState, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechGrammarRuleState {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Rule(&self) -> windows_core::Result<ISpeechGrammarRule> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Rule)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Transitions(&self) -> windows_core::Result<ISpeechGrammarRuleStateTransitions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Transitions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddWordTransition<P0, P1, P2, P3>(&self, deststate: P0, words: P1, separators: P2, r#type: SpeechGrammarWordType, propertyname: P3, propertyid: i32, propertyvalue: *const windows_core::VARIANT, weight: f32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpeechGrammarRuleState>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AddWordTransition)(windows_core::Interface::as_raw(self), deststate.param().abi(), words.param().abi(), separators.param().abi(), r#type, propertyname.param().abi(), propertyid, core::mem::transmute(propertyvalue), weight).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddRuleTransition<P0, P1, P2>(&self, destinationstate: P0, rule: P1, propertyname: P2, propertyid: i32, propertyvalue: *const windows_core::VARIANT, weight: f32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpeechGrammarRuleState>,
        P1: windows_core::Param<ISpeechGrammarRule>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AddRuleTransition)(windows_core::Interface::as_raw(self), destinationstate.param().abi(), rule.param().abi(), propertyname.param().abi(), propertyid, core::mem::transmute(propertyvalue), weight).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddSpecialTransition<P0, P1>(&self, destinationstate: P0, r#type: SpeechSpecialTransitionType, propertyname: P1, propertyid: i32, propertyvalue: *const windows_core::VARIANT, weight: f32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpeechGrammarRuleState>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AddSpecialTransition)(windows_core::Interface::as_raw(self), destinationstate.param().abi(), r#type, propertyname.param().abi(), propertyid, core::mem::transmute(propertyvalue), weight).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechGrammarRuleState_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Rule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Rule: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Transitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Transitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddWordTransition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, SpeechGrammarWordType, core::mem::MaybeUninit<windows_core::BSTR>, i32, *const core::mem::MaybeUninit<windows_core::VARIANT>, f32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddWordTransition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRuleTransition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *const core::mem::MaybeUninit<windows_core::VARIANT>, f32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRuleTransition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddSpecialTransition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, SpeechSpecialTransitionType, core::mem::MaybeUninit<windows_core::BSTR>, i32, *const core::mem::MaybeUninit<windows_core::VARIANT>, f32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddSpecialTransition: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechGrammarRuleStateTransition, ISpeechGrammarRuleStateTransition_Vtbl, 0xcafd1db1_41d1_4a06_9863_e2e81da17a9a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechGrammarRuleStateTransition {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechGrammarRuleStateTransition, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechGrammarRuleStateTransition {
    pub unsafe fn Type(&self) -> windows_core::Result<SpeechGrammarRuleStateTransitionType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Text(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Text)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Rule(&self) -> windows_core::Result<ISpeechGrammarRule> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Rule)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Weight(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Weight)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PropertyName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PropertyName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PropertyId(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PropertyId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PropertyValue(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PropertyValue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NextState(&self) -> windows_core::Result<ISpeechGrammarRuleState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NextState)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechGrammarRuleStateTransition_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechGrammarRuleStateTransitionType) -> windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Rule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Rule: usize,
    pub Weight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PropertyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PropertyId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub NextState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NextState: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechGrammarRuleStateTransitions, ISpeechGrammarRuleStateTransitions_Vtbl, 0xeabce657_75bc_44a2_aa7f_c56476742963);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechGrammarRuleStateTransitions {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechGrammarRuleStateTransitions, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechGrammarRuleStateTransitions {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<ISpeechGrammarRuleStateTransition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechGrammarRuleStateTransitions_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechGrammarRules, ISpeechGrammarRules_Vtbl, 0x6ffa3b44_fc2d_40d1_8afc_32911c7f1ad1);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechGrammarRules {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechGrammarRules, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechGrammarRules {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FindRule<P0>(&self, rulenameorid: P0) -> windows_core::Result<ISpeechGrammarRule>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindRule)(windows_core::Interface::as_raw(self), rulenameorid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<ISpeechGrammarRule> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Dynamic(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Dynamic)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, rulename: P0, attributes: SpeechRuleAttributes, ruleid: i32) -> windows_core::Result<ISpeechGrammarRule>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), rulename.param().abi(), attributes, ruleid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Commit(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CommitAndSave(&self, errortext: *mut windows_core::BSTR, savestream: *mut windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CommitAndSave)(windows_core::Interface::as_raw(self), core::mem::transmute(errortext), core::mem::transmute(savestream)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechGrammarRules_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub FindRule: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FindRule: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Dynamic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, SpeechRuleAttributes, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CommitAndSave: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechLexicon, ISpeechLexicon_Vtbl, 0x3da7627a_c7ae_4b23_8708_638c50362c25);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechLexicon {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechLexicon, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechLexicon {
    pub unsafe fn GenerationId(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GenerationId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWords(&self, flags: SpeechLexiconType, generationid: *mut i32, words: *mut Option<ISpeechLexiconWords>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetWords)(windows_core::Interface::as_raw(self), flags, generationid, core::mem::transmute(words)).ok()
    }
    pub unsafe fn AddPronunciation<P0, P1>(&self, bstrword: P0, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AddPronunciation)(windows_core::Interface::as_raw(self), bstrword.param().abi(), langid, partofspeech, bstrpronunciation.param().abi()).ok()
    }
    pub unsafe fn AddPronunciationByPhoneIds<P0>(&self, bstrword: P0, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AddPronunciationByPhoneIds)(windows_core::Interface::as_raw(self), bstrword.param().abi(), langid, partofspeech, core::mem::transmute(phoneids)).ok()
    }
    pub unsafe fn RemovePronunciation<P0, P1>(&self, bstrword: P0, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).RemovePronunciation)(windows_core::Interface::as_raw(self), bstrword.param().abi(), langid, partofspeech, bstrpronunciation.param().abi()).ok()
    }
    pub unsafe fn RemovePronunciationByPhoneIds<P0>(&self, bstrword: P0, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).RemovePronunciationByPhoneIds)(windows_core::Interface::as_raw(self), bstrword.param().abi(), langid, partofspeech, core::mem::transmute(phoneids)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPronunciations<P0>(&self, bstrword: P0, langid: i32, typeflags: SpeechLexiconType) -> windows_core::Result<ISpeechLexiconPronunciations>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPronunciations)(windows_core::Interface::as_raw(self), bstrword.param().abi(), langid, typeflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetGenerationChange(&self, generationid: *mut i32, ppwords: *mut Option<ISpeechLexiconWords>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGenerationChange)(windows_core::Interface::as_raw(self), generationid, core::mem::transmute(ppwords)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechLexicon_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub GenerationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWords: unsafe extern "system" fn(*mut core::ffi::c_void, SpeechLexiconType, *mut i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWords: usize,
    pub AddPronunciation: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, SpeechPartOfSpeech, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AddPronunciationByPhoneIds: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, SpeechPartOfSpeech, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub RemovePronunciation: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, SpeechPartOfSpeech, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RemovePronunciationByPhoneIds: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, SpeechPartOfSpeech, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPronunciations: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, SpeechLexiconType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPronunciations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetGenerationChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetGenerationChange: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechLexiconPronunciation, ISpeechLexiconPronunciation_Vtbl, 0x95252c5d_9e43_4f4a_9899_48ee73352f9f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechLexiconPronunciation {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechLexiconPronunciation, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechLexiconPronunciation {
    pub unsafe fn Type(&self) -> windows_core::Result<SpeechLexiconType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LangId(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LangId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PartOfSpeech(&self) -> windows_core::Result<SpeechPartOfSpeech> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PartOfSpeech)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PhoneIds(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PhoneIds)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Symbolic(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Symbolic)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechLexiconPronunciation_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechLexiconType) -> windows_core::HRESULT,
    pub LangId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PartOfSpeech: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechPartOfSpeech) -> windows_core::HRESULT,
    pub PhoneIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Symbolic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechLexiconPronunciations, ISpeechLexiconPronunciations_Vtbl, 0x72829128_5682_4704_a0d4_3e2bb6f2ead3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechLexiconPronunciations {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechLexiconPronunciations, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechLexiconPronunciations {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<ISpeechLexiconPronunciation> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechLexiconPronunciations_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechLexiconWord, ISpeechLexiconWord_Vtbl, 0x4e5b933c_c9be_48ed_8842_1ee51bb1d4ff);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechLexiconWord {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechLexiconWord, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechLexiconWord {
    pub unsafe fn LangId(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LangId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Type(&self) -> windows_core::Result<SpeechWordType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Word(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Word)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Pronunciations(&self) -> windows_core::Result<ISpeechLexiconPronunciations> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Pronunciations)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechLexiconWord_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub LangId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechWordType) -> windows_core::HRESULT,
    pub Word: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Pronunciations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Pronunciations: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechLexiconWords, ISpeechLexiconWords_Vtbl, 0x8d199862_415e_47d5_ac4f_faa608b424e6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechLexiconWords {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechLexiconWords, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechLexiconWords {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<ISpeechLexiconWord> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechLexiconWords_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechMMSysAudio, ISpeechMMSysAudio_Vtbl, 0x3c76af6d_1fd7_4831_81d1_3b71d5a13c44);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechMMSysAudio {
    type Target = ISpeechAudio;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechMMSysAudio, windows_core::IUnknown, super::super::System::Com::IDispatch, ISpeechBaseStream, ISpeechAudio);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechMMSysAudio {
    pub unsafe fn DeviceId(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DeviceId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDeviceId(&self, deviceid: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDeviceId)(windows_core::Interface::as_raw(self), deviceid).ok()
    }
    pub unsafe fn LineId(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LineId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLineId(&self, lineid: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLineId)(windows_core::Interface::as_raw(self), lineid).ok()
    }
    pub unsafe fn MMHandle(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MMHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechMMSysAudio_Vtbl {
    pub base__: ISpeechAudio_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub LineId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetLineId: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MMHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechMemoryStream, ISpeechMemoryStream_Vtbl, 0xeeb14b68_808b_4abe_a5ea_b51da7588008);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechMemoryStream {
    type Target = ISpeechBaseStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechMemoryStream, windows_core::IUnknown, super::super::System::Com::IDispatch, ISpeechBaseStream);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechMemoryStream {
    pub unsafe fn SetData<P0>(&self, data: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetData)(windows_core::Interface::as_raw(self), data.param().abi()).ok()
    }
    pub unsafe fn GetData(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechMemoryStream_Vtbl {
    pub base__: ISpeechBaseStream_Vtbl,
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechObjectToken, ISpeechObjectToken_Vtbl, 0xc74a3adc_b727_4500_a84a_b526721c8b8c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechObjectToken {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechObjectToken, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechObjectToken {
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DataKey(&self) -> windows_core::Result<ISpeechDataKey> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DataKey)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Category(&self) -> windows_core::Result<ISpeechObjectTokenCategory> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Category)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDescription(&self, locale: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), locale, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetId<P0, P1, P2>(&self, id: P0, categoryid: P1, createifnotexist: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetId)(windows_core::Interface::as_raw(self), id.param().abi(), categoryid.param().abi(), createifnotexist.param().abi()).ok()
    }
    pub unsafe fn GetAttribute<P0>(&self, attributename: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAttribute)(windows_core::Interface::as_raw(self), attributename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateInstance<P0>(&self, punkouter: P0, clscontext: SpeechTokenContext) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), punkouter.param().abi(), clscontext, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Remove<P0>(&self, objectstorageclsid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), objectstorageclsid.param().abi()).ok()
    }
    pub unsafe fn GetStorageFileName<P0, P1, P2>(&self, objectstorageclsid: P0, keyname: P1, filename: P2, folder: SpeechTokenShellFolder) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStorageFileName)(windows_core::Interface::as_raw(self), objectstorageclsid.param().abi(), keyname.param().abi(), filename.param().abi(), folder, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemoveStorageFileName<P0, P1, P2>(&self, objectstorageclsid: P0, keyname: P1, deletefile: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).RemoveStorageFileName)(windows_core::Interface::as_raw(self), objectstorageclsid.param().abi(), keyname.param().abi(), deletefile.param().abi()).ok()
    }
    pub unsafe fn IsUISupported<P0, P1>(&self, typeofui: P0, extradata: *const windows_core::VARIANT, object: P1) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsUISupported)(windows_core::Interface::as_raw(self), typeofui.param().abi(), core::mem::transmute(extradata), object.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn DisplayUI<P0, P1, P2>(&self, hwnd: i32, title: P0, typeofui: P1, extradata: *const windows_core::VARIANT, object: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).DisplayUI)(windows_core::Interface::as_raw(self), hwnd, title.param().abi(), typeofui.param().abi(), core::mem::transmute(extradata), object.param().abi()).ok()
    }
    pub unsafe fn MatchesAttributes<P0>(&self, attributes: P0) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MatchesAttributes)(windows_core::Interface::as_raw(self), attributes.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechObjectToken_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DataKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DataKey: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Category: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Category: usize,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, SpeechTokenContext, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetStorageFileName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, SpeechTokenShellFolder, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RemoveStorageFileName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsUISupported: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DisplayUI: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MatchesAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechObjectTokenCategory, ISpeechObjectTokenCategory_Vtbl, 0xca7eac50_2d01_4145_86d4_5ae7d70f4469);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechObjectTokenCategory {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechObjectTokenCategory, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechObjectTokenCategory {
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDefault<P0>(&self, tokenid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDefault)(windows_core::Interface::as_raw(self), tokenid.param().abi()).ok()
    }
    pub unsafe fn Default(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Default)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetId<P0, P1>(&self, id: P0, createifnotexist: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetId)(windows_core::Interface::as_raw(self), id.param().abi(), createifnotexist.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDataKey(&self, location: SpeechDataKeyLocation) -> windows_core::Result<ISpeechDataKey> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDataKey)(windows_core::Interface::as_raw(self), location, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumerateTokens<P0, P1>(&self, requiredattributes: P0, optionalattributes: P1) -> windows_core::Result<ISpeechObjectTokens>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateTokens)(windows_core::Interface::as_raw(self), requiredattributes.param().abi(), optionalattributes.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechObjectTokenCategory_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Default: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDataKey: unsafe extern "system" fn(*mut core::ffi::c_void, SpeechDataKeyLocation, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDataKey: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumerateTokens: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumerateTokens: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechObjectTokens, ISpeechObjectTokens_Vtbl, 0x9285b776_2e7b_4bc0_b53e_580eb6fa967f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechObjectTokens {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechObjectTokens, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechObjectTokens {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<ISpeechObjectToken> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechObjectTokens_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechPhoneConverter, ISpeechPhoneConverter_Vtbl, 0xc3e4f353_433f_43d6_89a1_6a62a7054c3d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechPhoneConverter {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechPhoneConverter, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhoneConverter {
    pub unsafe fn LanguageId(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LanguageId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLanguageId(&self, languageid: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLanguageId)(windows_core::Interface::as_raw(self), languageid).ok()
    }
    pub unsafe fn PhoneToId<P0>(&self, phonemes: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PhoneToId)(windows_core::Interface::as_raw(self), phonemes.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IdToPhone<P0>(&self, idarray: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IdToPhone)(windows_core::Interface::as_raw(self), idarray.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechPhoneConverter_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub LanguageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetLanguageId: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub PhoneToId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub IdToPhone: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechPhraseAlternate, ISpeechPhraseAlternate_Vtbl, 0x27864a2a_2b9f_4cb8_92d3_0d2722fd1e73);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechPhraseAlternate {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechPhraseAlternate, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseAlternate {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RecoResult(&self) -> windows_core::Result<ISpeechRecoResult> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RecoResult)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn StartElementInResult(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StartElementInResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NumberOfElementsInResult(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NumberOfElementsInResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PhraseInfo(&self) -> windows_core::Result<ISpeechPhraseInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PhraseInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Commit(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechPhraseAlternate_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RecoResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RecoResult: usize,
    pub StartElementInResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NumberOfElementsInResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PhraseInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PhraseInfo: usize,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechPhraseAlternates, ISpeechPhraseAlternates_Vtbl, 0xb238b6d5_f276_4c3d_a6c1_2974801c3cc2);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechPhraseAlternates {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechPhraseAlternates, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseAlternates {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<ISpeechPhraseAlternate> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechPhraseAlternates_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechPhraseElement, ISpeechPhraseElement_Vtbl, 0xe6176f96_e373_4801_b223_3b62c068c0b4);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechPhraseElement {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechPhraseElement, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseElement {
    pub unsafe fn AudioTimeOffset(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AudioTimeOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AudioSizeTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AudioSizeTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AudioStreamOffset(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AudioStreamOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AudioSizeBytes(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AudioSizeBytes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RetainedStreamOffset(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RetainedStreamOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RetainedSizeBytes(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RetainedSizeBytes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DisplayText(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisplayText)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LexicalForm(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LexicalForm)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Pronunciation(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Pronunciation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DisplayAttributes(&self) -> windows_core::Result<SpeechDisplayAttributes> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisplayAttributes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RequiredConfidence(&self) -> windows_core::Result<SpeechEngineConfidence> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequiredConfidence)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ActualConfidence(&self) -> windows_core::Result<SpeechEngineConfidence> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ActualConfidence)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn EngineConfidence(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EngineConfidence)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechPhraseElement_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub AudioTimeOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub AudioSizeTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub AudioStreamOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub AudioSizeBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RetainedStreamOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RetainedSizeBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DisplayText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LexicalForm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Pronunciation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DisplayAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechDisplayAttributes) -> windows_core::HRESULT,
    pub RequiredConfidence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechEngineConfidence) -> windows_core::HRESULT,
    pub ActualConfidence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechEngineConfidence) -> windows_core::HRESULT,
    pub EngineConfidence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechPhraseElements, ISpeechPhraseElements_Vtbl, 0x0626b328_3478_467d_a0b3_d0853b93dda3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechPhraseElements {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechPhraseElements, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseElements {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<ISpeechPhraseElement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechPhraseElements_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechPhraseInfo, ISpeechPhraseInfo_Vtbl, 0x961559cf_4e67_4662_8bf0_d93f1fcd61b3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechPhraseInfo {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechPhraseInfo, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseInfo {
    pub unsafe fn LanguageId(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LanguageId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GrammarId(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GrammarId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn StartTime(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StartTime)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AudioStreamPosition(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AudioStreamPosition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AudioSizeBytes(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AudioSizeBytes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RetainedSizeBytes(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RetainedSizeBytes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AudioSizeTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AudioSizeTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Rule(&self) -> windows_core::Result<ISpeechPhraseRule> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Rule)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<ISpeechPhraseProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Elements(&self) -> windows_core::Result<ISpeechPhraseElements> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Elements)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Replacements(&self) -> windows_core::Result<ISpeechPhraseReplacements> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Replacements)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EngineId(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EngineId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnginePrivateData(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnginePrivateData)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SaveToMemory(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SaveToMemory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetText<P0>(&self, startelement: i32, elements: i32, usereplacements: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), startelement, elements, usereplacements.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDisplayAttributes<P0>(&self, startelement: i32, elements: i32, usereplacements: P0) -> windows_core::Result<SpeechDisplayAttributes>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDisplayAttributes)(windows_core::Interface::as_raw(self), startelement, elements, usereplacements.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechPhraseInfo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub LanguageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GrammarId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AudioStreamPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AudioSizeBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RetainedSizeBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub AudioSizeTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Rule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Rule: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Elements: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Elements: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Replacements: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Replacements: usize,
    pub EngineId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EnginePrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SaveToMemory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, super::super::Foundation::VARIANT_BOOL, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetDisplayAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, super::super::Foundation::VARIANT_BOOL, *mut SpeechDisplayAttributes) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechPhraseInfoBuilder, ISpeechPhraseInfoBuilder_Vtbl, 0x3b151836_df3a_4e0a_846c_d2adc9334333);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechPhraseInfoBuilder {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechPhraseInfoBuilder, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseInfoBuilder {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RestorePhraseFromMemory(&self, phraseinmemory: *const windows_core::VARIANT) -> windows_core::Result<ISpeechPhraseInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RestorePhraseFromMemory)(windows_core::Interface::as_raw(self), core::mem::transmute(phraseinmemory), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechPhraseInfoBuilder_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RestorePhraseFromMemory: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RestorePhraseFromMemory: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechPhraseProperties, ISpeechPhraseProperties_Vtbl, 0x08166b47_102e_4b23_a599_bdb98dbfd1f4);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechPhraseProperties {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechPhraseProperties, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseProperties {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<ISpeechPhraseProperty> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechPhraseProperties_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechPhraseProperty, ISpeechPhraseProperty_Vtbl, 0xce563d48_961e_4732_a2e1_378a42b430be);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechPhraseProperty {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechPhraseProperty, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseProperty {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Id(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FirstElement(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FirstElement)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NumberOfElements(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NumberOfElements)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn EngineConfidence(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EngineConfidence)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Confidence(&self) -> windows_core::Result<SpeechEngineConfidence> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Confidence)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> windows_core::Result<ISpeechPhraseProperty> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Parent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Children(&self) -> windows_core::Result<ISpeechPhraseProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Children)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechPhraseProperty_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub FirstElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NumberOfElements: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EngineConfidence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub Confidence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechEngineConfidence) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Parent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Children: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Children: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechPhraseReplacement, ISpeechPhraseReplacement_Vtbl, 0x2890a410_53a7_4fb5_94ec_06d4998e3d02);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechPhraseReplacement {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechPhraseReplacement, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseReplacement {
    pub unsafe fn DisplayAttributes(&self) -> windows_core::Result<SpeechDisplayAttributes> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisplayAttributes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Text(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Text)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FirstElement(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FirstElement)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NumberOfElements(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NumberOfElements)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechPhraseReplacement_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub DisplayAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechDisplayAttributes) -> windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub FirstElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NumberOfElements: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechPhraseReplacements, ISpeechPhraseReplacements_Vtbl, 0x38bc662f_2257_4525_959e_2069d2596c05);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechPhraseReplacements {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechPhraseReplacements, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseReplacements {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<ISpeechPhraseReplacement> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechPhraseReplacements_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechPhraseRule, ISpeechPhraseRule_Vtbl, 0xa7bfe112_a4a0_48d9_b602_c313843f6964);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechPhraseRule {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechPhraseRule, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseRule {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Id(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn FirstElement(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FirstElement)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NumberOfElements(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NumberOfElements)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> windows_core::Result<ISpeechPhraseRule> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Parent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Children(&self) -> windows_core::Result<ISpeechPhraseRules> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Children)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Confidence(&self) -> windows_core::Result<SpeechEngineConfidence> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Confidence)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn EngineConfidence(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EngineConfidence)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechPhraseRule_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub FirstElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NumberOfElements: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Parent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Children: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Children: usize,
    pub Confidence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechEngineConfidence) -> windows_core::HRESULT,
    pub EngineConfidence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechPhraseRules, ISpeechPhraseRules_Vtbl, 0x9047d593_01dd_4b72_81a3_e4a0ca69f407);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechPhraseRules {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechPhraseRules, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseRules {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<ISpeechPhraseRule> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechPhraseRules_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechRecoContext, ISpeechRecoContext_Vtbl, 0x580aa49d_7e1e_4809_b8e2_57da806104b8);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechRecoContext {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechRecoContext, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechRecoContext {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recognizer(&self) -> windows_core::Result<ISpeechRecognizer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Recognizer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AudioInputInterferenceStatus(&self) -> windows_core::Result<SpeechInterference> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AudioInputInterferenceStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RequestedUIType(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestedUIType)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_Voice<P0>(&self, voice: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpeechVoice>,
    {
        (windows_core::Interface::vtable(self).putref_Voice)(windows_core::Interface::as_raw(self), voice.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Voice(&self) -> windows_core::Result<ISpeechVoice> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Voice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetAllowVoiceFormatMatchingOnNextSet<P0>(&self, allow: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllowVoiceFormatMatchingOnNextSet)(windows_core::Interface::as_raw(self), allow.param().abi()).ok()
    }
    pub unsafe fn AllowVoiceFormatMatchingOnNextSet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllowVoiceFormatMatchingOnNextSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetVoicePurgeEvent(&self, eventinterest: SpeechRecoEvents) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetVoicePurgeEvent)(windows_core::Interface::as_raw(self), eventinterest).ok()
    }
    pub unsafe fn VoicePurgeEvent(&self) -> windows_core::Result<SpeechRecoEvents> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VoicePurgeEvent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEventInterests(&self, eventinterest: SpeechRecoEvents) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEventInterests)(windows_core::Interface::as_raw(self), eventinterest).ok()
    }
    pub unsafe fn EventInterests(&self) -> windows_core::Result<SpeechRecoEvents> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventInterests)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCmdMaxAlternates(&self, maxalternates: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCmdMaxAlternates)(windows_core::Interface::as_raw(self), maxalternates).ok()
    }
    pub unsafe fn CmdMaxAlternates(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CmdMaxAlternates)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetState(&self, state: SpeechRecoContextState) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetState)(windows_core::Interface::as_raw(self), state).ok()
    }
    pub unsafe fn State(&self) -> windows_core::Result<SpeechRecoContextState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetRetainedAudio(&self, option: SpeechRetainedAudioOptions) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRetainedAudio)(windows_core::Interface::as_raw(self), option).ok()
    }
    pub unsafe fn RetainedAudio(&self) -> windows_core::Result<SpeechRetainedAudioOptions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RetainedAudio)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_RetainedAudioFormat<P0>(&self, format: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpeechAudioFormat>,
    {
        (windows_core::Interface::vtable(self).putref_RetainedAudioFormat)(windows_core::Interface::as_raw(self), format.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RetainedAudioFormat(&self) -> windows_core::Result<ISpeechAudioFormat> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RetainedAudioFormat)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateGrammar<P0>(&self, grammarid: P0) -> windows_core::Result<ISpeechRecoGrammar>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateGrammar)(windows_core::Interface::as_raw(self), grammarid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateResultFromMemory(&self, resultblock: *const windows_core::VARIANT) -> windows_core::Result<ISpeechRecoResult> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateResultFromMemory)(windows_core::Interface::as_raw(self), core::mem::transmute(resultblock), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Bookmark<P0, P1>(&self, options: SpeechBookmarkOptions, streampos: P0, bookmarkid: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Bookmark)(windows_core::Interface::as_raw(self), options, streampos.param().abi(), bookmarkid.param().abi()).ok()
    }
    pub unsafe fn SetAdaptationData<P0>(&self, adaptationstring: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetAdaptationData)(windows_core::Interface::as_raw(self), adaptationstring.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechRecoContext_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Recognizer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recognizer: usize,
    pub AudioInputInterferenceStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechInterference) -> windows_core::HRESULT,
    pub RequestedUIType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Voice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Voice: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Voice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Voice: usize,
    pub SetAllowVoiceFormatMatchingOnNextSet: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AllowVoiceFormatMatchingOnNextSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetVoicePurgeEvent: unsafe extern "system" fn(*mut core::ffi::c_void, SpeechRecoEvents) -> windows_core::HRESULT,
    pub VoicePurgeEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechRecoEvents) -> windows_core::HRESULT,
    pub SetEventInterests: unsafe extern "system" fn(*mut core::ffi::c_void, SpeechRecoEvents) -> windows_core::HRESULT,
    pub EventInterests: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechRecoEvents) -> windows_core::HRESULT,
    pub SetCmdMaxAlternates: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CmdMaxAlternates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetState: unsafe extern "system" fn(*mut core::ffi::c_void, SpeechRecoContextState) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechRecoContextState) -> windows_core::HRESULT,
    pub SetRetainedAudio: unsafe extern "system" fn(*mut core::ffi::c_void, SpeechRetainedAudioOptions) -> windows_core::HRESULT,
    pub RetainedAudio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechRetainedAudioOptions) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_RetainedAudioFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_RetainedAudioFormat: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RetainedAudioFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RetainedAudioFormat: usize,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateGrammar: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateGrammar: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateResultFromMemory: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateResultFromMemory: usize,
    pub Bookmark: unsafe extern "system" fn(*mut core::ffi::c_void, SpeechBookmarkOptions, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetAdaptationData: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechRecoGrammar, ISpeechRecoGrammar_Vtbl, 0xb6d6f79f_2158_4e50_b5bc_9a9ccd852a09);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechRecoGrammar {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechRecoGrammar, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechRecoGrammar {
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RecoContext(&self) -> windows_core::Result<ISpeechRecoContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RecoContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetState(&self, state: SpeechGrammarState) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetState)(windows_core::Interface::as_raw(self), state).ok()
    }
    pub unsafe fn State(&self) -> windows_core::Result<SpeechGrammarState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Rules(&self) -> windows_core::Result<ISpeechGrammarRules> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Rules)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Reset(&self, newlanguage: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), newlanguage).ok()
    }
    pub unsafe fn CmdLoadFromFile<P0>(&self, filename: P0, loadoption: SpeechLoadOption) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).CmdLoadFromFile)(windows_core::Interface::as_raw(self), filename.param().abi(), loadoption).ok()
    }
    pub unsafe fn CmdLoadFromObject<P0, P1>(&self, classid: P0, grammarname: P1, loadoption: SpeechLoadOption) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).CmdLoadFromObject)(windows_core::Interface::as_raw(self), classid.param().abi(), grammarname.param().abi(), loadoption).ok()
    }
    pub unsafe fn CmdLoadFromResource<P0, P1>(&self, hmodule: i32, resourcename: P0, resourcetype: P1, languageid: i32, loadoption: SpeechLoadOption) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).CmdLoadFromResource)(windows_core::Interface::as_raw(self), hmodule, resourcename.param().abi(), resourcetype.param().abi(), languageid, loadoption).ok()
    }
    pub unsafe fn CmdLoadFromMemory<P0>(&self, grammardata: P0, loadoption: SpeechLoadOption) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).CmdLoadFromMemory)(windows_core::Interface::as_raw(self), grammardata.param().abi(), loadoption).ok()
    }
    pub unsafe fn CmdLoadFromProprietaryGrammar<P0, P1, P2>(&self, proprietaryguid: P0, proprietarystring: P1, proprietarydata: P2, loadoption: SpeechLoadOption) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).CmdLoadFromProprietaryGrammar)(windows_core::Interface::as_raw(self), proprietaryguid.param().abi(), proprietarystring.param().abi(), proprietarydata.param().abi(), loadoption).ok()
    }
    pub unsafe fn CmdSetRuleState<P0>(&self, name: P0, state: SpeechRuleState) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).CmdSetRuleState)(windows_core::Interface::as_raw(self), name.param().abi(), state).ok()
    }
    pub unsafe fn CmdSetRuleIdState(&self, ruleid: i32, state: SpeechRuleState) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CmdSetRuleIdState)(windows_core::Interface::as_raw(self), ruleid, state).ok()
    }
    pub unsafe fn DictationLoad<P0>(&self, topicname: P0, loadoption: SpeechLoadOption) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DictationLoad)(windows_core::Interface::as_raw(self), topicname.param().abi(), loadoption).ok()
    }
    pub unsafe fn DictationUnload(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DictationUnload)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DictationSetState(&self, state: SpeechRuleState) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DictationSetState)(windows_core::Interface::as_raw(self), state).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetWordSequenceData<P0, P1>(&self, text: P0, textlength: i32, info: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<ISpeechTextSelectionInformation>,
    {
        (windows_core::Interface::vtable(self).SetWordSequenceData)(windows_core::Interface::as_raw(self), text.param().abi(), textlength, info.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetTextSelection<P0>(&self, info: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpeechTextSelectionInformation>,
    {
        (windows_core::Interface::vtable(self).SetTextSelection)(windows_core::Interface::as_raw(self), info.param().abi()).ok()
    }
    pub unsafe fn IsPronounceable<P0>(&self, word: P0) -> windows_core::Result<SpeechWordPronounceable>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsPronounceable)(windows_core::Interface::as_raw(self), word.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechRecoGrammar_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RecoContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RecoContext: usize,
    pub SetState: unsafe extern "system" fn(*mut core::ffi::c_void, SpeechGrammarState) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechGrammarState) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Rules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Rules: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CmdLoadFromFile: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, SpeechLoadOption) -> windows_core::HRESULT,
    pub CmdLoadFromObject: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, SpeechLoadOption) -> windows_core::HRESULT,
    pub CmdLoadFromResource: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>, i32, SpeechLoadOption) -> windows_core::HRESULT,
    pub CmdLoadFromMemory: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, SpeechLoadOption) -> windows_core::HRESULT,
    pub CmdLoadFromProprietaryGrammar: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, SpeechLoadOption) -> windows_core::HRESULT,
    pub CmdSetRuleState: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, SpeechRuleState) -> windows_core::HRESULT,
    pub CmdSetRuleIdState: unsafe extern "system" fn(*mut core::ffi::c_void, i32, SpeechRuleState) -> windows_core::HRESULT,
    pub DictationLoad: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, SpeechLoadOption) -> windows_core::HRESULT,
    pub DictationUnload: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DictationSetState: unsafe extern "system" fn(*mut core::ffi::c_void, SpeechRuleState) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetWordSequenceData: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetWordSequenceData: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetTextSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetTextSelection: usize,
    pub IsPronounceable: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut SpeechWordPronounceable) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechRecoResult, ISpeechRecoResult_Vtbl, 0xed2879cf_ced9_4ee6_a534_de0191d5468d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechRecoResult {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechRecoResult, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechRecoResult {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RecoContext(&self) -> windows_core::Result<ISpeechRecoContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RecoContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Times(&self) -> windows_core::Result<ISpeechRecoResultTimes> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Times)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AudioFormat<P0>(&self, format: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpeechAudioFormat>,
    {
        (windows_core::Interface::vtable(self).putref_AudioFormat)(windows_core::Interface::as_raw(self), format.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AudioFormat(&self) -> windows_core::Result<ISpeechAudioFormat> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AudioFormat)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PhraseInfo(&self) -> windows_core::Result<ISpeechPhraseInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PhraseInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Alternates(&self, requestcount: i32, startelement: i32, elements: i32) -> windows_core::Result<ISpeechPhraseAlternates> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Alternates)(windows_core::Interface::as_raw(self), requestcount, startelement, elements, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Audio(&self, startelement: i32, elements: i32) -> windows_core::Result<ISpeechMemoryStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Audio)(windows_core::Interface::as_raw(self), startelement, elements, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SpeakAudio(&self, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SpeakAudio)(windows_core::Interface::as_raw(self), startelement, elements, flags, &mut result__).map(|| result__)
    }
    pub unsafe fn SaveToMemory(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SaveToMemory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DiscardResultInfo(&self, valuetypes: SpeechDiscardType) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DiscardResultInfo)(windows_core::Interface::as_raw(self), valuetypes).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechRecoResult_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RecoContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RecoContext: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Times: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Times: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AudioFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AudioFormat: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AudioFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AudioFormat: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PhraseInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PhraseInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Alternates: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Alternates: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Audio: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Audio: usize,
    pub SpeakAudio: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, SpeechVoiceSpeakFlags, *mut i32) -> windows_core::HRESULT,
    pub SaveToMemory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DiscardResultInfo: unsafe extern "system" fn(*mut core::ffi::c_void, SpeechDiscardType) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechRecoResult2, ISpeechRecoResult2_Vtbl, 0x8e0a246d_d3c8_45de_8657_04290c458c3c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechRecoResult2 {
    type Target = ISpeechRecoResult;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechRecoResult2, windows_core::IUnknown, super::super::System::Com::IDispatch, ISpeechRecoResult);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechRecoResult2 {
    pub unsafe fn SetTextFeedback<P0, P1>(&self, feedback: P0, wassuccessful: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetTextFeedback)(windows_core::Interface::as_raw(self), feedback.param().abi(), wassuccessful.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechRecoResult2_Vtbl {
    pub base__: ISpeechRecoResult_Vtbl,
    pub SetTextFeedback: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechRecoResultDispatch, ISpeechRecoResultDispatch_Vtbl, 0x6d60eb64_aced_40a6_bbf3_4e557f71dee2);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechRecoResultDispatch {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechRecoResultDispatch, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechRecoResultDispatch {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RecoContext(&self) -> windows_core::Result<ISpeechRecoContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RecoContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Times(&self) -> windows_core::Result<ISpeechRecoResultTimes> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Times)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AudioFormat<P0>(&self, format: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpeechAudioFormat>,
    {
        (windows_core::Interface::vtable(self).putref_AudioFormat)(windows_core::Interface::as_raw(self), format.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AudioFormat(&self) -> windows_core::Result<ISpeechAudioFormat> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AudioFormat)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PhraseInfo(&self) -> windows_core::Result<ISpeechPhraseInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PhraseInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Alternates(&self, requestcount: i32, startelement: i32, elements: i32) -> windows_core::Result<ISpeechPhraseAlternates> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Alternates)(windows_core::Interface::as_raw(self), requestcount, startelement, elements, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Audio(&self, startelement: i32, elements: i32) -> windows_core::Result<ISpeechMemoryStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Audio)(windows_core::Interface::as_raw(self), startelement, elements, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SpeakAudio(&self, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SpeakAudio)(windows_core::Interface::as_raw(self), startelement, elements, flags, &mut result__).map(|| result__)
    }
    pub unsafe fn SaveToMemory(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SaveToMemory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DiscardResultInfo(&self, valuetypes: SpeechDiscardType) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DiscardResultInfo)(windows_core::Interface::as_raw(self), valuetypes).ok()
    }
    pub unsafe fn GetXMLResult(&self, options: SPXMLRESULTOPTIONS) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetXMLResult)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetXMLErrorInfo(&self, linenumber: *mut i32, scriptline: *mut windows_core::BSTR, source: *mut windows_core::BSTR, description: *mut windows_core::BSTR, resultcode: *mut windows_core::HRESULT, iserror: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetXMLErrorInfo)(windows_core::Interface::as_raw(self), linenumber, core::mem::transmute(scriptline), core::mem::transmute(source), core::mem::transmute(description), resultcode, iserror).ok()
    }
    pub unsafe fn SetTextFeedback<P0, P1>(&self, feedback: P0, wassuccessful: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetTextFeedback)(windows_core::Interface::as_raw(self), feedback.param().abi(), wassuccessful.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechRecoResultDispatch_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RecoContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RecoContext: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Times: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Times: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AudioFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AudioFormat: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AudioFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AudioFormat: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PhraseInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PhraseInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Alternates: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Alternates: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Audio: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Audio: usize,
    pub SpeakAudio: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, SpeechVoiceSpeakFlags, *mut i32) -> windows_core::HRESULT,
    pub SaveToMemory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DiscardResultInfo: unsafe extern "system" fn(*mut core::ffi::c_void, SpeechDiscardType) -> windows_core::HRESULT,
    pub GetXMLResult: unsafe extern "system" fn(*mut core::ffi::c_void, SPXMLRESULTOPTIONS, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetXMLErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut windows_core::HRESULT, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetTextFeedback: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechRecoResultTimes, ISpeechRecoResultTimes_Vtbl, 0x62b3b8fb_f6e7_41be_bdcb_056b1c29efc0);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechRecoResultTimes {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechRecoResultTimes, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechRecoResultTimes {
    pub unsafe fn StreamTime(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StreamTime)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Length(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Length)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TickCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TickCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn OffsetFromStart(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OffsetFromStart)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechRecoResultTimes_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub StreamTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub TickCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub OffsetFromStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechRecognizer, ISpeechRecognizer_Vtbl, 0x2d5f1c0c_bd75_4b08_9478_3b11fea2586c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechRecognizer {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechRecognizer, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechRecognizer {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_Recognizer<P0>(&self, recognizer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpeechObjectToken>,
    {
        (windows_core::Interface::vtable(self).putref_Recognizer)(windows_core::Interface::as_raw(self), recognizer.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recognizer(&self) -> windows_core::Result<ISpeechObjectToken> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Recognizer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetAllowAudioInputFormatChangesOnNextSet<P0>(&self, allow: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllowAudioInputFormatChangesOnNextSet)(windows_core::Interface::as_raw(self), allow.param().abi()).ok()
    }
    pub unsafe fn AllowAudioInputFormatChangesOnNextSet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllowAudioInputFormatChangesOnNextSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AudioInput<P0>(&self, audioinput: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpeechObjectToken>,
    {
        (windows_core::Interface::vtable(self).putref_AudioInput)(windows_core::Interface::as_raw(self), audioinput.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AudioInput(&self) -> windows_core::Result<ISpeechObjectToken> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AudioInput)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AudioInputStream<P0>(&self, audioinputstream: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpeechBaseStream>,
    {
        (windows_core::Interface::vtable(self).putref_AudioInputStream)(windows_core::Interface::as_raw(self), audioinputstream.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AudioInputStream(&self) -> windows_core::Result<ISpeechBaseStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AudioInputStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsShared(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsShared)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetState(&self, state: SpeechRecognizerState) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetState)(windows_core::Interface::as_raw(self), state).ok()
    }
    pub unsafe fn State(&self) -> windows_core::Result<SpeechRecognizerState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Status(&self) -> windows_core::Result<ISpeechRecognizerStatus> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_Profile<P0>(&self, profile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpeechObjectToken>,
    {
        (windows_core::Interface::vtable(self).putref_Profile)(windows_core::Interface::as_raw(self), profile.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Profile(&self) -> windows_core::Result<ISpeechObjectToken> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Profile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EmulateRecognition<P0>(&self, textelements: P0, elementdisplayattributes: *const windows_core::VARIANT, languageid: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).EmulateRecognition)(windows_core::Interface::as_raw(self), textelements.param().abi(), core::mem::transmute(elementdisplayattributes), languageid).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateRecoContext(&self) -> windows_core::Result<ISpeechRecoContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateRecoContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormat(&self, r#type: SpeechFormatType) -> windows_core::Result<ISpeechAudioFormat> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFormat)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPropertyNumber<P0>(&self, name: P0, value: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetPropertyNumber)(windows_core::Interface::as_raw(self), name.param().abi(), value, &mut result__).map(|| result__)
    }
    pub unsafe fn GetPropertyNumber<P0>(&self, name: P0, value: *mut i32, supported: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetPropertyNumber)(windows_core::Interface::as_raw(self), name.param().abi(), value, supported).ok()
    }
    pub unsafe fn SetPropertyString<P0, P1>(&self, name: P0, value: P1) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetPropertyString)(windows_core::Interface::as_raw(self), name.param().abi(), value.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPropertyString<P0>(&self, name: P0, value: *mut windows_core::BSTR, supported: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetPropertyString)(windows_core::Interface::as_raw(self), name.param().abi(), core::mem::transmute(value), supported).ok()
    }
    pub unsafe fn IsUISupported<P0>(&self, typeofui: P0, extradata: *const windows_core::VARIANT) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsUISupported)(windows_core::Interface::as_raw(self), typeofui.param().abi(), core::mem::transmute(extradata), &mut result__).map(|| result__)
    }
    pub unsafe fn DisplayUI<P0, P1>(&self, hwndparent: i32, title: P0, typeofui: P1, extradata: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DisplayUI)(windows_core::Interface::as_raw(self), hwndparent, title.param().abi(), typeofui.param().abi(), core::mem::transmute(extradata)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRecognizers<P0, P1>(&self, requiredattributes: P0, optionalattributes: P1) -> windows_core::Result<ISpeechObjectTokens>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRecognizers)(windows_core::Interface::as_raw(self), requiredattributes.param().abi(), optionalattributes.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAudioInputs<P0, P1>(&self, requiredattributes: P0, optionalattributes: P1) -> windows_core::Result<ISpeechObjectTokens>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAudioInputs)(windows_core::Interface::as_raw(self), requiredattributes.param().abi(), optionalattributes.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProfiles<P0, P1>(&self, requiredattributes: P0, optionalattributes: P1) -> windows_core::Result<ISpeechObjectTokens>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProfiles)(windows_core::Interface::as_raw(self), requiredattributes.param().abi(), optionalattributes.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechRecognizer_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Recognizer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Recognizer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recognizer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recognizer: usize,
    pub SetAllowAudioInputFormatChangesOnNextSet: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AllowAudioInputFormatChangesOnNextSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AudioInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AudioInput: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AudioInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AudioInput: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AudioInputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AudioInputStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AudioInputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AudioInputStream: usize,
    pub IsShared: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetState: unsafe extern "system" fn(*mut core::ffi::c_void, SpeechRecognizerState) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechRecognizerState) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Status: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Profile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Profile: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Profile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Profile: usize,
    pub EmulateRecognition: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRecoContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRecoContext: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, SpeechFormatType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFormat: usize,
    pub SetPropertyNumber: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetPropertyNumber: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetPropertyString: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetPropertyString: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsUISupported: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DisplayUI: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRecognizers: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRecognizers: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAudioInputs: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAudioInputs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetProfiles: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechRecognizerStatus, ISpeechRecognizerStatus_Vtbl, 0xbff9e781_53ec_484e_bb8a_0e1b5551e35c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechRecognizerStatus {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechRecognizerStatus, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechRecognizerStatus {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AudioStatus(&self) -> windows_core::Result<ISpeechAudioStatus> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AudioStatus)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentStreamPosition(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentStreamPosition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentStreamNumber(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentStreamNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NumberOfActiveRules(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NumberOfActiveRules)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ClsidEngine(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClsidEngine)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SupportedLanguages(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SupportedLanguages)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechRecognizerStatus_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AudioStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AudioStatus: usize,
    pub CurrentStreamPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub CurrentStreamNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NumberOfActiveRules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ClsidEngine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SupportedLanguages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechResourceLoader, ISpeechResourceLoader_Vtbl, 0xb9ac5783_fcd0_4b21_b119_b4f8da8fd2c3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechResourceLoader {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechResourceLoader, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechResourceLoader {
    pub unsafe fn LoadResource<P0, P1>(&self, bstrresourceuri: P0, falwaysreload: P1, pstream: *mut Option<windows_core::IUnknown>, pbstrmimetype: Option<*mut windows_core::BSTR>, pfmodified: *mut super::super::Foundation::VARIANT_BOOL, pbstrredirecturl: Option<*mut windows_core::BSTR>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).LoadResource)(windows_core::Interface::as_raw(self), bstrresourceuri.param().abi(), falwaysreload.param().abi(), core::mem::transmute(pstream), core::mem::transmute(pbstrmimetype.unwrap_or(std::ptr::null_mut())), pfmodified, core::mem::transmute(pbstrredirecturl.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetLocalCopy<P0>(&self, bstrresourceuri: P0, pbstrlocalpath: *mut windows_core::BSTR, pbstrmimetype: Option<*mut windows_core::BSTR>, pbstrredirecturl: Option<*mut windows_core::BSTR>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetLocalCopy)(windows_core::Interface::as_raw(self), bstrresourceuri.param().abi(), core::mem::transmute(pbstrlocalpath), core::mem::transmute(pbstrmimetype.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pbstrredirecturl.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ReleaseLocalCopy<P0>(&self, pbstrlocalpath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ReleaseLocalCopy)(windows_core::Interface::as_raw(self), pbstrlocalpath.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechResourceLoader_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub LoadResource: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::VARIANT_BOOL, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetLocalCopy: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ReleaseLocalCopy: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechTextSelectionInformation, ISpeechTextSelectionInformation_Vtbl, 0x3b9c7e7a_6eee_4ded_9092_11657279adbe);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechTextSelectionInformation {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechTextSelectionInformation, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechTextSelectionInformation {
    pub unsafe fn SetActiveOffset(&self, activeoffset: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetActiveOffset)(windows_core::Interface::as_raw(self), activeoffset).ok()
    }
    pub unsafe fn ActiveOffset(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ActiveOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetActiveLength(&self, activelength: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetActiveLength)(windows_core::Interface::as_raw(self), activelength).ok()
    }
    pub unsafe fn ActiveLength(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ActiveLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSelectionOffset(&self, selectionoffset: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSelectionOffset)(windows_core::Interface::as_raw(self), selectionoffset).ok()
    }
    pub unsafe fn SelectionOffset(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SelectionOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSelectionLength(&self, selectionlength: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSelectionLength)(windows_core::Interface::as_raw(self), selectionlength).ok()
    }
    pub unsafe fn SelectionLength(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SelectionLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechTextSelectionInformation_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SetActiveOffset: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ActiveOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetActiveLength: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ActiveLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSelectionOffset: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SelectionOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSelectionLength: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SelectionLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechVoice, ISpeechVoice_Vtbl, 0x269316d8_57bd_11d2_9eee_00c04f797396);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechVoice {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechVoice, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechVoice {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Status(&self) -> windows_core::Result<ISpeechVoiceStatus> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Voice(&self) -> windows_core::Result<ISpeechObjectToken> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Voice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_Voice<P0>(&self, voice: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpeechObjectToken>,
    {
        (windows_core::Interface::vtable(self).putref_Voice)(windows_core::Interface::as_raw(self), voice.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AudioOutput(&self) -> windows_core::Result<ISpeechObjectToken> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AudioOutput)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AudioOutput<P0>(&self, audiooutput: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpeechObjectToken>,
    {
        (windows_core::Interface::vtable(self).putref_AudioOutput)(windows_core::Interface::as_raw(self), audiooutput.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AudioOutputStream(&self) -> windows_core::Result<ISpeechBaseStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AudioOutputStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AudioOutputStream<P0>(&self, audiooutputstream: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpeechBaseStream>,
    {
        (windows_core::Interface::vtable(self).putref_AudioOutputStream)(windows_core::Interface::as_raw(self), audiooutputstream.param().abi()).ok()
    }
    pub unsafe fn Rate(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Rate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetRate(&self, rate: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRate)(windows_core::Interface::as_raw(self), rate).ok()
    }
    pub unsafe fn Volume(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Volume)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetVolume(&self, volume: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetVolume)(windows_core::Interface::as_raw(self), volume).ok()
    }
    pub unsafe fn SetAllowAudioOutputFormatChangesOnNextSet<P0>(&self, allow: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllowAudioOutputFormatChangesOnNextSet)(windows_core::Interface::as_raw(self), allow.param().abi()).ok()
    }
    pub unsafe fn AllowAudioOutputFormatChangesOnNextSet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllowAudioOutputFormatChangesOnNextSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn EventInterests(&self) -> windows_core::Result<SpeechVoiceEvents> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventInterests)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEventInterests(&self, eventinterestflags: SpeechVoiceEvents) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEventInterests)(windows_core::Interface::as_raw(self), eventinterestflags).ok()
    }
    pub unsafe fn SetPriority(&self, priority: SpeechVoicePriority) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), priority).ok()
    }
    pub unsafe fn Priority(&self) -> windows_core::Result<SpeechVoicePriority> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Priority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAlertBoundary(&self, boundary: SpeechVoiceEvents) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAlertBoundary)(windows_core::Interface::as_raw(self), boundary).ok()
    }
    pub unsafe fn AlertBoundary(&self) -> windows_core::Result<SpeechVoiceEvents> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AlertBoundary)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSynchronousSpeakTimeout(&self, mstimeout: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSynchronousSpeakTimeout)(windows_core::Interface::as_raw(self), mstimeout).ok()
    }
    pub unsafe fn SynchronousSpeakTimeout(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SynchronousSpeakTimeout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Speak<P0>(&self, text: P0, flags: SpeechVoiceSpeakFlags) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Speak)(windows_core::Interface::as_raw(self), text.param().abi(), flags, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SpeakStream<P0>(&self, stream: P0, flags: SpeechVoiceSpeakFlags) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<ISpeechBaseStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SpeakStream)(windows_core::Interface::as_raw(self), stream.param().abi(), flags, &mut result__).map(|| result__)
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip<P0>(&self, r#type: P0, numitems: i32) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), r#type.param().abi(), numitems, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetVoices<P0, P1>(&self, requiredattributes: P0, optionalattributes: P1) -> windows_core::Result<ISpeechObjectTokens>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVoices)(windows_core::Interface::as_raw(self), requiredattributes.param().abi(), optionalattributes.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAudioOutputs<P0, P1>(&self, requiredattributes: P0, optionalattributes: P1) -> windows_core::Result<ISpeechObjectTokens>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAudioOutputs)(windows_core::Interface::as_raw(self), requiredattributes.param().abi(), optionalattributes.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn WaitUntilDone(&self, mstimeout: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WaitUntilDone)(windows_core::Interface::as_raw(self), mstimeout, &mut result__).map(|| result__)
    }
    pub unsafe fn SpeakCompleteEvent(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SpeakCompleteEvent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsUISupported<P0>(&self, typeofui: P0, extradata: *const windows_core::VARIANT) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsUISupported)(windows_core::Interface::as_raw(self), typeofui.param().abi(), core::mem::transmute(extradata), &mut result__).map(|| result__)
    }
    pub unsafe fn DisplayUI<P0, P1>(&self, hwndparent: i32, title: P0, typeofui: P1, extradata: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DisplayUI)(windows_core::Interface::as_raw(self), hwndparent, title.param().abi(), typeofui.param().abi(), core::mem::transmute(extradata)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechVoice_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Status: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Voice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Voice: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Voice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Voice: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AudioOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AudioOutput: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AudioOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AudioOutput: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AudioOutputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AudioOutputStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AudioOutputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AudioOutputStream: usize,
    pub Rate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRate: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Volume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetAllowAudioOutputFormatChangesOnNextSet: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AllowAudioOutputFormatChangesOnNextSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub EventInterests: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechVoiceEvents) -> windows_core::HRESULT,
    pub SetEventInterests: unsafe extern "system" fn(*mut core::ffi::c_void, SpeechVoiceEvents) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, SpeechVoicePriority) -> windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechVoicePriority) -> windows_core::HRESULT,
    pub SetAlertBoundary: unsafe extern "system" fn(*mut core::ffi::c_void, SpeechVoiceEvents) -> windows_core::HRESULT,
    pub AlertBoundary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechVoiceEvents) -> windows_core::HRESULT,
    pub SetSynchronousSpeakTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SynchronousSpeakTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Speak: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, SpeechVoiceSpeakFlags, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SpeakStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, SpeechVoiceSpeakFlags, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SpeakStream: usize,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetVoices: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetVoices: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAudioOutputs: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAudioOutputs: usize,
    pub WaitUntilDone: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SpeakCompleteEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsUISupported: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DisplayUI: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechVoiceStatus, ISpeechVoiceStatus_Vtbl, 0x8be47b07_57f6_11d2_9eee_00c04f797396);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechVoiceStatus {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechVoiceStatus, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechVoiceStatus {
    pub unsafe fn CurrentStreamNumber(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentStreamNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LastStreamNumberQueued(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastStreamNumberQueued)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LastHResult(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastHResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RunningState(&self) -> windows_core::Result<SpeechRunState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RunningState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn InputWordPosition(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InputWordPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn InputWordLength(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InputWordLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn InputSentencePosition(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InputSentencePosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn InputSentenceLength(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InputSentenceLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LastBookmark(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastBookmark)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LastBookmarkId(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastBookmarkId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PhonemeId(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PhonemeId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn VisemeId(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VisemeId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechVoiceStatus_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub CurrentStreamNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastStreamNumberQueued: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastHResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RunningState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpeechRunState) -> windows_core::HRESULT,
    pub InputWordPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub InputWordLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub InputSentencePosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub InputSentenceLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastBookmark: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LastBookmarkId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PhonemeId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub VisemeId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechWaveFormatEx, ISpeechWaveFormatEx_Vtbl, 0x7a1ef0d5_1581_4741_88e4_209a49f11a10);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechWaveFormatEx {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechWaveFormatEx, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechWaveFormatEx {
    pub unsafe fn FormatTag(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FormatTag)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFormatTag(&self, formattag: i16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFormatTag)(windows_core::Interface::as_raw(self), formattag).ok()
    }
    pub unsafe fn Channels(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Channels)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetChannels(&self, channels: i16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetChannels)(windows_core::Interface::as_raw(self), channels).ok()
    }
    pub unsafe fn SamplesPerSec(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SamplesPerSec)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSamplesPerSec(&self, samplespersec: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSamplesPerSec)(windows_core::Interface::as_raw(self), samplespersec).ok()
    }
    pub unsafe fn AvgBytesPerSec(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AvgBytesPerSec)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAvgBytesPerSec(&self, avgbytespersec: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAvgBytesPerSec)(windows_core::Interface::as_raw(self), avgbytespersec).ok()
    }
    pub unsafe fn BlockAlign(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BlockAlign)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBlockAlign(&self, blockalign: i16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBlockAlign)(windows_core::Interface::as_raw(self), blockalign).ok()
    }
    pub unsafe fn BitsPerSample(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BitsPerSample)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBitsPerSample(&self, bitspersample: i16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBitsPerSample)(windows_core::Interface::as_raw(self), bitspersample).ok()
    }
    pub unsafe fn ExtraData(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExtraData)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetExtraData<P0>(&self, extradata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetExtraData)(windows_core::Interface::as_raw(self), extradata.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechWaveFormatEx_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub FormatTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub SetFormatTag: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub Channels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub SetChannels: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub SamplesPerSec: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSamplesPerSec: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AvgBytesPerSec: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAvgBytesPerSec: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub BlockAlign: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub SetBlockAlign: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub BitsPerSample: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub SetBitsPerSample: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub ExtraData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetExtraData: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISpeechXMLRecoResult, ISpeechXMLRecoResult_Vtbl, 0xaaec54af_8f85_4924_944d_b79d39d72e19);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISpeechXMLRecoResult {
    type Target = ISpeechRecoResult;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISpeechXMLRecoResult, windows_core::IUnknown, super::super::System::Com::IDispatch, ISpeechRecoResult);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechXMLRecoResult {
    pub unsafe fn GetXMLResult(&self, options: SPXMLRESULTOPTIONS) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetXMLResult)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetXMLErrorInfo(&self, linenumber: *mut i32, scriptline: *mut windows_core::BSTR, source: *mut windows_core::BSTR, description: *mut windows_core::BSTR, resultcode: *mut i32, iserror: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetXMLErrorInfo)(windows_core::Interface::as_raw(self), linenumber, core::mem::transmute(scriptline), core::mem::transmute(source), core::mem::transmute(description), resultcode, iserror).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISpeechXMLRecoResult_Vtbl {
    pub base__: ISpeechRecoResult_Vtbl,
    pub GetXMLResult: unsafe extern "system" fn(*mut core::ffi::c_void, SPXMLRESULTOPTIONS, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetXMLErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut i32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(_ISpPrivateEngineCall, _ISpPrivateEngineCall_Vtbl, 0x8e7c791e_4467_11d3_9723_00c04f72db08);
impl core::ops::Deref for _ISpPrivateEngineCall {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(_ISpPrivateEngineCall, windows_core::IUnknown);
impl _ISpPrivateEngineCall {
    pub unsafe fn CallEngine(&self, pcallframe: *mut core::ffi::c_void, ulcallframesize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CallEngine)(windows_core::Interface::as_raw(self), pcallframe, ulcallframesize).ok()
    }
    pub unsafe fn CallEngineEx(&self, pinframe: *const core::ffi::c_void, ulinframesize: u32, ppcomemoutframe: *mut *mut core::ffi::c_void, puloutframesize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CallEngineEx)(windows_core::Interface::as_raw(self), pinframe, ulinframesize, ppcomemoutframe, puloutframesize).ok()
    }
}
#[repr(C)]
pub struct _ISpPrivateEngineCall_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CallEngine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CallEngineEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(_ISpeechRecoContextEvents, _ISpeechRecoContextEvents_Vtbl, 0x7b8fcb42_0e9d_4f00_a048_7b04d6179d3d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for _ISpeechRecoContextEvents {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(_ISpeechRecoContextEvents, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl _ISpeechRecoContextEvents {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _ISpeechRecoContextEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(_ISpeechVoiceEvents, _ISpeechVoiceEvents_Vtbl, 0xa372acd1_3bef_4bbd_8ffb_cb3e2b416af8);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for _ISpeechVoiceEvents {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(_ISpeechVoiceEvents, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl _ISpeechVoiceEvents {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _ISpeechVoiceEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
}
pub const AllWords: SPMATCHINGMODE = SPMATCHINGMODE(0i32);
pub const DEFAULT_WEIGHT: u32 = 1u32;
pub const DISPIDSPTSI_ActiveLength: DISPIDSPTSI = DISPIDSPTSI(2i32);
pub const DISPIDSPTSI_ActiveOffset: DISPIDSPTSI = DISPIDSPTSI(1i32);
pub const DISPIDSPTSI_SelectionLength: DISPIDSPTSI = DISPIDSPTSI(4i32);
pub const DISPIDSPTSI_SelectionOffset: DISPIDSPTSI = DISPIDSPTSI(3i32);
pub const DISPID_SABIBufferSize: DISPID_SpeechAudioBufferInfo = DISPID_SpeechAudioBufferInfo(2i32);
pub const DISPID_SABIEventBias: DISPID_SpeechAudioBufferInfo = DISPID_SpeechAudioBufferInfo(3i32);
pub const DISPID_SABIMinNotification: DISPID_SpeechAudioBufferInfo = DISPID_SpeechAudioBufferInfo(1i32);
pub const DISPID_SABufferInfo: DISPID_SpeechAudio = DISPID_SpeechAudio(201i32);
pub const DISPID_SABufferNotifySize: DISPID_SpeechAudio = DISPID_SpeechAudio(204i32);
pub const DISPID_SADefaultFormat: DISPID_SpeechAudio = DISPID_SpeechAudio(202i32);
pub const DISPID_SAEventHandle: DISPID_SpeechAudio = DISPID_SpeechAudio(205i32);
pub const DISPID_SAFGetWaveFormatEx: DISPID_SpeechAudioFormat = DISPID_SpeechAudioFormat(3i32);
pub const DISPID_SAFGuid: DISPID_SpeechAudioFormat = DISPID_SpeechAudioFormat(2i32);
pub const DISPID_SAFSetWaveFormatEx: DISPID_SpeechAudioFormat = DISPID_SpeechAudioFormat(4i32);
pub const DISPID_SAFType: DISPID_SpeechAudioFormat = DISPID_SpeechAudioFormat(1i32);
pub const DISPID_SASCurrentDevicePosition: DISPID_SpeechAudioStatus = DISPID_SpeechAudioStatus(5i32);
pub const DISPID_SASCurrentSeekPosition: DISPID_SpeechAudioStatus = DISPID_SpeechAudioStatus(4i32);
pub const DISPID_SASFreeBufferSpace: DISPID_SpeechAudioStatus = DISPID_SpeechAudioStatus(1i32);
pub const DISPID_SASNonBlockingIO: DISPID_SpeechAudioStatus = DISPID_SpeechAudioStatus(2i32);
pub const DISPID_SASState: DISPID_SpeechAudioStatus = DISPID_SpeechAudioStatus(3i32);
pub const DISPID_SASetState: DISPID_SpeechAudio = DISPID_SpeechAudio(206i32);
pub const DISPID_SAStatus: DISPID_SpeechAudio = DISPID_SpeechAudio(200i32);
pub const DISPID_SAVolume: DISPID_SpeechAudio = DISPID_SpeechAudio(203i32);
pub const DISPID_SBSFormat: DISPID_SpeechBaseStream = DISPID_SpeechBaseStream(1i32);
pub const DISPID_SBSRead: DISPID_SpeechBaseStream = DISPID_SpeechBaseStream(2i32);
pub const DISPID_SBSSeek: DISPID_SpeechBaseStream = DISPID_SpeechBaseStream(4i32);
pub const DISPID_SBSWrite: DISPID_SpeechBaseStream = DISPID_SpeechBaseStream(3i32);
pub const DISPID_SCSBaseStream: DISPID_SpeechCustomStream = DISPID_SpeechCustomStream(100i32);
pub const DISPID_SDKCreateKey: DISPID_SpeechDataKey = DISPID_SpeechDataKey(8i32);
pub const DISPID_SDKDeleteKey: DISPID_SpeechDataKey = DISPID_SpeechDataKey(9i32);
pub const DISPID_SDKDeleteValue: DISPID_SpeechDataKey = DISPID_SpeechDataKey(10i32);
pub const DISPID_SDKEnumKeys: DISPID_SpeechDataKey = DISPID_SpeechDataKey(11i32);
pub const DISPID_SDKEnumValues: DISPID_SpeechDataKey = DISPID_SpeechDataKey(12i32);
pub const DISPID_SDKGetBinaryValue: DISPID_SpeechDataKey = DISPID_SpeechDataKey(2i32);
pub const DISPID_SDKGetStringValue: DISPID_SpeechDataKey = DISPID_SpeechDataKey(4i32);
pub const DISPID_SDKGetlongValue: DISPID_SpeechDataKey = DISPID_SpeechDataKey(6i32);
pub const DISPID_SDKOpenKey: DISPID_SpeechDataKey = DISPID_SpeechDataKey(7i32);
pub const DISPID_SDKSetBinaryValue: DISPID_SpeechDataKey = DISPID_SpeechDataKey(1i32);
pub const DISPID_SDKSetLongValue: DISPID_SpeechDataKey = DISPID_SpeechDataKey(5i32);
pub const DISPID_SDKSetStringValue: DISPID_SpeechDataKey = DISPID_SpeechDataKey(3i32);
pub const DISPID_SFSClose: DISPID_SpeechFileStream = DISPID_SpeechFileStream(101i32);
pub const DISPID_SFSOpen: DISPID_SpeechFileStream = DISPID_SpeechFileStream(100i32);
pub const DISPID_SGRAddResource: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(6i32);
pub const DISPID_SGRAddState: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(7i32);
pub const DISPID_SGRAttributes: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(1i32);
pub const DISPID_SGRClear: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(5i32);
pub const DISPID_SGRId: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(4i32);
pub const DISPID_SGRInitialState: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(2i32);
pub const DISPID_SGRName: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(3i32);
pub const DISPID_SGRSAddRuleTransition: DISPID_SpeechGrammarRuleState = DISPID_SpeechGrammarRuleState(4i32);
pub const DISPID_SGRSAddSpecialTransition: DISPID_SpeechGrammarRuleState = DISPID_SpeechGrammarRuleState(5i32);
pub const DISPID_SGRSAddWordTransition: DISPID_SpeechGrammarRuleState = DISPID_SpeechGrammarRuleState(3i32);
pub const DISPID_SGRSRule: DISPID_SpeechGrammarRuleState = DISPID_SpeechGrammarRuleState(1i32);
pub const DISPID_SGRSTNextState: DISPID_SpeechGrammarRuleStateTransition = DISPID_SpeechGrammarRuleStateTransition(8i32);
pub const DISPID_SGRSTPropertyId: DISPID_SpeechGrammarRuleStateTransition = DISPID_SpeechGrammarRuleStateTransition(6i32);
pub const DISPID_SGRSTPropertyName: DISPID_SpeechGrammarRuleStateTransition = DISPID_SpeechGrammarRuleStateTransition(5i32);
pub const DISPID_SGRSTPropertyValue: DISPID_SpeechGrammarRuleStateTransition = DISPID_SpeechGrammarRuleStateTransition(7i32);
pub const DISPID_SGRSTRule: DISPID_SpeechGrammarRuleStateTransition = DISPID_SpeechGrammarRuleStateTransition(3i32);
pub const DISPID_SGRSTText: DISPID_SpeechGrammarRuleStateTransition = DISPID_SpeechGrammarRuleStateTransition(2i32);
pub const DISPID_SGRSTType: DISPID_SpeechGrammarRuleStateTransition = DISPID_SpeechGrammarRuleStateTransition(1i32);
pub const DISPID_SGRSTWeight: DISPID_SpeechGrammarRuleStateTransition = DISPID_SpeechGrammarRuleStateTransition(4i32);
pub const DISPID_SGRSTransitions: DISPID_SpeechGrammarRuleState = DISPID_SpeechGrammarRuleState(2i32);
pub const DISPID_SGRSTsCount: DISPID_SpeechGrammarRuleStateTransitions = DISPID_SpeechGrammarRuleStateTransitions(1i32);
pub const DISPID_SGRSTsItem: DISPID_SpeechGrammarRuleStateTransitions = DISPID_SpeechGrammarRuleStateTransitions(0i32);
pub const DISPID_SGRSTs_NewEnum: DISPID_SpeechGrammarRuleStateTransitions = DISPID_SpeechGrammarRuleStateTransitions(-4i32);
pub const DISPID_SGRsAdd: DISPID_SpeechGrammarRules = DISPID_SpeechGrammarRules(3i32);
pub const DISPID_SGRsCommit: DISPID_SpeechGrammarRules = DISPID_SpeechGrammarRules(4i32);
pub const DISPID_SGRsCommitAndSave: DISPID_SpeechGrammarRules = DISPID_SpeechGrammarRules(5i32);
pub const DISPID_SGRsCount: DISPID_SpeechGrammarRules = DISPID_SpeechGrammarRules(1i32);
pub const DISPID_SGRsDynamic: DISPID_SpeechGrammarRules = DISPID_SpeechGrammarRules(2i32);
pub const DISPID_SGRsFindRule: DISPID_SpeechGrammarRules = DISPID_SpeechGrammarRules(6i32);
pub const DISPID_SGRsItem: DISPID_SpeechGrammarRules = DISPID_SpeechGrammarRules(0i32);
pub const DISPID_SGRs_NewEnum: DISPID_SpeechGrammarRules = DISPID_SpeechGrammarRules(-4i32);
pub const DISPID_SLAddPronunciation: DISPID_SpeechLexicon = DISPID_SpeechLexicon(3i32);
pub const DISPID_SLAddPronunciationByPhoneIds: DISPID_SpeechLexicon = DISPID_SpeechLexicon(4i32);
pub const DISPID_SLGenerationId: DISPID_SpeechLexicon = DISPID_SpeechLexicon(1i32);
pub const DISPID_SLGetGenerationChange: DISPID_SpeechLexicon = DISPID_SpeechLexicon(8i32);
pub const DISPID_SLGetPronunciations: DISPID_SpeechLexicon = DISPID_SpeechLexicon(7i32);
pub const DISPID_SLGetWords: DISPID_SpeechLexicon = DISPID_SpeechLexicon(2i32);
pub const DISPID_SLPLangId: DISPID_SpeechLexiconPronunciation = DISPID_SpeechLexiconPronunciation(2i32);
pub const DISPID_SLPPartOfSpeech: DISPID_SpeechLexiconPronunciation = DISPID_SpeechLexiconPronunciation(3i32);
pub const DISPID_SLPPhoneIds: DISPID_SpeechLexiconPronunciation = DISPID_SpeechLexiconPronunciation(4i32);
pub const DISPID_SLPSymbolic: DISPID_SpeechLexiconPronunciation = DISPID_SpeechLexiconPronunciation(5i32);
pub const DISPID_SLPType: DISPID_SpeechLexiconPronunciation = DISPID_SpeechLexiconPronunciation(1i32);
pub const DISPID_SLPsCount: DISPID_SpeechLexiconProns = DISPID_SpeechLexiconProns(1i32);
pub const DISPID_SLPsItem: DISPID_SpeechLexiconProns = DISPID_SpeechLexiconProns(0i32);
pub const DISPID_SLPs_NewEnum: DISPID_SpeechLexiconProns = DISPID_SpeechLexiconProns(-4i32);
pub const DISPID_SLRemovePronunciation: DISPID_SpeechLexicon = DISPID_SpeechLexicon(5i32);
pub const DISPID_SLRemovePronunciationByPhoneIds: DISPID_SpeechLexicon = DISPID_SpeechLexicon(6i32);
pub const DISPID_SLWLangId: DISPID_SpeechLexiconWord = DISPID_SpeechLexiconWord(1i32);
pub const DISPID_SLWPronunciations: DISPID_SpeechLexiconWord = DISPID_SpeechLexiconWord(4i32);
pub const DISPID_SLWType: DISPID_SpeechLexiconWord = DISPID_SpeechLexiconWord(2i32);
pub const DISPID_SLWWord: DISPID_SpeechLexiconWord = DISPID_SpeechLexiconWord(3i32);
pub const DISPID_SLWsCount: DISPID_SpeechLexiconWords = DISPID_SpeechLexiconWords(1i32);
pub const DISPID_SLWsItem: DISPID_SpeechLexiconWords = DISPID_SpeechLexiconWords(0i32);
pub const DISPID_SLWs_NewEnum: DISPID_SpeechLexiconWords = DISPID_SpeechLexiconWords(-4i32);
pub const DISPID_SMSADeviceId: DISPID_SpeechMMSysAudio = DISPID_SpeechMMSysAudio(300i32);
pub const DISPID_SMSALineId: DISPID_SpeechMMSysAudio = DISPID_SpeechMMSysAudio(301i32);
pub const DISPID_SMSAMMHandle: DISPID_SpeechMMSysAudio = DISPID_SpeechMMSysAudio(302i32);
pub const DISPID_SMSGetData: DISPID_SpeechMemoryStream = DISPID_SpeechMemoryStream(101i32);
pub const DISPID_SMSSetData: DISPID_SpeechMemoryStream = DISPID_SpeechMemoryStream(100i32);
pub const DISPID_SOTCDefault: DISPID_SpeechObjectTokenCategory = DISPID_SpeechObjectTokenCategory(2i32);
pub const DISPID_SOTCEnumerateTokens: DISPID_SpeechObjectTokenCategory = DISPID_SpeechObjectTokenCategory(5i32);
pub const DISPID_SOTCGetDataKey: DISPID_SpeechObjectTokenCategory = DISPID_SpeechObjectTokenCategory(4i32);
pub const DISPID_SOTCId: DISPID_SpeechObjectTokenCategory = DISPID_SpeechObjectTokenCategory(1i32);
pub const DISPID_SOTCSetId: DISPID_SpeechObjectTokenCategory = DISPID_SpeechObjectTokenCategory(3i32);
pub const DISPID_SOTCategory: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(3i32);
pub const DISPID_SOTCreateInstance: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(7i32);
pub const DISPID_SOTDataKey: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(2i32);
pub const DISPID_SOTDisplayUI: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(12i32);
pub const DISPID_SOTGetAttribute: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(6i32);
pub const DISPID_SOTGetDescription: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(4i32);
pub const DISPID_SOTGetStorageFileName: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(9i32);
pub const DISPID_SOTId: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(1i32);
pub const DISPID_SOTIsUISupported: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(11i32);
pub const DISPID_SOTMatchesAttributes: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(13i32);
pub const DISPID_SOTRemove: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(8i32);
pub const DISPID_SOTRemoveStorageFileName: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(10i32);
pub const DISPID_SOTSetId: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(5i32);
pub const DISPID_SOTsCount: DISPID_SpeechObjectTokens = DISPID_SpeechObjectTokens(1i32);
pub const DISPID_SOTsItem: DISPID_SpeechObjectTokens = DISPID_SpeechObjectTokens(0i32);
pub const DISPID_SOTs_NewEnum: DISPID_SpeechObjectTokens = DISPID_SpeechObjectTokens(-4i32);
pub const DISPID_SPACommit: DISPID_SpeechPhraseAlternate = DISPID_SpeechPhraseAlternate(5i32);
pub const DISPID_SPANumberOfElementsInResult: DISPID_SpeechPhraseAlternate = DISPID_SpeechPhraseAlternate(3i32);
pub const DISPID_SPAPhraseInfo: DISPID_SpeechPhraseAlternate = DISPID_SpeechPhraseAlternate(4i32);
pub const DISPID_SPARecoResult: DISPID_SpeechPhraseAlternate = DISPID_SpeechPhraseAlternate(1i32);
pub const DISPID_SPAStartElementInResult: DISPID_SpeechPhraseAlternate = DISPID_SpeechPhraseAlternate(2i32);
pub const DISPID_SPAsCount: DISPID_SpeechPhraseAlternates = DISPID_SpeechPhraseAlternates(1i32);
pub const DISPID_SPAsItem: DISPID_SpeechPhraseAlternates = DISPID_SpeechPhraseAlternates(0i32);
pub const DISPID_SPAs_NewEnum: DISPID_SpeechPhraseAlternates = DISPID_SpeechPhraseAlternates(-4i32);
pub const DISPID_SPCIdToPhone: DISPID_SpeechPhoneConverter = DISPID_SpeechPhoneConverter(3i32);
pub const DISPID_SPCLangId: DISPID_SpeechPhoneConverter = DISPID_SpeechPhoneConverter(1i32);
pub const DISPID_SPCPhoneToId: DISPID_SpeechPhoneConverter = DISPID_SpeechPhoneConverter(2i32);
pub const DISPID_SPEActualConfidence: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(12i32);
pub const DISPID_SPEAudioSizeBytes: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(4i32);
pub const DISPID_SPEAudioSizeTime: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(2i32);
pub const DISPID_SPEAudioStreamOffset: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(3i32);
pub const DISPID_SPEAudioTimeOffset: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(1i32);
pub const DISPID_SPEDisplayAttributes: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(10i32);
pub const DISPID_SPEDisplayText: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(7i32);
pub const DISPID_SPEEngineConfidence: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(13i32);
pub const DISPID_SPELexicalForm: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(8i32);
pub const DISPID_SPEPronunciation: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(9i32);
pub const DISPID_SPERequiredConfidence: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(11i32);
pub const DISPID_SPERetainedSizeBytes: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(6i32);
pub const DISPID_SPERetainedStreamOffset: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(5i32);
pub const DISPID_SPEsCount: DISPID_SpeechPhraseElements = DISPID_SpeechPhraseElements(1i32);
pub const DISPID_SPEsItem: DISPID_SpeechPhraseElements = DISPID_SpeechPhraseElements(0i32);
pub const DISPID_SPEs_NewEnum: DISPID_SpeechPhraseElements = DISPID_SpeechPhraseElements(-4i32);
pub const DISPID_SPIAudioSizeBytes: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(5i32);
pub const DISPID_SPIAudioSizeTime: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(7i32);
pub const DISPID_SPIAudioStreamPosition: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(4i32);
pub const DISPID_SPIElements: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(10i32);
pub const DISPID_SPIEngineId: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(12i32);
pub const DISPID_SPIEnginePrivateData: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(13i32);
pub const DISPID_SPIGetDisplayAttributes: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(16i32);
pub const DISPID_SPIGetText: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(15i32);
pub const DISPID_SPIGrammarId: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(2i32);
pub const DISPID_SPILanguageId: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(1i32);
pub const DISPID_SPIProperties: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(9i32);
pub const DISPID_SPIReplacements: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(11i32);
pub const DISPID_SPIRetainedSizeBytes: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(6i32);
pub const DISPID_SPIRule: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(8i32);
pub const DISPID_SPISaveToMemory: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(14i32);
pub const DISPID_SPIStartTime: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(3i32);
pub const DISPID_SPPBRestorePhraseFromMemory: DISPID_SpeechPhraseBuilder = DISPID_SpeechPhraseBuilder(1i32);
pub const DISPID_SPPChildren: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(9i32);
pub const DISPID_SPPConfidence: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(7i32);
pub const DISPID_SPPEngineConfidence: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(6i32);
pub const DISPID_SPPFirstElement: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(4i32);
pub const DISPID_SPPId: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(2i32);
pub const DISPID_SPPName: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(1i32);
pub const DISPID_SPPNumberOfElements: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(5i32);
pub const DISPID_SPPParent: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(8i32);
pub const DISPID_SPPValue: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(3i32);
pub const DISPID_SPPsCount: DISPID_SpeechPhraseProperties = DISPID_SpeechPhraseProperties(1i32);
pub const DISPID_SPPsItem: DISPID_SpeechPhraseProperties = DISPID_SpeechPhraseProperties(0i32);
pub const DISPID_SPPs_NewEnum: DISPID_SpeechPhraseProperties = DISPID_SpeechPhraseProperties(-4i32);
pub const DISPID_SPRDisplayAttributes: DISPID_SpeechPhraseReplacement = DISPID_SpeechPhraseReplacement(1i32);
pub const DISPID_SPRFirstElement: DISPID_SpeechPhraseReplacement = DISPID_SpeechPhraseReplacement(3i32);
pub const DISPID_SPRNumberOfElements: DISPID_SpeechPhraseReplacement = DISPID_SpeechPhraseReplacement(4i32);
pub const DISPID_SPRText: DISPID_SpeechPhraseReplacement = DISPID_SpeechPhraseReplacement(2i32);
pub const DISPID_SPRsCount: DISPID_SpeechPhraseReplacements = DISPID_SpeechPhraseReplacements(1i32);
pub const DISPID_SPRsItem: DISPID_SpeechPhraseReplacements = DISPID_SpeechPhraseReplacements(0i32);
pub const DISPID_SPRs_NewEnum: DISPID_SpeechPhraseReplacements = DISPID_SpeechPhraseReplacements(-4i32);
pub const DISPID_SPRuleChildren: DISPID_SpeechPhraseRule = DISPID_SpeechPhraseRule(6i32);
pub const DISPID_SPRuleConfidence: DISPID_SpeechPhraseRule = DISPID_SpeechPhraseRule(7i32);
pub const DISPID_SPRuleEngineConfidence: DISPID_SpeechPhraseRule = DISPID_SpeechPhraseRule(8i32);
pub const DISPID_SPRuleFirstElement: DISPID_SpeechPhraseRule = DISPID_SpeechPhraseRule(3i32);
pub const DISPID_SPRuleId: DISPID_SpeechPhraseRule = DISPID_SpeechPhraseRule(2i32);
pub const DISPID_SPRuleName: DISPID_SpeechPhraseRule = DISPID_SpeechPhraseRule(1i32);
pub const DISPID_SPRuleNumberOfElements: DISPID_SpeechPhraseRule = DISPID_SpeechPhraseRule(4i32);
pub const DISPID_SPRuleParent: DISPID_SpeechPhraseRule = DISPID_SpeechPhraseRule(5i32);
pub const DISPID_SPRulesCount: DISPID_SpeechPhraseRules = DISPID_SpeechPhraseRules(1i32);
pub const DISPID_SPRulesItem: DISPID_SpeechPhraseRules = DISPID_SpeechPhraseRules(0i32);
pub const DISPID_SPRules_NewEnum: DISPID_SpeechPhraseRules = DISPID_SpeechPhraseRules(-4i32);
pub const DISPID_SRAllowAudioInputFormatChangesOnNextSet: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(2i32);
pub const DISPID_SRAllowVoiceFormatMatchingOnNextSet: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(5i32);
pub const DISPID_SRAudioInput: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(3i32);
pub const DISPID_SRAudioInputStream: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(4i32);
pub const DISPID_SRCAudioInInterferenceStatus: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(2i32);
pub const DISPID_SRCBookmark: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(16i32);
pub const DISPID_SRCCmdMaxAlternates: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(8i32);
pub const DISPID_SRCCreateGrammar: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(14i32);
pub const DISPID_SRCCreateResultFromMemory: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(15i32);
pub const DISPID_SRCEAdaptation: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(15i32);
pub const DISPID_SRCEAudioLevel: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(17i32);
pub const DISPID_SRCEBookmark: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(3i32);
pub const DISPID_SRCEEndStream: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(2i32);
pub const DISPID_SRCEEnginePrivate: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(18i32);
pub const DISPID_SRCEFalseRecognition: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(11i32);
pub const DISPID_SRCEHypothesis: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(8i32);
pub const DISPID_SRCEInterference: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(12i32);
pub const DISPID_SRCEPhraseStart: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(6i32);
pub const DISPID_SRCEPropertyNumberChange: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(9i32);
pub const DISPID_SRCEPropertyStringChange: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(10i32);
pub const DISPID_SRCERecognition: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(7i32);
pub const DISPID_SRCERecognitionForOtherContext: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(16i32);
pub const DISPID_SRCERecognizerStateChange: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(14i32);
pub const DISPID_SRCERequestUI: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(13i32);
pub const DISPID_SRCESoundEnd: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(5i32);
pub const DISPID_SRCESoundStart: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(4i32);
pub const DISPID_SRCEStartStream: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(1i32);
pub const DISPID_SRCEventInterests: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(7i32);
pub const DISPID_SRCPause: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(12i32);
pub const DISPID_SRCRecognizer: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(1i32);
pub const DISPID_SRCRequestedUIType: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(3i32);
pub const DISPID_SRCResume: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(13i32);
pub const DISPID_SRCRetainedAudio: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(10i32);
pub const DISPID_SRCRetainedAudioFormat: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(11i32);
pub const DISPID_SRCSetAdaptationData: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(17i32);
pub const DISPID_SRCState: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(9i32);
pub const DISPID_SRCVoice: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(4i32);
pub const DISPID_SRCVoicePurgeEvent: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(6i32);
pub const DISPID_SRCreateRecoContext: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(10i32);
pub const DISPID_SRDisplayUI: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(17i32);
pub const DISPID_SREmulateRecognition: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(9i32);
pub const DISPID_SRGCmdLoadFromFile: DISPIDSPRG = DISPIDSPRG(7i32);
pub const DISPID_SRGCmdLoadFromMemory: DISPIDSPRG = DISPIDSPRG(10i32);
pub const DISPID_SRGCmdLoadFromObject: DISPIDSPRG = DISPIDSPRG(8i32);
pub const DISPID_SRGCmdLoadFromProprietaryGrammar: DISPIDSPRG = DISPIDSPRG(11i32);
pub const DISPID_SRGCmdLoadFromResource: DISPIDSPRG = DISPIDSPRG(9i32);
pub const DISPID_SRGCmdSetRuleIdState: DISPIDSPRG = DISPIDSPRG(13i32);
pub const DISPID_SRGCmdSetRuleState: DISPIDSPRG = DISPIDSPRG(12i32);
pub const DISPID_SRGCommit: DISPIDSPRG = DISPIDSPRG(6i32);
pub const DISPID_SRGDictationLoad: DISPIDSPRG = DISPIDSPRG(14i32);
pub const DISPID_SRGDictationSetState: DISPIDSPRG = DISPIDSPRG(16i32);
pub const DISPID_SRGDictationUnload: DISPIDSPRG = DISPIDSPRG(15i32);
pub const DISPID_SRGId: DISPIDSPRG = DISPIDSPRG(1i32);
pub const DISPID_SRGIsPronounceable: DISPIDSPRG = DISPIDSPRG(19i32);
pub const DISPID_SRGRecoContext: DISPIDSPRG = DISPIDSPRG(2i32);
pub const DISPID_SRGReset: DISPIDSPRG = DISPIDSPRG(5i32);
pub const DISPID_SRGRules: DISPIDSPRG = DISPIDSPRG(4i32);
pub const DISPID_SRGSetTextSelection: DISPIDSPRG = DISPIDSPRG(18i32);
pub const DISPID_SRGSetWordSequenceData: DISPIDSPRG = DISPIDSPRG(17i32);
pub const DISPID_SRGState: DISPIDSPRG = DISPIDSPRG(3i32);
pub const DISPID_SRGetFormat: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(11i32);
pub const DISPID_SRGetPropertyNumber: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(13i32);
pub const DISPID_SRGetPropertyString: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(15i32);
pub const DISPID_SRGetRecognizers: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(18i32);
pub const DISPID_SRIsShared: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(5i32);
pub const DISPID_SRIsUISupported: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(16i32);
pub const DISPID_SRProfile: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(8i32);
pub const DISPID_SRRAlternates: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(5i32);
pub const DISPID_SRRAudio: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(6i32);
pub const DISPID_SRRAudioFormat: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(3i32);
pub const DISPID_SRRDiscardResultInfo: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(9i32);
pub const DISPID_SRRGetXMLErrorInfo: DISPID_SpeechXMLRecoResult = DISPID_SpeechXMLRecoResult(11i32);
pub const DISPID_SRRGetXMLResult: DISPID_SpeechXMLRecoResult = DISPID_SpeechXMLRecoResult(10i32);
pub const DISPID_SRRPhraseInfo: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(4i32);
pub const DISPID_SRRRecoContext: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(1i32);
pub const DISPID_SRRSaveToMemory: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(8i32);
pub const DISPID_SRRSetTextFeedback: DISPID_SpeechRecoResult2 = DISPID_SpeechRecoResult2(12i32);
pub const DISPID_SRRSpeakAudio: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(7i32);
pub const DISPID_SRRTLength: DISPID_SpeechRecoResultTimes = DISPID_SpeechRecoResultTimes(2i32);
pub const DISPID_SRRTOffsetFromStart: DISPID_SpeechRecoResultTimes = DISPID_SpeechRecoResultTimes(4i32);
pub const DISPID_SRRTStreamTime: DISPID_SpeechRecoResultTimes = DISPID_SpeechRecoResultTimes(1i32);
pub const DISPID_SRRTTickCount: DISPID_SpeechRecoResultTimes = DISPID_SpeechRecoResultTimes(3i32);
pub const DISPID_SRRTimes: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(2i32);
pub const DISPID_SRRecognizer: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(1i32);
pub const DISPID_SRSAudioStatus: DISPID_SpeechRecognizerStatus = DISPID_SpeechRecognizerStatus(1i32);
pub const DISPID_SRSClsidEngine: DISPID_SpeechRecognizerStatus = DISPID_SpeechRecognizerStatus(5i32);
pub const DISPID_SRSCurrentStreamNumber: DISPID_SpeechRecognizerStatus = DISPID_SpeechRecognizerStatus(3i32);
pub const DISPID_SRSCurrentStreamPosition: DISPID_SpeechRecognizerStatus = DISPID_SpeechRecognizerStatus(2i32);
pub const DISPID_SRSNumberOfActiveRules: DISPID_SpeechRecognizerStatus = DISPID_SpeechRecognizerStatus(4i32);
pub const DISPID_SRSSupportedLanguages: DISPID_SpeechRecognizerStatus = DISPID_SpeechRecognizerStatus(6i32);
pub const DISPID_SRSetPropertyNumber: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(12i32);
pub const DISPID_SRSetPropertyString: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(14i32);
pub const DISPID_SRState: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(6i32);
pub const DISPID_SRStatus: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(7i32);
pub const DISPID_SVAlertBoundary: DISPID_SpeechVoice = DISPID_SpeechVoice(10i32);
pub const DISPID_SVAllowAudioOuputFormatChangesOnNextSet: DISPID_SpeechVoice = DISPID_SpeechVoice(7i32);
pub const DISPID_SVAudioOutput: DISPID_SpeechVoice = DISPID_SpeechVoice(3i32);
pub const DISPID_SVAudioOutputStream: DISPID_SpeechVoice = DISPID_SpeechVoice(4i32);
pub const DISPID_SVDisplayUI: DISPID_SpeechVoice = DISPID_SpeechVoice(22i32);
pub const DISPID_SVEAudioLevel: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(9i32);
pub const DISPID_SVEBookmark: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(4i32);
pub const DISPID_SVEEnginePrivate: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(10i32);
pub const DISPID_SVEPhoneme: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(6i32);
pub const DISPID_SVESentenceBoundary: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(7i32);
pub const DISPID_SVEStreamEnd: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(2i32);
pub const DISPID_SVEStreamStart: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(1i32);
pub const DISPID_SVEViseme: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(8i32);
pub const DISPID_SVEVoiceChange: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(3i32);
pub const DISPID_SVEWord: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(5i32);
pub const DISPID_SVEventInterests: DISPID_SpeechVoice = DISPID_SpeechVoice(8i32);
pub const DISPID_SVGetAudioInputs: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(19i32);
pub const DISPID_SVGetAudioOutputs: DISPID_SpeechVoice = DISPID_SpeechVoice(18i32);
pub const DISPID_SVGetProfiles: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(20i32);
pub const DISPID_SVGetVoices: DISPID_SpeechVoice = DISPID_SpeechVoice(17i32);
pub const DISPID_SVIsUISupported: DISPID_SpeechVoice = DISPID_SpeechVoice(21i32);
pub const DISPID_SVPause: DISPID_SpeechVoice = DISPID_SpeechVoice(14i32);
pub const DISPID_SVPriority: DISPID_SpeechVoice = DISPID_SpeechVoice(9i32);
pub const DISPID_SVRate: DISPID_SpeechVoice = DISPID_SpeechVoice(5i32);
pub const DISPID_SVResume: DISPID_SpeechVoice = DISPID_SpeechVoice(15i32);
pub const DISPID_SVSCurrentStreamNumber: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(1i32);
pub const DISPID_SVSInputSentenceLength: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(8i32);
pub const DISPID_SVSInputSentencePosition: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(7i32);
pub const DISPID_SVSInputWordLength: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(6i32);
pub const DISPID_SVSInputWordPosition: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(5i32);
pub const DISPID_SVSLastBookmark: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(9i32);
pub const DISPID_SVSLastBookmarkId: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(10i32);
pub const DISPID_SVSLastResult: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(3i32);
pub const DISPID_SVSLastStreamNumberQueued: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(2i32);
pub const DISPID_SVSPhonemeId: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(11i32);
pub const DISPID_SVSRunningState: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(4i32);
pub const DISPID_SVSVisemeId: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(12i32);
pub const DISPID_SVSkip: DISPID_SpeechVoice = DISPID_SpeechVoice(16i32);
pub const DISPID_SVSpeak: DISPID_SpeechVoice = DISPID_SpeechVoice(12i32);
pub const DISPID_SVSpeakCompleteEvent: DISPID_SpeechVoice = DISPID_SpeechVoice(20i32);
pub const DISPID_SVSpeakStream: DISPID_SpeechVoice = DISPID_SpeechVoice(13i32);
pub const DISPID_SVStatus: DISPID_SpeechVoice = DISPID_SpeechVoice(1i32);
pub const DISPID_SVSyncronousSpeakTimeout: DISPID_SpeechVoice = DISPID_SpeechVoice(11i32);
pub const DISPID_SVVoice: DISPID_SpeechVoice = DISPID_SpeechVoice(2i32);
pub const DISPID_SVVolume: DISPID_SpeechVoice = DISPID_SpeechVoice(6i32);
pub const DISPID_SVWaitUntilDone: DISPID_SpeechVoice = DISPID_SpeechVoice(19i32);
pub const DISPID_SWFEAvgBytesPerSec: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(4i32);
pub const DISPID_SWFEBitsPerSample: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(6i32);
pub const DISPID_SWFEBlockAlign: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(5i32);
pub const DISPID_SWFEChannels: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(2i32);
pub const DISPID_SWFEExtraData: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(7i32);
pub const DISPID_SWFEFormatTag: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(1i32);
pub const DISPID_SWFESamplesPerSec: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(3i32);
pub const OrderedSubset: SPMATCHINGMODE = SPMATCHINGMODE(3i32);
pub const OrderedSubsetContentRequired: SPMATCHINGMODE = SPMATCHINGMODE(7i32);
pub const PA_Ipa: PHONETICALPHABET = PHONETICALPHABET(0i32);
pub const PA_Sapi: PHONETICALPHABET = PHONETICALPHABET(2i32);
pub const PA_Ups: PHONETICALPHABET = PHONETICALPHABET(1i32);
pub const SAFT11kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(10i32);
pub const SAFT11kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(11i32);
pub const SAFT11kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(8i32);
pub const SAFT11kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(9i32);
pub const SAFT12kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(14i32);
pub const SAFT12kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(15i32);
pub const SAFT12kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(12i32);
pub const SAFT12kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(13i32);
pub const SAFT16kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(18i32);
pub const SAFT16kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(19i32);
pub const SAFT16kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(16i32);
pub const SAFT16kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(17i32);
pub const SAFT22kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(22i32);
pub const SAFT22kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(23i32);
pub const SAFT22kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(20i32);
pub const SAFT22kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(21i32);
pub const SAFT24kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(26i32);
pub const SAFT24kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(27i32);
pub const SAFT24kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(24i32);
pub const SAFT24kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(25i32);
pub const SAFT32kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(30i32);
pub const SAFT32kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(31i32);
pub const SAFT32kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(28i32);
pub const SAFT32kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(29i32);
pub const SAFT44kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(34i32);
pub const SAFT44kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(35i32);
pub const SAFT44kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(32i32);
pub const SAFT44kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(33i32);
pub const SAFT48kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(38i32);
pub const SAFT48kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(39i32);
pub const SAFT48kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(36i32);
pub const SAFT48kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(37i32);
pub const SAFT8kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(6i32);
pub const SAFT8kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(7i32);
pub const SAFT8kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(4i32);
pub const SAFT8kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(5i32);
pub const SAFTADPCM_11kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(59i32);
pub const SAFTADPCM_11kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(60i32);
pub const SAFTADPCM_22kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(61i32);
pub const SAFTADPCM_22kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(62i32);
pub const SAFTADPCM_44kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(63i32);
pub const SAFTADPCM_44kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(64i32);
pub const SAFTADPCM_8kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(57i32);
pub const SAFTADPCM_8kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(58i32);
pub const SAFTCCITT_ALaw_11kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(43i32);
pub const SAFTCCITT_ALaw_11kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(44i32);
pub const SAFTCCITT_ALaw_22kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(45i32);
pub const SAFTCCITT_ALaw_22kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(46i32);
pub const SAFTCCITT_ALaw_44kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(47i32);
pub const SAFTCCITT_ALaw_44kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(48i32);
pub const SAFTCCITT_ALaw_8kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(41i32);
pub const SAFTCCITT_ALaw_8kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(42i32);
pub const SAFTCCITT_uLaw_11kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(51i32);
pub const SAFTCCITT_uLaw_11kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(52i32);
pub const SAFTCCITT_uLaw_22kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(53i32);
pub const SAFTCCITT_uLaw_22kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(54i32);
pub const SAFTCCITT_uLaw_44kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(55i32);
pub const SAFTCCITT_uLaw_44kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(56i32);
pub const SAFTCCITT_uLaw_8kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(49i32);
pub const SAFTCCITT_uLaw_8kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(50i32);
pub const SAFTDefault: SpeechAudioFormatType = SpeechAudioFormatType(-1i32);
pub const SAFTExtendedAudioFormat: SpeechAudioFormatType = SpeechAudioFormatType(3i32);
pub const SAFTGSM610_11kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(66i32);
pub const SAFTGSM610_22kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(67i32);
pub const SAFTGSM610_44kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(68i32);
pub const SAFTGSM610_8kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(65i32);
pub const SAFTNoAssignedFormat: SpeechAudioFormatType = SpeechAudioFormatType(0i32);
pub const SAFTNonStandardFormat: SpeechAudioFormatType = SpeechAudioFormatType(2i32);
pub const SAFTText: SpeechAudioFormatType = SpeechAudioFormatType(1i32);
pub const SAFTTrueSpeech_8kHz1BitMono: SpeechAudioFormatType = SpeechAudioFormatType(40i32);
pub const SAPI_ERROR_BASE: u32 = 20480u32;
pub const SASClosed: SpeechAudioState = SpeechAudioState(0i32);
pub const SASPause: SpeechAudioState = SpeechAudioState(2i32);
pub const SASRun: SpeechAudioState = SpeechAudioState(3i32);
pub const SASStop: SpeechAudioState = SpeechAudioState(1i32);
pub const SBONone: SpeechBookmarkOptions = SpeechBookmarkOptions(0i32);
pub const SBOPause: SpeechBookmarkOptions = SpeechBookmarkOptions(1i32);
pub const SDA_Consume_Leading_Spaces: SpeechDisplayAttributes = SpeechDisplayAttributes(8i32);
pub const SDA_No_Trailing_Space: SpeechDisplayAttributes = SpeechDisplayAttributes(0i32);
pub const SDA_One_Trailing_Space: SpeechDisplayAttributes = SpeechDisplayAttributes(2i32);
pub const SDA_Two_Trailing_Spaces: SpeechDisplayAttributes = SpeechDisplayAttributes(4i32);
pub const SDKLCurrentConfig: SpeechDataKeyLocation = SpeechDataKeyLocation(5i32);
pub const SDKLCurrentUser: SpeechDataKeyLocation = SpeechDataKeyLocation(1i32);
pub const SDKLDefaultLocation: SpeechDataKeyLocation = SpeechDataKeyLocation(0i32);
pub const SDKLLocalMachine: SpeechDataKeyLocation = SpeechDataKeyLocation(2i32);
pub const SDTAll: SpeechDiscardType = SpeechDiscardType(255i32);
pub const SDTAlternates: SpeechDiscardType = SpeechDiscardType(128i32);
pub const SDTAudio: SpeechDiscardType = SpeechDiscardType(64i32);
pub const SDTDisplayText: SpeechDiscardType = SpeechDiscardType(8i32);
pub const SDTLexicalForm: SpeechDiscardType = SpeechDiscardType(16i32);
pub const SDTPronunciation: SpeechDiscardType = SpeechDiscardType(32i32);
pub const SDTProperty: SpeechDiscardType = SpeechDiscardType(1i32);
pub const SDTReplacement: SpeechDiscardType = SpeechDiscardType(2i32);
pub const SDTRule: SpeechDiscardType = SpeechDiscardType(4i32);
pub const SECFDefault: SpeechEmulationCompareFlags = SpeechEmulationCompareFlags(196609i32);
pub const SECFEmulateResult: SpeechEmulationCompareFlags = SpeechEmulationCompareFlags(1073741824i32);
pub const SECFIgnoreCase: SpeechEmulationCompareFlags = SpeechEmulationCompareFlags(1i32);
pub const SECFIgnoreKanaType: SpeechEmulationCompareFlags = SpeechEmulationCompareFlags(65536i32);
pub const SECFIgnoreWidth: SpeechEmulationCompareFlags = SpeechEmulationCompareFlags(131072i32);
pub const SECFNoSpecialChars: SpeechEmulationCompareFlags = SpeechEmulationCompareFlags(536870912i32);
pub const SECHighConfidence: SpeechEngineConfidence = SpeechEngineConfidence(1i32);
pub const SECLowConfidence: SpeechEngineConfidence = SpeechEngineConfidence(-1i32);
pub const SECNormalConfidence: SpeechEngineConfidence = SpeechEngineConfidence(0i32);
pub const SFTInput: SpeechFormatType = SpeechFormatType(0i32);
pub const SFTSREngine: SpeechFormatType = SpeechFormatType(1i32);
pub const SGDSActive: SpeechRuleState = SpeechRuleState(1i32);
pub const SGDSActiveUserDelimited: SpeechRuleState = SpeechRuleState(4i32);
pub const SGDSActiveWithAutoPause: SpeechRuleState = SpeechRuleState(3i32);
pub const SGDSInactive: SpeechRuleState = SpeechRuleState(0i32);
pub const SGDisplay: SpeechGrammarWordType = SpeechGrammarWordType(0i32);
pub const SGLexical: SpeechGrammarWordType = SpeechGrammarWordType(1i32);
pub const SGLexicalNoSpecialChars: SpeechGrammarWordType = SpeechGrammarWordType(3i32);
pub const SGPronounciation: SpeechGrammarWordType = SpeechGrammarWordType(2i32);
pub const SGRSTTDictation: SpeechGrammarRuleStateTransitionType = SpeechGrammarRuleStateTransitionType(3i32);
pub const SGRSTTEpsilon: SpeechGrammarRuleStateTransitionType = SpeechGrammarRuleStateTransitionType(0i32);
pub const SGRSTTRule: SpeechGrammarRuleStateTransitionType = SpeechGrammarRuleStateTransitionType(2i32);
pub const SGRSTTTextBuffer: SpeechGrammarRuleStateTransitionType = SpeechGrammarRuleStateTransitionType(5i32);
pub const SGRSTTWildcard: SpeechGrammarRuleStateTransitionType = SpeechGrammarRuleStateTransitionType(4i32);
pub const SGRSTTWord: SpeechGrammarRuleStateTransitionType = SpeechGrammarRuleStateTransitionType(1i32);
pub const SGSDisabled: SpeechGrammarState = SpeechGrammarState(0i32);
pub const SGSEnabled: SpeechGrammarState = SpeechGrammarState(1i32);
pub const SGSExclusive: SpeechGrammarState = SpeechGrammarState(3i32);
pub const SINoSignal: SpeechInterference = SpeechInterference(2i32);
pub const SINoise: SpeechInterference = SpeechInterference(1i32);
pub const SINone: SpeechInterference = SpeechInterference(0i32);
pub const SITooFast: SpeechInterference = SpeechInterference(5i32);
pub const SITooLoud: SpeechInterference = SpeechInterference(3i32);
pub const SITooQuiet: SpeechInterference = SpeechInterference(4i32);
pub const SITooSlow: SpeechInterference = SpeechInterference(6i32);
pub const SLODynamic: SpeechLoadOption = SpeechLoadOption(1i32);
pub const SLOStatic: SpeechLoadOption = SpeechLoadOption(0i32);
pub const SLTApp: SpeechLexiconType = SpeechLexiconType(2i32);
pub const SLTUser: SpeechLexiconType = SpeechLexiconType(1i32);
pub const SPADS_CurrentRecognizer: SPADAPTATIONSETTINGS = SPADAPTATIONSETTINGS(1i32);
pub const SPADS_Default: SPADAPTATIONSETTINGS = SPADAPTATIONSETTINGS(0i32);
pub const SPADS_HighVolumeDataSource: SPADAPTATIONSETTINGS = SPADAPTATIONSETTINGS(16i32);
pub const SPADS_Immediate: SPADAPTATIONSETTINGS = SPADAPTATIONSETTINGS(4i32);
pub const SPADS_RecoProfile: SPADAPTATIONSETTINGS = SPADAPTATIONSETTINGS(2i32);
pub const SPADS_Reset: SPADAPTATIONSETTINGS = SPADAPTATIONSETTINGS(8i32);
pub const SPAF_ALL: SPDISPLAYATTRIBUTES = SPDISPLAYATTRIBUTES(31i32);
pub const SPAF_BUFFER_POSITION: SPDISPLAYATTRIBUTES = SPDISPLAYATTRIBUTES(16i32);
pub const SPAF_CONSUME_LEADING_SPACES: SPDISPLAYATTRIBUTES = SPDISPLAYATTRIBUTES(8i32);
pub const SPAF_ONE_TRAILING_SPACE: SPDISPLAYATTRIBUTES = SPDISPLAYATTRIBUTES(2i32);
pub const SPAF_TWO_TRAILING_SPACES: SPDISPLAYATTRIBUTES = SPDISPLAYATTRIBUTES(4i32);
pub const SPAF_USER_SPECIFIED: SPDISPLAYATTRIBUTES = SPDISPLAYATTRIBUTES(128i32);
pub const SPALTERNATESCLSID: windows_core::PCWSTR = windows_core::w!("AlternatesCLSID");
pub const SPAO_NONE: SPAUDIOOPTIONS = SPAUDIOOPTIONS(0i32);
pub const SPAO_RETAIN_AUDIO: SPAUDIOOPTIONS = SPAUDIOOPTIONS(1i32);
pub const SPAR_High: SPADAPTATIONRELEVANCE = SPADAPTATIONRELEVANCE(3i32);
pub const SPAR_Low: SPADAPTATIONRELEVANCE = SPADAPTATIONRELEVANCE(1i32);
pub const SPAR_Medium: SPADAPTATIONRELEVANCE = SPADAPTATIONRELEVANCE(2i32);
pub const SPAR_Unknown: SPADAPTATIONRELEVANCE = SPADAPTATIONRELEVANCE(0i32);
pub const SPAS_CLOSED: SPAUDIOSTATE = SPAUDIOSTATE(0i32);
pub const SPAS_PAUSE: SPAUDIOSTATE = SPAUDIOSTATE(2i32);
pub const SPAS_RUN: SPAUDIOSTATE = SPAUDIOSTATE(3i32);
pub const SPAS_STOP: SPAUDIOSTATE = SPAUDIOSTATE(1i32);
pub const SPBO_AHEAD: SPBOOKMARKOPTIONS = SPBOOKMARKOPTIONS(2i32);
pub const SPBO_NONE: SPBOOKMARKOPTIONS = SPBOOKMARKOPTIONS(0i32);
pub const SPBO_PAUSE: SPBOOKMARKOPTIONS = SPBOOKMARKOPTIONS(1i32);
pub const SPBO_TIME_UNITS: SPBOOKMARKOPTIONS = SPBOOKMARKOPTIONS(4i32);
pub const SPCAT_APPLEXICONS: windows_core::PCWSTR = windows_core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AppLexicons");
pub const SPCAT_AUDIOIN: windows_core::PCWSTR = windows_core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AudioInput");
pub const SPCAT_AUDIOOUT: windows_core::PCWSTR = windows_core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AudioOutput");
pub const SPCAT_PHONECONVERTERS: windows_core::PCWSTR = windows_core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\PhoneConverters");
pub const SPCAT_RECOGNIZERS: windows_core::PCWSTR = windows_core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\Recognizers");
pub const SPCAT_RECOPROFILES: windows_core::PCWSTR = windows_core::w!("HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Speech\\RecoProfiles");
pub const SPCAT_TEXTNORMALIZERS: windows_core::PCWSTR = windows_core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\TextNormalizers");
pub const SPCAT_VOICES: windows_core::PCWSTR = windows_core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\Voices");
pub const SPCFGN_ACTIVATE: SPCFGNOTIFY = SPCFGNOTIFY(3i32);
pub const SPCFGN_ADD: SPCFGNOTIFY = SPCFGNOTIFY(0i32);
pub const SPCFGN_DEACTIVATE: SPCFGNOTIFY = SPCFGNOTIFY(4i32);
pub const SPCFGN_INVALIDATE: SPCFGNOTIFY = SPCFGNOTIFY(2i32);
pub const SPCFGN_REMOVE: SPCFGNOTIFY = SPCFGNOTIFY(1i32);
pub const SPCF_ADD_TO_USER_LEXICON: SPCOMMITFLAGS = SPCOMMITFLAGS(1i32);
pub const SPCF_DEFINITE_CORRECTION: SPCOMMITFLAGS = SPCOMMITFLAGS(2i32);
pub const SPCF_NONE: SPCOMMITFLAGS = SPCOMMITFLAGS(0i32);
pub const SPCS_DISABLED: SPCONTEXTSTATE = SPCONTEXTSTATE(0i32);
pub const SPCS_ENABLED: SPCONTEXTSTATE = SPCONTEXTSTATE(1i32);
pub const SPCURRENT_USER_LEXICON_TOKEN_ID: windows_core::PCWSTR = windows_core::w!("HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Speech\\CurrentUserLexicon");
pub const SPCURRENT_USER_SHORTCUT_TOKEN_ID: windows_core::PCWSTR = windows_core::w!("HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Speech\\CurrentUserShortcut");
pub const SPDF_ALL: SPVALUETYPE = SPVALUETYPE(255i32);
pub const SPDF_ALTERNATES: SPVALUETYPE = SPVALUETYPE(128i32);
pub const SPDF_AUDIO: SPVALUETYPE = SPVALUETYPE(64i32);
pub const SPDF_DISPLAYTEXT: SPVALUETYPE = SPVALUETYPE(8i32);
pub const SPDF_LEXICALFORM: SPVALUETYPE = SPVALUETYPE(16i32);
pub const SPDF_PRONUNCIATION: SPVALUETYPE = SPVALUETYPE(32i32);
pub const SPDF_PROPERTY: SPVALUETYPE = SPVALUETYPE(1i32);
pub const SPDF_REPLACEMENT: SPVALUETYPE = SPVALUETYPE(2i32);
pub const SPDF_RULE: SPVALUETYPE = SPVALUETYPE(4i32);
pub const SPDICTATION: windows_core::PCWSTR = windows_core::w!("*");
pub const SPDKL_CurrentConfig: SPDATAKEYLOCATION = SPDATAKEYLOCATION(5i32);
pub const SPDKL_CurrentUser: SPDATAKEYLOCATION = SPDATAKEYLOCATION(1i32);
pub const SPDKL_DefaultLocation: SPDATAKEYLOCATION = SPDATAKEYLOCATION(0i32);
pub const SPDKL_LocalMachine: SPDATAKEYLOCATION = SPDATAKEYLOCATION(2i32);
pub const SPDUI_AddRemoveWord: windows_core::PCWSTR = windows_core::w!("AddRemoveWord");
pub const SPDUI_AudioProperties: windows_core::PCWSTR = windows_core::w!("AudioProperties");
pub const SPDUI_AudioVolume: windows_core::PCWSTR = windows_core::w!("AudioVolume");
pub const SPDUI_EngineProperties: windows_core::PCWSTR = windows_core::w!("EngineProperties");
pub const SPDUI_MicTraining: windows_core::PCWSTR = windows_core::w!("MicTraining");
pub const SPDUI_RecoProfileProperties: windows_core::PCWSTR = windows_core::w!("RecoProfileProperties");
pub const SPDUI_ShareData: windows_core::PCWSTR = windows_core::w!("ShareData");
pub const SPDUI_Tutorial: windows_core::PCWSTR = windows_core::w!("Tutorial");
pub const SPDUI_UserEnrollment: windows_core::PCWSTR = windows_core::w!("UserEnrollment");
pub const SPDUI_UserTraining: windows_core::PCWSTR = windows_core::w!("UserTraining");
pub const SPEI_ADAPTATION: SPEVENTENUM = SPEVENTENUM(47i32);
pub const SPEI_END_INPUT_STREAM: SPEVENTENUM = SPEVENTENUM(2i32);
pub const SPEI_END_SR_STREAM: SPEVENTENUM = SPEVENTENUM(34i32);
pub const SPEI_FALSE_RECOGNITION: SPEVENTENUM = SPEVENTENUM(43i32);
pub const SPEI_HYPOTHESIS: SPEVENTENUM = SPEVENTENUM(39i32);
pub const SPEI_INTERFERENCE: SPEVENTENUM = SPEVENTENUM(44i32);
pub const SPEI_MAX_SR: SPEVENTENUM = SPEVENTENUM(55i32);
pub const SPEI_MAX_TTS: SPEVENTENUM = SPEVENTENUM(15i32);
pub const SPEI_MIN_SR: SPEVENTENUM = SPEVENTENUM(34i32);
pub const SPEI_MIN_TTS: SPEVENTENUM = SPEVENTENUM(1i32);
pub const SPEI_PHONEME: SPEVENTENUM = SPEVENTENUM(6i32);
pub const SPEI_PHRASE_START: SPEVENTENUM = SPEVENTENUM(37i32);
pub const SPEI_PROPERTY_NUM_CHANGE: SPEVENTENUM = SPEVENTENUM(41i32);
pub const SPEI_PROPERTY_STRING_CHANGE: SPEVENTENUM = SPEVENTENUM(42i32);
pub const SPEI_RECOGNITION: SPEVENTENUM = SPEVENTENUM(38i32);
pub const SPEI_RECO_OTHER_CONTEXT: SPEVENTENUM = SPEVENTENUM(49i32);
pub const SPEI_RECO_STATE_CHANGE: SPEVENTENUM = SPEVENTENUM(46i32);
pub const SPEI_REQUEST_UI: SPEVENTENUM = SPEVENTENUM(45i32);
pub const SPEI_RESERVED1: SPEVENTENUM = SPEVENTENUM(30i32);
pub const SPEI_RESERVED2: SPEVENTENUM = SPEVENTENUM(33i32);
pub const SPEI_RESERVED3: SPEVENTENUM = SPEVENTENUM(63i32);
pub const SPEI_RESERVED4: SPEVENTENUM = SPEVENTENUM(53i32);
pub const SPEI_RESERVED5: SPEVENTENUM = SPEVENTENUM(54i32);
pub const SPEI_RESERVED6: SPEVENTENUM = SPEVENTENUM(55i32);
pub const SPEI_SENTENCE_BOUNDARY: SPEVENTENUM = SPEVENTENUM(7i32);
pub const SPEI_SOUND_END: SPEVENTENUM = SPEVENTENUM(36i32);
pub const SPEI_SOUND_START: SPEVENTENUM = SPEVENTENUM(35i32);
pub const SPEI_SR_AUDIO_LEVEL: SPEVENTENUM = SPEVENTENUM(50i32);
pub const SPEI_SR_BOOKMARK: SPEVENTENUM = SPEVENTENUM(40i32);
pub const SPEI_SR_PRIVATE: SPEVENTENUM = SPEVENTENUM(52i32);
pub const SPEI_SR_RETAINEDAUDIO: SPEVENTENUM = SPEVENTENUM(51i32);
pub const SPEI_START_INPUT_STREAM: SPEVENTENUM = SPEVENTENUM(1i32);
pub const SPEI_START_SR_STREAM: SPEVENTENUM = SPEVENTENUM(48i32);
pub const SPEI_TTS_AUDIO_LEVEL: SPEVENTENUM = SPEVENTENUM(9i32);
pub const SPEI_TTS_BOOKMARK: SPEVENTENUM = SPEVENTENUM(4i32);
pub const SPEI_TTS_PRIVATE: SPEVENTENUM = SPEVENTENUM(15i32);
pub const SPEI_UNDEFINED: SPEVENTENUM = SPEVENTENUM(0i32);
pub const SPEI_VISEME: SPEVENTENUM = SPEVENTENUM(8i32);
pub const SPEI_VOICE_CHANGE: SPEVENTENUM = SPEVENTENUM(3i32);
pub const SPEI_WORD_BOUNDARY: SPEVENTENUM = SPEVENTENUM(5i32);
pub const SPESF_EMULATED: SPENDSRSTREAMFLAGS = SPENDSRSTREAMFLAGS(2i32);
pub const SPESF_NONE: SPENDSRSTREAMFLAGS = SPENDSRSTREAMFLAGS(0i32);
pub const SPESF_STREAM_RELEASED: SPENDSRSTREAMFLAGS = SPENDSRSTREAMFLAGS(1i32);
pub const SPET_LPARAM_IS_OBJECT: SPEVENTLPARAMTYPE = SPEVENTLPARAMTYPE(2i32);
pub const SPET_LPARAM_IS_POINTER: SPEVENTLPARAMTYPE = SPEVENTLPARAMTYPE(3i32);
pub const SPET_LPARAM_IS_STRING: SPEVENTLPARAMTYPE = SPEVENTLPARAMTYPE(4i32);
pub const SPET_LPARAM_IS_TOKEN: SPEVENTLPARAMTYPE = SPEVENTLPARAMTYPE(1i32);
pub const SPET_LPARAM_IS_UNDEFINED: SPEVENTLPARAMTYPE = SPEVENTLPARAMTYPE(0i32);
pub const SPFM_CREATE: SPFILEMODE = SPFILEMODE(2i32);
pub const SPFM_CREATE_ALWAYS: SPFILEMODE = SPFILEMODE(3i32);
pub const SPFM_NUM_MODES: SPFILEMODE = SPFILEMODE(4i32);
pub const SPFM_OPEN_READONLY: SPFILEMODE = SPFILEMODE(0i32);
pub const SPFM_OPEN_READWRITE: SPFILEMODE = SPFILEMODE(1i32);
pub const SPF_ASYNC: SPEAKFLAGS = SPEAKFLAGS(1i32);
pub const SPF_DEFAULT: SPEAKFLAGS = SPEAKFLAGS(0i32);
pub const SPF_IS_FILENAME: SPEAKFLAGS = SPEAKFLAGS(4i32);
pub const SPF_IS_NOT_XML: SPEAKFLAGS = SPEAKFLAGS(16i32);
pub const SPF_IS_XML: SPEAKFLAGS = SPEAKFLAGS(8i32);
pub const SPF_NLP_MASK: SPEAKFLAGS = SPEAKFLAGS(64i32);
pub const SPF_NLP_SPEAK_PUNC: SPEAKFLAGS = SPEAKFLAGS(64i32);
pub const SPF_PARSE_AUTODETECT: SPEAKFLAGS = SPEAKFLAGS(0i32);
pub const SPF_PARSE_MASK: SPEAKFLAGS = SPEAKFLAGS(384i32);
pub const SPF_PARSE_SAPI: SPEAKFLAGS = SPEAKFLAGS(128i32);
pub const SPF_PARSE_SSML: SPEAKFLAGS = SPEAKFLAGS(256i32);
pub const SPF_PERSIST_XML: SPEAKFLAGS = SPEAKFLAGS(32i32);
pub const SPF_PURGEBEFORESPEAK: SPEAKFLAGS = SPEAKFLAGS(2i32);
pub const SPF_UNUSED_FLAGS: SPEAKFLAGS = SPEAKFLAGS(-512i32);
pub const SPF_VOICE_MASK: SPEAKFLAGS = SPEAKFLAGS(511i32);
pub const SPGO_ALL: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(1023i32);
pub const SPGO_DEFAULT: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(1019i32);
pub const SPGO_FILE: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(16i32);
pub const SPGO_HTTP: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(32i32);
pub const SPGO_OBJECT: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(128i32);
pub const SPGO_RES: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(64i32);
pub const SPGO_SAPI: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(1i32);
pub const SPGO_SRGS: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(2i32);
pub const SPGO_SRGS_MS_SCRIPT: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(8i32);
pub const SPGO_SRGS_SCRIPT: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(778i32);
pub const SPGO_SRGS_STG_SCRIPT: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(512i32);
pub const SPGO_SRGS_W3C_SCRIPT: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(256i32);
pub const SPGO_UPS: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(4i32);
pub const SPGS_DISABLED: SPGRAMMARSTATE = SPGRAMMARSTATE(0i32);
pub const SPGS_ENABLED: SPGRAMMARSTATE = SPGRAMMARSTATE(1i32);
pub const SPGS_EXCLUSIVE: SPGRAMMARSTATE = SPGRAMMARSTATE(3i32);
pub const SPINFDICTATION: windows_core::PCWSTR = windows_core::w!("*+");
pub const SPINTERFERENCE_LATENCY_TRUNCATE_BEGIN: SPINTERFERENCE = SPINTERFERENCE(8i32);
pub const SPINTERFERENCE_LATENCY_TRUNCATE_END: SPINTERFERENCE = SPINTERFERENCE(9i32);
pub const SPINTERFERENCE_LATENCY_WARNING: SPINTERFERENCE = SPINTERFERENCE(7i32);
pub const SPINTERFERENCE_NOISE: SPINTERFERENCE = SPINTERFERENCE(1i32);
pub const SPINTERFERENCE_NONE: SPINTERFERENCE = SPINTERFERENCE(0i32);
pub const SPINTERFERENCE_NOSIGNAL: SPINTERFERENCE = SPINTERFERENCE(2i32);
pub const SPINTERFERENCE_TOOFAST: SPINTERFERENCE = SPINTERFERENCE(5i32);
pub const SPINTERFERENCE_TOOLOUD: SPINTERFERENCE = SPINTERFERENCE(3i32);
pub const SPINTERFERENCE_TOOQUIET: SPINTERFERENCE = SPINTERFERENCE(4i32);
pub const SPINTERFERENCE_TOOSLOW: SPINTERFERENCE = SPINTERFERENCE(6i32);
pub const SPLO_DYNAMIC: SPLOADOPTIONS = SPLOADOPTIONS(1i32);
pub const SPLO_STATIC: SPLOADOPTIONS = SPLOADOPTIONS(0i32);
pub const SPMAX_RATE: SPVLIMITS = SPVLIMITS(10i32);
pub const SPMAX_VOLUME: SPVLIMITS = SPVLIMITS(100i32);
pub const SPMIN_RATE: SPVLIMITS = SPVLIMITS(-10i32);
pub const SPMIN_VOLUME: SPVLIMITS = SPVLIMITS(0i32);
pub const SPMMSYS_AUDIO_IN_TOKEN_ID: windows_core::PCWSTR = windows_core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AudioInput\\TokenEnums\\MMAudioIn\\");
pub const SPMMSYS_AUDIO_OUT_TOKEN_ID: windows_core::PCWSTR = windows_core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AudioOutput\\TokenEnums\\MMAudioOut\\");
pub const SPPPUT_ARRAY_INDEX: SPPHRASEPROPERTYUNIONTYPE = SPPHRASEPROPERTYUNIONTYPE(1i32);
pub const SPPPUT_UNUSED: SPPHRASEPROPERTYUNIONTYPE = SPPHRASEPROPERTYUNIONTYPE(0i32);
pub const SPPROPSRC_RECO_CTX: SPPROPSRC = SPPROPSRC(1i32);
pub const SPPROPSRC_RECO_GRAMMAR: SPPROPSRC = SPPROPSRC(2i32);
pub const SPPROPSRC_RECO_INST: SPPROPSRC = SPPROPSRC(0i32);
pub const SPPROP_ADAPTATION_ON: windows_core::PCWSTR = windows_core::w!("AdaptationOn");
pub const SPPROP_COMPLEX_RESPONSE_SPEED: windows_core::PCWSTR = windows_core::w!("ComplexResponseSpeed");
pub const SPPROP_HIGH_CONFIDENCE_THRESHOLD: windows_core::PCWSTR = windows_core::w!("HighConfidenceThreshold");
pub const SPPROP_LOW_CONFIDENCE_THRESHOLD: windows_core::PCWSTR = windows_core::w!("LowConfidenceThreshold");
pub const SPPROP_NORMAL_CONFIDENCE_THRESHOLD: windows_core::PCWSTR = windows_core::w!("NormalConfidenceThreshold");
pub const SPPROP_PERSISTED_BACKGROUND_ADAPTATION: windows_core::PCWSTR = windows_core::w!("PersistedBackgroundAdaptation");
pub const SPPROP_PERSISTED_LANGUAGE_MODEL_ADAPTATION: windows_core::PCWSTR = windows_core::w!("PersistedLanguageModelAdaptation");
pub const SPPROP_RESOURCE_USAGE: windows_core::PCWSTR = windows_core::w!("ResourceUsage");
pub const SPPROP_RESPONSE_SPEED: windows_core::PCWSTR = windows_core::w!("ResponseSpeed");
pub const SPPROP_UX_IS_LISTENING: windows_core::PCWSTR = windows_core::w!("UXIsListening");
pub const SPPR_ALL_ELEMENTS: SPPHRASERNG = SPPHRASERNG(-1i32);
pub const SPPS_Function: SPPARTOFSPEECH = SPPARTOFSPEECH(16384i32);
pub const SPPS_Interjection: SPPARTOFSPEECH = SPPARTOFSPEECH(20480i32);
pub const SPPS_LMA: SPPARTOFSPEECH = SPPARTOFSPEECH(28672i32);
pub const SPPS_Modifier: SPPARTOFSPEECH = SPPARTOFSPEECH(12288i32);
pub const SPPS_Noncontent: SPPARTOFSPEECH = SPPARTOFSPEECH(24576i32);
pub const SPPS_NotOverriden: SPPARTOFSPEECH = SPPARTOFSPEECH(-1i32);
pub const SPPS_Noun: SPPARTOFSPEECH = SPPARTOFSPEECH(4096i32);
pub const SPPS_RESERVED1: SPSHORTCUTTYPE = SPSHORTCUTTYPE(12288i32);
pub const SPPS_RESERVED2: SPSHORTCUTTYPE = SPSHORTCUTTYPE(16384i32);
pub const SPPS_RESERVED3: SPSHORTCUTTYPE = SPSHORTCUTTYPE(20480i32);
pub const SPPS_RESERVED4: SPSHORTCUTTYPE = SPSHORTCUTTYPE(61440i32);
pub const SPPS_SuppressWord: SPPARTOFSPEECH = SPPARTOFSPEECH(61440i32);
pub const SPPS_Unknown: SPPARTOFSPEECH = SPPARTOFSPEECH(0i32);
pub const SPPS_Verb: SPPARTOFSPEECH = SPPARTOFSPEECH(8192i32);
pub const SPRAF_Active: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(2i32);
pub const SPRAF_AutoPause: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(65536i32);
pub const SPRAF_Dynamic: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(32i32);
pub const SPRAF_Export: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(4i32);
pub const SPRAF_Import: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(8i32);
pub const SPRAF_Interpreter: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(16i32);
pub const SPRAF_Root: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(64i32);
pub const SPRAF_TopLevel: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(1i32);
pub const SPRAF_UserDelimited: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(131072i32);
pub const SPRECOEXTENSION: windows_core::PCWSTR = windows_core::w!("RecoExtension");
pub const SPREF_AutoPause: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(1i32);
pub const SPREF_Emulated: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(2i32);
pub const SPREF_ExtendableParse: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(8i32);
pub const SPREF_FalseRecognition: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(64i32);
pub const SPREF_Hypothesis: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(32i32);
pub const SPREF_ReSent: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(16i32);
pub const SPREF_SMLTimeout: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(4i32);
pub const SPREG_LOCAL_MACHINE_ROOT: windows_core::PCWSTR = windows_core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech");
pub const SPREG_SAFE_USER_TOKENS: windows_core::PCWSTR = windows_core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\UserTokens");
pub const SPREG_USER_ROOT: windows_core::PCWSTR = windows_core::w!("HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Speech");
pub const SPRIO_NONE: SPRULEINFOOPT = SPRULEINFOOPT(0i32);
pub const SPRP_NORMAL: u32 = 0u32;
pub const SPRST_ACTIVE: SPRECOSTATE = SPRECOSTATE(1i32);
pub const SPRST_ACTIVE_ALWAYS: SPRECOSTATE = SPRECOSTATE(2i32);
pub const SPRST_INACTIVE: SPRECOSTATE = SPRECOSTATE(0i32);
pub const SPRST_INACTIVE_WITH_PURGE: SPRECOSTATE = SPRECOSTATE(3i32);
pub const SPRST_NUM_STATES: SPRECOSTATE = SPRECOSTATE(4i32);
pub const SPRS_ACTIVE: SPRULESTATE = SPRULESTATE(1i32);
pub const SPRS_ACTIVE_USER_DELIMITED: SPRULESTATE = SPRULESTATE(4i32);
pub const SPRS_ACTIVE_WITH_AUTO_PAUSE: SPRULESTATE = SPRULESTATE(3i32);
pub const SPRS_DONE: SPRUNSTATE = SPRUNSTATE(1i32);
pub const SPRS_INACTIVE: SPRULESTATE = SPRULESTATE(0i32);
pub const SPRS_IS_SPEAKING: SPRUNSTATE = SPRUNSTATE(2i32);
pub const SPRT_CFG: SPRESULTTYPE = SPRESULTTYPE(0i32);
pub const SPRT_EMULATED: SPRESULTTYPE = SPRESULTTYPE(8i32);
pub const SPRT_EXTENDABLE_PARSE: SPRESULTTYPE = SPRESULTTYPE(16i32);
pub const SPRT_FALSE_RECOGNITION: SPRESULTTYPE = SPRESULTTYPE(4i32);
pub const SPRT_PROPRIETARY: SPRESULTTYPE = SPRESULTTYPE(2i32);
pub const SPRT_SLM: SPRESULTTYPE = SPRESULTTYPE(1i32);
pub const SPRT_TYPE_MASK: SPRESULTTYPE = SPRESULTTYPE(3i32);
pub const SPSF_11kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(10i32);
pub const SPSF_11kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(11i32);
pub const SPSF_11kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(8i32);
pub const SPSF_11kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(9i32);
pub const SPSF_12kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(14i32);
pub const SPSF_12kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(15i32);
pub const SPSF_12kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(12i32);
pub const SPSF_12kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(13i32);
pub const SPSF_16kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(18i32);
pub const SPSF_16kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(19i32);
pub const SPSF_16kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(16i32);
pub const SPSF_16kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(17i32);
pub const SPSF_22kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(22i32);
pub const SPSF_22kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(23i32);
pub const SPSF_22kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(20i32);
pub const SPSF_22kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(21i32);
pub const SPSF_24kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(26i32);
pub const SPSF_24kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(27i32);
pub const SPSF_24kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(24i32);
pub const SPSF_24kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(25i32);
pub const SPSF_32kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(30i32);
pub const SPSF_32kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(31i32);
pub const SPSF_32kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(28i32);
pub const SPSF_32kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(29i32);
pub const SPSF_44kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(34i32);
pub const SPSF_44kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(35i32);
pub const SPSF_44kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(32i32);
pub const SPSF_44kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(33i32);
pub const SPSF_48kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(38i32);
pub const SPSF_48kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(39i32);
pub const SPSF_48kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(36i32);
pub const SPSF_48kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(37i32);
pub const SPSF_8kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(6i32);
pub const SPSF_8kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(7i32);
pub const SPSF_8kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(4i32);
pub const SPSF_8kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(5i32);
pub const SPSF_ADPCM_11kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(59i32);
pub const SPSF_ADPCM_11kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(60i32);
pub const SPSF_ADPCM_22kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(61i32);
pub const SPSF_ADPCM_22kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(62i32);
pub const SPSF_ADPCM_44kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(63i32);
pub const SPSF_ADPCM_44kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(64i32);
pub const SPSF_ADPCM_8kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(57i32);
pub const SPSF_ADPCM_8kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(58i32);
pub const SPSF_CCITT_ALaw_11kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(43i32);
pub const SPSF_CCITT_ALaw_11kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(44i32);
pub const SPSF_CCITT_ALaw_22kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(45i32);
pub const SPSF_CCITT_ALaw_22kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(46i32);
pub const SPSF_CCITT_ALaw_44kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(47i32);
pub const SPSF_CCITT_ALaw_44kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(48i32);
pub const SPSF_CCITT_ALaw_8kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(41i32);
pub const SPSF_CCITT_ALaw_8kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(42i32);
pub const SPSF_CCITT_uLaw_11kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(51i32);
pub const SPSF_CCITT_uLaw_11kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(52i32);
pub const SPSF_CCITT_uLaw_22kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(53i32);
pub const SPSF_CCITT_uLaw_22kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(54i32);
pub const SPSF_CCITT_uLaw_44kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(55i32);
pub const SPSF_CCITT_uLaw_44kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(56i32);
pub const SPSF_CCITT_uLaw_8kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(49i32);
pub const SPSF_CCITT_uLaw_8kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(50i32);
pub const SPSF_Default: SPSTREAMFORMAT = SPSTREAMFORMAT(-1i32);
pub const SPSF_ExtendedAudioFormat: SPSTREAMFORMAT = SPSTREAMFORMAT(3i32);
pub const SPSF_GSM610_11kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(66i32);
pub const SPSF_GSM610_22kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(67i32);
pub const SPSF_GSM610_44kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(68i32);
pub const SPSF_GSM610_8kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(65i32);
pub const SPSF_NUM_FORMATS: SPSTREAMFORMAT = SPSTREAMFORMAT(69i32);
pub const SPSF_NoAssignedFormat: SPSTREAMFORMAT = SPSTREAMFORMAT(0i32);
pub const SPSF_NonStandardFormat: SPSTREAMFORMAT = SPSTREAMFORMAT(2i32);
pub const SPSF_Text: SPSTREAMFORMAT = SPSTREAMFORMAT(1i32);
pub const SPSF_TrueSpeech_8kHz1BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(40i32);
pub const SPSFunction: SpeechPartOfSpeech = SpeechPartOfSpeech(16384i32);
pub const SPSHT_EMAIL: SPSHORTCUTTYPE = SPSHORTCUTTYPE(4096i32);
pub const SPSHT_NotOverriden: SPSHORTCUTTYPE = SPSHORTCUTTYPE(-1i32);
pub const SPSHT_OTHER: SPSHORTCUTTYPE = SPSHORTCUTTYPE(8192i32);
pub const SPSHT_Unknown: SPSHORTCUTTYPE = SPSHORTCUTTYPE(0i32);
pub const SPSInterjection: SpeechPartOfSpeech = SpeechPartOfSpeech(20480i32);
pub const SPSLMA: SpeechPartOfSpeech = SpeechPartOfSpeech(28672i32);
pub const SPSMF_SAPI_PROPERTIES: SPSEMANTICFORMAT = SPSEMANTICFORMAT(0i32);
pub const SPSMF_SRGS_SAPIPROPERTIES: SPSEMANTICFORMAT = SPSEMANTICFORMAT(2i32);
pub const SPSMF_SRGS_SEMANTICINTERPRETATION_MS: SPSEMANTICFORMAT = SPSEMANTICFORMAT(1i32);
pub const SPSMF_SRGS_SEMANTICINTERPRETATION_W3C: SPSEMANTICFORMAT = SPSEMANTICFORMAT(8i32);
pub const SPSMF_UPS: SPSEMANTICFORMAT = SPSEMANTICFORMAT(4i32);
pub const SPSModifier: SpeechPartOfSpeech = SpeechPartOfSpeech(12288i32);
pub const SPSNotOverriden: SpeechPartOfSpeech = SpeechPartOfSpeech(-1i32);
pub const SPSNoun: SpeechPartOfSpeech = SpeechPartOfSpeech(4096i32);
pub const SPSSuppressWord: SpeechPartOfSpeech = SpeechPartOfSpeech(61440i32);
pub const SPSUnknown: SpeechPartOfSpeech = SpeechPartOfSpeech(0i32);
pub const SPSVerb: SpeechPartOfSpeech = SpeechPartOfSpeech(8192i32);
pub const SPTOKENKEY_ATTRIBUTES: windows_core::PCWSTR = windows_core::w!("Attributes");
pub const SPTOKENKEY_AUDIO_LATENCY_TRUNCATE: windows_core::PCWSTR = windows_core::w!("LatencyTruncateThreshold");
pub const SPTOKENKEY_AUDIO_LATENCY_UPDATE_INTERVAL: windows_core::PCWSTR = windows_core::w!("LatencyUpdateInterval");
pub const SPTOKENKEY_AUDIO_LATENCY_WARNING: windows_core::PCWSTR = windows_core::w!("LatencyWarningThreshold");
pub const SPTOKENKEY_FILES: windows_core::PCWSTR = windows_core::w!("Files");
pub const SPTOKENKEY_RETAINEDAUDIO: windows_core::PCWSTR = windows_core::w!("SecondsPerRetainedAudioEvent");
pub const SPTOKENKEY_UI: windows_core::PCWSTR = windows_core::w!("UI");
pub const SPTOKENVALUE_CLSID: windows_core::PCWSTR = windows_core::w!("CLSID");
pub const SPTOPIC_SPELLING: windows_core::PCWSTR = windows_core::w!("Spelling");
pub const SPTRANSDICTATION: SPTRANSITIONTYPE = SPTRANSITIONTYPE(5i32);
pub const SPTRANSEPSILON: SPTRANSITIONTYPE = SPTRANSITIONTYPE(0i32);
pub const SPTRANSRULE: SPTRANSITIONTYPE = SPTRANSITIONTYPE(2i32);
pub const SPTRANSTEXTBUF: SPTRANSITIONTYPE = SPTRANSITIONTYPE(3i32);
pub const SPTRANSWILDCARD: SPTRANSITIONTYPE = SPTRANSITIONTYPE(4i32);
pub const SPTRANSWORD: SPTRANSITIONTYPE = SPTRANSITIONTYPE(1i32);
pub const SPVA_Bookmark: SPVACTIONS = SPVACTIONS(3i32);
pub const SPVA_ParseUnknownTag: SPVACTIONS = SPVACTIONS(6i32);
pub const SPVA_Pronounce: SPVACTIONS = SPVACTIONS(2i32);
pub const SPVA_Section: SPVACTIONS = SPVACTIONS(5i32);
pub const SPVA_Silence: SPVACTIONS = SPVACTIONS(1i32);
pub const SPVA_Speak: SPVACTIONS = SPVACTIONS(0i32);
pub const SPVA_SpellOut: SPVACTIONS = SPVACTIONS(4i32);
pub const SPVES_ABORT: SPVESACTIONS = SPVESACTIONS(1i32);
pub const SPVES_CONTINUE: SPVESACTIONS = SPVESACTIONS(0i32);
pub const SPVES_RATE: SPVESACTIONS = SPVESACTIONS(4i32);
pub const SPVES_SKIP: SPVESACTIONS = SPVESACTIONS(2i32);
pub const SPVES_VOLUME: SPVESACTIONS = SPVESACTIONS(8i32);
pub const SPVFEATURE_EMPHASIS: SPVFEATURE = SPVFEATURE(2i32);
pub const SPVFEATURE_STRESSED: SPVFEATURE = SPVFEATURE(1i32);
pub const SPVOICECATEGORY_TTSRATE: windows_core::PCWSTR = windows_core::w!("DefaultTTSRate");
pub const SPVPRI_ALERT: SPVPRIORITY = SPVPRIORITY(1i32);
pub const SPVPRI_NORMAL: SPVPRIORITY = SPVPRIORITY(0i32);
pub const SPVPRI_OVER: SPVPRIORITY = SPVPRIORITY(2i32);
pub const SPVST_SENTENCE: SPVSKIPTYPE = SPVSKIPTYPE(1i32);
pub const SPWF_INPUT: SPSTREAMFORMATTYPE = SPSTREAMFORMATTYPE(0i32);
pub const SPWF_SRENGINE: SPSTREAMFORMATTYPE = SPSTREAMFORMATTYPE(1i32);
pub const SPWILDCARD: windows_core::PCWSTR = windows_core::w!("...");
pub const SPWIO_NONE: SPWORDINFOOPT = SPWORDINFOOPT(0i32);
pub const SPWIO_WANT_TEXT: SPWORDINFOOPT = SPWORDINFOOPT(1i32);
pub const SPWP_KNOWN_WORD_PRONOUNCEABLE: SPWORDPRONOUNCEABLE = SPWORDPRONOUNCEABLE(2i32);
pub const SPWP_UNKNOWN_WORD_PRONOUNCEABLE: SPWORDPRONOUNCEABLE = SPWORDPRONOUNCEABLE(1i32);
pub const SPWP_UNKNOWN_WORD_UNPRONOUNCEABLE: SPWORDPRONOUNCEABLE = SPWORDPRONOUNCEABLE(0i32);
pub const SPWT_DISPLAY: SPGRAMMARWORDTYPE = SPGRAMMARWORDTYPE(0i32);
pub const SPWT_LEXICAL: SPGRAMMARWORDTYPE = SPGRAMMARWORDTYPE(1i32);
pub const SPWT_LEXICAL_NO_SPECIAL_CHARS: SPGRAMMARWORDTYPE = SPGRAMMARWORDTYPE(3i32);
pub const SPWT_PRONUNCIATION: SPGRAMMARWORDTYPE = SPGRAMMARWORDTYPE(2i32);
pub const SPXRO_Alternates_SML: SPXMLRESULTOPTIONS = SPXMLRESULTOPTIONS(1i32);
pub const SPXRO_SML: SPXMLRESULTOPTIONS = SPXMLRESULTOPTIONS(0i32);
pub const SP_EMULATE_RESULT: u32 = 1073741824u32;
pub const SP_LOW_CONFIDENCE: i32 = -1i32;
pub const SP_MAX_LANGIDS: u32 = 20u32;
pub const SP_MAX_PRON_LENGTH: u32 = 384u32;
pub const SP_MAX_WORD_LENGTH: u32 = 128u32;
pub const SP_NORMAL_CONFIDENCE: u32 = 0u32;
pub const SP_STREAMPOS_ASAP: u32 = 0u32;
pub const SP_STREAMPOS_REALTIME: i32 = -1i32;
pub const SP_VISEME_0: SPVISEMES = SPVISEMES(0i32);
pub const SP_VISEME_1: SPVISEMES = SPVISEMES(1i32);
pub const SP_VISEME_10: SPVISEMES = SPVISEMES(10i32);
pub const SP_VISEME_11: SPVISEMES = SPVISEMES(11i32);
pub const SP_VISEME_12: SPVISEMES = SPVISEMES(12i32);
pub const SP_VISEME_13: SPVISEMES = SPVISEMES(13i32);
pub const SP_VISEME_14: SPVISEMES = SPVISEMES(14i32);
pub const SP_VISEME_15: SPVISEMES = SPVISEMES(15i32);
pub const SP_VISEME_16: SPVISEMES = SPVISEMES(16i32);
pub const SP_VISEME_17: SPVISEMES = SPVISEMES(17i32);
pub const SP_VISEME_18: SPVISEMES = SPVISEMES(18i32);
pub const SP_VISEME_19: SPVISEMES = SPVISEMES(19i32);
pub const SP_VISEME_2: SPVISEMES = SPVISEMES(2i32);
pub const SP_VISEME_20: SPVISEMES = SPVISEMES(20i32);
pub const SP_VISEME_21: SPVISEMES = SPVISEMES(21i32);
pub const SP_VISEME_3: SPVISEMES = SPVISEMES(3i32);
pub const SP_VISEME_4: SPVISEMES = SPVISEMES(4i32);
pub const SP_VISEME_5: SPVISEMES = SPVISEMES(5i32);
pub const SP_VISEME_6: SPVISEMES = SPVISEMES(6i32);
pub const SP_VISEME_7: SPVISEMES = SPVISEMES(7i32);
pub const SP_VISEME_8: SPVISEMES = SPVISEMES(8i32);
pub const SP_VISEME_9: SPVISEMES = SPVISEMES(9i32);
pub const SRADefaultToActive: SpeechRuleAttributes = SpeechRuleAttributes(2i32);
pub const SRADynamic: SpeechRuleAttributes = SpeechRuleAttributes(32i32);
pub const SRAExport: SpeechRuleAttributes = SpeechRuleAttributes(4i32);
pub const SRAImport: SpeechRuleAttributes = SpeechRuleAttributes(8i32);
pub const SRAInterpreter: SpeechRuleAttributes = SpeechRuleAttributes(16i32);
pub const SRAONone: SpeechRetainedAudioOptions = SpeechRetainedAudioOptions(0i32);
pub const SRAORetainAudio: SpeechRetainedAudioOptions = SpeechRetainedAudioOptions(1i32);
pub const SRARoot: SpeechRuleAttributes = SpeechRuleAttributes(64i32);
pub const SRATopLevel: SpeechRuleAttributes = SpeechRuleAttributes(1i32);
pub const SRCS_Disabled: SpeechRecoContextState = SpeechRecoContextState(0i32);
pub const SRCS_Enabled: SpeechRecoContextState = SpeechRecoContextState(1i32);
pub const SREAdaptation: SpeechRecoEvents = SpeechRecoEvents(8192i32);
pub const SREAllEvents: SpeechRecoEvents = SpeechRecoEvents(393215i32);
pub const SREAudioLevel: SpeechRecoEvents = SpeechRecoEvents(65536i32);
pub const SREBookmark: SpeechRecoEvents = SpeechRecoEvents(64i32);
pub const SREFalseRecognition: SpeechRecoEvents = SpeechRecoEvents(512i32);
pub const SREHypothesis: SpeechRecoEvents = SpeechRecoEvents(32i32);
pub const SREInterference: SpeechRecoEvents = SpeechRecoEvents(1024i32);
pub const SREPhraseStart: SpeechRecoEvents = SpeechRecoEvents(8i32);
pub const SREPrivate: SpeechRecoEvents = SpeechRecoEvents(262144i32);
pub const SREPropertyNumChange: SpeechRecoEvents = SpeechRecoEvents(128i32);
pub const SREPropertyStringChange: SpeechRecoEvents = SpeechRecoEvents(256i32);
pub const SRERecoOtherContext: SpeechRecoEvents = SpeechRecoEvents(32768i32);
pub const SRERecognition: SpeechRecoEvents = SpeechRecoEvents(16i32);
pub const SRERequestUI: SpeechRecoEvents = SpeechRecoEvents(2048i32);
pub const SRESoundEnd: SpeechRecoEvents = SpeechRecoEvents(4i32);
pub const SRESoundStart: SpeechRecoEvents = SpeechRecoEvents(2i32);
pub const SREStateChange: SpeechRecoEvents = SpeechRecoEvents(4096i32);
pub const SREStreamEnd: SpeechRecoEvents = SpeechRecoEvents(1i32);
pub const SREStreamStart: SpeechRecoEvents = SpeechRecoEvents(16384i32);
pub const SRSActive: SpeechRecognizerState = SpeechRecognizerState(1i32);
pub const SRSActiveAlways: SpeechRecognizerState = SpeechRecognizerState(2i32);
pub const SRSEDone: SpeechRunState = SpeechRunState(1i32);
pub const SRSEIsSpeaking: SpeechRunState = SpeechRunState(2i32);
pub const SRSInactive: SpeechRecognizerState = SpeechRecognizerState(0i32);
pub const SRSInactiveWithPurge: SpeechRecognizerState = SpeechRecognizerState(3i32);
pub const SRTAutopause: SpeechRecognitionType = SpeechRecognitionType(1i32);
pub const SRTEmulated: SpeechRecognitionType = SpeechRecognitionType(2i32);
pub const SRTExtendableParse: SpeechRecognitionType = SpeechRecognitionType(8i32);
pub const SRTReSent: SpeechRecognitionType = SpeechRecognitionType(16i32);
pub const SRTSMLTimeout: SpeechRecognitionType = SpeechRecognitionType(4i32);
pub const SRTStandard: SpeechRecognitionType = SpeechRecognitionType(0i32);
pub const SR_LOCALIZED_DESCRIPTION: windows_core::PCWSTR = windows_core::w!("Description");
pub const SSFMCreate: SpeechStreamFileMode = SpeechStreamFileMode(2i32);
pub const SSFMCreateForWrite: SpeechStreamFileMode = SpeechStreamFileMode(3i32);
pub const SSFMOpenForRead: SpeechStreamFileMode = SpeechStreamFileMode(0i32);
pub const SSFMOpenReadWrite: SpeechStreamFileMode = SpeechStreamFileMode(1i32);
pub const SSSPTRelativeToCurrentPosition: SpeechStreamSeekPositionType = SpeechStreamSeekPositionType(1u32);
pub const SSSPTRelativeToEnd: SpeechStreamSeekPositionType = SpeechStreamSeekPositionType(2u32);
pub const SSSPTRelativeToStart: SpeechStreamSeekPositionType = SpeechStreamSeekPositionType(0u32);
pub const SSTTDictation: SpeechSpecialTransitionType = SpeechSpecialTransitionType(2i32);
pub const SSTTTextBuffer: SpeechSpecialTransitionType = SpeechSpecialTransitionType(3i32);
pub const SSTTWildcard: SpeechSpecialTransitionType = SpeechSpecialTransitionType(1i32);
pub const STCAll: SpeechTokenContext = SpeechTokenContext(23u32);
pub const STCInprocHandler: SpeechTokenContext = SpeechTokenContext(2u32);
pub const STCInprocServer: SpeechTokenContext = SpeechTokenContext(1u32);
pub const STCLocalServer: SpeechTokenContext = SpeechTokenContext(4u32);
pub const STCRemoteServer: SpeechTokenContext = SpeechTokenContext(16u32);
pub const STSF_AppData: SpeechTokenShellFolder = SpeechTokenShellFolder(26i32);
pub const STSF_CommonAppData: SpeechTokenShellFolder = SpeechTokenShellFolder(35i32);
pub const STSF_FlagCreate: SpeechTokenShellFolder = SpeechTokenShellFolder(32768i32);
pub const STSF_LocalAppData: SpeechTokenShellFolder = SpeechTokenShellFolder(28i32);
pub const SVEAllEvents: SpeechVoiceEvents = SpeechVoiceEvents(33790i32);
pub const SVEAudioLevel: SpeechVoiceEvents = SpeechVoiceEvents(512i32);
pub const SVEBookmark: SpeechVoiceEvents = SpeechVoiceEvents(16i32);
pub const SVEEndInputStream: SpeechVoiceEvents = SpeechVoiceEvents(4i32);
pub const SVEPhoneme: SpeechVoiceEvents = SpeechVoiceEvents(64i32);
pub const SVEPrivate: SpeechVoiceEvents = SpeechVoiceEvents(32768i32);
pub const SVESentenceBoundary: SpeechVoiceEvents = SpeechVoiceEvents(128i32);
pub const SVEStartInputStream: SpeechVoiceEvents = SpeechVoiceEvents(2i32);
pub const SVEViseme: SpeechVoiceEvents = SpeechVoiceEvents(256i32);
pub const SVEVoiceChange: SpeechVoiceEvents = SpeechVoiceEvents(8i32);
pub const SVEWordBoundary: SpeechVoiceEvents = SpeechVoiceEvents(32i32);
pub const SVF_Emphasis: SpeechVisemeFeature = SpeechVisemeFeature(2i32);
pub const SVF_None: SpeechVisemeFeature = SpeechVisemeFeature(0i32);
pub const SVF_Stressed: SpeechVisemeFeature = SpeechVisemeFeature(1i32);
pub const SVPAlert: SpeechVoicePriority = SpeechVoicePriority(1i32);
pub const SVPNormal: SpeechVoicePriority = SpeechVoicePriority(0i32);
pub const SVPOver: SpeechVoicePriority = SpeechVoicePriority(2i32);
pub const SVP_0: SpeechVisemeType = SpeechVisemeType(0i32);
pub const SVP_1: SpeechVisemeType = SpeechVisemeType(1i32);
pub const SVP_10: SpeechVisemeType = SpeechVisemeType(10i32);
pub const SVP_11: SpeechVisemeType = SpeechVisemeType(11i32);
pub const SVP_12: SpeechVisemeType = SpeechVisemeType(12i32);
pub const SVP_13: SpeechVisemeType = SpeechVisemeType(13i32);
pub const SVP_14: SpeechVisemeType = SpeechVisemeType(14i32);
pub const SVP_15: SpeechVisemeType = SpeechVisemeType(15i32);
pub const SVP_16: SpeechVisemeType = SpeechVisemeType(16i32);
pub const SVP_17: SpeechVisemeType = SpeechVisemeType(17i32);
pub const SVP_18: SpeechVisemeType = SpeechVisemeType(18i32);
pub const SVP_19: SpeechVisemeType = SpeechVisemeType(19i32);
pub const SVP_2: SpeechVisemeType = SpeechVisemeType(2i32);
pub const SVP_20: SpeechVisemeType = SpeechVisemeType(20i32);
pub const SVP_21: SpeechVisemeType = SpeechVisemeType(21i32);
pub const SVP_3: SpeechVisemeType = SpeechVisemeType(3i32);
pub const SVP_4: SpeechVisemeType = SpeechVisemeType(4i32);
pub const SVP_5: SpeechVisemeType = SpeechVisemeType(5i32);
pub const SVP_6: SpeechVisemeType = SpeechVisemeType(6i32);
pub const SVP_7: SpeechVisemeType = SpeechVisemeType(7i32);
pub const SVP_8: SpeechVisemeType = SpeechVisemeType(8i32);
pub const SVP_9: SpeechVisemeType = SpeechVisemeType(9i32);
pub const SVSFDefault: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(0i32);
pub const SVSFIsFilename: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(4i32);
pub const SVSFIsNotXML: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(16i32);
pub const SVSFIsXML: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(8i32);
pub const SVSFNLPMask: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(64i32);
pub const SVSFNLPSpeakPunc: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(64i32);
pub const SVSFParseAutodetect: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(0i32);
pub const SVSFParseMask: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(384i32);
pub const SVSFParseSapi: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(128i32);
pub const SVSFParseSsml: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(256i32);
pub const SVSFPersistXML: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(32i32);
pub const SVSFPurgeBeforeSpeak: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(2i32);
pub const SVSFUnusedFlags: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(-512i32);
pub const SVSFVoiceMask: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(511i32);
pub const SVSFlagsAsync: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(1i32);
pub const SWPKnownWordPronounceable: SpeechWordPronounceable = SpeechWordPronounceable(2i32);
pub const SWPUnknownWordPronounceable: SpeechWordPronounceable = SpeechWordPronounceable(1i32);
pub const SWPUnknownWordUnpronounceable: SpeechWordPronounceable = SpeechWordPronounceable(0i32);
pub const SWTAdded: SpeechWordType = SpeechWordType(1i32);
pub const SWTDeleted: SpeechWordType = SpeechWordType(2i32);
pub const SpeechAllElements: i32 = -1i32;
pub const Speech_Default_Weight: f32 = 1f32;
pub const Speech_Max_Pron_Length: i32 = 384i32;
pub const Speech_Max_Word_Length: i32 = 128i32;
pub const Speech_StreamPos_Asap: i32 = 0i32;
pub const Speech_StreamPos_RealTime: i32 = -1i32;
pub const Subsequence: SPMATCHINGMODE = SPMATCHINGMODE(1i32);
pub const SubsequenceContentRequired: SPMATCHINGMODE = SPMATCHINGMODE(5i32);
pub const eLEXTYPE_APP: SPLEXICONTYPE = SPLEXICONTYPE(2i32);
pub const eLEXTYPE_LETTERTOSOUND: SPLEXICONTYPE = SPLEXICONTYPE(8i32);
pub const eLEXTYPE_MORPHOLOGY: SPLEXICONTYPE = SPLEXICONTYPE(16i32);
pub const eLEXTYPE_PRIVATE1: SPLEXICONTYPE = SPLEXICONTYPE(4096i32);
pub const eLEXTYPE_PRIVATE10: SPLEXICONTYPE = SPLEXICONTYPE(2097152i32);
pub const eLEXTYPE_PRIVATE11: SPLEXICONTYPE = SPLEXICONTYPE(4194304i32);
pub const eLEXTYPE_PRIVATE12: SPLEXICONTYPE = SPLEXICONTYPE(8388608i32);
pub const eLEXTYPE_PRIVATE13: SPLEXICONTYPE = SPLEXICONTYPE(16777216i32);
pub const eLEXTYPE_PRIVATE14: SPLEXICONTYPE = SPLEXICONTYPE(33554432i32);
pub const eLEXTYPE_PRIVATE15: SPLEXICONTYPE = SPLEXICONTYPE(67108864i32);
pub const eLEXTYPE_PRIVATE16: SPLEXICONTYPE = SPLEXICONTYPE(134217728i32);
pub const eLEXTYPE_PRIVATE17: SPLEXICONTYPE = SPLEXICONTYPE(268435456i32);
pub const eLEXTYPE_PRIVATE18: SPLEXICONTYPE = SPLEXICONTYPE(536870912i32);
pub const eLEXTYPE_PRIVATE19: SPLEXICONTYPE = SPLEXICONTYPE(1073741824i32);
pub const eLEXTYPE_PRIVATE2: SPLEXICONTYPE = SPLEXICONTYPE(8192i32);
pub const eLEXTYPE_PRIVATE20: SPLEXICONTYPE = SPLEXICONTYPE(-2147483648i32);
pub const eLEXTYPE_PRIVATE3: SPLEXICONTYPE = SPLEXICONTYPE(16384i32);
pub const eLEXTYPE_PRIVATE4: SPLEXICONTYPE = SPLEXICONTYPE(32768i32);
pub const eLEXTYPE_PRIVATE5: SPLEXICONTYPE = SPLEXICONTYPE(65536i32);
pub const eLEXTYPE_PRIVATE6: SPLEXICONTYPE = SPLEXICONTYPE(131072i32);
pub const eLEXTYPE_PRIVATE7: SPLEXICONTYPE = SPLEXICONTYPE(262144i32);
pub const eLEXTYPE_PRIVATE8: SPLEXICONTYPE = SPLEXICONTYPE(524288i32);
pub const eLEXTYPE_PRIVATE9: SPLEXICONTYPE = SPLEXICONTYPE(1048576i32);
pub const eLEXTYPE_RESERVED10: SPLEXICONTYPE = SPLEXICONTYPE(2048i32);
pub const eLEXTYPE_RESERVED4: SPLEXICONTYPE = SPLEXICONTYPE(32i32);
pub const eLEXTYPE_RESERVED6: SPLEXICONTYPE = SPLEXICONTYPE(128i32);
pub const eLEXTYPE_RESERVED7: SPLEXICONTYPE = SPLEXICONTYPE(256i32);
pub const eLEXTYPE_RESERVED8: SPLEXICONTYPE = SPLEXICONTYPE(512i32);
pub const eLEXTYPE_RESERVED9: SPLEXICONTYPE = SPLEXICONTYPE(1024i32);
pub const eLEXTYPE_USER: SPLEXICONTYPE = SPLEXICONTYPE(1i32);
pub const eLEXTYPE_USER_SHORTCUT: SPLEXICONTYPE = SPLEXICONTYPE(64i32);
pub const eLEXTYPE_VENDORLEXICON: SPLEXICONTYPE = SPLEXICONTYPE(4i32);
pub const ePRONFLAG_USED: SPPRONUNCIATIONFLAGS = SPPRONUNCIATIONFLAGS(1i32);
pub const eWORDTYPE_ADDED: SPWORDTYPE = SPWORDTYPE(1i32);
pub const eWORDTYPE_DELETED: SPWORDTYPE = SPWORDTYPE(2i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPIDSPRG(pub i32);
impl windows_core::TypeKind for DISPIDSPRG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPIDSPRG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPIDSPRG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPIDSPTSI(pub i32);
impl windows_core::TypeKind for DISPIDSPTSI {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPIDSPTSI {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPIDSPTSI").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechAudio(pub i32);
impl windows_core::TypeKind for DISPID_SpeechAudio {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechAudio {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechAudio").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechAudioBufferInfo(pub i32);
impl windows_core::TypeKind for DISPID_SpeechAudioBufferInfo {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechAudioBufferInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechAudioBufferInfo").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechAudioFormat(pub i32);
impl windows_core::TypeKind for DISPID_SpeechAudioFormat {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechAudioFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechAudioFormat").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechAudioStatus(pub i32);
impl windows_core::TypeKind for DISPID_SpeechAudioStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechAudioStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechAudioStatus").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechBaseStream(pub i32);
impl windows_core::TypeKind for DISPID_SpeechBaseStream {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechBaseStream {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechBaseStream").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechCustomStream(pub i32);
impl windows_core::TypeKind for DISPID_SpeechCustomStream {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechCustomStream {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechCustomStream").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechDataKey(pub i32);
impl windows_core::TypeKind for DISPID_SpeechDataKey {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechDataKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechDataKey").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechFileStream(pub i32);
impl windows_core::TypeKind for DISPID_SpeechFileStream {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechFileStream {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechFileStream").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechGrammarRule(pub i32);
impl windows_core::TypeKind for DISPID_SpeechGrammarRule {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechGrammarRule {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechGrammarRule").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechGrammarRuleState(pub i32);
impl windows_core::TypeKind for DISPID_SpeechGrammarRuleState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechGrammarRuleState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechGrammarRuleState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechGrammarRuleStateTransition(pub i32);
impl windows_core::TypeKind for DISPID_SpeechGrammarRuleStateTransition {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechGrammarRuleStateTransition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechGrammarRuleStateTransition").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechGrammarRuleStateTransitions(pub i32);
impl windows_core::TypeKind for DISPID_SpeechGrammarRuleStateTransitions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechGrammarRuleStateTransitions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechGrammarRuleStateTransitions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechGrammarRules(pub i32);
impl windows_core::TypeKind for DISPID_SpeechGrammarRules {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechGrammarRules {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechGrammarRules").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechLexicon(pub i32);
impl windows_core::TypeKind for DISPID_SpeechLexicon {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechLexicon {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechLexicon").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechLexiconProns(pub i32);
impl windows_core::TypeKind for DISPID_SpeechLexiconProns {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechLexiconProns {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechLexiconProns").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechLexiconPronunciation(pub i32);
impl windows_core::TypeKind for DISPID_SpeechLexiconPronunciation {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechLexiconPronunciation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechLexiconPronunciation").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechLexiconWord(pub i32);
impl windows_core::TypeKind for DISPID_SpeechLexiconWord {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechLexiconWord {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechLexiconWord").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechLexiconWords(pub i32);
impl windows_core::TypeKind for DISPID_SpeechLexiconWords {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechLexiconWords {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechLexiconWords").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechMMSysAudio(pub i32);
impl windows_core::TypeKind for DISPID_SpeechMMSysAudio {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechMMSysAudio {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechMMSysAudio").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechMemoryStream(pub i32);
impl windows_core::TypeKind for DISPID_SpeechMemoryStream {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechMemoryStream {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechMemoryStream").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechObjectToken(pub i32);
impl windows_core::TypeKind for DISPID_SpeechObjectToken {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechObjectToken {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechObjectToken").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechObjectTokenCategory(pub i32);
impl windows_core::TypeKind for DISPID_SpeechObjectTokenCategory {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechObjectTokenCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechObjectTokenCategory").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechObjectTokens(pub i32);
impl windows_core::TypeKind for DISPID_SpeechObjectTokens {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechObjectTokens {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechObjectTokens").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechPhoneConverter(pub i32);
impl windows_core::TypeKind for DISPID_SpeechPhoneConverter {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechPhoneConverter {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechPhoneConverter").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechPhraseAlternate(pub i32);
impl windows_core::TypeKind for DISPID_SpeechPhraseAlternate {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechPhraseAlternate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechPhraseAlternate").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechPhraseAlternates(pub i32);
impl windows_core::TypeKind for DISPID_SpeechPhraseAlternates {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechPhraseAlternates {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechPhraseAlternates").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechPhraseBuilder(pub i32);
impl windows_core::TypeKind for DISPID_SpeechPhraseBuilder {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechPhraseBuilder {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechPhraseBuilder").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechPhraseElement(pub i32);
impl windows_core::TypeKind for DISPID_SpeechPhraseElement {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechPhraseElement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechPhraseElement").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechPhraseElements(pub i32);
impl windows_core::TypeKind for DISPID_SpeechPhraseElements {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechPhraseElements {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechPhraseElements").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechPhraseInfo(pub i32);
impl windows_core::TypeKind for DISPID_SpeechPhraseInfo {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechPhraseInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechPhraseInfo").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechPhraseProperties(pub i32);
impl windows_core::TypeKind for DISPID_SpeechPhraseProperties {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechPhraseProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechPhraseProperties").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechPhraseProperty(pub i32);
impl windows_core::TypeKind for DISPID_SpeechPhraseProperty {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechPhraseProperty {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechPhraseProperty").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechPhraseReplacement(pub i32);
impl windows_core::TypeKind for DISPID_SpeechPhraseReplacement {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechPhraseReplacement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechPhraseReplacement").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechPhraseReplacements(pub i32);
impl windows_core::TypeKind for DISPID_SpeechPhraseReplacements {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechPhraseReplacements {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechPhraseReplacements").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechPhraseRule(pub i32);
impl windows_core::TypeKind for DISPID_SpeechPhraseRule {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechPhraseRule {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechPhraseRule").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechPhraseRules(pub i32);
impl windows_core::TypeKind for DISPID_SpeechPhraseRules {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechPhraseRules {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechPhraseRules").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechRecoContext(pub i32);
impl windows_core::TypeKind for DISPID_SpeechRecoContext {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechRecoContext {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechRecoContext").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechRecoContextEvents(pub i32);
impl windows_core::TypeKind for DISPID_SpeechRecoContextEvents {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechRecoContextEvents {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechRecoContextEvents").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechRecoResult(pub i32);
impl windows_core::TypeKind for DISPID_SpeechRecoResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechRecoResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechRecoResult").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechRecoResult2(pub i32);
impl windows_core::TypeKind for DISPID_SpeechRecoResult2 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechRecoResult2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechRecoResult2").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechRecoResultTimes(pub i32);
impl windows_core::TypeKind for DISPID_SpeechRecoResultTimes {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechRecoResultTimes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechRecoResultTimes").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechRecognizer(pub i32);
impl windows_core::TypeKind for DISPID_SpeechRecognizer {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechRecognizer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechRecognizer").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechRecognizerStatus(pub i32);
impl windows_core::TypeKind for DISPID_SpeechRecognizerStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechRecognizerStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechRecognizerStatus").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechVoice(pub i32);
impl windows_core::TypeKind for DISPID_SpeechVoice {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechVoice {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechVoice").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechVoiceEvent(pub i32);
impl windows_core::TypeKind for DISPID_SpeechVoiceEvent {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechVoiceEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechVoiceEvent").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechVoiceStatus(pub i32);
impl windows_core::TypeKind for DISPID_SpeechVoiceStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechVoiceStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechVoiceStatus").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechWaveFormatEx(pub i32);
impl windows_core::TypeKind for DISPID_SpeechWaveFormatEx {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechWaveFormatEx {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechWaveFormatEx").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPID_SpeechXMLRecoResult(pub i32);
impl windows_core::TypeKind for DISPID_SpeechXMLRecoResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPID_SpeechXMLRecoResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPID_SpeechXMLRecoResult").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PHONETICALPHABET(pub i32);
impl windows_core::TypeKind for PHONETICALPHABET {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PHONETICALPHABET {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PHONETICALPHABET").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPADAPTATIONRELEVANCE(pub i32);
impl windows_core::TypeKind for SPADAPTATIONRELEVANCE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPADAPTATIONRELEVANCE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPADAPTATIONRELEVANCE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPADAPTATIONSETTINGS(pub i32);
impl windows_core::TypeKind for SPADAPTATIONSETTINGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPADAPTATIONSETTINGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPADAPTATIONSETTINGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPAUDIOOPTIONS(pub i32);
impl windows_core::TypeKind for SPAUDIOOPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPAUDIOOPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPAUDIOOPTIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPAUDIOSTATE(pub i32);
impl windows_core::TypeKind for SPAUDIOSTATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPAUDIOSTATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPAUDIOSTATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPBOOKMARKOPTIONS(pub i32);
impl windows_core::TypeKind for SPBOOKMARKOPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPBOOKMARKOPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPBOOKMARKOPTIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPCFGNOTIFY(pub i32);
impl windows_core::TypeKind for SPCFGNOTIFY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPCFGNOTIFY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPCFGNOTIFY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPCFGRULEATTRIBUTES(pub i32);
impl windows_core::TypeKind for SPCFGRULEATTRIBUTES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPCFGRULEATTRIBUTES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPCFGRULEATTRIBUTES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPCOMMITFLAGS(pub i32);
impl windows_core::TypeKind for SPCOMMITFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPCOMMITFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPCOMMITFLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPCONTEXTSTATE(pub i32);
impl windows_core::TypeKind for SPCONTEXTSTATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPCONTEXTSTATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPCONTEXTSTATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPDATAKEYLOCATION(pub i32);
impl windows_core::TypeKind for SPDATAKEYLOCATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPDATAKEYLOCATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPDATAKEYLOCATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPDISPLAYATTRIBUTES(pub i32);
impl windows_core::TypeKind for SPDISPLAYATTRIBUTES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPDISPLAYATTRIBUTES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPDISPLAYATTRIBUTES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPEAKFLAGS(pub i32);
impl windows_core::TypeKind for SPEAKFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPEAKFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPEAKFLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPENDSRSTREAMFLAGS(pub i32);
impl windows_core::TypeKind for SPENDSRSTREAMFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPENDSRSTREAMFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPENDSRSTREAMFLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPEVENTENUM(pub i32);
impl windows_core::TypeKind for SPEVENTENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPEVENTENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPEVENTENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPEVENTLPARAMTYPE(pub i32);
impl windows_core::TypeKind for SPEVENTLPARAMTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPEVENTLPARAMTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPEVENTLPARAMTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPFILEMODE(pub i32);
impl windows_core::TypeKind for SPFILEMODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPFILEMODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPFILEMODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPGRAMMAROPTIONS(pub i32);
impl windows_core::TypeKind for SPGRAMMAROPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPGRAMMAROPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPGRAMMAROPTIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPGRAMMARSTATE(pub i32);
impl windows_core::TypeKind for SPGRAMMARSTATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPGRAMMARSTATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPGRAMMARSTATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPGRAMMARWORDTYPE(pub i32);
impl windows_core::TypeKind for SPGRAMMARWORDTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPGRAMMARWORDTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPGRAMMARWORDTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPINTERFERENCE(pub i32);
impl windows_core::TypeKind for SPINTERFERENCE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPINTERFERENCE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPINTERFERENCE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPLEXICONTYPE(pub i32);
impl windows_core::TypeKind for SPLEXICONTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPLEXICONTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPLEXICONTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPLOADOPTIONS(pub i32);
impl windows_core::TypeKind for SPLOADOPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPLOADOPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPLOADOPTIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPMATCHINGMODE(pub i32);
impl windows_core::TypeKind for SPMATCHINGMODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPMATCHINGMODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPMATCHINGMODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPPARTOFSPEECH(pub i32);
impl windows_core::TypeKind for SPPARTOFSPEECH {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPPARTOFSPEECH {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPPARTOFSPEECH").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPPHRASEPROPERTYUNIONTYPE(pub i32);
impl windows_core::TypeKind for SPPHRASEPROPERTYUNIONTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPPHRASEPROPERTYUNIONTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPPHRASEPROPERTYUNIONTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPPHRASERNG(pub i32);
impl windows_core::TypeKind for SPPHRASERNG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPPHRASERNG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPPHRASERNG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPPRONUNCIATIONFLAGS(pub i32);
impl windows_core::TypeKind for SPPRONUNCIATIONFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPPRONUNCIATIONFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPPRONUNCIATIONFLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPPROPSRC(pub i32);
impl windows_core::TypeKind for SPPROPSRC {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPPROPSRC {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPPROPSRC").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPRECOEVENTFLAGS(pub i32);
impl windows_core::TypeKind for SPRECOEVENTFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPRECOEVENTFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPRECOEVENTFLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPRECOSTATE(pub i32);
impl windows_core::TypeKind for SPRECOSTATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPRECOSTATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPRECOSTATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPRESULTTYPE(pub i32);
impl windows_core::TypeKind for SPRESULTTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPRESULTTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPRESULTTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPRULEINFOOPT(pub i32);
impl windows_core::TypeKind for SPRULEINFOOPT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPRULEINFOOPT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPRULEINFOOPT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPRULESTATE(pub i32);
impl windows_core::TypeKind for SPRULESTATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPRULESTATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPRULESTATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPRUNSTATE(pub i32);
impl windows_core::TypeKind for SPRUNSTATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPRUNSTATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPRUNSTATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPSEMANTICFORMAT(pub i32);
impl windows_core::TypeKind for SPSEMANTICFORMAT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPSEMANTICFORMAT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPSEMANTICFORMAT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPSHORTCUTTYPE(pub i32);
impl windows_core::TypeKind for SPSHORTCUTTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPSHORTCUTTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPSHORTCUTTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPSTREAMFORMAT(pub i32);
impl windows_core::TypeKind for SPSTREAMFORMAT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPSTREAMFORMAT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPSTREAMFORMAT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPSTREAMFORMATTYPE(pub i32);
impl windows_core::TypeKind for SPSTREAMFORMATTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPSTREAMFORMATTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPSTREAMFORMATTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPTRANSITIONTYPE(pub i32);
impl windows_core::TypeKind for SPTRANSITIONTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPTRANSITIONTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPTRANSITIONTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPVACTIONS(pub i32);
impl windows_core::TypeKind for SPVACTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPVACTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPVACTIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPVALUETYPE(pub i32);
impl windows_core::TypeKind for SPVALUETYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPVALUETYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPVALUETYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPVESACTIONS(pub i32);
impl windows_core::TypeKind for SPVESACTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPVESACTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPVESACTIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPVFEATURE(pub i32);
impl windows_core::TypeKind for SPVFEATURE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPVFEATURE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPVFEATURE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPVISEMES(pub i32);
impl windows_core::TypeKind for SPVISEMES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPVISEMES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPVISEMES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPVLIMITS(pub i32);
impl windows_core::TypeKind for SPVLIMITS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPVLIMITS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPVLIMITS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPVPRIORITY(pub i32);
impl windows_core::TypeKind for SPVPRIORITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPVPRIORITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPVPRIORITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPVSKIPTYPE(pub i32);
impl windows_core::TypeKind for SPVSKIPTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPVSKIPTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPVSKIPTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPWORDINFOOPT(pub i32);
impl windows_core::TypeKind for SPWORDINFOOPT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPWORDINFOOPT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPWORDINFOOPT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPWORDPRONOUNCEABLE(pub i32);
impl windows_core::TypeKind for SPWORDPRONOUNCEABLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPWORDPRONOUNCEABLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPWORDPRONOUNCEABLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPWORDTYPE(pub i32);
impl windows_core::TypeKind for SPWORDTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPWORDTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPWORDTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPXMLRESULTOPTIONS(pub i32);
impl windows_core::TypeKind for SPXMLRESULTOPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPXMLRESULTOPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPXMLRESULTOPTIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechAudioFormatType(pub i32);
impl windows_core::TypeKind for SpeechAudioFormatType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechAudioFormatType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechAudioFormatType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechAudioState(pub i32);
impl windows_core::TypeKind for SpeechAudioState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechAudioState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechAudioState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechBookmarkOptions(pub i32);
impl windows_core::TypeKind for SpeechBookmarkOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechBookmarkOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechBookmarkOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechDataKeyLocation(pub i32);
impl windows_core::TypeKind for SpeechDataKeyLocation {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechDataKeyLocation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechDataKeyLocation").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechDiscardType(pub i32);
impl windows_core::TypeKind for SpeechDiscardType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechDiscardType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechDiscardType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechDisplayAttributes(pub i32);
impl windows_core::TypeKind for SpeechDisplayAttributes {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechDisplayAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechDisplayAttributes").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechEmulationCompareFlags(pub i32);
impl windows_core::TypeKind for SpeechEmulationCompareFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechEmulationCompareFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechEmulationCompareFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechEngineConfidence(pub i32);
impl windows_core::TypeKind for SpeechEngineConfidence {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechEngineConfidence {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechEngineConfidence").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechFormatType(pub i32);
impl windows_core::TypeKind for SpeechFormatType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechFormatType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechFormatType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechGrammarRuleStateTransitionType(pub i32);
impl windows_core::TypeKind for SpeechGrammarRuleStateTransitionType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechGrammarRuleStateTransitionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechGrammarRuleStateTransitionType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechGrammarState(pub i32);
impl windows_core::TypeKind for SpeechGrammarState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechGrammarState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechGrammarState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechGrammarWordType(pub i32);
impl windows_core::TypeKind for SpeechGrammarWordType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechGrammarWordType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechGrammarWordType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechInterference(pub i32);
impl windows_core::TypeKind for SpeechInterference {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechInterference {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechInterference").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechLexiconType(pub i32);
impl windows_core::TypeKind for SpeechLexiconType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechLexiconType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechLexiconType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechLoadOption(pub i32);
impl windows_core::TypeKind for SpeechLoadOption {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechLoadOption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechLoadOption").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechPartOfSpeech(pub i32);
impl windows_core::TypeKind for SpeechPartOfSpeech {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechPartOfSpeech {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechPartOfSpeech").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechRecoContextState(pub i32);
impl windows_core::TypeKind for SpeechRecoContextState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechRecoContextState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechRecoContextState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechRecoEvents(pub i32);
impl windows_core::TypeKind for SpeechRecoEvents {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechRecoEvents {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechRecoEvents").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechRecognitionType(pub i32);
impl windows_core::TypeKind for SpeechRecognitionType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechRecognitionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechRecognitionType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechRecognizerState(pub i32);
impl windows_core::TypeKind for SpeechRecognizerState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechRecognizerState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechRecognizerState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechRetainedAudioOptions(pub i32);
impl windows_core::TypeKind for SpeechRetainedAudioOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechRetainedAudioOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechRetainedAudioOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechRuleAttributes(pub i32);
impl windows_core::TypeKind for SpeechRuleAttributes {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechRuleAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechRuleAttributes").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechRuleState(pub i32);
impl windows_core::TypeKind for SpeechRuleState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechRuleState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechRuleState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechRunState(pub i32);
impl windows_core::TypeKind for SpeechRunState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechRunState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechRunState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechSpecialTransitionType(pub i32);
impl windows_core::TypeKind for SpeechSpecialTransitionType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechSpecialTransitionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechSpecialTransitionType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechStreamFileMode(pub i32);
impl windows_core::TypeKind for SpeechStreamFileMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechStreamFileMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechStreamFileMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechStreamSeekPositionType(pub u32);
impl windows_core::TypeKind for SpeechStreamSeekPositionType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechStreamSeekPositionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechStreamSeekPositionType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechTokenContext(pub u32);
impl windows_core::TypeKind for SpeechTokenContext {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechTokenContext {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechTokenContext").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechTokenShellFolder(pub i32);
impl windows_core::TypeKind for SpeechTokenShellFolder {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechTokenShellFolder {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechTokenShellFolder").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechVisemeFeature(pub i32);
impl windows_core::TypeKind for SpeechVisemeFeature {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechVisemeFeature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechVisemeFeature").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechVisemeType(pub i32);
impl windows_core::TypeKind for SpeechVisemeType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechVisemeType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechVisemeType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechVoiceEvents(pub i32);
impl windows_core::TypeKind for SpeechVoiceEvents {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechVoiceEvents {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechVoiceEvents").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechVoicePriority(pub i32);
impl windows_core::TypeKind for SpeechVoicePriority {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechVoicePriority {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechVoicePriority").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechVoiceSpeakFlags(pub i32);
impl windows_core::TypeKind for SpeechVoiceSpeakFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechVoiceSpeakFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechVoiceSpeakFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechWordPronounceable(pub i32);
impl windows_core::TypeKind for SpeechWordPronounceable {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechWordPronounceable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechWordPronounceable").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpeechWordType(pub i32);
impl windows_core::TypeKind for SpeechWordType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpeechWordType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpeechWordType").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPAUDIOBUFFERINFO {
    pub ulMsMinNotification: u32,
    pub ulMsBufferSize: u32,
    pub ulMsEventBias: u32,
}
impl windows_core::TypeKind for SPAUDIOBUFFERINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPAUDIOBUFFERINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPAUDIOSTATUS {
    pub cbFreeBuffSpace: i32,
    pub cbNonBlockingIO: u32,
    pub State: SPAUDIOSTATE,
    pub CurSeekPos: u64,
    pub CurDevicePos: u64,
    pub dwAudioLevel: u32,
    pub dwReserved2: u32,
}
impl windows_core::TypeKind for SPAUDIOSTATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPAUDIOSTATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPBINARYGRAMMAR {
    pub ulTotalSerializedSize: u32,
}
impl windows_core::TypeKind for SPBINARYGRAMMAR {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPBINARYGRAMMAR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPDISPLAYPHRASE {
    pub ulNumTokens: u32,
    pub pTokens: *mut SPDISPLAYTOKEN,
}
impl windows_core::TypeKind for SPDISPLAYPHRASE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPDISPLAYPHRASE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPDISPLAYTOKEN {
    pub pszLexical: windows_core::PCWSTR,
    pub pszDisplay: windows_core::PCWSTR,
    pub bDisplayAttributes: u8,
}
impl windows_core::TypeKind for SPDISPLAYTOKEN {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPDISPLAYTOKEN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPEVENT {
    pub _bitfield: i32,
    pub ulStreamNum: u32,
    pub ullAudioStreamOffset: u64,
    pub wParam: super::super::Foundation::WPARAM,
    pub lParam: super::super::Foundation::LPARAM,
}
impl windows_core::TypeKind for SPEVENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPEVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPEVENTEX {
    pub _bitfield: i32,
    pub ulStreamNum: u32,
    pub ullAudioStreamOffset: u64,
    pub wParam: super::super::Foundation::WPARAM,
    pub lParam: super::super::Foundation::LPARAM,
    pub ullAudioTimeOffset: u64,
}
impl windows_core::TypeKind for SPEVENTEX {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPEVENTEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPEVENTSOURCEINFO {
    pub ullEventInterest: u64,
    pub ullQueuedInterest: u64,
    pub ulCount: u32,
}
impl windows_core::TypeKind for SPEVENTSOURCEINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPEVENTSOURCEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SPGRAMMARHANDLE(pub isize);
impl SPGRAMMARHANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for SPGRAMMARHANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SPGRAMMARHANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPNORMALIZATIONLIST {
    pub ulSize: u32,
    pub ppszzNormalizedList: *mut *mut u16,
}
impl windows_core::TypeKind for SPNORMALIZATIONLIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPNORMALIZATIONLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPPARSEINFO {
    pub cbSize: u32,
    pub hRule: SPRULEHANDLE,
    pub ullAudioStreamPosition: u64,
    pub ulAudioSize: u32,
    pub cTransitions: u32,
    pub pPath: *mut SPPATHENTRY,
    pub SREngineID: windows_core::GUID,
    pub ulSREnginePrivateDataSize: u32,
    pub pSREnginePrivateData: *const u8,
    pub fHypothesis: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for SPPARSEINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPPARSEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SPPATHENTRY {
    pub hTransition: SPTRANSITIONID,
    pub elem: SPPHRASEELEMENT,
}
impl windows_core::TypeKind for SPPATHENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPPATHENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SPPHRASE {
    pub Base: SPPHRASE_50,
    pub pSML: windows_core::PWSTR,
    pub pSemanticErrorInfo: *mut SPSEMANTICERRORINFO,
}
impl windows_core::TypeKind for SPPHRASE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPPHRASE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct SPPHRASEALT {
    pub pPhrase: core::mem::ManuallyDrop<Option<ISpPhraseBuilder>>,
    pub ulStartElementInParent: u32,
    pub cElementsInParent: u32,
    pub cElementsInAlternate: u32,
    pub pvAltExtra: *mut core::ffi::c_void,
    pub cbAltExtra: u32,
}
impl Clone for SPPHRASEALT {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for SPPHRASEALT {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPPHRASEALT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct SPPHRASEALTREQUEST {
    pub ulStartElement: u32,
    pub cElements: u32,
    pub ulRequestAltCount: u32,
    pub pvResultExtra: *mut core::ffi::c_void,
    pub cbResultExtra: u32,
    pub pPhrase: core::mem::ManuallyDrop<Option<ISpPhrase>>,
    pub pRecoContext: core::mem::ManuallyDrop<Option<ISpRecoContext>>,
}
impl Clone for SPPHRASEALTREQUEST {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for SPPHRASEALTREQUEST {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPPHRASEALTREQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SPPHRASEELEMENT {
    pub ulAudioTimeOffset: u32,
    pub ulAudioSizeTime: u32,
    pub ulAudioStreamOffset: u32,
    pub ulAudioSizeBytes: u32,
    pub ulRetainedStreamOffset: u32,
    pub ulRetainedSizeBytes: u32,
    pub pszDisplayText: windows_core::PCWSTR,
    pub pszLexicalForm: windows_core::PCWSTR,
    pub pszPronunciation: *const u16,
    pub bDisplayAttributes: u8,
    pub RequiredConfidence: i8,
    pub ActualConfidence: i8,
    pub Reserved: u8,
    pub SREngineConfidence: f32,
}
impl windows_core::TypeKind for SPPHRASEELEMENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPPHRASEELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SPPHRASEPROPERTY {
    pub pszName: windows_core::PCWSTR,
    pub Anonymous: SPPHRASEPROPERTY_0,
    pub pszValue: windows_core::PCWSTR,
    pub vValue: core::mem::ManuallyDrop<windows_core::VARIANT>,
    pub ulFirstElement: u32,
    pub ulCountOfElements: u32,
    pub pNextSibling: *const SPPHRASEPROPERTY,
    pub pFirstChild: *const SPPHRASEPROPERTY,
    pub SREngineConfidence: f32,
    pub Confidence: i8,
}
impl Clone for SPPHRASEPROPERTY {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for SPPHRASEPROPERTY {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPPHRASEPROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SPPHRASEPROPERTY_0 {
    pub ulId: u32,
    pub Anonymous: SPPHRASEPROPERTY_0_0,
}
impl windows_core::TypeKind for SPPHRASEPROPERTY_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPPHRASEPROPERTY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPPHRASEPROPERTY_0_0 {
    pub bType: u8,
    pub bReserved: u8,
    pub usArrayIndex: u16,
}
impl windows_core::TypeKind for SPPHRASEPROPERTY_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPPHRASEPROPERTY_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SPPHRASEPROPERTYHANDLE(pub isize);
impl SPPHRASEPROPERTYHANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for SPPHRASEPROPERTYHANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SPPHRASEPROPERTYHANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPPHRASEREPLACEMENT {
    pub bDisplayAttributes: u8,
    pub pszReplacementText: windows_core::PCWSTR,
    pub ulFirstElement: u32,
    pub ulCountOfElements: u32,
}
impl windows_core::TypeKind for SPPHRASEREPLACEMENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPPHRASEREPLACEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SPPHRASERULE {
    pub pszName: windows_core::PCWSTR,
    pub ulId: u32,
    pub ulFirstElement: u32,
    pub ulCountOfElements: u32,
    pub pNextSibling: *const SPPHRASERULE,
    pub pFirstChild: *const SPPHRASERULE,
    pub SREngineConfidence: f32,
    pub Confidence: i8,
}
impl windows_core::TypeKind for SPPHRASERULE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPPHRASERULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SPPHRASERULEHANDLE(pub isize);
impl SPPHRASERULEHANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for SPPHRASERULEHANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SPPHRASERULEHANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SPPHRASE_50 {
    pub cbSize: u32,
    pub LangID: u16,
    pub wHomophoneGroupId: u16,
    pub ullGrammarID: u64,
    pub ftStartTime: u64,
    pub ullAudioStreamPosition: u64,
    pub ulAudioSizeBytes: u32,
    pub ulRetainedSizeBytes: u32,
    pub ulAudioSizeTime: u32,
    pub Rule: SPPHRASERULE,
    pub pProperties: *const SPPHRASEPROPERTY,
    pub pElements: *const SPPHRASEELEMENT,
    pub cReplacements: u32,
    pub pReplacements: *const SPPHRASEREPLACEMENT,
    pub SREngineID: windows_core::GUID,
    pub ulSREnginePrivateDataSize: u32,
    pub pSREnginePrivateData: *const u8,
}
impl windows_core::TypeKind for SPPHRASE_50 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPPHRASE_50 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct SPPROPERTYINFO {
    pub pszName: windows_core::PCWSTR,
    pub ulId: u32,
    pub pszValue: windows_core::PCWSTR,
    pub vValue: core::mem::ManuallyDrop<windows_core::VARIANT>,
}
impl Clone for SPPROPERTYINFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for SPPROPERTYINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPPROPERTYINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SPRECOCONTEXTHANDLE(pub isize);
impl SPRECOCONTEXTHANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for SPRECOCONTEXTHANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SPRECOCONTEXTHANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPRECOCONTEXTSTATUS {
    pub eInterference: SPINTERFERENCE,
    pub szRequestTypeOfUI: [u16; 255],
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl windows_core::TypeKind for SPRECOCONTEXTSTATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPRECOCONTEXTSTATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPRECOGNIZERSTATUS {
    pub AudioStatus: SPAUDIOSTATUS,
    pub ullRecognitionStreamPos: u64,
    pub ulStreamNumber: u32,
    pub ulNumActive: u32,
    pub clsidEngine: windows_core::GUID,
    pub cLangIDs: u32,
    pub aLangID: [u16; 20],
    pub ullRecognitionStreamTime: u64,
}
impl windows_core::TypeKind for SPRECOGNIZERSTATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPRECOGNIZERSTATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct SPRECORESULTINFO {
    pub cbSize: u32,
    pub eResultType: SPRESULTTYPE,
    pub fHypothesis: super::super::Foundation::BOOL,
    pub fProprietaryAutoPause: super::super::Foundation::BOOL,
    pub ullStreamPosStart: u64,
    pub ullStreamPosEnd: u64,
    pub hGrammar: SPGRAMMARHANDLE,
    pub ulSizeEngineData: u32,
    pub pvEngineData: *mut core::ffi::c_void,
    pub pPhrase: core::mem::ManuallyDrop<Option<ISpPhraseBuilder>>,
    pub aPhraseAlts: *mut SPPHRASEALT,
    pub ulNumAlts: u32,
}
impl Clone for SPRECORESULTINFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for SPRECORESULTINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPRECORESULTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct SPRECORESULTINFOEX {
    pub Base: SPRECORESULTINFO,
    pub ullStreamTimeStart: u64,
    pub ullStreamTimeEnd: u64,
}
impl Clone for SPRECORESULTINFOEX {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for SPRECORESULTINFOEX {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPRECORESULTINFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPRECORESULTTIMES {
    pub ftStreamTime: super::super::Foundation::FILETIME,
    pub ullLength: u64,
    pub dwTickCount: u32,
    pub ullStart: u64,
}
impl windows_core::TypeKind for SPRECORESULTTIMES {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPRECORESULTTIMES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPRULE {
    pub pszRuleName: windows_core::PCWSTR,
    pub ulRuleId: u32,
    pub dwAttributes: u32,
}
impl windows_core::TypeKind for SPRULE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPRULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPRULEENTRY {
    pub hRule: SPRULEHANDLE,
    pub hInitialState: SPSTATEHANDLE,
    pub Attributes: u32,
    pub pvClientRuleContext: *mut core::ffi::c_void,
    pub pvClientGrammarContext: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for SPRULEENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPRULEENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SPRULEHANDLE(pub isize);
impl SPRULEHANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for SPRULEHANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SPRULEHANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPSEMANTICERRORINFO {
    pub ulLineNumber: u32,
    pub pszScriptLine: windows_core::PWSTR,
    pub pszSource: windows_core::PWSTR,
    pub pszDescription: windows_core::PWSTR,
    pub hrResultCode: windows_core::HRESULT,
}
impl windows_core::TypeKind for SPSEMANTICERRORINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPSEMANTICERRORINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPSERIALIZEDEVENT {
    pub _bitfield: i32,
    pub ulStreamNum: u32,
    pub ullAudioStreamOffset: u64,
    pub SerializedwParam: u32,
    pub SerializedlParam: i32,
}
impl windows_core::TypeKind for SPSERIALIZEDEVENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPSERIALIZEDEVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPSERIALIZEDEVENT64 {
    pub _bitfield: i32,
    pub ulStreamNum: u32,
    pub ullAudioStreamOffset: u64,
    pub SerializedwParam: u64,
    pub SerializedlParam: i64,
}
impl windows_core::TypeKind for SPSERIALIZEDEVENT64 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPSERIALIZEDEVENT64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPSERIALIZEDPHRASE {
    pub ulSerializedSize: u32,
}
impl windows_core::TypeKind for SPSERIALIZEDPHRASE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPSERIALIZEDPHRASE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPSERIALIZEDRESULT {
    pub ulSerializedSize: u32,
}
impl windows_core::TypeKind for SPSERIALIZEDRESULT {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPSERIALIZEDRESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPSHORTCUTPAIR {
    pub pNextSHORTCUTPAIR: *mut SPSHORTCUTPAIR,
    pub LangID: u16,
    pub shType: SPSHORTCUTTYPE,
    pub pszDisplay: windows_core::PWSTR,
    pub pszSpoken: windows_core::PWSTR,
}
impl windows_core::TypeKind for SPSHORTCUTPAIR {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPSHORTCUTPAIR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPSHORTCUTPAIRLIST {
    pub ulSize: u32,
    pub pvBuffer: *mut u8,
    pub pFirstShortcutPair: *mut SPSHORTCUTPAIR,
}
impl windows_core::TypeKind for SPSHORTCUTPAIRLIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPSHORTCUTPAIRLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SPSTATEHANDLE(pub isize);
impl SPSTATEHANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for SPSTATEHANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SPSTATEHANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPSTATEINFO {
    pub cAllocatedEntries: u32,
    pub pTransitions: *mut SPTRANSITIONENTRY,
    pub cEpsilons: u32,
    pub cRules: u32,
    pub cWords: u32,
    pub cSpecialTransitions: u32,
}
impl windows_core::TypeKind for SPSTATEINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPSTATEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPTEXTSELECTIONINFO {
    pub ulStartActiveOffset: u32,
    pub cchActiveChars: u32,
    pub ulStartSelection: u32,
    pub cchSelection: u32,
}
impl windows_core::TypeKind for SPTEXTSELECTIONINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPTEXTSELECTIONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPTMTHREADINFO {
    pub lPoolSize: i32,
    pub lPriority: i32,
    pub ulConcurrencyLimit: u32,
    pub ulMaxQuickAllocThreads: u32,
}
impl windows_core::TypeKind for SPTMTHREADINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPTMTHREADINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SPTRANSITIONENTRY {
    pub ID: SPTRANSITIONID,
    pub hNextState: SPSTATEHANDLE,
    pub Type: u8,
    pub RequiredConfidence: u8,
    pub Anonymous1: SPTRANSITIONENTRY_0,
    pub Weight: f32,
    pub Anonymous2: SPTRANSITIONENTRY_1,
}
impl windows_core::TypeKind for SPTRANSITIONENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPTRANSITIONENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPTRANSITIONENTRY_0 {
    pub fHasProperty: u32,
}
impl windows_core::TypeKind for SPTRANSITIONENTRY_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPTRANSITIONENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SPTRANSITIONENTRY_1 {
    pub Anonymous1: SPTRANSITIONENTRY_1_0,
    pub Anonymous2: SPTRANSITIONENTRY_1_1,
    pub Anonymous3: SPTRANSITIONENTRY_1_2,
}
impl windows_core::TypeKind for SPTRANSITIONENTRY_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPTRANSITIONENTRY_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPTRANSITIONENTRY_1_0 {
    pub hRuleInitialState: SPSTATEHANDLE,
    pub hRule: SPRULEHANDLE,
    pub pvClientRuleContext: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for SPTRANSITIONENTRY_1_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPTRANSITIONENTRY_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPTRANSITIONENTRY_1_1 {
    pub hWord: SPWORDHANDLE,
    pub pvClientWordContext: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for SPTRANSITIONENTRY_1_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPTRANSITIONENTRY_1_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPTRANSITIONENTRY_1_2 {
    pub pvGrammarCookie: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for SPTRANSITIONENTRY_1_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPTRANSITIONENTRY_1_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SPTRANSITIONID(pub isize);
impl SPTRANSITIONID {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for SPTRANSITIONID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SPTRANSITIONID {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct SPTRANSITIONPROPERTY {
    pub pszName: windows_core::PCWSTR,
    pub ulId: u32,
    pub pszValue: windows_core::PCWSTR,
    pub vValue: core::mem::ManuallyDrop<windows_core::VARIANT>,
}
impl Clone for SPTRANSITIONPROPERTY {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for SPTRANSITIONPROPERTY {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPTRANSITIONPROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPVCONTEXT {
    pub pCategory: windows_core::PCWSTR,
    pub pBefore: windows_core::PCWSTR,
    pub pAfter: windows_core::PCWSTR,
}
impl windows_core::TypeKind for SPVCONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPVCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPVOICESTATUS {
    pub ulCurrentStream: u32,
    pub ulLastStreamQueued: u32,
    pub hrLastResult: windows_core::HRESULT,
    pub dwRunningState: u32,
    pub ulInputWordPos: u32,
    pub ulInputWordLen: u32,
    pub ulInputSentPos: u32,
    pub ulInputSentLen: u32,
    pub lBookmarkId: i32,
    pub PhonemeId: u16,
    pub VisemeId: SPVISEMES,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl windows_core::TypeKind for SPVOICESTATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPVOICESTATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPVPITCH {
    pub MiddleAdj: i32,
    pub RangeAdj: i32,
}
impl windows_core::TypeKind for SPVPITCH {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPVPITCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPVSTATE {
    pub eAction: SPVACTIONS,
    pub LangID: u16,
    pub wReserved: u16,
    pub EmphAdj: i32,
    pub RateAdj: i32,
    pub Volume: u32,
    pub PitchAdj: SPVPITCH,
    pub SilenceMSecs: u32,
    pub pPhoneIds: *mut u16,
    pub ePartOfSpeech: SPPARTOFSPEECH,
    pub Context: SPVCONTEXT,
}
impl windows_core::TypeKind for SPVSTATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPVSTATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPVTEXTFRAG {
    pub pNext: *mut SPVTEXTFRAG,
    pub State: SPVSTATE,
    pub pTextStart: windows_core::PCWSTR,
    pub ulTextLen: u32,
    pub ulTextSrcOffset: u32,
}
impl windows_core::TypeKind for SPVTEXTFRAG {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPVTEXTFRAG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPWORD {
    pub pNextWord: *mut SPWORD,
    pub LangID: u16,
    pub wReserved: u16,
    pub eWordType: SPWORDTYPE,
    pub pszWord: windows_core::PWSTR,
    pub pFirstWordPronunciation: *mut SPWORDPRONUNCIATION,
}
impl windows_core::TypeKind for SPWORD {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPWORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPWORDENTRY {
    pub hWord: SPWORDHANDLE,
    pub LangID: u16,
    pub pszDisplayText: windows_core::PWSTR,
    pub pszLexicalForm: windows_core::PWSTR,
    pub aPhoneId: *mut u16,
    pub pvClientContext: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for SPWORDENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPWORDENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SPWORDHANDLE(pub isize);
impl SPWORDHANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for SPWORDHANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SPWORDHANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPWORDLIST {
    pub ulSize: u32,
    pub pvBuffer: *mut u8,
    pub pFirstWord: *mut SPWORD,
}
impl windows_core::TypeKind for SPWORDLIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPWORDLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPWORDPRONUNCIATION {
    pub pNextWordPronunciation: *mut SPWORDPRONUNCIATION,
    pub eLexiconType: SPLEXICONTYPE,
    pub LangID: u16,
    pub wPronunciationFlags: u16,
    pub ePartOfSpeech: SPPARTOFSPEECH,
    pub szPronunciation: [u16; 1],
}
impl windows_core::TypeKind for SPWORDPRONUNCIATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPWORDPRONUNCIATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SPWORDPRONUNCIATIONLIST {
    pub ulSize: u32,
    pub pvBuffer: *mut u8,
    pub pFirstWordPronunciation: *mut SPWORDPRONUNCIATION,
}
impl windows_core::TypeKind for SPWORDPRONUNCIATIONLIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for SPWORDPRONUNCIATIONLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SpAudioFormat: windows_core::GUID = windows_core::GUID::from_u128(0x9ef96870_e160_4792_820d_48cf0649e4ec);
pub const SpCompressedLexicon: windows_core::GUID = windows_core::GUID::from_u128(0x90903716_2f42_11d3_9c26_00c04f8ef87c);
pub const SpCustomStream: windows_core::GUID = windows_core::GUID::from_u128(0x8dbef13f_1948_4aa8_8cf0_048eebed95d8);
pub const SpDataKey: windows_core::GUID = windows_core::GUID::from_u128(0xd9f6ee60_58c9_458b_88e1_2f908fd7f87c);
pub const SpFileStream: windows_core::GUID = windows_core::GUID::from_u128(0x947812b3_2ae1_4644_ba86_9e90ded7ec91);
pub const SpGramCompBackend: windows_core::GUID = windows_core::GUID::from_u128(0xda93e903_c843_11d2_a084_00c04f8ef9b5);
pub const SpGrammarCompiler: windows_core::GUID = windows_core::GUID::from_u128(0xb1e29d59_a675_11d2_8302_00c04f8ee6c0);
pub const SpITNProcessor: windows_core::GUID = windows_core::GUID::from_u128(0x12d73610_a1c9_11d3_bc90_00c04f72df9f);
pub const SpInProcRecoContext: windows_core::GUID = windows_core::GUID::from_u128(0x73ad6842_ace0_45e8_a4dd_8795881a2c2a);
pub const SpInprocRecognizer: windows_core::GUID = windows_core::GUID::from_u128(0x41b89b6b_9399_11d2_9623_00c04f8ee628);
pub const SpLexicon: windows_core::GUID = windows_core::GUID::from_u128(0x0655e396_25d0_11d3_9c26_00c04f8ef87c);
pub const SpMMAudioEnum: windows_core::GUID = windows_core::GUID::from_u128(0xab1890a0_e91f_11d2_bb91_00c04f8ee6c0);
pub const SpMMAudioIn: windows_core::GUID = windows_core::GUID::from_u128(0xcf3d2e50_53f2_11d2_960c_00c04f8ee628);
pub const SpMMAudioOut: windows_core::GUID = windows_core::GUID::from_u128(0xa8c680eb_3d32_11d2_9ee7_00c04f797396);
pub const SpMemoryStream: windows_core::GUID = windows_core::GUID::from_u128(0x5fb7ef7d_dff4_468a_b6b7_2fcbd188f994);
pub const SpNotifyTranslator: windows_core::GUID = windows_core::GUID::from_u128(0xe2ae5372_5d40_11d2_960e_00c04f8ee628);
pub const SpNullPhoneConverter: windows_core::GUID = windows_core::GUID::from_u128(0x455f24e9_7396_4a16_9715_7c0fdbe3efe3);
pub const SpObjectToken: windows_core::GUID = windows_core::GUID::from_u128(0xef411752_3736_4cb4_9c8c_8ef4ccb58efe);
pub const SpObjectTokenCategory: windows_core::GUID = windows_core::GUID::from_u128(0xa910187f_0c7a_45ac_92cc_59edafb77b53);
pub const SpObjectTokenEnum: windows_core::GUID = windows_core::GUID::from_u128(0x3918d75f_0acb_41f2_b733_92aa15bcecf6);
pub const SpPhoneConverter: windows_core::GUID = windows_core::GUID::from_u128(0x9185f743_1143_4c28_86b5_bff14f20e5c8);
pub const SpPhoneticAlphabetConverter: windows_core::GUID = windows_core::GUID::from_u128(0x4f414126_dfe3_4629_99ee_797978317ead);
pub const SpPhraseBuilder: windows_core::GUID = windows_core::GUID::from_u128(0x777b6bbd_2ff2_11d3_88fe_00c04f8ef9b5);
pub const SpPhraseInfoBuilder: windows_core::GUID = windows_core::GUID::from_u128(0xc23fc28d_c55f_4720_8b32_91f73c2bd5d1);
pub const SpResourceManager: windows_core::GUID = windows_core::GUID::from_u128(0x96749373_3391_11d2_9ee3_00c04f797396);
pub const SpSharedRecoContext: windows_core::GUID = windows_core::GUID::from_u128(0x47206204_5eca_11d2_960f_00c04f8ee628);
pub const SpSharedRecognizer: windows_core::GUID = windows_core::GUID::from_u128(0x3bee4890_4fe9_4a37_8c1e_5e7e12791c1f);
pub const SpShortcut: windows_core::GUID = windows_core::GUID::from_u128(0x0d722f1a_9fcf_4e62_96d8_6df8f01a26aa);
pub const SpStream: windows_core::GUID = windows_core::GUID::from_u128(0x715d9c59_4442_11d2_9605_00c04f8ee628);
pub const SpStreamFormatConverter: windows_core::GUID = windows_core::GUID::from_u128(0x7013943a_e2ec_11d2_a086_00c04f8ef9b5);
pub const SpTextSelectionInformation: windows_core::GUID = windows_core::GUID::from_u128(0x0f92030a_cbfd_4ab8_a164_ff5985547ff6);
pub const SpUnCompressedLexicon: windows_core::GUID = windows_core::GUID::from_u128(0xc9e37c15_df92_4727_85d6_72e5eeb6995a);
pub const SpVoice: windows_core::GUID = windows_core::GUID::from_u128(0x96749377_3391_11d2_9ee3_00c04f797396);
pub const SpW3CGrammarCompiler: windows_core::GUID = windows_core::GUID::from_u128(0xd2c13906_51ef_454e_bc67_a52475ff074c);
pub const SpWaveFormatEx: windows_core::GUID = windows_core::GUID::from_u128(0xc79a574c_63be_44b9_801f_283f87f898be);
pub type SPNOTIFYCALLBACK = Option<unsafe extern "system" fn(wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM)>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
