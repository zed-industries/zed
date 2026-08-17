pub trait IComprehensiveSpellCheckProvider_Impl: Sized {
    fn ComprehensiveCheck(&self, text: &windows_core::PCWSTR) -> windows_core::Result<IEnumSpellingError>;
}
impl windows_core::RuntimeName for IComprehensiveSpellCheckProvider {}
impl IComprehensiveSpellCheckProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComprehensiveSpellCheckProvider_Vtbl
    where
        Identity: IComprehensiveSpellCheckProvider_Impl,
    {
        unsafe extern "system" fn ComprehensiveCheck<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IComprehensiveSpellCheckProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IComprehensiveSpellCheckProvider_Impl::ComprehensiveCheck(this, core::mem::transmute(&text)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ComprehensiveCheck: ComprehensiveCheck::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComprehensiveSpellCheckProvider as windows_core::Interface>::IID
    }
}
pub trait IEnumCodePage_Impl: Sized {
    fn Clone(&self, ppenum: *const Option<IEnumCodePage>) -> windows_core::Result<()>;
    fn Next(&self, celt: u32, rgelt: *mut MIMECPINFO, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumCodePage {}
impl IEnumCodePage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumCodePage_Vtbl
    where
        Identity: IEnumCodePage_Impl,
    {
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *const *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumCodePage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCodePage_Impl::Clone(this, core::mem::transmute_copy(&ppenum)).into()
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut MIMECPINFO, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumCodePage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCodePage_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumCodePage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCodePage_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumCodePage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCodePage_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumCodePage as windows_core::Interface>::IID
    }
}
pub trait IEnumRfc1766_Impl: Sized {
    fn Clone(&self, ppenum: *const Option<IEnumRfc1766>) -> windows_core::Result<()>;
    fn Next(&self, celt: u32, rgelt: *mut RFC1766INFO, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumRfc1766 {}
impl IEnumRfc1766_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumRfc1766_Vtbl
    where
        Identity: IEnumRfc1766_Impl,
    {
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *const *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumRfc1766_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumRfc1766_Impl::Clone(this, core::mem::transmute_copy(&ppenum)).into()
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut RFC1766INFO, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumRfc1766_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumRfc1766_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumRfc1766_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumRfc1766_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumRfc1766_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumRfc1766_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumRfc1766 as windows_core::Interface>::IID
    }
}
pub trait IEnumScript_Impl: Sized {
    fn Clone(&self, ppenum: *const Option<IEnumScript>) -> windows_core::Result<()>;
    fn Next(&self, celt: u32, rgelt: *mut SCRIPTINFO, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumScript {}
impl IEnumScript_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumScript_Vtbl
    where
        Identity: IEnumScript_Impl,
    {
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *const *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumScript_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumScript_Impl::Clone(this, core::mem::transmute_copy(&ppenum)).into()
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut SCRIPTINFO, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumScript_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumScript_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumScript_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumScript_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumScript_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumScript_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumScript as windows_core::Interface>::IID
    }
}
pub trait IEnumSpellingError_Impl: Sized {
    fn Next(&self, value: *mut Option<ISpellingError>) -> windows_core::HRESULT;
}
impl windows_core::RuntimeName for IEnumSpellingError {}
impl IEnumSpellingError_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumSpellingError_Vtbl
    where
        Identity: IEnumSpellingError_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumSpellingError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSpellingError_Impl::Next(this, core::mem::transmute_copy(&value))
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSpellingError as windows_core::Interface>::IID
    }
}
pub trait IMLangCodePages_Impl: Sized {
    fn GetCharCodePages(&self, chsrc: u16) -> windows_core::Result<u32>;
    fn GetStrCodePages(&self, pszsrc: &windows_core::PCWSTR, cchsrc: i32, dwprioritycodepages: u32, pdwcodepages: *mut u32, pcchcodepages: *mut i32) -> windows_core::Result<()>;
    fn CodePageToCodePages(&self, ucodepage: u32) -> windows_core::Result<u32>;
    fn CodePagesToCodePage(&self, dwcodepages: u32, udefaultcodepage: u32) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IMLangCodePages {}
impl IMLangCodePages_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMLangCodePages_Vtbl
    where
        Identity: IMLangCodePages_Impl,
    {
        unsafe extern "system" fn GetCharCodePages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, chsrc: u16, pdwcodepages: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMLangCodePages_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMLangCodePages_Impl::GetCharCodePages(this, core::mem::transmute_copy(&chsrc)) {
                Ok(ok__) => {
                    pdwcodepages.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrCodePages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsrc: windows_core::PCWSTR, cchsrc: i32, dwprioritycodepages: u32, pdwcodepages: *mut u32, pcchcodepages: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMLangCodePages_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangCodePages_Impl::GetStrCodePages(this, core::mem::transmute(&pszsrc), core::mem::transmute_copy(&cchsrc), core::mem::transmute_copy(&dwprioritycodepages), core::mem::transmute_copy(&pdwcodepages), core::mem::transmute_copy(&pcchcodepages)).into()
        }
        unsafe extern "system" fn CodePageToCodePages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ucodepage: u32, pdwcodepages: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMLangCodePages_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMLangCodePages_Impl::CodePageToCodePages(this, core::mem::transmute_copy(&ucodepage)) {
                Ok(ok__) => {
                    pdwcodepages.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CodePagesToCodePage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcodepages: u32, udefaultcodepage: u32, pucodepage: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMLangCodePages_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMLangCodePages_Impl::CodePagesToCodePage(this, core::mem::transmute_copy(&dwcodepages), core::mem::transmute_copy(&udefaultcodepage)) {
                Ok(ok__) => {
                    pucodepage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCharCodePages: GetCharCodePages::<Identity, OFFSET>,
            GetStrCodePages: GetStrCodePages::<Identity, OFFSET>,
            CodePageToCodePages: CodePageToCodePages::<Identity, OFFSET>,
            CodePagesToCodePage: CodePagesToCodePage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangCodePages as windows_core::Interface>::IID
    }
}
pub trait IMLangConvertCharset_Impl: Sized {
    fn Initialize(&self, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> windows_core::Result<()>;
    fn GetSourceCodePage(&self) -> windows_core::Result<u32>;
    fn GetDestinationCodePage(&self) -> windows_core::Result<u32>;
    fn GetProperty(&self) -> windows_core::Result<u32>;
    fn DoConversion(&self, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> windows_core::Result<()>;
    fn DoConversionToUnicode(&self, psrcstr: &windows_core::PCSTR, pcsrcsize: *mut u32, pdststr: windows_core::PWSTR, pcdstsize: *mut u32) -> windows_core::Result<()>;
    fn DoConversionFromUnicode(&self, psrcstr: &windows_core::PCWSTR, pcsrcsize: *mut u32, pdststr: windows_core::PSTR, pcdstsize: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMLangConvertCharset {}
impl IMLangConvertCharset_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMLangConvertCharset_Vtbl
    where
        Identity: IMLangConvertCharset_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> windows_core::HRESULT
        where
            Identity: IMLangConvertCharset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangConvertCharset_Impl::Initialize(this, core::mem::transmute_copy(&uisrccodepage), core::mem::transmute_copy(&uidstcodepage), core::mem::transmute_copy(&dwproperty)).into()
        }
        unsafe extern "system" fn GetSourceCodePage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puisrccodepage: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMLangConvertCharset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMLangConvertCharset_Impl::GetSourceCodePage(this) {
                Ok(ok__) => {
                    puisrccodepage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationCodePage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puidstcodepage: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMLangConvertCharset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMLangConvertCharset_Impl::GetDestinationCodePage(this) {
                Ok(ok__) => {
                    puidstcodepage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwproperty: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMLangConvertCharset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMLangConvertCharset_Impl::GetProperty(this) {
                Ok(ok__) => {
                    pdwproperty.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoConversion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMLangConvertCharset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangConvertCharset_Impl::DoConversion(this, core::mem::transmute_copy(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
        }
        unsafe extern "system" fn DoConversionToUnicode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrcstr: windows_core::PCSTR, pcsrcsize: *mut u32, pdststr: windows_core::PWSTR, pcdstsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMLangConvertCharset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangConvertCharset_Impl::DoConversionToUnicode(this, core::mem::transmute(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
        }
        unsafe extern "system" fn DoConversionFromUnicode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrcstr: windows_core::PCWSTR, pcsrcsize: *mut u32, pdststr: windows_core::PSTR, pcdstsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMLangConvertCharset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangConvertCharset_Impl::DoConversionFromUnicode(this, core::mem::transmute(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetSourceCodePage: GetSourceCodePage::<Identity, OFFSET>,
            GetDestinationCodePage: GetDestinationCodePage::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            DoConversion: DoConversion::<Identity, OFFSET>,
            DoConversionToUnicode: DoConversionToUnicode::<Identity, OFFSET>,
            DoConversionFromUnicode: DoConversionFromUnicode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangConvertCharset as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IMLangFontLink_Impl: Sized + IMLangCodePages_Impl {
    fn GetFontCodePages(&self, hdc: super::Graphics::Gdi::HDC, hfont: super::Graphics::Gdi::HFONT, pdwcodepages: *mut u32) -> windows_core::Result<()>;
    fn MapFont(&self, hdc: super::Graphics::Gdi::HDC, dwcodepages: u32, hsrcfont: super::Graphics::Gdi::HFONT, phdestfont: *mut super::Graphics::Gdi::HFONT) -> windows_core::Result<()>;
    fn ReleaseFont(&self, hfont: super::Graphics::Gdi::HFONT) -> windows_core::Result<()>;
    fn ResetFontMapping(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IMLangFontLink {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IMLangFontLink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMLangFontLink_Vtbl
    where
        Identity: IMLangFontLink_Impl,
    {
        unsafe extern "system" fn GetFontCodePages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, hfont: super::Graphics::Gdi::HFONT, pdwcodepages: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMLangFontLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangFontLink_Impl::GetFontCodePages(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&hfont), core::mem::transmute_copy(&pdwcodepages)).into()
        }
        unsafe extern "system" fn MapFont<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, dwcodepages: u32, hsrcfont: super::Graphics::Gdi::HFONT, phdestfont: *mut super::Graphics::Gdi::HFONT) -> windows_core::HRESULT
        where
            Identity: IMLangFontLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangFontLink_Impl::MapFont(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&dwcodepages), core::mem::transmute_copy(&hsrcfont), core::mem::transmute_copy(&phdestfont)).into()
        }
        unsafe extern "system" fn ReleaseFont<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hfont: super::Graphics::Gdi::HFONT) -> windows_core::HRESULT
        where
            Identity: IMLangFontLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangFontLink_Impl::ReleaseFont(this, core::mem::transmute_copy(&hfont)).into()
        }
        unsafe extern "system" fn ResetFontMapping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMLangFontLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangFontLink_Impl::ResetFontMapping(this).into()
        }
        Self {
            base__: IMLangCodePages_Vtbl::new::<Identity, OFFSET>(),
            GetFontCodePages: GetFontCodePages::<Identity, OFFSET>,
            MapFont: MapFont::<Identity, OFFSET>,
            ReleaseFont: ReleaseFont::<Identity, OFFSET>,
            ResetFontMapping: ResetFontMapping::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangFontLink as windows_core::Interface>::IID || iid == &<IMLangCodePages as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IMLangFontLink2_Impl: Sized + IMLangCodePages_Impl {
    fn GetFontCodePages(&self, hdc: super::Graphics::Gdi::HDC, hfont: super::Graphics::Gdi::HFONT, pdwcodepages: *mut u32) -> windows_core::Result<()>;
    fn ReleaseFont(&self, hfont: super::Graphics::Gdi::HFONT) -> windows_core::Result<()>;
    fn ResetFontMapping(&self) -> windows_core::Result<()>;
    fn MapFont(&self, hdc: super::Graphics::Gdi::HDC, dwcodepages: u32, chsrc: u16, pfont: *mut super::Graphics::Gdi::HFONT) -> windows_core::Result<()>;
    fn GetFontUnicodeRanges(&self, hdc: super::Graphics::Gdi::HDC, puiranges: *const u32, puranges: *mut UNICODERANGE) -> windows_core::Result<()>;
    fn GetScriptFontInfo(&self, sid: u8, dwflags: u32, puifonts: *mut u32, pscriptfont: *mut SCRIPTFONTINFO) -> windows_core::Result<()>;
    fn CodePageToScriptID(&self, uicodepage: u32) -> windows_core::Result<u8>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IMLangFontLink2 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IMLangFontLink2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMLangFontLink2_Vtbl
    where
        Identity: IMLangFontLink2_Impl,
    {
        unsafe extern "system" fn GetFontCodePages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, hfont: super::Graphics::Gdi::HFONT, pdwcodepages: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMLangFontLink2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangFontLink2_Impl::GetFontCodePages(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&hfont), core::mem::transmute_copy(&pdwcodepages)).into()
        }
        unsafe extern "system" fn ReleaseFont<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hfont: super::Graphics::Gdi::HFONT) -> windows_core::HRESULT
        where
            Identity: IMLangFontLink2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangFontLink2_Impl::ReleaseFont(this, core::mem::transmute_copy(&hfont)).into()
        }
        unsafe extern "system" fn ResetFontMapping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMLangFontLink2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangFontLink2_Impl::ResetFontMapping(this).into()
        }
        unsafe extern "system" fn MapFont<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, dwcodepages: u32, chsrc: u16, pfont: *mut super::Graphics::Gdi::HFONT) -> windows_core::HRESULT
        where
            Identity: IMLangFontLink2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangFontLink2_Impl::MapFont(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&dwcodepages), core::mem::transmute_copy(&chsrc), core::mem::transmute_copy(&pfont)).into()
        }
        unsafe extern "system" fn GetFontUnicodeRanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, puiranges: *const u32, puranges: *mut UNICODERANGE) -> windows_core::HRESULT
        where
            Identity: IMLangFontLink2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangFontLink2_Impl::GetFontUnicodeRanges(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&puiranges), core::mem::transmute_copy(&puranges)).into()
        }
        unsafe extern "system" fn GetScriptFontInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sid: u8, dwflags: u32, puifonts: *mut u32, pscriptfont: *mut SCRIPTFONTINFO) -> windows_core::HRESULT
        where
            Identity: IMLangFontLink2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangFontLink2_Impl::GetScriptFontInfo(this, core::mem::transmute_copy(&sid), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&puifonts), core::mem::transmute_copy(&pscriptfont)).into()
        }
        unsafe extern "system" fn CodePageToScriptID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32, psid: *mut u8) -> windows_core::HRESULT
        where
            Identity: IMLangFontLink2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMLangFontLink2_Impl::CodePageToScriptID(this, core::mem::transmute_copy(&uicodepage)) {
                Ok(ok__) => {
                    psid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IMLangCodePages_Vtbl::new::<Identity, OFFSET>(),
            GetFontCodePages: GetFontCodePages::<Identity, OFFSET>,
            ReleaseFont: ReleaseFont::<Identity, OFFSET>,
            ResetFontMapping: ResetFontMapping::<Identity, OFFSET>,
            MapFont: MapFont::<Identity, OFFSET>,
            GetFontUnicodeRanges: GetFontUnicodeRanges::<Identity, OFFSET>,
            GetScriptFontInfo: GetScriptFontInfo::<Identity, OFFSET>,
            CodePageToScriptID: CodePageToScriptID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangFontLink2 as windows_core::Interface>::IID || iid == &<IMLangCodePages as windows_core::Interface>::IID
    }
}
pub trait IMLangLineBreakConsole_Impl: Sized {
    fn BreakLineML(&self, psrcmlstr: Option<&IMLangString>, lsrcpos: i32, lsrclen: i32, cmincolumns: i32, cmaxcolumns: i32, pllinelen: *mut i32, plskiplen: *mut i32) -> windows_core::Result<()>;
    fn BreakLineW(&self, locale: u32, pszsrc: &windows_core::PCWSTR, cchsrc: i32, cmaxcolumns: i32, pcchline: *mut i32, pcchskip: *mut i32) -> windows_core::Result<()>;
    fn BreakLineA(&self, locale: u32, ucodepage: u32, pszsrc: &windows_core::PCSTR, cchsrc: i32, cmaxcolumns: i32, pcchline: *mut i32, pcchskip: *mut i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMLangLineBreakConsole {}
impl IMLangLineBreakConsole_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMLangLineBreakConsole_Vtbl
    where
        Identity: IMLangLineBreakConsole_Impl,
    {
        unsafe extern "system" fn BreakLineML<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrcmlstr: *mut core::ffi::c_void, lsrcpos: i32, lsrclen: i32, cmincolumns: i32, cmaxcolumns: i32, pllinelen: *mut i32, plskiplen: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMLangLineBreakConsole_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangLineBreakConsole_Impl::BreakLineML(this, windows_core::from_raw_borrowed(&psrcmlstr), core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrclen), core::mem::transmute_copy(&cmincolumns), core::mem::transmute_copy(&cmaxcolumns), core::mem::transmute_copy(&pllinelen), core::mem::transmute_copy(&plskiplen)).into()
        }
        unsafe extern "system" fn BreakLineW<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, locale: u32, pszsrc: windows_core::PCWSTR, cchsrc: i32, cmaxcolumns: i32, pcchline: *mut i32, pcchskip: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMLangLineBreakConsole_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangLineBreakConsole_Impl::BreakLineW(this, core::mem::transmute_copy(&locale), core::mem::transmute(&pszsrc), core::mem::transmute_copy(&cchsrc), core::mem::transmute_copy(&cmaxcolumns), core::mem::transmute_copy(&pcchline), core::mem::transmute_copy(&pcchskip)).into()
        }
        unsafe extern "system" fn BreakLineA<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, locale: u32, ucodepage: u32, pszsrc: windows_core::PCSTR, cchsrc: i32, cmaxcolumns: i32, pcchline: *mut i32, pcchskip: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMLangLineBreakConsole_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangLineBreakConsole_Impl::BreakLineA(this, core::mem::transmute_copy(&locale), core::mem::transmute_copy(&ucodepage), core::mem::transmute(&pszsrc), core::mem::transmute_copy(&cchsrc), core::mem::transmute_copy(&cmaxcolumns), core::mem::transmute_copy(&pcchline), core::mem::transmute_copy(&pcchskip)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BreakLineML: BreakLineML::<Identity, OFFSET>,
            BreakLineW: BreakLineW::<Identity, OFFSET>,
            BreakLineA: BreakLineA::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangLineBreakConsole as windows_core::Interface>::IID
    }
}
pub trait IMLangString_Impl: Sized {
    fn Sync(&self, fnoaccess: super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetLength(&self, pllen: *mut i32) -> windows_core::Result<()>;
    fn SetMLStr(&self, ldestpos: i32, ldestlen: i32, psrcmlstr: Option<&windows_core::IUnknown>, lsrcpos: i32, lsrclen: i32) -> windows_core::Result<()>;
    fn GetMLStr(&self, lsrcpos: i32, lsrclen: i32, punkouter: Option<&windows_core::IUnknown>, dwclscontext: u32, piid: *const windows_core::GUID, ppdestmlstr: *mut Option<windows_core::IUnknown>, pldestpos: *mut i32, pldestlen: *mut i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMLangString {}
impl IMLangString_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMLangString_Vtbl
    where
        Identity: IMLangString_Impl,
    {
        unsafe extern "system" fn Sync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fnoaccess: super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IMLangString_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangString_Impl::Sync(this, core::mem::transmute_copy(&fnoaccess)).into()
        }
        unsafe extern "system" fn GetLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllen: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMLangString_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangString_Impl::GetLength(this, core::mem::transmute_copy(&pllen)).into()
        }
        unsafe extern "system" fn SetMLStr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldestpos: i32, ldestlen: i32, psrcmlstr: *mut core::ffi::c_void, lsrcpos: i32, lsrclen: i32) -> windows_core::HRESULT
        where
            Identity: IMLangString_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangString_Impl::SetMLStr(this, core::mem::transmute_copy(&ldestpos), core::mem::transmute_copy(&ldestlen), windows_core::from_raw_borrowed(&psrcmlstr), core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrclen)).into()
        }
        unsafe extern "system" fn GetMLStr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrclen: i32, punkouter: *mut core::ffi::c_void, dwclscontext: u32, piid: *const windows_core::GUID, ppdestmlstr: *mut *mut core::ffi::c_void, pldestpos: *mut i32, pldestlen: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMLangString_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangString_Impl::GetMLStr(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrclen), windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&dwclscontext), core::mem::transmute_copy(&piid), core::mem::transmute_copy(&ppdestmlstr), core::mem::transmute_copy(&pldestpos), core::mem::transmute_copy(&pldestlen)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Sync: Sync::<Identity, OFFSET>,
            GetLength: GetLength::<Identity, OFFSET>,
            SetMLStr: SetMLStr::<Identity, OFFSET>,
            GetMLStr: GetMLStr::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangString as windows_core::Interface>::IID
    }
}
pub trait IMLangStringAStr_Impl: Sized + IMLangString_Impl {
    fn SetAStr(&self, ldestpos: i32, ldestlen: i32, ucodepage: u32, pszsrc: &windows_core::PCSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::Result<()>;
    fn SetStrBufA(&self, ldestpos: i32, ldestlen: i32, ucodepage: u32, psrcbuf: Option<&IMLangStringBufA>, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::Result<()>;
    fn GetAStr(&self, lsrcpos: i32, lsrclen: i32, ucodepagein: u32, pucodepageout: *const u32, pszdest: windows_core::PSTR, cchdest: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::Result<()>;
    fn GetStrBufA(&self, lsrcpos: i32, lsrcmaxlen: i32, pudestcodepage: *mut u32, ppdestbuf: *mut Option<IMLangStringBufA>, pldestlen: *mut i32) -> windows_core::Result<()>;
    fn LockAStr(&self, lsrcpos: i32, lsrclen: i32, lflags: i32, ucodepagein: u32, cchrequest: i32, pucodepageout: *mut u32, ppszdest: *mut windows_core::PSTR, pcchdest: *mut i32, pldestlen: *mut i32) -> windows_core::Result<()>;
    fn UnlockAStr(&self, pszsrc: &windows_core::PCSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::Result<()>;
    fn SetLocale(&self, ldestpos: i32, ldestlen: i32, locale: u32) -> windows_core::Result<()>;
    fn GetLocale(&self, lsrcpos: i32, lsrcmaxlen: i32, plocale: *mut u32, pllocalepos: *mut i32, pllocalelen: *mut i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMLangStringAStr {}
impl IMLangStringAStr_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMLangStringAStr_Vtbl
    where
        Identity: IMLangStringAStr_Impl,
    {
        unsafe extern "system" fn SetAStr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldestpos: i32, ldestlen: i32, ucodepage: u32, pszsrc: windows_core::PCSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMLangStringAStr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangStringAStr_Impl::SetAStr(this, core::mem::transmute_copy(&ldestpos), core::mem::transmute_copy(&ldestlen), core::mem::transmute_copy(&ucodepage), core::mem::transmute(&pszsrc), core::mem::transmute_copy(&cchsrc), core::mem::transmute_copy(&pcchactual), core::mem::transmute_copy(&plactuallen)).into()
        }
        unsafe extern "system" fn SetStrBufA<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldestpos: i32, ldestlen: i32, ucodepage: u32, psrcbuf: *mut core::ffi::c_void, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMLangStringAStr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangStringAStr_Impl::SetStrBufA(this, core::mem::transmute_copy(&ldestpos), core::mem::transmute_copy(&ldestlen), core::mem::transmute_copy(&ucodepage), windows_core::from_raw_borrowed(&psrcbuf), core::mem::transmute_copy(&pcchactual), core::mem::transmute_copy(&plactuallen)).into()
        }
        unsafe extern "system" fn GetAStr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrclen: i32, ucodepagein: u32, pucodepageout: *const u32, pszdest: windows_core::PSTR, cchdest: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMLangStringAStr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangStringAStr_Impl::GetAStr(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrclen), core::mem::transmute_copy(&ucodepagein), core::mem::transmute_copy(&pucodepageout), core::mem::transmute_copy(&pszdest), core::mem::transmute_copy(&cchdest), core::mem::transmute_copy(&pcchactual), core::mem::transmute_copy(&plactuallen)).into()
        }
        unsafe extern "system" fn GetStrBufA<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, pudestcodepage: *mut u32, ppdestbuf: *mut *mut core::ffi::c_void, pldestlen: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMLangStringAStr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangStringAStr_Impl::GetStrBufA(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrcmaxlen), core::mem::transmute_copy(&pudestcodepage), core::mem::transmute_copy(&ppdestbuf), core::mem::transmute_copy(&pldestlen)).into()
        }
        unsafe extern "system" fn LockAStr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrclen: i32, lflags: i32, ucodepagein: u32, cchrequest: i32, pucodepageout: *mut u32, ppszdest: *mut windows_core::PSTR, pcchdest: *mut i32, pldestlen: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMLangStringAStr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangStringAStr_Impl::LockAStr(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrclen), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&ucodepagein), core::mem::transmute_copy(&cchrequest), core::mem::transmute_copy(&pucodepageout), core::mem::transmute_copy(&ppszdest), core::mem::transmute_copy(&pcchdest), core::mem::transmute_copy(&pldestlen)).into()
        }
        unsafe extern "system" fn UnlockAStr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsrc: windows_core::PCSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMLangStringAStr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangStringAStr_Impl::UnlockAStr(this, core::mem::transmute(&pszsrc), core::mem::transmute_copy(&cchsrc), core::mem::transmute_copy(&pcchactual), core::mem::transmute_copy(&plactuallen)).into()
        }
        unsafe extern "system" fn SetLocale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldestpos: i32, ldestlen: i32, locale: u32) -> windows_core::HRESULT
        where
            Identity: IMLangStringAStr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangStringAStr_Impl::SetLocale(this, core::mem::transmute_copy(&ldestpos), core::mem::transmute_copy(&ldestlen), core::mem::transmute_copy(&locale)).into()
        }
        unsafe extern "system" fn GetLocale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, plocale: *mut u32, pllocalepos: *mut i32, pllocalelen: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMLangStringAStr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangStringAStr_Impl::GetLocale(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrcmaxlen), core::mem::transmute_copy(&plocale), core::mem::transmute_copy(&pllocalepos), core::mem::transmute_copy(&pllocalelen)).into()
        }
        Self {
            base__: IMLangString_Vtbl::new::<Identity, OFFSET>(),
            SetAStr: SetAStr::<Identity, OFFSET>,
            SetStrBufA: SetStrBufA::<Identity, OFFSET>,
            GetAStr: GetAStr::<Identity, OFFSET>,
            GetStrBufA: GetStrBufA::<Identity, OFFSET>,
            LockAStr: LockAStr::<Identity, OFFSET>,
            UnlockAStr: UnlockAStr::<Identity, OFFSET>,
            SetLocale: SetLocale::<Identity, OFFSET>,
            GetLocale: GetLocale::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangStringAStr as windows_core::Interface>::IID || iid == &<IMLangString as windows_core::Interface>::IID
    }
}
pub trait IMLangStringBufA_Impl: Sized {
    fn GetStatus(&self, plflags: *mut i32, pcchbuf: *mut i32) -> windows_core::Result<()>;
    fn LockBuf(&self, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut i8, pcchbuf: *mut i32) -> windows_core::Result<()>;
    fn UnlockBuf(&self, pszbuf: &windows_core::PCSTR, cchoffset: i32, cchwrite: i32) -> windows_core::Result<()>;
    fn Insert(&self, cchoffset: i32, cchmaxinsert: i32, pcchactual: *mut i32) -> windows_core::Result<()>;
    fn Delete(&self, cchoffset: i32, cchdelete: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMLangStringBufA {}
impl IMLangStringBufA_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMLangStringBufA_Vtbl
    where
        Identity: IMLangStringBufA_Impl,
    {
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plflags: *mut i32, pcchbuf: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMLangStringBufA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangStringBufA_Impl::GetStatus(this, core::mem::transmute_copy(&plflags), core::mem::transmute_copy(&pcchbuf)).into()
        }
        unsafe extern "system" fn LockBuf<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut i8, pcchbuf: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMLangStringBufA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangStringBufA_Impl::LockBuf(this, core::mem::transmute_copy(&cchoffset), core::mem::transmute_copy(&cchmaxlock), core::mem::transmute_copy(&ppszbuf), core::mem::transmute_copy(&pcchbuf)).into()
        }
        unsafe extern "system" fn UnlockBuf<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszbuf: windows_core::PCSTR, cchoffset: i32, cchwrite: i32) -> windows_core::HRESULT
        where
            Identity: IMLangStringBufA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangStringBufA_Impl::UnlockBuf(this, core::mem::transmute(&pszbuf), core::mem::transmute_copy(&cchoffset), core::mem::transmute_copy(&cchwrite)).into()
        }
        unsafe extern "system" fn Insert<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchoffset: i32, cchmaxinsert: i32, pcchactual: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMLangStringBufA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangStringBufA_Impl::Insert(this, core::mem::transmute_copy(&cchoffset), core::mem::transmute_copy(&cchmaxinsert), core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchoffset: i32, cchdelete: i32) -> windows_core::HRESULT
        where
            Identity: IMLangStringBufA_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangStringBufA_Impl::Delete(this, core::mem::transmute_copy(&cchoffset), core::mem::transmute_copy(&cchdelete)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStatus: GetStatus::<Identity, OFFSET>,
            LockBuf: LockBuf::<Identity, OFFSET>,
            UnlockBuf: UnlockBuf::<Identity, OFFSET>,
            Insert: Insert::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangStringBufA as windows_core::Interface>::IID
    }
}
pub trait IMLangStringBufW_Impl: Sized {
    fn GetStatus(&self, plflags: *mut i32, pcchbuf: *mut i32) -> windows_core::Result<()>;
    fn LockBuf(&self, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut u16, pcchbuf: *mut i32) -> windows_core::Result<()>;
    fn UnlockBuf(&self, pszbuf: &windows_core::PCWSTR, cchoffset: i32, cchwrite: i32) -> windows_core::Result<()>;
    fn Insert(&self, cchoffset: i32, cchmaxinsert: i32, pcchactual: *mut i32) -> windows_core::Result<()>;
    fn Delete(&self, cchoffset: i32, cchdelete: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMLangStringBufW {}
impl IMLangStringBufW_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMLangStringBufW_Vtbl
    where
        Identity: IMLangStringBufW_Impl,
    {
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plflags: *mut i32, pcchbuf: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMLangStringBufW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangStringBufW_Impl::GetStatus(this, core::mem::transmute_copy(&plflags), core::mem::transmute_copy(&pcchbuf)).into()
        }
        unsafe extern "system" fn LockBuf<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut u16, pcchbuf: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMLangStringBufW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangStringBufW_Impl::LockBuf(this, core::mem::transmute_copy(&cchoffset), core::mem::transmute_copy(&cchmaxlock), core::mem::transmute_copy(&ppszbuf), core::mem::transmute_copy(&pcchbuf)).into()
        }
        unsafe extern "system" fn UnlockBuf<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszbuf: windows_core::PCWSTR, cchoffset: i32, cchwrite: i32) -> windows_core::HRESULT
        where
            Identity: IMLangStringBufW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangStringBufW_Impl::UnlockBuf(this, core::mem::transmute(&pszbuf), core::mem::transmute_copy(&cchoffset), core::mem::transmute_copy(&cchwrite)).into()
        }
        unsafe extern "system" fn Insert<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchoffset: i32, cchmaxinsert: i32, pcchactual: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMLangStringBufW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangStringBufW_Impl::Insert(this, core::mem::transmute_copy(&cchoffset), core::mem::transmute_copy(&cchmaxinsert), core::mem::transmute_copy(&pcchactual)).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchoffset: i32, cchdelete: i32) -> windows_core::HRESULT
        where
            Identity: IMLangStringBufW_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangStringBufW_Impl::Delete(this, core::mem::transmute_copy(&cchoffset), core::mem::transmute_copy(&cchdelete)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStatus: GetStatus::<Identity, OFFSET>,
            LockBuf: LockBuf::<Identity, OFFSET>,
            UnlockBuf: UnlockBuf::<Identity, OFFSET>,
            Insert: Insert::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangStringBufW as windows_core::Interface>::IID
    }
}
pub trait IMLangStringWStr_Impl: Sized + IMLangString_Impl {
    fn SetWStr(&self, ldestpos: i32, ldestlen: i32, pszsrc: &windows_core::PCWSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::Result<()>;
    fn SetStrBufW(&self, ldestpos: i32, ldestlen: i32, psrcbuf: Option<&IMLangStringBufW>, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::Result<()>;
    fn GetWStr(&self, lsrcpos: i32, lsrclen: i32, pszdest: windows_core::PWSTR, cchdest: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::Result<()>;
    fn GetStrBufW(&self, lsrcpos: i32, lsrcmaxlen: i32, ppdestbuf: *mut Option<IMLangStringBufW>, pldestlen: *mut i32) -> windows_core::Result<()>;
    fn LockWStr(&self, lsrcpos: i32, lsrclen: i32, lflags: i32, cchrequest: i32, ppszdest: *mut windows_core::PWSTR, pcchdest: *mut i32, pldestlen: *mut i32) -> windows_core::Result<()>;
    fn UnlockWStr(&self, pszsrc: &windows_core::PCWSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::Result<()>;
    fn SetLocale(&self, ldestpos: i32, ldestlen: i32, locale: u32) -> windows_core::Result<()>;
    fn GetLocale(&self, lsrcpos: i32, lsrcmaxlen: i32, plocale: *mut u32, pllocalepos: *mut i32, pllocalelen: *mut i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMLangStringWStr {}
impl IMLangStringWStr_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMLangStringWStr_Vtbl
    where
        Identity: IMLangStringWStr_Impl,
    {
        unsafe extern "system" fn SetWStr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldestpos: i32, ldestlen: i32, pszsrc: windows_core::PCWSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMLangStringWStr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangStringWStr_Impl::SetWStr(this, core::mem::transmute_copy(&ldestpos), core::mem::transmute_copy(&ldestlen), core::mem::transmute(&pszsrc), core::mem::transmute_copy(&cchsrc), core::mem::transmute_copy(&pcchactual), core::mem::transmute_copy(&plactuallen)).into()
        }
        unsafe extern "system" fn SetStrBufW<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldestpos: i32, ldestlen: i32, psrcbuf: *mut core::ffi::c_void, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMLangStringWStr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangStringWStr_Impl::SetStrBufW(this, core::mem::transmute_copy(&ldestpos), core::mem::transmute_copy(&ldestlen), windows_core::from_raw_borrowed(&psrcbuf), core::mem::transmute_copy(&pcchactual), core::mem::transmute_copy(&plactuallen)).into()
        }
        unsafe extern "system" fn GetWStr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrclen: i32, pszdest: windows_core::PWSTR, cchdest: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMLangStringWStr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangStringWStr_Impl::GetWStr(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrclen), core::mem::transmute_copy(&pszdest), core::mem::transmute_copy(&cchdest), core::mem::transmute_copy(&pcchactual), core::mem::transmute_copy(&plactuallen)).into()
        }
        unsafe extern "system" fn GetStrBufW<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, ppdestbuf: *mut *mut core::ffi::c_void, pldestlen: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMLangStringWStr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangStringWStr_Impl::GetStrBufW(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrcmaxlen), core::mem::transmute_copy(&ppdestbuf), core::mem::transmute_copy(&pldestlen)).into()
        }
        unsafe extern "system" fn LockWStr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrclen: i32, lflags: i32, cchrequest: i32, ppszdest: *mut windows_core::PWSTR, pcchdest: *mut i32, pldestlen: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMLangStringWStr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangStringWStr_Impl::LockWStr(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrclen), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&cchrequest), core::mem::transmute_copy(&ppszdest), core::mem::transmute_copy(&pcchdest), core::mem::transmute_copy(&pldestlen)).into()
        }
        unsafe extern "system" fn UnlockWStr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsrc: windows_core::PCWSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMLangStringWStr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangStringWStr_Impl::UnlockWStr(this, core::mem::transmute(&pszsrc), core::mem::transmute_copy(&cchsrc), core::mem::transmute_copy(&pcchactual), core::mem::transmute_copy(&plactuallen)).into()
        }
        unsafe extern "system" fn SetLocale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldestpos: i32, ldestlen: i32, locale: u32) -> windows_core::HRESULT
        where
            Identity: IMLangStringWStr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangStringWStr_Impl::SetLocale(this, core::mem::transmute_copy(&ldestpos), core::mem::transmute_copy(&ldestlen), core::mem::transmute_copy(&locale)).into()
        }
        unsafe extern "system" fn GetLocale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, plocale: *mut u32, pllocalepos: *mut i32, pllocalelen: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMLangStringWStr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMLangStringWStr_Impl::GetLocale(this, core::mem::transmute_copy(&lsrcpos), core::mem::transmute_copy(&lsrcmaxlen), core::mem::transmute_copy(&plocale), core::mem::transmute_copy(&pllocalepos), core::mem::transmute_copy(&pllocalelen)).into()
        }
        Self {
            base__: IMLangString_Vtbl::new::<Identity, OFFSET>(),
            SetWStr: SetWStr::<Identity, OFFSET>,
            SetStrBufW: SetStrBufW::<Identity, OFFSET>,
            GetWStr: GetWStr::<Identity, OFFSET>,
            GetStrBufW: GetStrBufW::<Identity, OFFSET>,
            LockWStr: LockWStr::<Identity, OFFSET>,
            UnlockWStr: UnlockWStr::<Identity, OFFSET>,
            SetLocale: SetLocale::<Identity, OFFSET>,
            GetLocale: GetLocale::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMLangStringWStr as windows_core::Interface>::IID || iid == &<IMLangString as windows_core::Interface>::IID
    }
}
pub trait IMultiLanguage_Impl: Sized {
    fn GetNumberOfCodePageInfo(&self) -> windows_core::Result<u32>;
    fn GetCodePageInfo(&self, uicodepage: u32, pcodepageinfo: *mut MIMECPINFO) -> windows_core::Result<()>;
    fn GetFamilyCodePage(&self, uicodepage: u32) -> windows_core::Result<u32>;
    fn EnumCodePages(&self, grfflags: u32) -> windows_core::Result<IEnumCodePage>;
    fn GetCharsetInfo(&self, charset: &windows_core::BSTR, pcharsetinfo: *mut MIMECSETINFO) -> windows_core::Result<()>;
    fn IsConvertible(&self, dwsrcencoding: u32, dwdstencoding: u32) -> windows_core::Result<()>;
    fn ConvertString(&self, pdwmode: *mut u32, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> windows_core::Result<()>;
    fn ConvertStringToUnicode(&self, pdwmode: *mut u32, dwencoding: u32, psrcstr: &windows_core::PCSTR, pcsrcsize: *mut u32, pdststr: windows_core::PWSTR, pcdstsize: *mut u32) -> windows_core::Result<()>;
    fn ConvertStringFromUnicode(&self, pdwmode: *mut u32, dwencoding: u32, psrcstr: &windows_core::PCWSTR, pcsrcsize: *mut u32, pdststr: windows_core::PSTR, pcdstsize: *mut u32) -> windows_core::Result<()>;
    fn ConvertStringReset(&self) -> windows_core::Result<()>;
    fn GetRfc1766FromLcid(&self, locale: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetLcidFromRfc1766(&self, plocale: *mut u32, bstrrfc1766: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EnumRfc1766(&self) -> windows_core::Result<IEnumRfc1766>;
    fn GetRfc1766Info(&self, locale: u32, prfc1766info: *mut RFC1766INFO) -> windows_core::Result<()>;
    fn CreateConvertCharset(&self, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> windows_core::Result<IMLangConvertCharset>;
}
impl windows_core::RuntimeName for IMultiLanguage {}
impl IMultiLanguage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMultiLanguage_Vtbl
    where
        Identity: IMultiLanguage_Impl,
    {
        unsafe extern "system" fn GetNumberOfCodePageInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccodepage: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultiLanguage_Impl::GetNumberOfCodePageInfo(this) {
                Ok(ok__) => {
                    pccodepage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodePageInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32, pcodepageinfo: *mut MIMECPINFO) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage_Impl::GetCodePageInfo(this, core::mem::transmute_copy(&uicodepage), core::mem::transmute_copy(&pcodepageinfo)).into()
        }
        unsafe extern "system" fn GetFamilyCodePage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32, puifamilycodepage: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultiLanguage_Impl::GetFamilyCodePage(this, core::mem::transmute_copy(&uicodepage)) {
                Ok(ok__) => {
                    puifamilycodepage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumCodePages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfflags: u32, ppenumcodepage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultiLanguage_Impl::EnumCodePages(this, core::mem::transmute_copy(&grfflags)) {
                Ok(ok__) => {
                    ppenumcodepage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCharsetInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, charset: core::mem::MaybeUninit<windows_core::BSTR>, pcharsetinfo: *mut MIMECSETINFO) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage_Impl::GetCharsetInfo(this, core::mem::transmute(&charset), core::mem::transmute_copy(&pcharsetinfo)).into()
        }
        unsafe extern "system" fn IsConvertible<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsrcencoding: u32, dwdstencoding: u32) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage_Impl::IsConvertible(this, core::mem::transmute_copy(&dwsrcencoding), core::mem::transmute_copy(&dwdstencoding)).into()
        }
        unsafe extern "system" fn ConvertString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage_Impl::ConvertString(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwsrcencoding), core::mem::transmute_copy(&dwdstencoding), core::mem::transmute_copy(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
        }
        unsafe extern "system" fn ConvertStringToUnicode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: windows_core::PCSTR, pcsrcsize: *mut u32, pdststr: windows_core::PWSTR, pcdstsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage_Impl::ConvertStringToUnicode(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwencoding), core::mem::transmute(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
        }
        unsafe extern "system" fn ConvertStringFromUnicode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: windows_core::PCWSTR, pcsrcsize: *mut u32, pdststr: windows_core::PSTR, pcdstsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage_Impl::ConvertStringFromUnicode(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwencoding), core::mem::transmute(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
        }
        unsafe extern "system" fn ConvertStringReset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage_Impl::ConvertStringReset(this).into()
        }
        unsafe extern "system" fn GetRfc1766FromLcid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, locale: u32, pbstrrfc1766: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultiLanguage_Impl::GetRfc1766FromLcid(this, core::mem::transmute_copy(&locale)) {
                Ok(ok__) => {
                    pbstrrfc1766.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLcidFromRfc1766<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plocale: *mut u32, bstrrfc1766: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage_Impl::GetLcidFromRfc1766(this, core::mem::transmute_copy(&plocale), core::mem::transmute(&bstrrfc1766)).into()
        }
        unsafe extern "system" fn EnumRfc1766<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumrfc1766: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultiLanguage_Impl::EnumRfc1766(this) {
                Ok(ok__) => {
                    ppenumrfc1766.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRfc1766Info<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, locale: u32, prfc1766info: *mut RFC1766INFO) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage_Impl::GetRfc1766Info(this, core::mem::transmute_copy(&locale), core::mem::transmute_copy(&prfc1766info)).into()
        }
        unsafe extern "system" fn CreateConvertCharset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32, ppmlangconvertcharset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultiLanguage_Impl::CreateConvertCharset(this, core::mem::transmute_copy(&uisrccodepage), core::mem::transmute_copy(&uidstcodepage), core::mem::transmute_copy(&dwproperty)) {
                Ok(ok__) => {
                    ppmlangconvertcharset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNumberOfCodePageInfo: GetNumberOfCodePageInfo::<Identity, OFFSET>,
            GetCodePageInfo: GetCodePageInfo::<Identity, OFFSET>,
            GetFamilyCodePage: GetFamilyCodePage::<Identity, OFFSET>,
            EnumCodePages: EnumCodePages::<Identity, OFFSET>,
            GetCharsetInfo: GetCharsetInfo::<Identity, OFFSET>,
            IsConvertible: IsConvertible::<Identity, OFFSET>,
            ConvertString: ConvertString::<Identity, OFFSET>,
            ConvertStringToUnicode: ConvertStringToUnicode::<Identity, OFFSET>,
            ConvertStringFromUnicode: ConvertStringFromUnicode::<Identity, OFFSET>,
            ConvertStringReset: ConvertStringReset::<Identity, OFFSET>,
            GetRfc1766FromLcid: GetRfc1766FromLcid::<Identity, OFFSET>,
            GetLcidFromRfc1766: GetLcidFromRfc1766::<Identity, OFFSET>,
            EnumRfc1766: EnumRfc1766::<Identity, OFFSET>,
            GetRfc1766Info: GetRfc1766Info::<Identity, OFFSET>,
            CreateConvertCharset: CreateConvertCharset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultiLanguage as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMultiLanguage2_Impl: Sized {
    fn GetNumberOfCodePageInfo(&self) -> windows_core::Result<u32>;
    fn GetCodePageInfo(&self, uicodepage: u32, langid: u16, pcodepageinfo: *mut MIMECPINFO) -> windows_core::Result<()>;
    fn GetFamilyCodePage(&self, uicodepage: u32) -> windows_core::Result<u32>;
    fn EnumCodePages(&self, grfflags: u32, langid: u16) -> windows_core::Result<IEnumCodePage>;
    fn GetCharsetInfo(&self, charset: &windows_core::BSTR, pcharsetinfo: *mut MIMECSETINFO) -> windows_core::Result<()>;
    fn IsConvertible(&self, dwsrcencoding: u32, dwdstencoding: u32) -> windows_core::Result<()>;
    fn ConvertString(&self, pdwmode: *mut u32, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> windows_core::Result<()>;
    fn ConvertStringToUnicode(&self, pdwmode: *mut u32, dwencoding: u32, psrcstr: &windows_core::PCSTR, pcsrcsize: *mut u32, pdststr: windows_core::PWSTR, pcdstsize: *mut u32) -> windows_core::Result<()>;
    fn ConvertStringFromUnicode(&self, pdwmode: *mut u32, dwencoding: u32, psrcstr: &windows_core::PCWSTR, pcsrcsize: *mut u32, pdststr: windows_core::PSTR, pcdstsize: *mut u32) -> windows_core::Result<()>;
    fn ConvertStringReset(&self) -> windows_core::Result<()>;
    fn GetRfc1766FromLcid(&self, locale: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetLcidFromRfc1766(&self, plocale: *mut u32, bstrrfc1766: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EnumRfc1766(&self, langid: u16) -> windows_core::Result<IEnumRfc1766>;
    fn GetRfc1766Info(&self, locale: u32, langid: u16, prfc1766info: *mut RFC1766INFO) -> windows_core::Result<()>;
    fn CreateConvertCharset(&self, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> windows_core::Result<IMLangConvertCharset>;
    fn ConvertStringInIStream(&self, pdwmode: *mut u32, dwflag: u32, lpfallback: &windows_core::PCWSTR, dwsrcencoding: u32, dwdstencoding: u32, pstmin: Option<&super::System::Com::IStream>, pstmout: Option<&super::System::Com::IStream>) -> windows_core::Result<()>;
    fn ConvertStringToUnicodeEx(&self, pdwmode: *mut u32, dwencoding: u32, psrcstr: &windows_core::PCSTR, pcsrcsize: *mut u32, pdststr: windows_core::PWSTR, pcdstsize: *mut u32, dwflag: u32, lpfallback: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ConvertStringFromUnicodeEx(&self, pdwmode: *mut u32, dwencoding: u32, psrcstr: &windows_core::PCWSTR, pcsrcsize: *mut u32, pdststr: windows_core::PSTR, pcdstsize: *mut u32, dwflag: u32, lpfallback: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn DetectCodepageInIStream(&self, dwflag: u32, dwprefwincodepage: u32, pstmin: Option<&super::System::Com::IStream>, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> windows_core::Result<()>;
    fn DetectInputCodepage(&self, dwflag: u32, dwprefwincodepage: u32, psrcstr: &windows_core::PCSTR, pcsrcsize: *mut i32, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> windows_core::Result<()>;
    fn ValidateCodePage(&self, uicodepage: u32, hwnd: super::Foundation::HWND) -> windows_core::Result<()>;
    fn GetCodePageDescription(&self, uicodepage: u32, lcid: u32, lpwidecharstr: windows_core::PWSTR, cchwidechar: i32) -> windows_core::Result<()>;
    fn IsCodePageInstallable(&self, uicodepage: u32) -> windows_core::Result<()>;
    fn SetMimeDBSource(&self, dwsource: MIMECONTF) -> windows_core::Result<()>;
    fn GetNumberOfScripts(&self) -> windows_core::Result<u32>;
    fn EnumScripts(&self, dwflags: u32, langid: u16) -> windows_core::Result<IEnumScript>;
    fn ValidateCodePageEx(&self, uicodepage: u32, hwnd: super::Foundation::HWND, dwfiodcontrol: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMultiLanguage2 {}
#[cfg(feature = "Win32_System_Com")]
impl IMultiLanguage2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMultiLanguage2_Vtbl
    where
        Identity: IMultiLanguage2_Impl,
    {
        unsafe extern "system" fn GetNumberOfCodePageInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccodepage: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultiLanguage2_Impl::GetNumberOfCodePageInfo(this) {
                Ok(ok__) => {
                    pccodepage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodePageInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32, langid: u16, pcodepageinfo: *mut MIMECPINFO) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage2_Impl::GetCodePageInfo(this, core::mem::transmute_copy(&uicodepage), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&pcodepageinfo)).into()
        }
        unsafe extern "system" fn GetFamilyCodePage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32, puifamilycodepage: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultiLanguage2_Impl::GetFamilyCodePage(this, core::mem::transmute_copy(&uicodepage)) {
                Ok(ok__) => {
                    puifamilycodepage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumCodePages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfflags: u32, langid: u16, ppenumcodepage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultiLanguage2_Impl::EnumCodePages(this, core::mem::transmute_copy(&grfflags), core::mem::transmute_copy(&langid)) {
                Ok(ok__) => {
                    ppenumcodepage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCharsetInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, charset: core::mem::MaybeUninit<windows_core::BSTR>, pcharsetinfo: *mut MIMECSETINFO) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage2_Impl::GetCharsetInfo(this, core::mem::transmute(&charset), core::mem::transmute_copy(&pcharsetinfo)).into()
        }
        unsafe extern "system" fn IsConvertible<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsrcencoding: u32, dwdstencoding: u32) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage2_Impl::IsConvertible(this, core::mem::transmute_copy(&dwsrcencoding), core::mem::transmute_copy(&dwdstencoding)).into()
        }
        unsafe extern "system" fn ConvertString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage2_Impl::ConvertString(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwsrcencoding), core::mem::transmute_copy(&dwdstencoding), core::mem::transmute_copy(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
        }
        unsafe extern "system" fn ConvertStringToUnicode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: windows_core::PCSTR, pcsrcsize: *mut u32, pdststr: windows_core::PWSTR, pcdstsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage2_Impl::ConvertStringToUnicode(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwencoding), core::mem::transmute(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
        }
        unsafe extern "system" fn ConvertStringFromUnicode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: windows_core::PCWSTR, pcsrcsize: *mut u32, pdststr: windows_core::PSTR, pcdstsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage2_Impl::ConvertStringFromUnicode(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwencoding), core::mem::transmute(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize)).into()
        }
        unsafe extern "system" fn ConvertStringReset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage2_Impl::ConvertStringReset(this).into()
        }
        unsafe extern "system" fn GetRfc1766FromLcid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, locale: u32, pbstrrfc1766: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultiLanguage2_Impl::GetRfc1766FromLcid(this, core::mem::transmute_copy(&locale)) {
                Ok(ok__) => {
                    pbstrrfc1766.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLcidFromRfc1766<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plocale: *mut u32, bstrrfc1766: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage2_Impl::GetLcidFromRfc1766(this, core::mem::transmute_copy(&plocale), core::mem::transmute(&bstrrfc1766)).into()
        }
        unsafe extern "system" fn EnumRfc1766<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: u16, ppenumrfc1766: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultiLanguage2_Impl::EnumRfc1766(this, core::mem::transmute_copy(&langid)) {
                Ok(ok__) => {
                    ppenumrfc1766.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRfc1766Info<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, locale: u32, langid: u16, prfc1766info: *mut RFC1766INFO) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage2_Impl::GetRfc1766Info(this, core::mem::transmute_copy(&locale), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&prfc1766info)).into()
        }
        unsafe extern "system" fn CreateConvertCharset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32, ppmlangconvertcharset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultiLanguage2_Impl::CreateConvertCharset(this, core::mem::transmute_copy(&uisrccodepage), core::mem::transmute_copy(&uidstcodepage), core::mem::transmute_copy(&dwproperty)) {
                Ok(ok__) => {
                    ppmlangconvertcharset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertStringInIStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwflag: u32, lpfallback: windows_core::PCWSTR, dwsrcencoding: u32, dwdstencoding: u32, pstmin: *mut core::ffi::c_void, pstmout: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage2_Impl::ConvertStringInIStream(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwflag), core::mem::transmute(&lpfallback), core::mem::transmute_copy(&dwsrcencoding), core::mem::transmute_copy(&dwdstencoding), windows_core::from_raw_borrowed(&pstmin), windows_core::from_raw_borrowed(&pstmout)).into()
        }
        unsafe extern "system" fn ConvertStringToUnicodeEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: windows_core::PCSTR, pcsrcsize: *mut u32, pdststr: windows_core::PWSTR, pcdstsize: *mut u32, dwflag: u32, lpfallback: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage2_Impl::ConvertStringToUnicodeEx(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwencoding), core::mem::transmute(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize), core::mem::transmute_copy(&dwflag), core::mem::transmute(&lpfallback)).into()
        }
        unsafe extern "system" fn ConvertStringFromUnicodeEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: windows_core::PCWSTR, pcsrcsize: *mut u32, pdststr: windows_core::PSTR, pcdstsize: *mut u32, dwflag: u32, lpfallback: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage2_Impl::ConvertStringFromUnicodeEx(this, core::mem::transmute_copy(&pdwmode), core::mem::transmute_copy(&dwencoding), core::mem::transmute(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&pdststr), core::mem::transmute_copy(&pcdstsize), core::mem::transmute_copy(&dwflag), core::mem::transmute(&lpfallback)).into()
        }
        unsafe extern "system" fn DetectCodepageInIStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflag: u32, dwprefwincodepage: u32, pstmin: *mut core::ffi::c_void, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage2_Impl::DetectCodepageInIStream(this, core::mem::transmute_copy(&dwflag), core::mem::transmute_copy(&dwprefwincodepage), windows_core::from_raw_borrowed(&pstmin), core::mem::transmute_copy(&lpencoding), core::mem::transmute_copy(&pnscores)).into()
        }
        unsafe extern "system" fn DetectInputCodepage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflag: u32, dwprefwincodepage: u32, psrcstr: windows_core::PCSTR, pcsrcsize: *mut i32, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage2_Impl::DetectInputCodepage(this, core::mem::transmute_copy(&dwflag), core::mem::transmute_copy(&dwprefwincodepage), core::mem::transmute(&psrcstr), core::mem::transmute_copy(&pcsrcsize), core::mem::transmute_copy(&lpencoding), core::mem::transmute_copy(&pnscores)).into()
        }
        unsafe extern "system" fn ValidateCodePage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32, hwnd: super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage2_Impl::ValidateCodePage(this, core::mem::transmute_copy(&uicodepage), core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn GetCodePageDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32, lcid: u32, lpwidecharstr: windows_core::PWSTR, cchwidechar: i32) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage2_Impl::GetCodePageDescription(this, core::mem::transmute_copy(&uicodepage), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&lpwidecharstr), core::mem::transmute_copy(&cchwidechar)).into()
        }
        unsafe extern "system" fn IsCodePageInstallable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage2_Impl::IsCodePageInstallable(this, core::mem::transmute_copy(&uicodepage)).into()
        }
        unsafe extern "system" fn SetMimeDBSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsource: MIMECONTF) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage2_Impl::SetMimeDBSource(this, core::mem::transmute_copy(&dwsource)).into()
        }
        unsafe extern "system" fn GetNumberOfScripts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnscripts: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultiLanguage2_Impl::GetNumberOfScripts(this) {
                Ok(ok__) => {
                    pnscripts.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumScripts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, langid: u16, ppenumscript: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultiLanguage2_Impl::EnumScripts(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&langid)) {
                Ok(ok__) => {
                    ppenumscript.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateCodePageEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicodepage: u32, hwnd: super::Foundation::HWND, dwfiodcontrol: u32) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage2_Impl::ValidateCodePageEx(this, core::mem::transmute_copy(&uicodepage), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&dwfiodcontrol)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNumberOfCodePageInfo: GetNumberOfCodePageInfo::<Identity, OFFSET>,
            GetCodePageInfo: GetCodePageInfo::<Identity, OFFSET>,
            GetFamilyCodePage: GetFamilyCodePage::<Identity, OFFSET>,
            EnumCodePages: EnumCodePages::<Identity, OFFSET>,
            GetCharsetInfo: GetCharsetInfo::<Identity, OFFSET>,
            IsConvertible: IsConvertible::<Identity, OFFSET>,
            ConvertString: ConvertString::<Identity, OFFSET>,
            ConvertStringToUnicode: ConvertStringToUnicode::<Identity, OFFSET>,
            ConvertStringFromUnicode: ConvertStringFromUnicode::<Identity, OFFSET>,
            ConvertStringReset: ConvertStringReset::<Identity, OFFSET>,
            GetRfc1766FromLcid: GetRfc1766FromLcid::<Identity, OFFSET>,
            GetLcidFromRfc1766: GetLcidFromRfc1766::<Identity, OFFSET>,
            EnumRfc1766: EnumRfc1766::<Identity, OFFSET>,
            GetRfc1766Info: GetRfc1766Info::<Identity, OFFSET>,
            CreateConvertCharset: CreateConvertCharset::<Identity, OFFSET>,
            ConvertStringInIStream: ConvertStringInIStream::<Identity, OFFSET>,
            ConvertStringToUnicodeEx: ConvertStringToUnicodeEx::<Identity, OFFSET>,
            ConvertStringFromUnicodeEx: ConvertStringFromUnicodeEx::<Identity, OFFSET>,
            DetectCodepageInIStream: DetectCodepageInIStream::<Identity, OFFSET>,
            DetectInputCodepage: DetectInputCodepage::<Identity, OFFSET>,
            ValidateCodePage: ValidateCodePage::<Identity, OFFSET>,
            GetCodePageDescription: GetCodePageDescription::<Identity, OFFSET>,
            IsCodePageInstallable: IsCodePageInstallable::<Identity, OFFSET>,
            SetMimeDBSource: SetMimeDBSource::<Identity, OFFSET>,
            GetNumberOfScripts: GetNumberOfScripts::<Identity, OFFSET>,
            EnumScripts: EnumScripts::<Identity, OFFSET>,
            ValidateCodePageEx: ValidateCodePageEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultiLanguage2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMultiLanguage3_Impl: Sized + IMultiLanguage2_Impl {
    fn DetectOutboundCodePage(&self, dwflags: u32, lpwidecharstr: &windows_core::PCWSTR, cchwidechar: u32, puipreferredcodepages: *const u32, npreferredcodepages: u32, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn DetectOutboundCodePageInIStream(&self, dwflags: u32, pstrin: Option<&super::System::Com::IStream>, puipreferredcodepages: *const u32, npreferredcodepages: u32, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMultiLanguage3 {}
#[cfg(feature = "Win32_System_Com")]
impl IMultiLanguage3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMultiLanguage3_Vtbl
    where
        Identity: IMultiLanguage3_Impl,
    {
        unsafe extern "system" fn DetectOutboundCodePage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, lpwidecharstr: windows_core::PCWSTR, cchwidechar: u32, puipreferredcodepages: *const u32, npreferredcodepages: u32, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage3_Impl::DetectOutboundCodePage(this, core::mem::transmute_copy(&dwflags), core::mem::transmute(&lpwidecharstr), core::mem::transmute_copy(&cchwidechar), core::mem::transmute_copy(&puipreferredcodepages), core::mem::transmute_copy(&npreferredcodepages), core::mem::transmute_copy(&puidetectedcodepages), core::mem::transmute_copy(&pndetectedcodepages), core::mem::transmute(&lpspecialchar)).into()
        }
        unsafe extern "system" fn DetectOutboundCodePageInIStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pstrin: *mut core::ffi::c_void, puipreferredcodepages: *const u32, npreferredcodepages: u32, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IMultiLanguage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiLanguage3_Impl::DetectOutboundCodePageInIStream(this, core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pstrin), core::mem::transmute_copy(&puipreferredcodepages), core::mem::transmute_copy(&npreferredcodepages), core::mem::transmute_copy(&puidetectedcodepages), core::mem::transmute_copy(&pndetectedcodepages), core::mem::transmute(&lpspecialchar)).into()
        }
        Self {
            base__: IMultiLanguage2_Vtbl::new::<Identity, OFFSET>(),
            DetectOutboundCodePage: DetectOutboundCodePage::<Identity, OFFSET>,
            DetectOutboundCodePageInIStream: DetectOutboundCodePageInIStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultiLanguage3 as windows_core::Interface>::IID || iid == &<IMultiLanguage2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOptionDescription_Impl: Sized {
    fn Id(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Heading(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Description(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Labels(&self) -> windows_core::Result<super::System::Com::IEnumString>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOptionDescription {}
#[cfg(feature = "Win32_System_Com")]
impl IOptionDescription_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOptionDescription_Vtbl
    where
        Identity: IOptionDescription_Impl,
    {
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IOptionDescription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOptionDescription_Impl::Id(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Heading<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IOptionDescription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOptionDescription_Impl::Heading(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IOptionDescription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOptionDescription_Impl::Description(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Labels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOptionDescription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOptionDescription_Impl::Labels(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            Heading: Heading::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            Labels: Labels::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOptionDescription as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpellCheckProvider_Impl: Sized {
    fn LanguageTag(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Check(&self, text: &windows_core::PCWSTR) -> windows_core::Result<IEnumSpellingError>;
    fn Suggest(&self, word: &windows_core::PCWSTR) -> windows_core::Result<super::System::Com::IEnumString>;
    fn GetOptionValue(&self, optionid: &windows_core::PCWSTR) -> windows_core::Result<u8>;
    fn SetOptionValue(&self, optionid: &windows_core::PCWSTR, value: u8) -> windows_core::Result<()>;
    fn OptionIds(&self) -> windows_core::Result<super::System::Com::IEnumString>;
    fn Id(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn LocalizedName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetOptionDescription(&self, optionid: &windows_core::PCWSTR) -> windows_core::Result<IOptionDescription>;
    fn InitializeWordlist(&self, wordlisttype: WORDLIST_TYPE, words: Option<&super::System::Com::IEnumString>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISpellCheckProvider {}
#[cfg(feature = "Win32_System_Com")]
impl ISpellCheckProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpellCheckProvider_Vtbl
    where
        Identity: ISpellCheckProvider_Impl,
    {
        unsafe extern "system" fn LanguageTag<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ISpellCheckProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellCheckProvider_Impl::LanguageTag(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Check<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpellCheckProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellCheckProvider_Impl::Check(this, core::mem::transmute(&text)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Suggest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, word: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpellCheckProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellCheckProvider_Impl::Suggest(this, core::mem::transmute(&word)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptionValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, optionid: windows_core::PCWSTR, value: *mut u8) -> windows_core::HRESULT
        where
            Identity: ISpellCheckProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellCheckProvider_Impl::GetOptionValue(this, core::mem::transmute(&optionid)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOptionValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, optionid: windows_core::PCWSTR, value: u8) -> windows_core::HRESULT
        where
            Identity: ISpellCheckProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpellCheckProvider_Impl::SetOptionValue(this, core::mem::transmute(&optionid), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn OptionIds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpellCheckProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellCheckProvider_Impl::OptionIds(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ISpellCheckProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellCheckProvider_Impl::Id(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalizedName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ISpellCheckProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellCheckProvider_Impl::LocalizedName(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptionDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, optionid: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpellCheckProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellCheckProvider_Impl::GetOptionDescription(this, core::mem::transmute(&optionid)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeWordlist<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wordlisttype: WORDLIST_TYPE, words: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpellCheckProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpellCheckProvider_Impl::InitializeWordlist(this, core::mem::transmute_copy(&wordlisttype), windows_core::from_raw_borrowed(&words)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LanguageTag: LanguageTag::<Identity, OFFSET>,
            Check: Check::<Identity, OFFSET>,
            Suggest: Suggest::<Identity, OFFSET>,
            GetOptionValue: GetOptionValue::<Identity, OFFSET>,
            SetOptionValue: SetOptionValue::<Identity, OFFSET>,
            OptionIds: OptionIds::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            LocalizedName: LocalizedName::<Identity, OFFSET>,
            GetOptionDescription: GetOptionDescription::<Identity, OFFSET>,
            InitializeWordlist: InitializeWordlist::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpellCheckProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpellCheckProviderFactory_Impl: Sized {
    fn SupportedLanguages(&self) -> windows_core::Result<super::System::Com::IEnumString>;
    fn IsSupported(&self, languagetag: &windows_core::PCWSTR) -> windows_core::Result<super::Foundation::BOOL>;
    fn CreateSpellCheckProvider(&self, languagetag: &windows_core::PCWSTR) -> windows_core::Result<ISpellCheckProvider>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISpellCheckProviderFactory {}
#[cfg(feature = "Win32_System_Com")]
impl ISpellCheckProviderFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpellCheckProviderFactory_Vtbl
    where
        Identity: ISpellCheckProviderFactory_Impl,
    {
        unsafe extern "system" fn SupportedLanguages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpellCheckProviderFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellCheckProviderFactory_Impl::SupportedLanguages(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, languagetag: windows_core::PCWSTR, value: *mut super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISpellCheckProviderFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellCheckProviderFactory_Impl::IsSupported(this, core::mem::transmute(&languagetag)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSpellCheckProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, languagetag: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpellCheckProviderFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellCheckProviderFactory_Impl::CreateSpellCheckProvider(this, core::mem::transmute(&languagetag)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SupportedLanguages: SupportedLanguages::<Identity, OFFSET>,
            IsSupported: IsSupported::<Identity, OFFSET>,
            CreateSpellCheckProvider: CreateSpellCheckProvider::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpellCheckProviderFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpellChecker_Impl: Sized {
    fn LanguageTag(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Check(&self, text: &windows_core::PCWSTR) -> windows_core::Result<IEnumSpellingError>;
    fn Suggest(&self, word: &windows_core::PCWSTR) -> windows_core::Result<super::System::Com::IEnumString>;
    fn Add(&self, word: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Ignore(&self, word: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AutoCorrect(&self, from: &windows_core::PCWSTR, to: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetOptionValue(&self, optionid: &windows_core::PCWSTR) -> windows_core::Result<u8>;
    fn OptionIds(&self) -> windows_core::Result<super::System::Com::IEnumString>;
    fn Id(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn LocalizedName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn add_SpellCheckerChanged(&self, handler: Option<&ISpellCheckerChangedEventHandler>) -> windows_core::Result<u32>;
    fn remove_SpellCheckerChanged(&self, eventcookie: u32) -> windows_core::Result<()>;
    fn GetOptionDescription(&self, optionid: &windows_core::PCWSTR) -> windows_core::Result<IOptionDescription>;
    fn ComprehensiveCheck(&self, text: &windows_core::PCWSTR) -> windows_core::Result<IEnumSpellingError>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISpellChecker {}
#[cfg(feature = "Win32_System_Com")]
impl ISpellChecker_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpellChecker_Vtbl
    where
        Identity: ISpellChecker_Impl,
    {
        unsafe extern "system" fn LanguageTag<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ISpellChecker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellChecker_Impl::LanguageTag(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Check<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpellChecker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellChecker_Impl::Check(this, core::mem::transmute(&text)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Suggest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, word: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpellChecker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellChecker_Impl::Suggest(this, core::mem::transmute(&word)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, word: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISpellChecker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpellChecker_Impl::Add(this, core::mem::transmute(&word)).into()
        }
        unsafe extern "system" fn Ignore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, word: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISpellChecker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpellChecker_Impl::Ignore(this, core::mem::transmute(&word)).into()
        }
        unsafe extern "system" fn AutoCorrect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, from: windows_core::PCWSTR, to: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISpellChecker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpellChecker_Impl::AutoCorrect(this, core::mem::transmute(&from), core::mem::transmute(&to)).into()
        }
        unsafe extern "system" fn GetOptionValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, optionid: windows_core::PCWSTR, value: *mut u8) -> windows_core::HRESULT
        where
            Identity: ISpellChecker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellChecker_Impl::GetOptionValue(this, core::mem::transmute(&optionid)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OptionIds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpellChecker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellChecker_Impl::OptionIds(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ISpellChecker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellChecker_Impl::Id(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalizedName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ISpellChecker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellChecker_Impl::LocalizedName(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn add_SpellCheckerChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, eventcookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISpellChecker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellChecker_Impl::add_SpellCheckerChanged(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    eventcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn remove_SpellCheckerChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventcookie: u32) -> windows_core::HRESULT
        where
            Identity: ISpellChecker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpellChecker_Impl::remove_SpellCheckerChanged(this, core::mem::transmute_copy(&eventcookie)).into()
        }
        unsafe extern "system" fn GetOptionDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, optionid: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpellChecker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellChecker_Impl::GetOptionDescription(this, core::mem::transmute(&optionid)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComprehensiveCheck<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpellChecker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellChecker_Impl::ComprehensiveCheck(this, core::mem::transmute(&text)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LanguageTag: LanguageTag::<Identity, OFFSET>,
            Check: Check::<Identity, OFFSET>,
            Suggest: Suggest::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Ignore: Ignore::<Identity, OFFSET>,
            AutoCorrect: AutoCorrect::<Identity, OFFSET>,
            GetOptionValue: GetOptionValue::<Identity, OFFSET>,
            OptionIds: OptionIds::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            LocalizedName: LocalizedName::<Identity, OFFSET>,
            add_SpellCheckerChanged: add_SpellCheckerChanged::<Identity, OFFSET>,
            remove_SpellCheckerChanged: remove_SpellCheckerChanged::<Identity, OFFSET>,
            GetOptionDescription: GetOptionDescription::<Identity, OFFSET>,
            ComprehensiveCheck: ComprehensiveCheck::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpellChecker as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpellChecker2_Impl: Sized + ISpellChecker_Impl {
    fn Remove(&self, word: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISpellChecker2 {}
#[cfg(feature = "Win32_System_Com")]
impl ISpellChecker2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpellChecker2_Vtbl
    where
        Identity: ISpellChecker2_Impl,
    {
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, word: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISpellChecker2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpellChecker2_Impl::Remove(this, core::mem::transmute(&word)).into()
        }
        Self { base__: ISpellChecker_Vtbl::new::<Identity, OFFSET>(), Remove: Remove::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpellChecker2 as windows_core::Interface>::IID || iid == &<ISpellChecker as windows_core::Interface>::IID
    }
}
pub trait ISpellCheckerChangedEventHandler_Impl: Sized {
    fn Invoke(&self, sender: Option<&ISpellChecker>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpellCheckerChangedEventHandler {}
impl ISpellCheckerChangedEventHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpellCheckerChangedEventHandler_Vtbl
    where
        Identity: ISpellCheckerChangedEventHandler_Impl,
    {
        unsafe extern "system" fn Invoke<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpellCheckerChangedEventHandler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpellCheckerChangedEventHandler_Impl::Invoke(this, windows_core::from_raw_borrowed(&sender)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpellCheckerChangedEventHandler as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpellCheckerFactory_Impl: Sized {
    fn SupportedLanguages(&self) -> windows_core::Result<super::System::Com::IEnumString>;
    fn IsSupported(&self, languagetag: &windows_core::PCWSTR) -> windows_core::Result<super::Foundation::BOOL>;
    fn CreateSpellChecker(&self, languagetag: &windows_core::PCWSTR) -> windows_core::Result<ISpellChecker>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISpellCheckerFactory {}
#[cfg(feature = "Win32_System_Com")]
impl ISpellCheckerFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpellCheckerFactory_Vtbl
    where
        Identity: ISpellCheckerFactory_Impl,
    {
        unsafe extern "system" fn SupportedLanguages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpellCheckerFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellCheckerFactory_Impl::SupportedLanguages(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, languagetag: windows_core::PCWSTR, value: *mut super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISpellCheckerFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellCheckerFactory_Impl::IsSupported(this, core::mem::transmute(&languagetag)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSpellChecker<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, languagetag: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpellCheckerFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellCheckerFactory_Impl::CreateSpellChecker(this, core::mem::transmute(&languagetag)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SupportedLanguages: SupportedLanguages::<Identity, OFFSET>,
            IsSupported: IsSupported::<Identity, OFFSET>,
            CreateSpellChecker: CreateSpellChecker::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpellCheckerFactory as windows_core::Interface>::IID
    }
}
pub trait ISpellingError_Impl: Sized {
    fn StartIndex(&self) -> windows_core::Result<u32>;
    fn Length(&self) -> windows_core::Result<u32>;
    fn CorrectiveAction(&self) -> windows_core::Result<CORRECTIVE_ACTION>;
    fn Replacement(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for ISpellingError {}
impl ISpellingError_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpellingError_Vtbl
    where
        Identity: ISpellingError_Impl,
    {
        unsafe extern "system" fn StartIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISpellingError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellingError_Impl::StartIndex(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISpellingError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellingError_Impl::Length(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrectiveAction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut CORRECTIVE_ACTION) -> windows_core::HRESULT
        where
            Identity: ISpellingError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellingError_Impl::CorrectiveAction(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Replacement<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ISpellingError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpellingError_Impl::Replacement(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartIndex: StartIndex::<Identity, OFFSET>,
            Length: Length::<Identity, OFFSET>,
            CorrectiveAction: CorrectiveAction::<Identity, OFFSET>,
            Replacement: Replacement::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpellingError as windows_core::Interface>::IID
    }
}
pub trait IUserDictionariesRegistrar_Impl: Sized {
    fn RegisterUserDictionary(&self, dictionarypath: &windows_core::PCWSTR, languagetag: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn UnregisterUserDictionary(&self, dictionarypath: &windows_core::PCWSTR, languagetag: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUserDictionariesRegistrar {}
impl IUserDictionariesRegistrar_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUserDictionariesRegistrar_Vtbl
    where
        Identity: IUserDictionariesRegistrar_Impl,
    {
        unsafe extern "system" fn RegisterUserDictionary<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionarypath: windows_core::PCWSTR, languagetag: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IUserDictionariesRegistrar_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUserDictionariesRegistrar_Impl::RegisterUserDictionary(this, core::mem::transmute(&dictionarypath), core::mem::transmute(&languagetag)).into()
        }
        unsafe extern "system" fn UnregisterUserDictionary<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionarypath: windows_core::PCWSTR, languagetag: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IUserDictionariesRegistrar_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUserDictionariesRegistrar_Impl::UnregisterUserDictionary(this, core::mem::transmute(&dictionarypath), core::mem::transmute(&languagetag)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterUserDictionary: RegisterUserDictionary::<Identity, OFFSET>,
            UnregisterUserDictionary: UnregisterUserDictionary::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUserDictionariesRegistrar as windows_core::Interface>::IID
    }
}
