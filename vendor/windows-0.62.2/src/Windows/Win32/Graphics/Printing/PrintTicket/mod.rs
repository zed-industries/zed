#[inline]
pub unsafe fn PTCloseProvider(hprovider: HPTPROVIDER) -> windows_core::Result<()> {
    windows_core::link!("prntvpt.dll" "system" fn PTCloseProvider(hprovider : HPTPROVIDER) -> windows_core::HRESULT);
    unsafe { PTCloseProvider(hprovider).ok() }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn PTConvertDevModeToPrintTicket<P4>(hprovider: HPTPROVIDER, cbdevmode: u32, pdevmode: *const super::super::Gdi::DEVMODEA, scope: EPrintTicketScope, pprintticket: P4) -> windows_core::Result<()>
where
    P4: windows_core::Param<super::super::super::System::Com::IStream>,
{
    windows_core::link!("prntvpt.dll" "system" fn PTConvertDevModeToPrintTicket(hprovider : HPTPROVIDER, cbdevmode : u32, pdevmode : *const super::super::Gdi:: DEVMODEA, scope : EPrintTicketScope, pprintticket : * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { PTConvertDevModeToPrintTicket(hprovider, cbdevmode, pdevmode, scope, pprintticket.param().abi()).ok() }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn PTConvertPrintTicketToDevMode<P1>(hprovider: HPTPROVIDER, pprintticket: P1, basedevmodetype: EDefaultDevmodeType, scope: EPrintTicketScope, pcbdevmode: *mut u32, ppdevmode: *mut *mut super::super::Gdi::DEVMODEA, pbstrerrormessage: Option<*mut windows_core::BSTR>) -> windows_core::Result<()>
where
    P1: windows_core::Param<super::super::super::System::Com::IStream>,
{
    windows_core::link!("prntvpt.dll" "system" fn PTConvertPrintTicketToDevMode(hprovider : HPTPROVIDER, pprintticket : * mut core::ffi::c_void, basedevmodetype : EDefaultDevmodeType, scope : EPrintTicketScope, pcbdevmode : *mut u32, ppdevmode : *mut *mut super::super::Gdi:: DEVMODEA, pbstrerrormessage : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { PTConvertPrintTicketToDevMode(hprovider, pprintticket.param().abi(), basedevmodetype, scope, pcbdevmode as _, ppdevmode as _, pbstrerrormessage.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn PTGetPrintCapabilities<P1, P2>(hprovider: HPTPROVIDER, pprintticket: P1, pcapabilities: P2, pbstrerrormessage: Option<*mut windows_core::BSTR>) -> windows_core::Result<()>
where
    P1: windows_core::Param<super::super::super::System::Com::IStream>,
    P2: windows_core::Param<super::super::super::System::Com::IStream>,
{
    windows_core::link!("prntvpt.dll" "system" fn PTGetPrintCapabilities(hprovider : HPTPROVIDER, pprintticket : * mut core::ffi::c_void, pcapabilities : * mut core::ffi::c_void, pbstrerrormessage : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { PTGetPrintCapabilities(hprovider, pprintticket.param().abi(), pcapabilities.param().abi(), pbstrerrormessage.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn PTGetPrintDeviceCapabilities<P1, P2>(hprovider: HPTPROVIDER, pprintticket: P1, pdevicecapabilities: P2, pbstrerrormessage: Option<*mut windows_core::BSTR>) -> windows_core::Result<()>
where
    P1: windows_core::Param<super::super::super::System::Com::IStream>,
    P2: windows_core::Param<super::super::super::System::Com::IStream>,
{
    windows_core::link!("prntvpt.dll" "system" fn PTGetPrintDeviceCapabilities(hprovider : HPTPROVIDER, pprintticket : * mut core::ffi::c_void, pdevicecapabilities : * mut core::ffi::c_void, pbstrerrormessage : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { PTGetPrintDeviceCapabilities(hprovider, pprintticket.param().abi(), pdevicecapabilities.param().abi(), pbstrerrormessage.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn PTGetPrintDeviceResources<P1, P2, P3>(hprovider: HPTPROVIDER, pszlocalename: P1, pprintticket: P2, pdeviceresources: P3, pbstrerrormessage: Option<*mut windows_core::BSTR>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::super::System::Com::IStream>,
    P3: windows_core::Param<super::super::super::System::Com::IStream>,
{
    windows_core::link!("prntvpt.dll" "system" fn PTGetPrintDeviceResources(hprovider : HPTPROVIDER, pszlocalename : windows_core::PCWSTR, pprintticket : * mut core::ffi::c_void, pdeviceresources : * mut core::ffi::c_void, pbstrerrormessage : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { PTGetPrintDeviceResources(hprovider, pszlocalename.param().abi(), pprintticket.param().abi(), pdeviceresources.param().abi(), pbstrerrormessage.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn PTMergeAndValidatePrintTicket<P1, P2, P4>(hprovider: HPTPROVIDER, pbaseticket: P1, pdeltaticket: P2, scope: EPrintTicketScope, presultticket: P4, pbstrerrormessage: Option<*mut windows_core::BSTR>) -> windows_core::Result<()>
where
    P1: windows_core::Param<super::super::super::System::Com::IStream>,
    P2: windows_core::Param<super::super::super::System::Com::IStream>,
    P4: windows_core::Param<super::super::super::System::Com::IStream>,
{
    windows_core::link!("prntvpt.dll" "system" fn PTMergeAndValidatePrintTicket(hprovider : HPTPROVIDER, pbaseticket : * mut core::ffi::c_void, pdeltaticket : * mut core::ffi::c_void, scope : EPrintTicketScope, presultticket : * mut core::ffi::c_void, pbstrerrormessage : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { PTMergeAndValidatePrintTicket(hprovider, pbaseticket.param().abi(), pdeltaticket.param().abi(), scope, presultticket.param().abi(), pbstrerrormessage.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn PTOpenProvider<P0>(pszprintername: P0, dwversion: u32) -> windows_core::Result<HPTPROVIDER>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("prntvpt.dll" "system" fn PTOpenProvider(pszprintername : windows_core::PCWSTR, dwversion : u32, phprovider : *mut HPTPROVIDER) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PTOpenProvider(pszprintername.param().abi(), dwversion, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn PTOpenProviderEx<P0>(pszprintername: P0, dwmaxversion: u32, dwprefversion: u32, phprovider: *mut HPTPROVIDER, pusedversion: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("prntvpt.dll" "system" fn PTOpenProviderEx(pszprintername : windows_core::PCWSTR, dwmaxversion : u32, dwprefversion : u32, phprovider : *mut HPTPROVIDER, pusedversion : *mut u32) -> windows_core::HRESULT);
    unsafe { PTOpenProviderEx(pszprintername.param().abi(), dwmaxversion, dwprefversion, phprovider as _, pusedversion as _).ok() }
}
#[inline]
pub unsafe fn PTQuerySchemaVersionSupport<P0>(pszprintername: P0) -> windows_core::Result<u32>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("prntvpt.dll" "system" fn PTQuerySchemaVersionSupport(pszprintername : windows_core::PCWSTR, pmaxversion : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PTQuerySchemaVersionSupport(pszprintername.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn PTReleaseMemory(pbuffer: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("prntvpt.dll" "system" fn PTReleaseMemory(pbuffer : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { PTReleaseMemory(pbuffer).ok() }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EDefaultDevmodeType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EPrintTicketScope(pub i32);
pub const E_DELTA_PRINTTICKET_FORMAT: u32 = 2147745797u32;
pub const E_PRINTCAPABILITIES_FORMAT: u32 = 2147745796u32;
pub const E_PRINTDEVICECAPABILITIES_FORMAT: u32 = 2147745798u32;
pub const E_PRINTTICKET_FORMAT: u32 = 2147745795u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HPTPROVIDER(pub *mut core::ffi::c_void);
impl HPTPROVIDER {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HPTPROVIDER {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("prntvpt.dll" "system" fn PTCloseProvider(hprovider : *mut core::ffi::c_void) -> i32);
            unsafe {
                PTCloseProvider(self.0);
            }
        }
    }
}
impl Default for HPTPROVIDER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRINTTICKET_ISTREAM_APIS: u32 = 1u32;
pub const S_PT_CONFLICT_RESOLVED: u32 = 262146u32;
pub const S_PT_NO_CONFLICT: u32 = 262145u32;
pub const kPTDocumentScope: EPrintTicketScope = EPrintTicketScope(1i32);
pub const kPTJobScope: EPrintTicketScope = EPrintTicketScope(2i32);
pub const kPTPageScope: EPrintTicketScope = EPrintTicketScope(0i32);
pub const kPrinterDefaultDevmode: EDefaultDevmodeType = EDefaultDevmodeType(1i32);
pub const kUserDefaultDevmode: EDefaultDevmodeType = EDefaultDevmodeType(0i32);
