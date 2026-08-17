#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateXmlReader<P2>(riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void, pmalloc: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<super::super::super::System::Com::IMalloc>,
{
    windows_core::link!("xmllite.dll" "system" fn CreateXmlReader(riid : *const windows_core::GUID, ppvobject : *mut *mut core::ffi::c_void, pmalloc : * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CreateXmlReader(riid, ppvobject as _, pmalloc.param().abi()).ok() }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateXmlReaderInputWithEncodingCodePage<P0, P1, P4>(pinputstream: P0, pmalloc: P1, nencodingcodepage: u32, fencodinghint: bool, pwszbaseuri: P4) -> windows_core::Result<windows_core::IUnknown>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<super::super::super::System::Com::IMalloc>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("xmllite.dll" "system" fn CreateXmlReaderInputWithEncodingCodePage(pinputstream : * mut core::ffi::c_void, pmalloc : * mut core::ffi::c_void, nencodingcodepage : u32, fencodinghint : windows_core::BOOL, pwszbaseuri : windows_core::PCWSTR, ppinput : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateXmlReaderInputWithEncodingCodePage(pinputstream.param().abi(), pmalloc.param().abi(), nencodingcodepage, fencodinghint.into(), pwszbaseuri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateXmlReaderInputWithEncodingName<P0, P1, P2, P4>(pinputstream: P0, pmalloc: P1, pwszencodingname: P2, fencodinghint: bool, pwszbaseuri: P4) -> windows_core::Result<windows_core::IUnknown>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<super::super::super::System::Com::IMalloc>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("xmllite.dll" "system" fn CreateXmlReaderInputWithEncodingName(pinputstream : * mut core::ffi::c_void, pmalloc : * mut core::ffi::c_void, pwszencodingname : windows_core::PCWSTR, fencodinghint : windows_core::BOOL, pwszbaseuri : windows_core::PCWSTR, ppinput : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateXmlReaderInputWithEncodingName(pinputstream.param().abi(), pmalloc.param().abi(), pwszencodingname.param().abi(), fencodinghint.into(), pwszbaseuri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateXmlWriter<P2>(riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void, pmalloc: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<super::super::super::System::Com::IMalloc>,
{
    windows_core::link!("xmllite.dll" "system" fn CreateXmlWriter(riid : *const windows_core::GUID, ppvobject : *mut *mut core::ffi::c_void, pmalloc : * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CreateXmlWriter(riid, ppvobject as _, pmalloc.param().abi()).ok() }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateXmlWriterOutputWithEncodingCodePage<P0, P1>(poutputstream: P0, pmalloc: P1, nencodingcodepage: u32) -> windows_core::Result<windows_core::IUnknown>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<super::super::super::System::Com::IMalloc>,
{
    windows_core::link!("xmllite.dll" "system" fn CreateXmlWriterOutputWithEncodingCodePage(poutputstream : * mut core::ffi::c_void, pmalloc : * mut core::ffi::c_void, nencodingcodepage : u32, ppoutput : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateXmlWriterOutputWithEncodingCodePage(poutputstream.param().abi(), pmalloc.param().abi(), nencodingcodepage, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateXmlWriterOutputWithEncodingName<P0, P1, P2>(poutputstream: P0, pmalloc: P1, pwszencodingname: P2) -> windows_core::Result<windows_core::IUnknown>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<super::super::super::System::Com::IMalloc>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("xmllite.dll" "system" fn CreateXmlWriterOutputWithEncodingName(poutputstream : * mut core::ffi::c_void, pmalloc : * mut core::ffi::c_void, pwszencodingname : windows_core::PCWSTR, ppoutput : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateXmlWriterOutputWithEncodingName(poutputstream.param().abi(), pmalloc.param().abi(), pwszencodingname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DtdProcessing(pub i32);
pub const DtdProcessing_Parse: DtdProcessing = DtdProcessing(1i32);
pub const DtdProcessing_Prohibit: DtdProcessing = DtdProcessing(0i32);
windows_core::imp::define_interface!(IXmlReader, IXmlReader_Vtbl, 0x7279fc81_709d_4095_b63d_69fe4b0d9030);
windows_core::imp::interface_hierarchy!(IXmlReader, windows_core::IUnknown);
impl IXmlReader {
    pub unsafe fn SetInput<P0>(&self, pinput: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInput)(windows_core::Interface::as_raw(self), pinput.param().abi()).ok() }
    }
    pub unsafe fn GetProperty(&self, nproperty: u32) -> windows_core::Result<isize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), nproperty, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetProperty(&self, nproperty: u32, pvalue: Option<isize>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), nproperty, pvalue.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Read(&self, pnodetype: Option<*mut XmlNodeType>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), pnodetype.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetNodeType(&self) -> windows_core::Result<XmlNodeType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNodeType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveToFirstAttribute(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MoveToFirstAttribute)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn MoveToNextAttribute(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MoveToNextAttribute)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn MoveToAttributeByName<P0, P1>(&self, pwszlocalname: P0, pwsznamespaceuri: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).MoveToAttributeByName)(windows_core::Interface::as_raw(self), pwszlocalname.param().abi(), pwsznamespaceuri.param().abi()) }
    }
    pub unsafe fn MoveToElement(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MoveToElement)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetQualifiedName(&self, ppwszqualifiedname: *mut windows_core::PCWSTR, pcwchqualifiedname: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetQualifiedName)(windows_core::Interface::as_raw(self), ppwszqualifiedname as _, pcwchqualifiedname.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetNamespaceUri(&self, ppwsznamespaceuri: *mut windows_core::PCWSTR, pcwchnamespaceuri: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetNamespaceUri)(windows_core::Interface::as_raw(self), ppwsznamespaceuri as _, pcwchnamespaceuri.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetLocalName(&self, ppwszlocalname: *mut windows_core::PCWSTR, pcwchlocalname: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLocalName)(windows_core::Interface::as_raw(self), ppwszlocalname as _, pcwchlocalname.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetPrefix(&self, ppwszprefix: *mut windows_core::PCWSTR, pcwchprefix: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPrefix)(windows_core::Interface::as_raw(self), ppwszprefix as _, pcwchprefix.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetValue(&self, ppwszvalue: *mut windows_core::PCWSTR, pcwchvalue: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), ppwszvalue as _, pcwchvalue.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn ReadValueChunk(&self, pwchbuffer: &mut [u16], pcwchread: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReadValueChunk)(windows_core::Interface::as_raw(self), core::mem::transmute(pwchbuffer.as_ptr()), pwchbuffer.len().try_into().unwrap(), pcwchread as _) }
    }
    pub unsafe fn GetBaseUri(&self, ppwszbaseuri: *mut windows_core::PCWSTR, pcwchbaseuri: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBaseUri)(windows_core::Interface::as_raw(self), ppwszbaseuri as _, pcwchbaseuri.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn IsDefault(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsDefault)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsEmptyElement(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsEmptyElement)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetLineNumber(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLineNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLinePosition(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLinePosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAttributeCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttributeCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDepth(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDepth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsEOF(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsEOF)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut isize) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, u32, isize) -> windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XmlNodeType) -> windows_core::HRESULT,
    pub GetNodeType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XmlNodeType) -> windows_core::HRESULT,
    pub MoveToFirstAttribute: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoveToNextAttribute: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoveToAttributeByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub MoveToElement: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetQualifiedName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetNamespaceUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetLocalName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetPrefix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub ReadValueChunk: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub GetBaseUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub IsDefault: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub IsEmptyElement: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetLineNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetLinePosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAttributeCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDepth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsEOF: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
pub trait IXmlReader_Impl: windows_core::IUnknownImpl {
    fn SetInput(&self, pinput: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetProperty(&self, nproperty: u32) -> windows_core::Result<isize>;
    fn SetProperty(&self, nproperty: u32, pvalue: isize) -> windows_core::Result<()>;
    fn Read(&self, pnodetype: *mut XmlNodeType) -> windows_core::HRESULT;
    fn GetNodeType(&self) -> windows_core::Result<XmlNodeType>;
    fn MoveToFirstAttribute(&self) -> windows_core::HRESULT;
    fn MoveToNextAttribute(&self) -> windows_core::HRESULT;
    fn MoveToAttributeByName(&self, pwszlocalname: &windows_core::PCWSTR, pwsznamespaceuri: &windows_core::PCWSTR) -> windows_core::HRESULT;
    fn MoveToElement(&self) -> windows_core::Result<()>;
    fn GetQualifiedName(&self, ppwszqualifiedname: *mut windows_core::PCWSTR, pcwchqualifiedname: *mut u32) -> windows_core::Result<()>;
    fn GetNamespaceUri(&self, ppwsznamespaceuri: *mut windows_core::PCWSTR, pcwchnamespaceuri: *mut u32) -> windows_core::Result<()>;
    fn GetLocalName(&self, ppwszlocalname: *mut windows_core::PCWSTR, pcwchlocalname: *mut u32) -> windows_core::Result<()>;
    fn GetPrefix(&self, ppwszprefix: *mut windows_core::PCWSTR, pcwchprefix: *mut u32) -> windows_core::Result<()>;
    fn GetValue(&self, ppwszvalue: *mut windows_core::PCWSTR, pcwchvalue: *mut u32) -> windows_core::Result<()>;
    fn ReadValueChunk(&self, pwchbuffer: windows_core::PWSTR, cwchchunksize: u32, pcwchread: *mut u32) -> windows_core::HRESULT;
    fn GetBaseUri(&self, ppwszbaseuri: *mut windows_core::PCWSTR, pcwchbaseuri: *mut u32) -> windows_core::Result<()>;
    fn IsDefault(&self) -> windows_core::BOOL;
    fn IsEmptyElement(&self) -> windows_core::BOOL;
    fn GetLineNumber(&self) -> windows_core::Result<u32>;
    fn GetLinePosition(&self) -> windows_core::Result<u32>;
    fn GetAttributeCount(&self) -> windows_core::Result<u32>;
    fn GetDepth(&self) -> windows_core::Result<u32>;
    fn IsEOF(&self) -> windows_core::BOOL;
}
impl IXmlReader_Vtbl {
    pub const fn new<Identity: IXmlReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetInput<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinput: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlReader_Impl::SetInput(this, core::mem::transmute_copy(&pinput)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXmlReader_Impl::GetProperty(this, core::mem::transmute_copy(&nproperty)) {
                    Ok(ok__) => {
                        ppvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nproperty: u32, pvalue: isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlReader_Impl::SetProperty(this, core::mem::transmute_copy(&nproperty), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn Read<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnodetype: *mut XmlNodeType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlReader_Impl::Read(this, core::mem::transmute_copy(&pnodetype))
            }
        }
        unsafe extern "system" fn GetNodeType<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnodetype: *mut XmlNodeType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXmlReader_Impl::GetNodeType(this) {
                    Ok(ok__) => {
                        pnodetype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveToFirstAttribute<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlReader_Impl::MoveToFirstAttribute(this)
            }
        }
        unsafe extern "system" fn MoveToNextAttribute<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlReader_Impl::MoveToNextAttribute(this)
            }
        }
        unsafe extern "system" fn MoveToAttributeByName<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszlocalname: windows_core::PCWSTR, pwsznamespaceuri: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlReader_Impl::MoveToAttributeByName(this, core::mem::transmute(&pwszlocalname), core::mem::transmute(&pwsznamespaceuri))
            }
        }
        unsafe extern "system" fn MoveToElement<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlReader_Impl::MoveToElement(this).into()
            }
        }
        unsafe extern "system" fn GetQualifiedName<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszqualifiedname: *mut windows_core::PCWSTR, pcwchqualifiedname: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlReader_Impl::GetQualifiedName(this, core::mem::transmute_copy(&ppwszqualifiedname), core::mem::transmute_copy(&pcwchqualifiedname)).into()
            }
        }
        unsafe extern "system" fn GetNamespaceUri<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwsznamespaceuri: *mut windows_core::PCWSTR, pcwchnamespaceuri: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlReader_Impl::GetNamespaceUri(this, core::mem::transmute_copy(&ppwsznamespaceuri), core::mem::transmute_copy(&pcwchnamespaceuri)).into()
            }
        }
        unsafe extern "system" fn GetLocalName<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszlocalname: *mut windows_core::PCWSTR, pcwchlocalname: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlReader_Impl::GetLocalName(this, core::mem::transmute_copy(&ppwszlocalname), core::mem::transmute_copy(&pcwchlocalname)).into()
            }
        }
        unsafe extern "system" fn GetPrefix<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszprefix: *mut windows_core::PCWSTR, pcwchprefix: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlReader_Impl::GetPrefix(this, core::mem::transmute_copy(&ppwszprefix), core::mem::transmute_copy(&pcwchprefix)).into()
            }
        }
        unsafe extern "system" fn GetValue<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszvalue: *mut windows_core::PCWSTR, pcwchvalue: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlReader_Impl::GetValue(this, core::mem::transmute_copy(&ppwszvalue), core::mem::transmute_copy(&pcwchvalue)).into()
            }
        }
        unsafe extern "system" fn ReadValueChunk<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchbuffer: windows_core::PWSTR, cwchchunksize: u32, pcwchread: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlReader_Impl::ReadValueChunk(this, core::mem::transmute_copy(&pwchbuffer), core::mem::transmute_copy(&cwchchunksize), core::mem::transmute_copy(&pcwchread))
            }
        }
        unsafe extern "system" fn GetBaseUri<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszbaseuri: *mut windows_core::PCWSTR, pcwchbaseuri: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlReader_Impl::GetBaseUri(this, core::mem::transmute_copy(&ppwszbaseuri), core::mem::transmute_copy(&pcwchbaseuri)).into()
            }
        }
        unsafe extern "system" fn IsDefault<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlReader_Impl::IsDefault(this)
            }
        }
        unsafe extern "system" fn IsEmptyElement<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlReader_Impl::IsEmptyElement(this)
            }
        }
        unsafe extern "system" fn GetLineNumber<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnlinenumber: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXmlReader_Impl::GetLineNumber(this) {
                    Ok(ok__) => {
                        pnlinenumber.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLinePosition<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnlineposition: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXmlReader_Impl::GetLinePosition(this) {
                    Ok(ok__) => {
                        pnlineposition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAttributeCount<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnattributecount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXmlReader_Impl::GetAttributeCount(this) {
                    Ok(ok__) => {
                        pnattributecount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDepth<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pndepth: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXmlReader_Impl::GetDepth(this) {
                    Ok(ok__) => {
                        pndepth.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEOF<Identity: IXmlReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlReader_Impl::IsEOF(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetInput: SetInput::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            Read: Read::<Identity, OFFSET>,
            GetNodeType: GetNodeType::<Identity, OFFSET>,
            MoveToFirstAttribute: MoveToFirstAttribute::<Identity, OFFSET>,
            MoveToNextAttribute: MoveToNextAttribute::<Identity, OFFSET>,
            MoveToAttributeByName: MoveToAttributeByName::<Identity, OFFSET>,
            MoveToElement: MoveToElement::<Identity, OFFSET>,
            GetQualifiedName: GetQualifiedName::<Identity, OFFSET>,
            GetNamespaceUri: GetNamespaceUri::<Identity, OFFSET>,
            GetLocalName: GetLocalName::<Identity, OFFSET>,
            GetPrefix: GetPrefix::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            ReadValueChunk: ReadValueChunk::<Identity, OFFSET>,
            GetBaseUri: GetBaseUri::<Identity, OFFSET>,
            IsDefault: IsDefault::<Identity, OFFSET>,
            IsEmptyElement: IsEmptyElement::<Identity, OFFSET>,
            GetLineNumber: GetLineNumber::<Identity, OFFSET>,
            GetLinePosition: GetLinePosition::<Identity, OFFSET>,
            GetAttributeCount: GetAttributeCount::<Identity, OFFSET>,
            GetDepth: GetDepth::<Identity, OFFSET>,
            IsEOF: IsEOF::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXmlReader as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXmlReader {}
windows_core::imp::define_interface!(IXmlResolver, IXmlResolver_Vtbl, 0x7279fc82_709d_4095_b63d_69fe4b0d9030);
windows_core::imp::interface_hierarchy!(IXmlResolver, windows_core::IUnknown);
impl IXmlResolver {
    pub unsafe fn ResolveUri<P0, P1, P2>(&self, pwszbaseuri: P0, pwszpublicidentifier: P1, pwszsystemidentifier: P2) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResolveUri)(windows_core::Interface::as_raw(self), pwszbaseuri.param().abi(), pwszpublicidentifier.param().abi(), pwszsystemidentifier.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlResolver_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ResolveUri: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXmlResolver_Impl: windows_core::IUnknownImpl {
    fn ResolveUri(&self, pwszbaseuri: &windows_core::PCWSTR, pwszpublicidentifier: &windows_core::PCWSTR, pwszsystemidentifier: &windows_core::PCWSTR) -> windows_core::Result<windows_core::IUnknown>;
}
impl IXmlResolver_Vtbl {
    pub const fn new<Identity: IXmlResolver_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ResolveUri<Identity: IXmlResolver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszbaseuri: windows_core::PCWSTR, pwszpublicidentifier: windows_core::PCWSTR, pwszsystemidentifier: windows_core::PCWSTR, ppresolvedinput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXmlResolver_Impl::ResolveUri(this, core::mem::transmute(&pwszbaseuri), core::mem::transmute(&pwszpublicidentifier), core::mem::transmute(&pwszsystemidentifier)) {
                    Ok(ok__) => {
                        ppresolvedinput.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ResolveUri: ResolveUri::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXmlResolver as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXmlResolver {}
windows_core::imp::define_interface!(IXmlWriter, IXmlWriter_Vtbl, 0x7279fc88_709d_4095_b63d_69fe4b0d9030);
windows_core::imp::interface_hierarchy!(IXmlWriter, windows_core::IUnknown);
impl IXmlWriter {
    pub unsafe fn SetOutput<P0>(&self, poutput: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutput)(windows_core::Interface::as_raw(self), poutput.param().abi()).ok() }
    }
    pub unsafe fn GetProperty(&self, nproperty: u32) -> windows_core::Result<isize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), nproperty, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetProperty(&self, nproperty: u32, pvalue: Option<isize>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), nproperty, pvalue.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn WriteAttributes<P0>(&self, preader: P0, fwritedefaultattributes: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXmlReader>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteAttributes)(windows_core::Interface::as_raw(self), preader.param().abi(), fwritedefaultattributes.into()).ok() }
    }
    pub unsafe fn WriteAttributeString<P0, P1, P2, P3>(&self, pwszprefix: P0, pwszlocalname: P1, pwsznamespaceuri: P2, pwszvalue: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteAttributeString)(windows_core::Interface::as_raw(self), pwszprefix.param().abi(), pwszlocalname.param().abi(), pwsznamespaceuri.param().abi(), pwszvalue.param().abi()).ok() }
    }
    pub unsafe fn WriteCData<P0>(&self, pwsztext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteCData)(windows_core::Interface::as_raw(self), pwsztext.param().abi()).ok() }
    }
    pub unsafe fn WriteCharEntity(&self, wch: u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteCharEntity)(windows_core::Interface::as_raw(self), wch).ok() }
    }
    pub unsafe fn WriteChars(&self, pwch: Option<&[u16]>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteChars)(windows_core::Interface::as_raw(self), core::mem::transmute(pwch.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pwch.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok() }
    }
    pub unsafe fn WriteComment<P0>(&self, pwszcomment: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteComment)(windows_core::Interface::as_raw(self), pwszcomment.param().abi()).ok() }
    }
    pub unsafe fn WriteDocType<P0, P1, P2, P3>(&self, pwszname: P0, pwszpublicid: P1, pwszsystemid: P2, pwszsubset: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteDocType)(windows_core::Interface::as_raw(self), pwszname.param().abi(), pwszpublicid.param().abi(), pwszsystemid.param().abi(), pwszsubset.param().abi()).ok() }
    }
    pub unsafe fn WriteElementString<P0, P1, P2, P3>(&self, pwszprefix: P0, pwszlocalname: P1, pwsznamespaceuri: P2, pwszvalue: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteElementString)(windows_core::Interface::as_raw(self), pwszprefix.param().abi(), pwszlocalname.param().abi(), pwsznamespaceuri.param().abi(), pwszvalue.param().abi()).ok() }
    }
    pub unsafe fn WriteEndDocument(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteEndDocument)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn WriteEndElement(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteEndElement)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn WriteEntityRef<P0>(&self, pwszname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteEntityRef)(windows_core::Interface::as_raw(self), pwszname.param().abi()).ok() }
    }
    pub unsafe fn WriteFullEndElement(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteFullEndElement)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn WriteName<P0>(&self, pwszname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteName)(windows_core::Interface::as_raw(self), pwszname.param().abi()).ok() }
    }
    pub unsafe fn WriteNmToken<P0>(&self, pwsznmtoken: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteNmToken)(windows_core::Interface::as_raw(self), pwsznmtoken.param().abi()).ok() }
    }
    pub unsafe fn WriteNode<P0>(&self, preader: P0, fwritedefaultattributes: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXmlReader>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteNode)(windows_core::Interface::as_raw(self), preader.param().abi(), fwritedefaultattributes.into()).ok() }
    }
    pub unsafe fn WriteNodeShallow<P0>(&self, preader: P0, fwritedefaultattributes: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXmlReader>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteNodeShallow)(windows_core::Interface::as_raw(self), preader.param().abi(), fwritedefaultattributes.into()).ok() }
    }
    pub unsafe fn WriteProcessingInstruction<P0, P1>(&self, pwszname: P0, pwsztext: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteProcessingInstruction)(windows_core::Interface::as_raw(self), pwszname.param().abi(), pwsztext.param().abi()).ok() }
    }
    pub unsafe fn WriteQualifiedName<P0, P1>(&self, pwszlocalname: P0, pwsznamespaceuri: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteQualifiedName)(windows_core::Interface::as_raw(self), pwszlocalname.param().abi(), pwsznamespaceuri.param().abi()).ok() }
    }
    pub unsafe fn WriteRaw<P0>(&self, pwszdata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteRaw)(windows_core::Interface::as_raw(self), pwszdata.param().abi()).ok() }
    }
    pub unsafe fn WriteRawChars(&self, pwch: Option<&[u16]>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteRawChars)(windows_core::Interface::as_raw(self), core::mem::transmute(pwch.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pwch.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok() }
    }
    pub unsafe fn WriteStartDocument(&self, standalone: XmlStandalone) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteStartDocument)(windows_core::Interface::as_raw(self), standalone).ok() }
    }
    pub unsafe fn WriteStartElement<P0, P1, P2>(&self, pwszprefix: P0, pwszlocalname: P1, pwsznamespaceuri: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteStartElement)(windows_core::Interface::as_raw(self), pwszprefix.param().abi(), pwszlocalname.param().abi(), pwsznamespaceuri.param().abi()).ok() }
    }
    pub unsafe fn WriteString<P0>(&self, pwsztext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteString)(windows_core::Interface::as_raw(self), pwsztext.param().abi()).ok() }
    }
    pub unsafe fn WriteSurrogateCharEntity(&self, wchlow: u16, wchhigh: u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteSurrogateCharEntity)(windows_core::Interface::as_raw(self), wchlow, wchhigh).ok() }
    }
    pub unsafe fn WriteWhitespace<P0>(&self, pwszwhitespace: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteWhitespace)(windows_core::Interface::as_raw(self), pwszwhitespace.param().abi()).ok() }
    }
    pub unsafe fn Flush(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlWriter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut isize) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, u32, isize) -> windows_core::HRESULT,
    pub WriteAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub WriteAttributeString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub WriteCData: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub WriteCharEntity: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub WriteChars: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub WriteComment: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub WriteDocType: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub WriteElementString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub WriteEndDocument: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WriteEndElement: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WriteEntityRef: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub WriteFullEndElement: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WriteName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub WriteNmToken: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub WriteNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub WriteNodeShallow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub WriteProcessingInstruction: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub WriteQualifiedName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub WriteRaw: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub WriteRawChars: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub WriteStartDocument: unsafe extern "system" fn(*mut core::ffi::c_void, XmlStandalone) -> windows_core::HRESULT,
    pub WriteStartElement: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub WriteString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub WriteSurrogateCharEntity: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16) -> windows_core::HRESULT,
    pub WriteWhitespace: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXmlWriter_Impl: windows_core::IUnknownImpl {
    fn SetOutput(&self, poutput: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetProperty(&self, nproperty: u32) -> windows_core::Result<isize>;
    fn SetProperty(&self, nproperty: u32, pvalue: isize) -> windows_core::Result<()>;
    fn WriteAttributes(&self, preader: windows_core::Ref<IXmlReader>, fwritedefaultattributes: windows_core::BOOL) -> windows_core::Result<()>;
    fn WriteAttributeString(&self, pwszprefix: &windows_core::PCWSTR, pwszlocalname: &windows_core::PCWSTR, pwsznamespaceuri: &windows_core::PCWSTR, pwszvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteCData(&self, pwsztext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteCharEntity(&self, wch: u16) -> windows_core::Result<()>;
    fn WriteChars(&self, pwch: &windows_core::PCWSTR, cwch: u32) -> windows_core::Result<()>;
    fn WriteComment(&self, pwszcomment: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteDocType(&self, pwszname: &windows_core::PCWSTR, pwszpublicid: &windows_core::PCWSTR, pwszsystemid: &windows_core::PCWSTR, pwszsubset: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteElementString(&self, pwszprefix: &windows_core::PCWSTR, pwszlocalname: &windows_core::PCWSTR, pwsznamespaceuri: &windows_core::PCWSTR, pwszvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteEndDocument(&self) -> windows_core::Result<()>;
    fn WriteEndElement(&self) -> windows_core::Result<()>;
    fn WriteEntityRef(&self, pwszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteFullEndElement(&self) -> windows_core::Result<()>;
    fn WriteName(&self, pwszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteNmToken(&self, pwsznmtoken: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteNode(&self, preader: windows_core::Ref<IXmlReader>, fwritedefaultattributes: windows_core::BOOL) -> windows_core::Result<()>;
    fn WriteNodeShallow(&self, preader: windows_core::Ref<IXmlReader>, fwritedefaultattributes: windows_core::BOOL) -> windows_core::Result<()>;
    fn WriteProcessingInstruction(&self, pwszname: &windows_core::PCWSTR, pwsztext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteQualifiedName(&self, pwszlocalname: &windows_core::PCWSTR, pwsznamespaceuri: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteRaw(&self, pwszdata: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteRawChars(&self, pwch: &windows_core::PCWSTR, cwch: u32) -> windows_core::Result<()>;
    fn WriteStartDocument(&self, standalone: XmlStandalone) -> windows_core::Result<()>;
    fn WriteStartElement(&self, pwszprefix: &windows_core::PCWSTR, pwszlocalname: &windows_core::PCWSTR, pwsznamespaceuri: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteString(&self, pwsztext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteSurrogateCharEntity(&self, wchlow: u16, wchhigh: u16) -> windows_core::Result<()>;
    fn WriteWhitespace(&self, pwszwhitespace: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Flush(&self) -> windows_core::Result<()>;
}
impl IXmlWriter_Vtbl {
    pub const fn new<Identity: IXmlWriter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetOutput<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poutput: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::SetOutput(this, core::mem::transmute_copy(&poutput)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXmlWriter_Impl::GetProperty(this, core::mem::transmute_copy(&nproperty)) {
                    Ok(ok__) => {
                        ppvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nproperty: u32, pvalue: isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::SetProperty(this, core::mem::transmute_copy(&nproperty), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn WriteAttributes<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preader: *mut core::ffi::c_void, fwritedefaultattributes: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::WriteAttributes(this, core::mem::transmute_copy(&preader), core::mem::transmute_copy(&fwritedefaultattributes)).into()
            }
        }
        unsafe extern "system" fn WriteAttributeString<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprefix: windows_core::PCWSTR, pwszlocalname: windows_core::PCWSTR, pwsznamespaceuri: windows_core::PCWSTR, pwszvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::WriteAttributeString(this, core::mem::transmute(&pwszprefix), core::mem::transmute(&pwszlocalname), core::mem::transmute(&pwsznamespaceuri), core::mem::transmute(&pwszvalue)).into()
            }
        }
        unsafe extern "system" fn WriteCData<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsztext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::WriteCData(this, core::mem::transmute(&pwsztext)).into()
            }
        }
        unsafe extern "system" fn WriteCharEntity<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wch: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::WriteCharEntity(this, core::mem::transmute_copy(&wch)).into()
            }
        }
        unsafe extern "system" fn WriteChars<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwch: windows_core::PCWSTR, cwch: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::WriteChars(this, core::mem::transmute(&pwch), core::mem::transmute_copy(&cwch)).into()
            }
        }
        unsafe extern "system" fn WriteComment<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszcomment: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::WriteComment(this, core::mem::transmute(&pwszcomment)).into()
            }
        }
        unsafe extern "system" fn WriteDocType<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR, pwszpublicid: windows_core::PCWSTR, pwszsystemid: windows_core::PCWSTR, pwszsubset: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::WriteDocType(this, core::mem::transmute(&pwszname), core::mem::transmute(&pwszpublicid), core::mem::transmute(&pwszsystemid), core::mem::transmute(&pwszsubset)).into()
            }
        }
        unsafe extern "system" fn WriteElementString<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprefix: windows_core::PCWSTR, pwszlocalname: windows_core::PCWSTR, pwsznamespaceuri: windows_core::PCWSTR, pwszvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::WriteElementString(this, core::mem::transmute(&pwszprefix), core::mem::transmute(&pwszlocalname), core::mem::transmute(&pwsznamespaceuri), core::mem::transmute(&pwszvalue)).into()
            }
        }
        unsafe extern "system" fn WriteEndDocument<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::WriteEndDocument(this).into()
            }
        }
        unsafe extern "system" fn WriteEndElement<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::WriteEndElement(this).into()
            }
        }
        unsafe extern "system" fn WriteEntityRef<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::WriteEntityRef(this, core::mem::transmute(&pwszname)).into()
            }
        }
        unsafe extern "system" fn WriteFullEndElement<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::WriteFullEndElement(this).into()
            }
        }
        unsafe extern "system" fn WriteName<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::WriteName(this, core::mem::transmute(&pwszname)).into()
            }
        }
        unsafe extern "system" fn WriteNmToken<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsznmtoken: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::WriteNmToken(this, core::mem::transmute(&pwsznmtoken)).into()
            }
        }
        unsafe extern "system" fn WriteNode<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preader: *mut core::ffi::c_void, fwritedefaultattributes: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::WriteNode(this, core::mem::transmute_copy(&preader), core::mem::transmute_copy(&fwritedefaultattributes)).into()
            }
        }
        unsafe extern "system" fn WriteNodeShallow<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preader: *mut core::ffi::c_void, fwritedefaultattributes: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::WriteNodeShallow(this, core::mem::transmute_copy(&preader), core::mem::transmute_copy(&fwritedefaultattributes)).into()
            }
        }
        unsafe extern "system" fn WriteProcessingInstruction<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR, pwsztext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::WriteProcessingInstruction(this, core::mem::transmute(&pwszname), core::mem::transmute(&pwsztext)).into()
            }
        }
        unsafe extern "system" fn WriteQualifiedName<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszlocalname: windows_core::PCWSTR, pwsznamespaceuri: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::WriteQualifiedName(this, core::mem::transmute(&pwszlocalname), core::mem::transmute(&pwsznamespaceuri)).into()
            }
        }
        unsafe extern "system" fn WriteRaw<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdata: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::WriteRaw(this, core::mem::transmute(&pwszdata)).into()
            }
        }
        unsafe extern "system" fn WriteRawChars<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwch: windows_core::PCWSTR, cwch: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::WriteRawChars(this, core::mem::transmute(&pwch), core::mem::transmute_copy(&cwch)).into()
            }
        }
        unsafe extern "system" fn WriteStartDocument<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, standalone: XmlStandalone) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::WriteStartDocument(this, core::mem::transmute_copy(&standalone)).into()
            }
        }
        unsafe extern "system" fn WriteStartElement<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprefix: windows_core::PCWSTR, pwszlocalname: windows_core::PCWSTR, pwsznamespaceuri: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::WriteStartElement(this, core::mem::transmute(&pwszprefix), core::mem::transmute(&pwszlocalname), core::mem::transmute(&pwsznamespaceuri)).into()
            }
        }
        unsafe extern "system" fn WriteString<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsztext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::WriteString(this, core::mem::transmute(&pwsztext)).into()
            }
        }
        unsafe extern "system" fn WriteSurrogateCharEntity<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wchlow: u16, wchhigh: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::WriteSurrogateCharEntity(this, core::mem::transmute_copy(&wchlow), core::mem::transmute_copy(&wchhigh)).into()
            }
        }
        unsafe extern "system" fn WriteWhitespace<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszwhitespace: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::WriteWhitespace(this, core::mem::transmute(&pwszwhitespace)).into()
            }
        }
        unsafe extern "system" fn Flush<Identity: IXmlWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriter_Impl::Flush(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetOutput: SetOutput::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            WriteAttributes: WriteAttributes::<Identity, OFFSET>,
            WriteAttributeString: WriteAttributeString::<Identity, OFFSET>,
            WriteCData: WriteCData::<Identity, OFFSET>,
            WriteCharEntity: WriteCharEntity::<Identity, OFFSET>,
            WriteChars: WriteChars::<Identity, OFFSET>,
            WriteComment: WriteComment::<Identity, OFFSET>,
            WriteDocType: WriteDocType::<Identity, OFFSET>,
            WriteElementString: WriteElementString::<Identity, OFFSET>,
            WriteEndDocument: WriteEndDocument::<Identity, OFFSET>,
            WriteEndElement: WriteEndElement::<Identity, OFFSET>,
            WriteEntityRef: WriteEntityRef::<Identity, OFFSET>,
            WriteFullEndElement: WriteFullEndElement::<Identity, OFFSET>,
            WriteName: WriteName::<Identity, OFFSET>,
            WriteNmToken: WriteNmToken::<Identity, OFFSET>,
            WriteNode: WriteNode::<Identity, OFFSET>,
            WriteNodeShallow: WriteNodeShallow::<Identity, OFFSET>,
            WriteProcessingInstruction: WriteProcessingInstruction::<Identity, OFFSET>,
            WriteQualifiedName: WriteQualifiedName::<Identity, OFFSET>,
            WriteRaw: WriteRaw::<Identity, OFFSET>,
            WriteRawChars: WriteRawChars::<Identity, OFFSET>,
            WriteStartDocument: WriteStartDocument::<Identity, OFFSET>,
            WriteStartElement: WriteStartElement::<Identity, OFFSET>,
            WriteString: WriteString::<Identity, OFFSET>,
            WriteSurrogateCharEntity: WriteSurrogateCharEntity::<Identity, OFFSET>,
            WriteWhitespace: WriteWhitespace::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXmlWriter as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXmlWriter {}
windows_core::imp::define_interface!(IXmlWriterLite, IXmlWriterLite_Vtbl, 0x862494c6_1310_4aad_b3cd_2dbeebf670d3);
windows_core::imp::interface_hierarchy!(IXmlWriterLite, windows_core::IUnknown);
impl IXmlWriterLite {
    pub unsafe fn SetOutput<P0>(&self, poutput: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutput)(windows_core::Interface::as_raw(self), poutput.param().abi()).ok() }
    }
    pub unsafe fn GetProperty(&self, nproperty: u32) -> windows_core::Result<isize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), nproperty, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetProperty(&self, nproperty: u32, pvalue: Option<isize>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), nproperty, pvalue.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn WriteAttributes<P0>(&self, preader: P0, fwritedefaultattributes: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXmlReader>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteAttributes)(windows_core::Interface::as_raw(self), preader.param().abi(), fwritedefaultattributes.into()).ok() }
    }
    pub unsafe fn WriteAttributeString(&self, pwszqname: &[u16], pwszvalue: Option<&[u16]>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteAttributeString)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszqname.as_ptr()), pwszqname.len().try_into().unwrap(), core::mem::transmute(pwszvalue.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pwszvalue.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok() }
    }
    pub unsafe fn WriteCData<P0>(&self, pwsztext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteCData)(windows_core::Interface::as_raw(self), pwsztext.param().abi()).ok() }
    }
    pub unsafe fn WriteCharEntity(&self, wch: u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteCharEntity)(windows_core::Interface::as_raw(self), wch).ok() }
    }
    pub unsafe fn WriteChars(&self, pwch: Option<&[u16]>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteChars)(windows_core::Interface::as_raw(self), core::mem::transmute(pwch.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pwch.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok() }
    }
    pub unsafe fn WriteComment<P0>(&self, pwszcomment: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteComment)(windows_core::Interface::as_raw(self), pwszcomment.param().abi()).ok() }
    }
    pub unsafe fn WriteDocType<P0, P1, P2, P3>(&self, pwszname: P0, pwszpublicid: P1, pwszsystemid: P2, pwszsubset: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteDocType)(windows_core::Interface::as_raw(self), pwszname.param().abi(), pwszpublicid.param().abi(), pwszsystemid.param().abi(), pwszsubset.param().abi()).ok() }
    }
    pub unsafe fn WriteElementString<P2>(&self, pwszqname: &[u16], pwszvalue: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteElementString)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszqname.as_ptr()), pwszqname.len().try_into().unwrap(), pwszvalue.param().abi()).ok() }
    }
    pub unsafe fn WriteEndDocument(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteEndDocument)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn WriteEndElement(&self, pwszqname: &[u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteEndElement)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszqname.as_ptr()), pwszqname.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn WriteEntityRef<P0>(&self, pwszname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteEntityRef)(windows_core::Interface::as_raw(self), pwszname.param().abi()).ok() }
    }
    pub unsafe fn WriteFullEndElement(&self, pwszqname: &[u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteFullEndElement)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszqname.as_ptr()), pwszqname.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn WriteName<P0>(&self, pwszname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteName)(windows_core::Interface::as_raw(self), pwszname.param().abi()).ok() }
    }
    pub unsafe fn WriteNmToken<P0>(&self, pwsznmtoken: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteNmToken)(windows_core::Interface::as_raw(self), pwsznmtoken.param().abi()).ok() }
    }
    pub unsafe fn WriteNode<P0>(&self, preader: P0, fwritedefaultattributes: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXmlReader>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteNode)(windows_core::Interface::as_raw(self), preader.param().abi(), fwritedefaultattributes.into()).ok() }
    }
    pub unsafe fn WriteNodeShallow<P0>(&self, preader: P0, fwritedefaultattributes: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXmlReader>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteNodeShallow)(windows_core::Interface::as_raw(self), preader.param().abi(), fwritedefaultattributes.into()).ok() }
    }
    pub unsafe fn WriteProcessingInstruction<P0, P1>(&self, pwszname: P0, pwsztext: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteProcessingInstruction)(windows_core::Interface::as_raw(self), pwszname.param().abi(), pwsztext.param().abi()).ok() }
    }
    pub unsafe fn WriteRaw<P0>(&self, pwszdata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteRaw)(windows_core::Interface::as_raw(self), pwszdata.param().abi()).ok() }
    }
    pub unsafe fn WriteRawChars(&self, pwch: Option<&[u16]>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteRawChars)(windows_core::Interface::as_raw(self), core::mem::transmute(pwch.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pwch.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok() }
    }
    pub unsafe fn WriteStartDocument(&self, standalone: XmlStandalone) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteStartDocument)(windows_core::Interface::as_raw(self), standalone).ok() }
    }
    pub unsafe fn WriteStartElement(&self, pwszqname: &[u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteStartElement)(windows_core::Interface::as_raw(self), core::mem::transmute(pwszqname.as_ptr()), pwszqname.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn WriteString<P0>(&self, pwsztext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteString)(windows_core::Interface::as_raw(self), pwsztext.param().abi()).ok() }
    }
    pub unsafe fn WriteSurrogateCharEntity(&self, wchlow: u16, wchhigh: u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteSurrogateCharEntity)(windows_core::Interface::as_raw(self), wchlow, wchhigh).ok() }
    }
    pub unsafe fn WriteWhitespace<P0>(&self, pwszwhitespace: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteWhitespace)(windows_core::Interface::as_raw(self), pwszwhitespace.param().abi()).ok() }
    }
    pub unsafe fn Flush(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlWriterLite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut isize) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, u32, isize) -> windows_core::HRESULT,
    pub WriteAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub WriteAttributeString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub WriteCData: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub WriteCharEntity: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub WriteChars: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub WriteComment: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub WriteDocType: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub WriteElementString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub WriteEndDocument: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WriteEndElement: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub WriteEntityRef: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub WriteFullEndElement: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub WriteName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub WriteNmToken: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub WriteNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub WriteNodeShallow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub WriteProcessingInstruction: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub WriteRaw: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub WriteRawChars: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub WriteStartDocument: unsafe extern "system" fn(*mut core::ffi::c_void, XmlStandalone) -> windows_core::HRESULT,
    pub WriteStartElement: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub WriteString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub WriteSurrogateCharEntity: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16) -> windows_core::HRESULT,
    pub WriteWhitespace: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXmlWriterLite_Impl: windows_core::IUnknownImpl {
    fn SetOutput(&self, poutput: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetProperty(&self, nproperty: u32) -> windows_core::Result<isize>;
    fn SetProperty(&self, nproperty: u32, pvalue: isize) -> windows_core::Result<()>;
    fn WriteAttributes(&self, preader: windows_core::Ref<IXmlReader>, fwritedefaultattributes: windows_core::BOOL) -> windows_core::Result<()>;
    fn WriteAttributeString(&self, pwszqname: &windows_core::PCWSTR, cwszqname: u32, pwszvalue: &windows_core::PCWSTR, cwszvalue: u32) -> windows_core::Result<()>;
    fn WriteCData(&self, pwsztext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteCharEntity(&self, wch: u16) -> windows_core::Result<()>;
    fn WriteChars(&self, pwch: &windows_core::PCWSTR, cwch: u32) -> windows_core::Result<()>;
    fn WriteComment(&self, pwszcomment: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteDocType(&self, pwszname: &windows_core::PCWSTR, pwszpublicid: &windows_core::PCWSTR, pwszsystemid: &windows_core::PCWSTR, pwszsubset: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteElementString(&self, pwszqname: &windows_core::PCWSTR, cwszqname: u32, pwszvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteEndDocument(&self) -> windows_core::Result<()>;
    fn WriteEndElement(&self, pwszqname: &windows_core::PCWSTR, cwszqname: u32) -> windows_core::Result<()>;
    fn WriteEntityRef(&self, pwszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteFullEndElement(&self, pwszqname: &windows_core::PCWSTR, cwszqname: u32) -> windows_core::Result<()>;
    fn WriteName(&self, pwszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteNmToken(&self, pwsznmtoken: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteNode(&self, preader: windows_core::Ref<IXmlReader>, fwritedefaultattributes: windows_core::BOOL) -> windows_core::Result<()>;
    fn WriteNodeShallow(&self, preader: windows_core::Ref<IXmlReader>, fwritedefaultattributes: windows_core::BOOL) -> windows_core::Result<()>;
    fn WriteProcessingInstruction(&self, pwszname: &windows_core::PCWSTR, pwsztext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteRaw(&self, pwszdata: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteRawChars(&self, pwch: &windows_core::PCWSTR, cwch: u32) -> windows_core::Result<()>;
    fn WriteStartDocument(&self, standalone: XmlStandalone) -> windows_core::Result<()>;
    fn WriteStartElement(&self, pwszqname: &windows_core::PCWSTR, cwszqname: u32) -> windows_core::Result<()>;
    fn WriteString(&self, pwsztext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn WriteSurrogateCharEntity(&self, wchlow: u16, wchhigh: u16) -> windows_core::Result<()>;
    fn WriteWhitespace(&self, pwszwhitespace: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Flush(&self) -> windows_core::Result<()>;
}
impl IXmlWriterLite_Vtbl {
    pub const fn new<Identity: IXmlWriterLite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetOutput<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poutput: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::SetOutput(this, core::mem::transmute_copy(&poutput)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXmlWriterLite_Impl::GetProperty(this, core::mem::transmute_copy(&nproperty)) {
                    Ok(ok__) => {
                        ppvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nproperty: u32, pvalue: isize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::SetProperty(this, core::mem::transmute_copy(&nproperty), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn WriteAttributes<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preader: *mut core::ffi::c_void, fwritedefaultattributes: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::WriteAttributes(this, core::mem::transmute_copy(&preader), core::mem::transmute_copy(&fwritedefaultattributes)).into()
            }
        }
        unsafe extern "system" fn WriteAttributeString<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszqname: windows_core::PCWSTR, cwszqname: u32, pwszvalue: windows_core::PCWSTR, cwszvalue: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::WriteAttributeString(this, core::mem::transmute(&pwszqname), core::mem::transmute_copy(&cwszqname), core::mem::transmute(&pwszvalue), core::mem::transmute_copy(&cwszvalue)).into()
            }
        }
        unsafe extern "system" fn WriteCData<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsztext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::WriteCData(this, core::mem::transmute(&pwsztext)).into()
            }
        }
        unsafe extern "system" fn WriteCharEntity<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wch: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::WriteCharEntity(this, core::mem::transmute_copy(&wch)).into()
            }
        }
        unsafe extern "system" fn WriteChars<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwch: windows_core::PCWSTR, cwch: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::WriteChars(this, core::mem::transmute(&pwch), core::mem::transmute_copy(&cwch)).into()
            }
        }
        unsafe extern "system" fn WriteComment<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszcomment: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::WriteComment(this, core::mem::transmute(&pwszcomment)).into()
            }
        }
        unsafe extern "system" fn WriteDocType<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR, pwszpublicid: windows_core::PCWSTR, pwszsystemid: windows_core::PCWSTR, pwszsubset: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::WriteDocType(this, core::mem::transmute(&pwszname), core::mem::transmute(&pwszpublicid), core::mem::transmute(&pwszsystemid), core::mem::transmute(&pwszsubset)).into()
            }
        }
        unsafe extern "system" fn WriteElementString<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszqname: windows_core::PCWSTR, cwszqname: u32, pwszvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::WriteElementString(this, core::mem::transmute(&pwszqname), core::mem::transmute_copy(&cwszqname), core::mem::transmute(&pwszvalue)).into()
            }
        }
        unsafe extern "system" fn WriteEndDocument<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::WriteEndDocument(this).into()
            }
        }
        unsafe extern "system" fn WriteEndElement<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszqname: windows_core::PCWSTR, cwszqname: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::WriteEndElement(this, core::mem::transmute(&pwszqname), core::mem::transmute_copy(&cwszqname)).into()
            }
        }
        unsafe extern "system" fn WriteEntityRef<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::WriteEntityRef(this, core::mem::transmute(&pwszname)).into()
            }
        }
        unsafe extern "system" fn WriteFullEndElement<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszqname: windows_core::PCWSTR, cwszqname: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::WriteFullEndElement(this, core::mem::transmute(&pwszqname), core::mem::transmute_copy(&cwszqname)).into()
            }
        }
        unsafe extern "system" fn WriteName<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::WriteName(this, core::mem::transmute(&pwszname)).into()
            }
        }
        unsafe extern "system" fn WriteNmToken<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsznmtoken: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::WriteNmToken(this, core::mem::transmute(&pwsznmtoken)).into()
            }
        }
        unsafe extern "system" fn WriteNode<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preader: *mut core::ffi::c_void, fwritedefaultattributes: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::WriteNode(this, core::mem::transmute_copy(&preader), core::mem::transmute_copy(&fwritedefaultattributes)).into()
            }
        }
        unsafe extern "system" fn WriteNodeShallow<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preader: *mut core::ffi::c_void, fwritedefaultattributes: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::WriteNodeShallow(this, core::mem::transmute_copy(&preader), core::mem::transmute_copy(&fwritedefaultattributes)).into()
            }
        }
        unsafe extern "system" fn WriteProcessingInstruction<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR, pwsztext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::WriteProcessingInstruction(this, core::mem::transmute(&pwszname), core::mem::transmute(&pwsztext)).into()
            }
        }
        unsafe extern "system" fn WriteRaw<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdata: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::WriteRaw(this, core::mem::transmute(&pwszdata)).into()
            }
        }
        unsafe extern "system" fn WriteRawChars<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwch: windows_core::PCWSTR, cwch: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::WriteRawChars(this, core::mem::transmute(&pwch), core::mem::transmute_copy(&cwch)).into()
            }
        }
        unsafe extern "system" fn WriteStartDocument<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, standalone: XmlStandalone) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::WriteStartDocument(this, core::mem::transmute_copy(&standalone)).into()
            }
        }
        unsafe extern "system" fn WriteStartElement<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszqname: windows_core::PCWSTR, cwszqname: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::WriteStartElement(this, core::mem::transmute(&pwszqname), core::mem::transmute_copy(&cwszqname)).into()
            }
        }
        unsafe extern "system" fn WriteString<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsztext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::WriteString(this, core::mem::transmute(&pwsztext)).into()
            }
        }
        unsafe extern "system" fn WriteSurrogateCharEntity<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wchlow: u16, wchhigh: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::WriteSurrogateCharEntity(this, core::mem::transmute_copy(&wchlow), core::mem::transmute_copy(&wchhigh)).into()
            }
        }
        unsafe extern "system" fn WriteWhitespace<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszwhitespace: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::WriteWhitespace(this, core::mem::transmute(&pwszwhitespace)).into()
            }
        }
        unsafe extern "system" fn Flush<Identity: IXmlWriterLite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXmlWriterLite_Impl::Flush(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetOutput: SetOutput::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            WriteAttributes: WriteAttributes::<Identity, OFFSET>,
            WriteAttributeString: WriteAttributeString::<Identity, OFFSET>,
            WriteCData: WriteCData::<Identity, OFFSET>,
            WriteCharEntity: WriteCharEntity::<Identity, OFFSET>,
            WriteChars: WriteChars::<Identity, OFFSET>,
            WriteComment: WriteComment::<Identity, OFFSET>,
            WriteDocType: WriteDocType::<Identity, OFFSET>,
            WriteElementString: WriteElementString::<Identity, OFFSET>,
            WriteEndDocument: WriteEndDocument::<Identity, OFFSET>,
            WriteEndElement: WriteEndElement::<Identity, OFFSET>,
            WriteEntityRef: WriteEntityRef::<Identity, OFFSET>,
            WriteFullEndElement: WriteFullEndElement::<Identity, OFFSET>,
            WriteName: WriteName::<Identity, OFFSET>,
            WriteNmToken: WriteNmToken::<Identity, OFFSET>,
            WriteNode: WriteNode::<Identity, OFFSET>,
            WriteNodeShallow: WriteNodeShallow::<Identity, OFFSET>,
            WriteProcessingInstruction: WriteProcessingInstruction::<Identity, OFFSET>,
            WriteRaw: WriteRaw::<Identity, OFFSET>,
            WriteRawChars: WriteRawChars::<Identity, OFFSET>,
            WriteStartDocument: WriteStartDocument::<Identity, OFFSET>,
            WriteStartElement: WriteStartElement::<Identity, OFFSET>,
            WriteString: WriteString::<Identity, OFFSET>,
            WriteSurrogateCharEntity: WriteSurrogateCharEntity::<Identity, OFFSET>,
            WriteWhitespace: WriteWhitespace::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXmlWriterLite as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXmlWriterLite {}
pub const MX_E_ENCODING: XmlError = XmlError(-1072894462i32);
pub const MX_E_ENCODINGSIGNATURE: XmlError = XmlError(-1072894460i32);
pub const MX_E_ENCODINGSWITCH: XmlError = XmlError(-1072894461i32);
pub const MX_E_INPUTEND: XmlError = XmlError(-1072894463i32);
pub const MX_E_MX: XmlError = XmlError(-1072894464i32);
pub const NC_E_DECLAREDPREFIX: XmlError = XmlError(-1072894364i32);
pub const NC_E_EMPTYURI: XmlError = XmlError(-1072894362i32);
pub const NC_E_NAMECOLON: XmlError = XmlError(-1072894365i32);
pub const NC_E_NC: XmlError = XmlError(-1072894368i32);
pub const NC_E_QNAMECHARACTER: XmlError = XmlError(-1072894367i32);
pub const NC_E_QNAMECOLON: XmlError = XmlError(-1072894366i32);
pub const NC_E_UNDECLAREDPREFIX: XmlError = XmlError(-1072894363i32);
pub const NC_E_XMLNSPREFIXRESERVED: XmlError = XmlError(-1072894360i32);
pub const NC_E_XMLNSURIRESERVED: XmlError = XmlError(-1072894358i32);
pub const NC_E_XMLPREFIXRESERVED: XmlError = XmlError(-1072894361i32);
pub const NC_E_XMLURIRESERVED: XmlError = XmlError(-1072894359i32);
pub const SC_E_MAXELEMENTDEPTH: XmlError = XmlError(-1072894335i32);
pub const SC_E_MAXENTITYEXPANSION: XmlError = XmlError(-1072894334i32);
pub const SC_E_SC: XmlError = XmlError(-1072894336i32);
pub const WC_E_CDSECT: XmlError = XmlError(-1072894418i32);
pub const WC_E_CDSECTEND: XmlError = XmlError(-1072894387i32);
pub const WC_E_COMMENT: XmlError = XmlError(-1072894417i32);
pub const WC_E_CONDSECT: XmlError = XmlError(-1072894416i32);
pub const WC_E_DECLATTLIST: XmlError = XmlError(-1072894415i32);
pub const WC_E_DECLDOCTYPE: XmlError = XmlError(-1072894414i32);
pub const WC_E_DECLELEMENT: XmlError = XmlError(-1072894413i32);
pub const WC_E_DECLENTITY: XmlError = XmlError(-1072894412i32);
pub const WC_E_DECLNOTATION: XmlError = XmlError(-1072894411i32);
pub const WC_E_DIGIT: XmlError = XmlError(-1072894424i32);
pub const WC_E_DTDPROHIBITED: XmlError = XmlError(-1072894385i32);
pub const WC_E_ELEMENTMATCH: XmlError = XmlError(-1072894405i32);
pub const WC_E_ENCNAME: XmlError = XmlError(-1072894399i32);
pub const WC_E_ENTITYCONTENT: XmlError = XmlError(-1072894394i32);
pub const WC_E_EQUAL: XmlError = XmlError(-1072894427i32);
pub const WC_E_GREATERTHAN: XmlError = XmlError(-1072894429i32);
pub const WC_E_HEXDIGIT: XmlError = XmlError(-1072894425i32);
pub const WC_E_INVALIDXMLSPACE: XmlError = XmlError(-1072894384i32);
pub const WC_E_LEADINGXML: XmlError = XmlError(-1072894402i32);
pub const WC_E_LEFTBRACKET: XmlError = XmlError(-1072894423i32);
pub const WC_E_LEFTPAREN: XmlError = XmlError(-1072894422i32);
pub const WC_E_LESSTHAN: XmlError = XmlError(-1072894426i32);
pub const WC_E_MOREDATA: XmlError = XmlError(-1072894386i32);
pub const WC_E_NAME: XmlError = XmlError(-1072894407i32);
pub const WC_E_NAMECHARACTER: XmlError = XmlError(-1072894420i32);
pub const WC_E_NDATA: XmlError = XmlError(-1072894410i32);
pub const WC_E_NOEXTERNALENTITYREF: XmlError = XmlError(-1072894391i32);
pub const WC_E_NORECURSION: XmlError = XmlError(-1072894395i32);
pub const WC_E_PARSEDENTITY: XmlError = XmlError(-1072894392i32);
pub const WC_E_PESBETWEENDECLS: XmlError = XmlError(-1072894396i32);
pub const WC_E_PESINTERNALSUBSET: XmlError = XmlError(-1072894397i32);
pub const WC_E_PI: XmlError = XmlError(-1072894390i32);
pub const WC_E_PUBLIC: XmlError = XmlError(-1072894409i32);
pub const WC_E_PUBLICID: XmlError = XmlError(-1072894398i32);
pub const WC_E_QUESTIONMARK: XmlError = XmlError(-1072894388i32);
pub const WC_E_QUOTE: XmlError = XmlError(-1072894428i32);
pub const WC_E_ROOTELEMENT: XmlError = XmlError(-1072894406i32);
pub const WC_E_SEMICOLON: XmlError = XmlError(-1072894430i32);
pub const WC_E_SYNTAX: XmlError = XmlError(-1072894419i32);
pub const WC_E_SYSTEM: XmlError = XmlError(-1072894408i32);
pub const WC_E_SYSTEMID: XmlError = XmlError(-1072894389i32);
pub const WC_E_TEXTDECL: XmlError = XmlError(-1072894401i32);
pub const WC_E_TEXTXMLDECL: XmlError = XmlError(-1072894403i32);
pub const WC_E_UNDECLAREDENTITY: XmlError = XmlError(-1072894393i32);
pub const WC_E_UNIQUEATTRIBUTE: XmlError = XmlError(-1072894404i32);
pub const WC_E_WC: XmlError = XmlError(-1072894432i32);
pub const WC_E_WHITESPACE: XmlError = XmlError(-1072894431i32);
pub const WC_E_XMLCHARACTER: XmlError = XmlError(-1072894421i32);
pub const WC_E_XMLDECL: XmlError = XmlError(-1072894400i32);
pub const WR_E_DUPLICATEATTRIBUTE: XmlError = XmlError(-1072894204i32);
pub const WR_E_INVALIDACTION: XmlError = XmlError(-1072894197i32);
pub const WR_E_INVALIDSURROGATEPAIR: XmlError = XmlError(-1072894196i32);
pub const WR_E_INVALIDXMLSPACE: XmlError = XmlError(-1072894198i32);
pub const WR_E_NAMESPACEUNDECLARED: XmlError = XmlError(-1072894199i32);
pub const WR_E_NONWHITESPACE: XmlError = XmlError(-1072894207i32);
pub const WR_E_NSPREFIXDECLARED: XmlError = XmlError(-1072894206i32);
pub const WR_E_NSPREFIXWITHEMPTYNSURI: XmlError = XmlError(-1072894205i32);
pub const WR_E_WR: XmlError = XmlError(-1072894208i32);
pub const WR_E_XMLNSPREFIXDECLARATION: XmlError = XmlError(-1072894203i32);
pub const WR_E_XMLNSURIDECLARATION: XmlError = XmlError(-1072894200i32);
pub const WR_E_XMLPREFIXDECLARATION: XmlError = XmlError(-1072894202i32);
pub const WR_E_XMLURIDECLARATION: XmlError = XmlError(-1072894201i32);
pub const XML_E_INVALIDENCODING: XmlError = XmlError(-1072897938i32);
pub const XML_E_INVALID_DECIMAL: XmlError = XmlError(-1072898019i32);
pub const XML_E_INVALID_HEXIDECIMAL: XmlError = XmlError(-1072898018i32);
pub const XML_E_INVALID_UNICODE: XmlError = XmlError(-1072898017i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XmlConformanceLevel(pub i32);
pub const XmlConformanceLevel_Auto: XmlConformanceLevel = XmlConformanceLevel(0i32);
pub const XmlConformanceLevel_Document: XmlConformanceLevel = XmlConformanceLevel(2i32);
pub const XmlConformanceLevel_Fragment: XmlConformanceLevel = XmlConformanceLevel(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XmlError(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XmlNodeType(pub i32);
pub const XmlNodeType_Attribute: XmlNodeType = XmlNodeType(2i32);
pub const XmlNodeType_CDATA: XmlNodeType = XmlNodeType(4i32);
pub const XmlNodeType_Comment: XmlNodeType = XmlNodeType(8i32);
pub const XmlNodeType_DocumentType: XmlNodeType = XmlNodeType(10i32);
pub const XmlNodeType_Element: XmlNodeType = XmlNodeType(1i32);
pub const XmlNodeType_EndElement: XmlNodeType = XmlNodeType(15i32);
pub const XmlNodeType_None: XmlNodeType = XmlNodeType(0i32);
pub const XmlNodeType_ProcessingInstruction: XmlNodeType = XmlNodeType(7i32);
pub const XmlNodeType_Text: XmlNodeType = XmlNodeType(3i32);
pub const XmlNodeType_Whitespace: XmlNodeType = XmlNodeType(13i32);
pub const XmlNodeType_XmlDeclaration: XmlNodeType = XmlNodeType(17i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XmlReadState(pub i32);
pub const XmlReadState_Closed: XmlReadState = XmlReadState(4i32);
pub const XmlReadState_EndOfFile: XmlReadState = XmlReadState(3i32);
pub const XmlReadState_Error: XmlReadState = XmlReadState(2i32);
pub const XmlReadState_Initial: XmlReadState = XmlReadState(0i32);
pub const XmlReadState_Interactive: XmlReadState = XmlReadState(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XmlReaderProperty(pub i32);
pub const XmlReaderProperty_ConformanceLevel: XmlReaderProperty = XmlReaderProperty(1i32);
pub const XmlReaderProperty_DtdProcessing: XmlReaderProperty = XmlReaderProperty(4i32);
pub const XmlReaderProperty_MaxElementDepth: XmlReaderProperty = XmlReaderProperty(6i32);
pub const XmlReaderProperty_MaxEntityExpansion: XmlReaderProperty = XmlReaderProperty(7i32);
pub const XmlReaderProperty_MultiLanguage: XmlReaderProperty = XmlReaderProperty(0i32);
pub const XmlReaderProperty_RandomAccess: XmlReaderProperty = XmlReaderProperty(2i32);
pub const XmlReaderProperty_ReadState: XmlReaderProperty = XmlReaderProperty(5i32);
pub const XmlReaderProperty_XmlResolver: XmlReaderProperty = XmlReaderProperty(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XmlStandalone(pub i32);
pub const XmlStandalone_No: XmlStandalone = XmlStandalone(2i32);
pub const XmlStandalone_Omit: XmlStandalone = XmlStandalone(0i32);
pub const XmlStandalone_Yes: XmlStandalone = XmlStandalone(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XmlWriterProperty(pub i32);
pub const XmlWriterProperty_ByteOrderMark: XmlWriterProperty = XmlWriterProperty(2i32);
pub const XmlWriterProperty_CompactEmptyElement: XmlWriterProperty = XmlWriterProperty(5i32);
pub const XmlWriterProperty_ConformanceLevel: XmlWriterProperty = XmlWriterProperty(4i32);
pub const XmlWriterProperty_Indent: XmlWriterProperty = XmlWriterProperty(1i32);
pub const XmlWriterProperty_MultiLanguage: XmlWriterProperty = XmlWriterProperty(0i32);
pub const XmlWriterProperty_OmitXmlDeclaration: XmlWriterProperty = XmlWriterProperty(3i32);
pub const _DtdProcessing_Last: DtdProcessing = DtdProcessing(1i32);
pub const _XmlConformanceLevel_Last: XmlConformanceLevel = XmlConformanceLevel(2i32);
pub const _XmlNodeType_Last: XmlNodeType = XmlNodeType(17i32);
pub const _XmlReaderProperty_Last: XmlReaderProperty = XmlReaderProperty(7i32);
pub const _XmlStandalone_Last: XmlStandalone = XmlStandalone(2i32);
pub const _XmlWriterProperty_Last: XmlWriterProperty = XmlWriterProperty(5i32);
