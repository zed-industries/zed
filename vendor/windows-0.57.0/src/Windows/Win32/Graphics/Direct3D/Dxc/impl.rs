pub trait IDxcAssembler_Impl: Sized {
    fn AssembleToContainer(&self, pshader: Option<&IDxcBlob>) -> windows_core::Result<IDxcOperationResult>;
}
impl windows_core::RuntimeName for IDxcAssembler {}
impl IDxcAssembler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcAssembler_Impl, const OFFSET: isize>() -> IDxcAssembler_Vtbl {
        unsafe extern "system" fn AssembleToContainer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcAssembler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcAssembler_Impl::AssembleToContainer(this, windows_core::from_raw_borrowed(&pshader)) {
                Ok(ok__) => {
                    core::ptr::write(ppresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AssembleToContainer: AssembleToContainer::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcAssembler as windows_core::Interface>::IID
    }
}
pub trait IDxcBlob_Impl: Sized {
    fn GetBufferPointer(&self) -> *mut core::ffi::c_void;
    fn GetBufferSize(&self) -> usize;
}
impl windows_core::RuntimeName for IDxcBlob {}
impl IDxcBlob_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcBlob_Impl, const OFFSET: isize>() -> IDxcBlob_Vtbl {
        unsafe extern "system" fn GetBufferPointer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcBlob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcBlob_Impl::GetBufferPointer(this)
        }
        unsafe extern "system" fn GetBufferSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcBlob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> usize {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcBlob_Impl::GetBufferSize(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBufferPointer: GetBufferPointer::<Identity, Impl, OFFSET>,
            GetBufferSize: GetBufferSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcBlob as windows_core::Interface>::IID
    }
}
pub trait IDxcBlobEncoding_Impl: Sized + IDxcBlob_Impl {
    fn GetEncoding(&self, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDxcBlobEncoding {}
impl IDxcBlobEncoding_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcBlobEncoding_Impl, const OFFSET: isize>() -> IDxcBlobEncoding_Vtbl {
        unsafe extern "system" fn GetEncoding<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcBlobEncoding_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcBlobEncoding_Impl::GetEncoding(this, core::mem::transmute_copy(&pknown), core::mem::transmute_copy(&pcodepage)).into()
        }
        Self { base__: IDxcBlob_Vtbl::new::<Identity, Impl, OFFSET>(), GetEncoding: GetEncoding::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcBlobEncoding as windows_core::Interface>::IID || iid == &<IDxcBlob as windows_core::Interface>::IID
    }
}
pub trait IDxcBlobUtf16_Impl: Sized + IDxcBlobEncoding_Impl {
    fn GetStringPointer(&self) -> windows_core::PCWSTR;
    fn GetStringLength(&self) -> usize;
}
impl windows_core::RuntimeName for IDxcBlobUtf16 {}
impl IDxcBlobUtf16_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcBlobUtf16_Impl, const OFFSET: isize>() -> IDxcBlobUtf16_Vtbl {
        unsafe extern "system" fn GetStringPointer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcBlobUtf16_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::PCWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcBlobUtf16_Impl::GetStringPointer(this)
        }
        unsafe extern "system" fn GetStringLength<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcBlobUtf16_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> usize {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcBlobUtf16_Impl::GetStringLength(this)
        }
        Self {
            base__: IDxcBlobEncoding_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStringPointer: GetStringPointer::<Identity, Impl, OFFSET>,
            GetStringLength: GetStringLength::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcBlobUtf16 as windows_core::Interface>::IID || iid == &<IDxcBlob as windows_core::Interface>::IID || iid == &<IDxcBlobEncoding as windows_core::Interface>::IID
    }
}
pub trait IDxcBlobUtf8_Impl: Sized + IDxcBlobEncoding_Impl {
    fn GetStringPointer(&self) -> windows_core::PCSTR;
    fn GetStringLength(&self) -> usize;
}
impl windows_core::RuntimeName for IDxcBlobUtf8 {}
impl IDxcBlobUtf8_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcBlobUtf8_Impl, const OFFSET: isize>() -> IDxcBlobUtf8_Vtbl {
        unsafe extern "system" fn GetStringPointer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcBlobUtf8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::PCSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcBlobUtf8_Impl::GetStringPointer(this)
        }
        unsafe extern "system" fn GetStringLength<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcBlobUtf8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> usize {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcBlobUtf8_Impl::GetStringLength(this)
        }
        Self {
            base__: IDxcBlobEncoding_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStringPointer: GetStringPointer::<Identity, Impl, OFFSET>,
            GetStringLength: GetStringLength::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcBlobUtf8 as windows_core::Interface>::IID || iid == &<IDxcBlob as windows_core::Interface>::IID || iid == &<IDxcBlobEncoding as windows_core::Interface>::IID
    }
}
pub trait IDxcCompiler_Impl: Sized {
    fn Compile(&self, psource: Option<&IDxcBlob>, psourcename: &windows_core::PCWSTR, pentrypoint: &windows_core::PCWSTR, ptargetprofile: &windows_core::PCWSTR, parguments: *const windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: Option<&IDxcIncludeHandler>) -> windows_core::Result<IDxcOperationResult>;
    fn Preprocess(&self, psource: Option<&IDxcBlob>, psourcename: &windows_core::PCWSTR, parguments: *const windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: Option<&IDxcIncludeHandler>) -> windows_core::Result<IDxcOperationResult>;
    fn Disassemble(&self, psource: Option<&IDxcBlob>) -> windows_core::Result<IDxcBlobEncoding>;
}
impl windows_core::RuntimeName for IDxcCompiler {}
impl IDxcCompiler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompiler_Impl, const OFFSET: isize>() -> IDxcCompiler_Vtbl {
        unsafe extern "system" fn Compile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompiler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psource: *mut core::ffi::c_void, psourcename: windows_core::PCWSTR, pentrypoint: windows_core::PCWSTR, ptargetprofile: windows_core::PCWSTR, parguments: *const windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcCompiler_Impl::Compile(this, windows_core::from_raw_borrowed(&psource), core::mem::transmute(&psourcename), core::mem::transmute(&pentrypoint), core::mem::transmute(&ptargetprofile), core::mem::transmute_copy(&parguments), core::mem::transmute_copy(&argcount), core::mem::transmute_copy(&pdefines), core::mem::transmute_copy(&definecount), windows_core::from_raw_borrowed(&pincludehandler)) {
                Ok(ok__) => {
                    core::ptr::write(ppresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Preprocess<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompiler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psource: *mut core::ffi::c_void, psourcename: windows_core::PCWSTR, parguments: *const windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcCompiler_Impl::Preprocess(this, windows_core::from_raw_borrowed(&psource), core::mem::transmute(&psourcename), core::mem::transmute_copy(&parguments), core::mem::transmute_copy(&argcount), core::mem::transmute_copy(&pdefines), core::mem::transmute_copy(&definecount), windows_core::from_raw_borrowed(&pincludehandler)) {
                Ok(ok__) => {
                    core::ptr::write(ppresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disassemble<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompiler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psource: *mut core::ffi::c_void, ppdisassembly: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcCompiler_Impl::Disassemble(this, windows_core::from_raw_borrowed(&psource)) {
                Ok(ok__) => {
                    core::ptr::write(ppdisassembly, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Compile: Compile::<Identity, Impl, OFFSET>,
            Preprocess: Preprocess::<Identity, Impl, OFFSET>,
            Disassemble: Disassemble::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcCompiler as windows_core::Interface>::IID
    }
}
pub trait IDxcCompiler2_Impl: Sized + IDxcCompiler_Impl {
    fn CompileWithDebug(&self, psource: Option<&IDxcBlob>, psourcename: &windows_core::PCWSTR, pentrypoint: &windows_core::PCWSTR, ptargetprofile: &windows_core::PCWSTR, parguments: *const windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: Option<&IDxcIncludeHandler>, ppresult: *mut Option<IDxcOperationResult>, ppdebugblobname: *mut windows_core::PWSTR, ppdebugblob: *mut Option<IDxcBlob>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDxcCompiler2 {}
impl IDxcCompiler2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompiler2_Impl, const OFFSET: isize>() -> IDxcCompiler2_Vtbl {
        unsafe extern "system" fn CompileWithDebug<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompiler2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psource: *mut core::ffi::c_void, psourcename: windows_core::PCWSTR, pentrypoint: windows_core::PCWSTR, ptargetprofile: windows_core::PCWSTR, parguments: *const windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void, ppdebugblobname: *mut windows_core::PWSTR, ppdebugblob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcCompiler2_Impl::CompileWithDebug(
                this,
                windows_core::from_raw_borrowed(&psource),
                core::mem::transmute(&psourcename),
                core::mem::transmute(&pentrypoint),
                core::mem::transmute(&ptargetprofile),
                core::mem::transmute_copy(&parguments),
                core::mem::transmute_copy(&argcount),
                core::mem::transmute_copy(&pdefines),
                core::mem::transmute_copy(&definecount),
                windows_core::from_raw_borrowed(&pincludehandler),
                core::mem::transmute_copy(&ppresult),
                core::mem::transmute_copy(&ppdebugblobname),
                core::mem::transmute_copy(&ppdebugblob),
            )
            .into()
        }
        Self { base__: IDxcCompiler_Vtbl::new::<Identity, Impl, OFFSET>(), CompileWithDebug: CompileWithDebug::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcCompiler2 as windows_core::Interface>::IID || iid == &<IDxcCompiler as windows_core::Interface>::IID
    }
}
pub trait IDxcCompiler3_Impl: Sized {
    fn Compile(&self, psource: *const DxcBuffer, parguments: *const windows_core::PCWSTR, argcount: u32, pincludehandler: Option<&IDxcIncludeHandler>, riid: *const windows_core::GUID, ppresult: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Disassemble(&self, pobject: *const DxcBuffer, riid: *const windows_core::GUID, ppresult: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDxcCompiler3 {}
impl IDxcCompiler3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompiler3_Impl, const OFFSET: isize>() -> IDxcCompiler3_Vtbl {
        unsafe extern "system" fn Compile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompiler3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psource: *const DxcBuffer, parguments: *const windows_core::PCWSTR, argcount: u32, pincludehandler: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcCompiler3_Impl::Compile(this, core::mem::transmute_copy(&psource), core::mem::transmute_copy(&parguments), core::mem::transmute_copy(&argcount), windows_core::from_raw_borrowed(&pincludehandler), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppresult)).into()
        }
        unsafe extern "system" fn Disassemble<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompiler3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *const DxcBuffer, riid: *const windows_core::GUID, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcCompiler3_Impl::Disassemble(this, core::mem::transmute_copy(&pobject), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppresult)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Compile: Compile::<Identity, Impl, OFFSET>,
            Disassemble: Disassemble::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcCompiler3 as windows_core::Interface>::IID
    }
}
pub trait IDxcCompilerArgs_Impl: Sized {
    fn GetArguments(&self) -> *mut windows_core::PCWSTR;
    fn GetCount(&self) -> u32;
    fn AddArguments(&self, parguments: *const windows_core::PCWSTR, argcount: u32) -> windows_core::Result<()>;
    fn AddArgumentsUTF8(&self, parguments: *const windows_core::PCSTR, argcount: u32) -> windows_core::Result<()>;
    fn AddDefines(&self, pdefines: *const DxcDefine, definecount: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDxcCompilerArgs {}
impl IDxcCompilerArgs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompilerArgs_Impl, const OFFSET: isize>() -> IDxcCompilerArgs_Vtbl {
        unsafe extern "system" fn GetArguments<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompilerArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> *mut windows_core::PCWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcCompilerArgs_Impl::GetArguments(this)
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompilerArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcCompilerArgs_Impl::GetCount(this)
        }
        unsafe extern "system" fn AddArguments<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompilerArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parguments: *const windows_core::PCWSTR, argcount: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcCompilerArgs_Impl::AddArguments(this, core::mem::transmute_copy(&parguments), core::mem::transmute_copy(&argcount)).into()
        }
        unsafe extern "system" fn AddArgumentsUTF8<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompilerArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parguments: *const windows_core::PCSTR, argcount: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcCompilerArgs_Impl::AddArgumentsUTF8(this, core::mem::transmute_copy(&parguments), core::mem::transmute_copy(&argcount)).into()
        }
        unsafe extern "system" fn AddDefines<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcCompilerArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdefines: *const DxcDefine, definecount: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcCompilerArgs_Impl::AddDefines(this, core::mem::transmute_copy(&pdefines), core::mem::transmute_copy(&definecount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetArguments: GetArguments::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            AddArguments: AddArguments::<Identity, Impl, OFFSET>,
            AddArgumentsUTF8: AddArgumentsUTF8::<Identity, Impl, OFFSET>,
            AddDefines: AddDefines::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcCompilerArgs as windows_core::Interface>::IID
    }
}
pub trait IDxcContainerBuilder_Impl: Sized {
    fn Load(&self, pdxilcontainerheader: Option<&IDxcBlob>) -> windows_core::Result<()>;
    fn AddPart(&self, fourcc: u32, psource: Option<&IDxcBlob>) -> windows_core::Result<()>;
    fn RemovePart(&self, fourcc: u32) -> windows_core::Result<()>;
    fn SerializeContainer(&self) -> windows_core::Result<IDxcOperationResult>;
}
impl windows_core::RuntimeName for IDxcContainerBuilder {}
impl IDxcContainerBuilder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcContainerBuilder_Impl, const OFFSET: isize>() -> IDxcContainerBuilder_Vtbl {
        unsafe extern "system" fn Load<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcContainerBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdxilcontainerheader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcContainerBuilder_Impl::Load(this, windows_core::from_raw_borrowed(&pdxilcontainerheader)).into()
        }
        unsafe extern "system" fn AddPart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcContainerBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fourcc: u32, psource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcContainerBuilder_Impl::AddPart(this, core::mem::transmute_copy(&fourcc), windows_core::from_raw_borrowed(&psource)).into()
        }
        unsafe extern "system" fn RemovePart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcContainerBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fourcc: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcContainerBuilder_Impl::RemovePart(this, core::mem::transmute_copy(&fourcc)).into()
        }
        unsafe extern "system" fn SerializeContainer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcContainerBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcContainerBuilder_Impl::SerializeContainer(this) {
                Ok(ok__) => {
                    core::ptr::write(ppresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Load: Load::<Identity, Impl, OFFSET>,
            AddPart: AddPart::<Identity, Impl, OFFSET>,
            RemovePart: RemovePart::<Identity, Impl, OFFSET>,
            SerializeContainer: SerializeContainer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcContainerBuilder as windows_core::Interface>::IID
    }
}
pub trait IDxcContainerReflection_Impl: Sized {
    fn Load(&self, pcontainer: Option<&IDxcBlob>) -> windows_core::Result<()>;
    fn GetPartCount(&self) -> windows_core::Result<u32>;
    fn GetPartKind(&self, idx: u32) -> windows_core::Result<u32>;
    fn GetPartContent(&self, idx: u32) -> windows_core::Result<IDxcBlob>;
    fn FindFirstPartKind(&self, kind: u32) -> windows_core::Result<u32>;
    fn GetPartReflection(&self, idx: u32, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDxcContainerReflection {}
impl IDxcContainerReflection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcContainerReflection_Impl, const OFFSET: isize>() -> IDxcContainerReflection_Vtbl {
        unsafe extern "system" fn Load<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontainer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcContainerReflection_Impl::Load(this, windows_core::from_raw_borrowed(&pcontainer)).into()
        }
        unsafe extern "system" fn GetPartCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcContainerReflection_Impl::GetPartCount(this) {
                Ok(ok__) => {
                    core::ptr::write(presult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartKind<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idx: u32, presult: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcContainerReflection_Impl::GetPartKind(this, core::mem::transmute_copy(&idx)) {
                Ok(ok__) => {
                    core::ptr::write(presult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartContent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idx: u32, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcContainerReflection_Impl::GetPartContent(this, core::mem::transmute_copy(&idx)) {
                Ok(ok__) => {
                    core::ptr::write(ppresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstPartKind<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, kind: u32, presult: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcContainerReflection_Impl::FindFirstPartKind(this, core::mem::transmute_copy(&kind)) {
                Ok(ok__) => {
                    core::ptr::write(presult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartReflection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcContainerReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idx: u32, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcContainerReflection_Impl::GetPartReflection(this, core::mem::transmute_copy(&idx), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppvobject)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Load: Load::<Identity, Impl, OFFSET>,
            GetPartCount: GetPartCount::<Identity, Impl, OFFSET>,
            GetPartKind: GetPartKind::<Identity, Impl, OFFSET>,
            GetPartContent: GetPartContent::<Identity, Impl, OFFSET>,
            FindFirstPartKind: FindFirstPartKind::<Identity, Impl, OFFSET>,
            GetPartReflection: GetPartReflection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcContainerReflection as windows_core::Interface>::IID
    }
}
pub trait IDxcExtraOutputs_Impl: Sized {
    fn GetOutputCount(&self) -> u32;
    fn GetOutput(&self, uindex: u32, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void, ppoutputtype: *mut Option<IDxcBlobUtf16>, ppoutputname: *mut Option<IDxcBlobUtf16>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDxcExtraOutputs {}
impl IDxcExtraOutputs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcExtraOutputs_Impl, const OFFSET: isize>() -> IDxcExtraOutputs_Vtbl {
        unsafe extern "system" fn GetOutputCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcExtraOutputs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcExtraOutputs_Impl::GetOutputCount(this)
        }
        unsafe extern "system" fn GetOutput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcExtraOutputs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void, ppoutputtype: *mut *mut core::ffi::c_void, ppoutputname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcExtraOutputs_Impl::GetOutput(this, core::mem::transmute_copy(&uindex), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppvobject), core::mem::transmute_copy(&ppoutputtype), core::mem::transmute_copy(&ppoutputname)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOutputCount: GetOutputCount::<Identity, Impl, OFFSET>,
            GetOutput: GetOutput::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcExtraOutputs as windows_core::Interface>::IID
    }
}
pub trait IDxcIncludeHandler_Impl: Sized {
    fn LoadSource(&self, pfilename: &windows_core::PCWSTR) -> windows_core::Result<IDxcBlob>;
}
impl windows_core::RuntimeName for IDxcIncludeHandler {}
impl IDxcIncludeHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcIncludeHandler_Impl, const OFFSET: isize>() -> IDxcIncludeHandler_Vtbl {
        unsafe extern "system" fn LoadSource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcIncludeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilename: windows_core::PCWSTR, ppincludesource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcIncludeHandler_Impl::LoadSource(this, core::mem::transmute(&pfilename)) {
                Ok(ok__) => {
                    core::ptr::write(ppincludesource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LoadSource: LoadSource::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcIncludeHandler as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDxcLibrary_Impl: Sized {
    fn SetMalloc(&self, pmalloc: Option<&super::super::super::System::Com::IMalloc>) -> windows_core::Result<()>;
    fn CreateBlobFromBlob(&self, pblob: Option<&IDxcBlob>, offset: u32, length: u32) -> windows_core::Result<IDxcBlob>;
    fn CreateBlobFromFile(&self, pfilename: &windows_core::PCWSTR, codepage: *const DXC_CP) -> windows_core::Result<IDxcBlobEncoding>;
    fn CreateBlobWithEncodingFromPinned(&self, ptext: *const core::ffi::c_void, size: u32, codepage: DXC_CP) -> windows_core::Result<IDxcBlobEncoding>;
    fn CreateBlobWithEncodingOnHeapCopy(&self, ptext: *const core::ffi::c_void, size: u32, codepage: DXC_CP) -> windows_core::Result<IDxcBlobEncoding>;
    fn CreateBlobWithEncodingOnMalloc(&self, ptext: *const core::ffi::c_void, pimalloc: Option<&super::super::super::System::Com::IMalloc>, size: u32, codepage: DXC_CP) -> windows_core::Result<IDxcBlobEncoding>;
    fn CreateIncludeHandler(&self) -> windows_core::Result<IDxcIncludeHandler>;
    fn CreateStreamFromBlobReadOnly(&self, pblob: Option<&IDxcBlob>) -> windows_core::Result<super::super::super::System::Com::IStream>;
    fn GetBlobAsUtf8(&self, pblob: Option<&IDxcBlob>) -> windows_core::Result<IDxcBlobEncoding>;
    fn GetBlobAsUtf16(&self, pblob: Option<&IDxcBlob>) -> windows_core::Result<IDxcBlobEncoding>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDxcLibrary {}
#[cfg(feature = "Win32_System_Com")]
impl IDxcLibrary_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: isize>() -> IDxcLibrary_Vtbl {
        unsafe extern "system" fn SetMalloc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmalloc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcLibrary_Impl::SetMalloc(this, windows_core::from_raw_borrowed(&pmalloc)).into()
        }
        unsafe extern "system" fn CreateBlobFromBlob<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, offset: u32, length: u32, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcLibrary_Impl::CreateBlobFromBlob(this, windows_core::from_raw_borrowed(&pblob), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&length)) {
                Ok(ok__) => {
                    core::ptr::write(ppresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobFromFile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilename: windows_core::PCWSTR, codepage: *const DXC_CP, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcLibrary_Impl::CreateBlobFromFile(this, core::mem::transmute(&pfilename), core::mem::transmute_copy(&codepage)) {
                Ok(ok__) => {
                    core::ptr::write(pblobencoding, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobWithEncodingFromPinned<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptext: *const core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcLibrary_Impl::CreateBlobWithEncodingFromPinned(this, core::mem::transmute_copy(&ptext), core::mem::transmute_copy(&size), core::mem::transmute_copy(&codepage)) {
                Ok(ok__) => {
                    core::ptr::write(pblobencoding, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobWithEncodingOnHeapCopy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptext: *const core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcLibrary_Impl::CreateBlobWithEncodingOnHeapCopy(this, core::mem::transmute_copy(&ptext), core::mem::transmute_copy(&size), core::mem::transmute_copy(&codepage)) {
                Ok(ok__) => {
                    core::ptr::write(pblobencoding, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobWithEncodingOnMalloc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptext: *const core::ffi::c_void, pimalloc: *mut core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcLibrary_Impl::CreateBlobWithEncodingOnMalloc(this, core::mem::transmute_copy(&ptext), windows_core::from_raw_borrowed(&pimalloc), core::mem::transmute_copy(&size), core::mem::transmute_copy(&codepage)) {
                Ok(ok__) => {
                    core::ptr::write(pblobencoding, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateIncludeHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcLibrary_Impl::CreateIncludeHandler(this) {
                Ok(ok__) => {
                    core::ptr::write(ppresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStreamFromBlobReadOnly<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcLibrary_Impl::CreateStreamFromBlobReadOnly(this, windows_core::from_raw_borrowed(&pblob)) {
                Ok(ok__) => {
                    core::ptr::write(ppstream, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBlobAsUtf8<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcLibrary_Impl::GetBlobAsUtf8(this, windows_core::from_raw_borrowed(&pblob)) {
                Ok(ok__) => {
                    core::ptr::write(pblobencoding, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBlobAsUtf16<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcLibrary_Impl::GetBlobAsUtf16(this, windows_core::from_raw_borrowed(&pblob)) {
                Ok(ok__) => {
                    core::ptr::write(pblobencoding, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetMalloc: SetMalloc::<Identity, Impl, OFFSET>,
            CreateBlobFromBlob: CreateBlobFromBlob::<Identity, Impl, OFFSET>,
            CreateBlobFromFile: CreateBlobFromFile::<Identity, Impl, OFFSET>,
            CreateBlobWithEncodingFromPinned: CreateBlobWithEncodingFromPinned::<Identity, Impl, OFFSET>,
            CreateBlobWithEncodingOnHeapCopy: CreateBlobWithEncodingOnHeapCopy::<Identity, Impl, OFFSET>,
            CreateBlobWithEncodingOnMalloc: CreateBlobWithEncodingOnMalloc::<Identity, Impl, OFFSET>,
            CreateIncludeHandler: CreateIncludeHandler::<Identity, Impl, OFFSET>,
            CreateStreamFromBlobReadOnly: CreateStreamFromBlobReadOnly::<Identity, Impl, OFFSET>,
            GetBlobAsUtf8: GetBlobAsUtf8::<Identity, Impl, OFFSET>,
            GetBlobAsUtf16: GetBlobAsUtf16::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcLibrary as windows_core::Interface>::IID
    }
}
pub trait IDxcLinker_Impl: Sized {
    fn RegisterLibrary(&self, plibname: &windows_core::PCWSTR, plib: Option<&IDxcBlob>) -> windows_core::Result<()>;
    fn Link(&self, pentryname: &windows_core::PCWSTR, ptargetprofile: &windows_core::PCWSTR, plibnames: *const windows_core::PCWSTR, libcount: u32, parguments: *const windows_core::PCWSTR, argcount: u32) -> windows_core::Result<IDxcOperationResult>;
}
impl windows_core::RuntimeName for IDxcLinker {}
impl IDxcLinker_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcLinker_Impl, const OFFSET: isize>() -> IDxcLinker_Vtbl {
        unsafe extern "system" fn RegisterLibrary<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcLinker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plibname: windows_core::PCWSTR, plib: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcLinker_Impl::RegisterLibrary(this, core::mem::transmute(&plibname), windows_core::from_raw_borrowed(&plib)).into()
        }
        unsafe extern "system" fn Link<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcLinker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pentryname: windows_core::PCWSTR, ptargetprofile: windows_core::PCWSTR, plibnames: *const windows_core::PCWSTR, libcount: u32, parguments: *const windows_core::PCWSTR, argcount: u32, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcLinker_Impl::Link(this, core::mem::transmute(&pentryname), core::mem::transmute(&ptargetprofile), core::mem::transmute_copy(&plibnames), core::mem::transmute_copy(&libcount), core::mem::transmute_copy(&parguments), core::mem::transmute_copy(&argcount)) {
                Ok(ok__) => {
                    core::ptr::write(ppresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterLibrary: RegisterLibrary::<Identity, Impl, OFFSET>,
            Link: Link::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcLinker as windows_core::Interface>::IID
    }
}
pub trait IDxcOperationResult_Impl: Sized {
    fn GetStatus(&self) -> windows_core::Result<windows_core::HRESULT>;
    fn GetResult(&self) -> windows_core::Result<IDxcBlob>;
    fn GetErrorBuffer(&self) -> windows_core::Result<IDxcBlobEncoding>;
}
impl windows_core::RuntimeName for IDxcOperationResult {}
impl IDxcOperationResult_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcOperationResult_Impl, const OFFSET: isize>() -> IDxcOperationResult_Vtbl {
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcOperationResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcOperationResult_Impl::GetStatus(this) {
                Ok(ok__) => {
                    core::ptr::write(pstatus, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResult<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcOperationResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcOperationResult_Impl::GetResult(this) {
                Ok(ok__) => {
                    core::ptr::write(ppresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorBuffer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcOperationResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pperrors: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcOperationResult_Impl::GetErrorBuffer(this) {
                Ok(ok__) => {
                    core::ptr::write(pperrors, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetResult: GetResult::<Identity, Impl, OFFSET>,
            GetErrorBuffer: GetErrorBuffer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcOperationResult as windows_core::Interface>::IID
    }
}
pub trait IDxcOptimizer_Impl: Sized {
    fn GetAvailablePassCount(&self) -> windows_core::Result<u32>;
    fn GetAvailablePass(&self, index: u32) -> windows_core::Result<IDxcOptimizerPass>;
    fn RunOptimizer(&self, pblob: Option<&IDxcBlob>, ppoptions: *const windows_core::PCWSTR, optioncount: u32, poutputmodule: *mut Option<IDxcBlob>, ppoutputtext: *mut Option<IDxcBlobEncoding>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDxcOptimizer {}
impl IDxcOptimizer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcOptimizer_Impl, const OFFSET: isize>() -> IDxcOptimizer_Vtbl {
        unsafe extern "system" fn GetAvailablePassCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcOptimizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcOptimizer_Impl::GetAvailablePassCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAvailablePass<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcOptimizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcOptimizer_Impl::GetAvailablePass(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(ppresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunOptimizer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcOptimizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, ppoptions: *const windows_core::PCWSTR, optioncount: u32, poutputmodule: *mut *mut core::ffi::c_void, ppoutputtext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcOptimizer_Impl::RunOptimizer(this, windows_core::from_raw_borrowed(&pblob), core::mem::transmute_copy(&ppoptions), core::mem::transmute_copy(&optioncount), core::mem::transmute_copy(&poutputmodule), core::mem::transmute_copy(&ppoutputtext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAvailablePassCount: GetAvailablePassCount::<Identity, Impl, OFFSET>,
            GetAvailablePass: GetAvailablePass::<Identity, Impl, OFFSET>,
            RunOptimizer: RunOptimizer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcOptimizer as windows_core::Interface>::IID
    }
}
pub trait IDxcOptimizerPass_Impl: Sized {
    fn GetOptionName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetOptionArgCount(&self) -> windows_core::Result<u32>;
    fn GetOptionArgName(&self, argindex: u32) -> windows_core::Result<windows_core::PWSTR>;
    fn GetOptionArgDescription(&self, argindex: u32) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IDxcOptimizerPass {}
impl IDxcOptimizerPass_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcOptimizerPass_Impl, const OFFSET: isize>() -> IDxcOptimizerPass_Vtbl {
        unsafe extern "system" fn GetOptionName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcOptimizerPass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcOptimizerPass_Impl::GetOptionName(this) {
                Ok(ok__) => {
                    core::ptr::write(ppresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcOptimizerPass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcOptimizerPass_Impl::GetDescription(this) {
                Ok(ok__) => {
                    core::ptr::write(ppresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptionArgCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcOptimizerPass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcOptimizerPass_Impl::GetOptionArgCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptionArgName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcOptimizerPass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, argindex: u32, ppresult: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcOptimizerPass_Impl::GetOptionArgName(this, core::mem::transmute_copy(&argindex)) {
                Ok(ok__) => {
                    core::ptr::write(ppresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptionArgDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcOptimizerPass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, argindex: u32, ppresult: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcOptimizerPass_Impl::GetOptionArgDescription(this, core::mem::transmute_copy(&argindex)) {
                Ok(ok__) => {
                    core::ptr::write(ppresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOptionName: GetOptionName::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            GetOptionArgCount: GetOptionArgCount::<Identity, Impl, OFFSET>,
            GetOptionArgName: GetOptionArgName::<Identity, Impl, OFFSET>,
            GetOptionArgDescription: GetOptionArgDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcOptimizerPass as windows_core::Interface>::IID
    }
}
pub trait IDxcPdbUtils_Impl: Sized {
    fn Load(&self, ppdbordxil: Option<&IDxcBlob>) -> windows_core::Result<()>;
    fn GetSourceCount(&self) -> windows_core::Result<u32>;
    fn GetSource(&self, uindex: u32) -> windows_core::Result<IDxcBlobEncoding>;
    fn GetSourceName(&self, uindex: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetFlagCount(&self) -> windows_core::Result<u32>;
    fn GetFlag(&self, uindex: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetArgCount(&self) -> windows_core::Result<u32>;
    fn GetArg(&self, uindex: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetArgPairCount(&self) -> windows_core::Result<u32>;
    fn GetArgPair(&self, uindex: u32, pname: *mut windows_core::BSTR, pvalue: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetDefineCount(&self) -> windows_core::Result<u32>;
    fn GetDefine(&self, uindex: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetTargetProfile(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetEntryPoint(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetMainFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetHash(&self) -> windows_core::Result<IDxcBlob>;
    fn GetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsFullPDB(&self) -> super::super::super::Foundation::BOOL;
    fn GetFullPDB(&self) -> windows_core::Result<IDxcBlob>;
    fn GetVersionInfo(&self) -> windows_core::Result<IDxcVersionInfo>;
    fn SetCompiler(&self, pcompiler: Option<&IDxcCompiler3>) -> windows_core::Result<()>;
    fn CompileForFullPDB(&self) -> windows_core::Result<IDxcResult>;
    fn OverrideArgs(&self, pargpairs: *const DxcArgPair, unumargpairs: u32) -> windows_core::Result<()>;
    fn OverrideRootSignature(&self, prootsignature: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDxcPdbUtils {}
impl IDxcPdbUtils_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>() -> IDxcPdbUtils_Vtbl {
        unsafe extern "system" fn Load<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdbordxil: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcPdbUtils_Impl::Load(this, windows_core::from_raw_borrowed(&ppdbordxil)).into()
        }
        unsafe extern "system" fn GetSourceCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcPdbUtils_Impl::GetSourceCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcPdbUtils_Impl::GetSource(this, core::mem::transmute_copy(&uindex)) {
                Ok(ok__) => {
                    core::ptr::write(ppresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, presult: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcPdbUtils_Impl::GetSourceName(this, core::mem::transmute_copy(&uindex)) {
                Ok(ok__) => {
                    core::ptr::write(presult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlagCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcPdbUtils_Impl::GetFlagCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, presult: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcPdbUtils_Impl::GetFlag(this, core::mem::transmute_copy(&uindex)) {
                Ok(ok__) => {
                    core::ptr::write(presult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArgCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcPdbUtils_Impl::GetArgCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArg<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, presult: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcPdbUtils_Impl::GetArg(this, core::mem::transmute_copy(&uindex)) {
                Ok(ok__) => {
                    core::ptr::write(presult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArgPairCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcPdbUtils_Impl::GetArgPairCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArgPair<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, pname: *mut core::mem::MaybeUninit<windows_core::BSTR>, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcPdbUtils_Impl::GetArgPair(this, core::mem::transmute_copy(&uindex), core::mem::transmute_copy(&pname), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetDefineCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcPdbUtils_Impl::GetDefineCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefine<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, presult: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcPdbUtils_Impl::GetDefine(this, core::mem::transmute_copy(&uindex)) {
                Ok(ok__) => {
                    core::ptr::write(presult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetProfile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcPdbUtils_Impl::GetTargetProfile(this) {
                Ok(ok__) => {
                    core::ptr::write(presult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEntryPoint<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcPdbUtils_Impl::GetEntryPoint(this) {
                Ok(ok__) => {
                    core::ptr::write(presult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMainFileName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcPdbUtils_Impl::GetMainFileName(this) {
                Ok(ok__) => {
                    core::ptr::write(presult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHash<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcPdbUtils_Impl::GetHash(this) {
                Ok(ok__) => {
                    core::ptr::write(ppresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcPdbUtils_Impl::GetName(this) {
                Ok(ok__) => {
                    core::ptr::write(presult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFullPDB<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcPdbUtils_Impl::IsFullPDB(this)
        }
        unsafe extern "system" fn GetFullPDB<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfullpdb: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcPdbUtils_Impl::GetFullPDB(this) {
                Ok(ok__) => {
                    core::ptr::write(ppfullpdb, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersionInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppversioninfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcPdbUtils_Impl::GetVersionInfo(this) {
                Ok(ok__) => {
                    core::ptr::write(ppversioninfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompiler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcompiler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcPdbUtils_Impl::SetCompiler(this, windows_core::from_raw_borrowed(&pcompiler)).into()
        }
        unsafe extern "system" fn CompileForFullPDB<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcPdbUtils_Impl::CompileForFullPDB(this) {
                Ok(ok__) => {
                    core::ptr::write(ppresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverrideArgs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pargpairs: *const DxcArgPair, unumargpairs: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcPdbUtils_Impl::OverrideArgs(this, core::mem::transmute_copy(&pargpairs), core::mem::transmute_copy(&unumargpairs)).into()
        }
        unsafe extern "system" fn OverrideRootSignature<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prootsignature: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcPdbUtils_Impl::OverrideRootSignature(this, core::mem::transmute(&prootsignature)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Load: Load::<Identity, Impl, OFFSET>,
            GetSourceCount: GetSourceCount::<Identity, Impl, OFFSET>,
            GetSource: GetSource::<Identity, Impl, OFFSET>,
            GetSourceName: GetSourceName::<Identity, Impl, OFFSET>,
            GetFlagCount: GetFlagCount::<Identity, Impl, OFFSET>,
            GetFlag: GetFlag::<Identity, Impl, OFFSET>,
            GetArgCount: GetArgCount::<Identity, Impl, OFFSET>,
            GetArg: GetArg::<Identity, Impl, OFFSET>,
            GetArgPairCount: GetArgPairCount::<Identity, Impl, OFFSET>,
            GetArgPair: GetArgPair::<Identity, Impl, OFFSET>,
            GetDefineCount: GetDefineCount::<Identity, Impl, OFFSET>,
            GetDefine: GetDefine::<Identity, Impl, OFFSET>,
            GetTargetProfile: GetTargetProfile::<Identity, Impl, OFFSET>,
            GetEntryPoint: GetEntryPoint::<Identity, Impl, OFFSET>,
            GetMainFileName: GetMainFileName::<Identity, Impl, OFFSET>,
            GetHash: GetHash::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            IsFullPDB: IsFullPDB::<Identity, Impl, OFFSET>,
            GetFullPDB: GetFullPDB::<Identity, Impl, OFFSET>,
            GetVersionInfo: GetVersionInfo::<Identity, Impl, OFFSET>,
            SetCompiler: SetCompiler::<Identity, Impl, OFFSET>,
            CompileForFullPDB: CompileForFullPDB::<Identity, Impl, OFFSET>,
            OverrideArgs: OverrideArgs::<Identity, Impl, OFFSET>,
            OverrideRootSignature: OverrideRootSignature::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcPdbUtils as windows_core::Interface>::IID
    }
}
pub trait IDxcResult_Impl: Sized + IDxcOperationResult_Impl {
    fn HasOutput(&self, dxcoutkind: DXC_OUT_KIND) -> super::super::super::Foundation::BOOL;
    fn GetOutput(&self, dxcoutkind: DXC_OUT_KIND, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void, ppoutputname: *mut Option<IDxcBlobUtf16>) -> windows_core::Result<()>;
    fn GetNumOutputs(&self) -> u32;
    fn GetOutputByIndex(&self, index: u32) -> DXC_OUT_KIND;
    fn PrimaryOutput(&self) -> DXC_OUT_KIND;
}
impl windows_core::RuntimeName for IDxcResult {}
impl IDxcResult_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcResult_Impl, const OFFSET: isize>() -> IDxcResult_Vtbl {
        unsafe extern "system" fn HasOutput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxcoutkind: DXC_OUT_KIND) -> super::super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcResult_Impl::HasOutput(this, core::mem::transmute_copy(&dxcoutkind))
        }
        unsafe extern "system" fn GetOutput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxcoutkind: DXC_OUT_KIND, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void, ppoutputname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcResult_Impl::GetOutput(this, core::mem::transmute_copy(&dxcoutkind), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppvobject), core::mem::transmute_copy(&ppoutputname)).into()
        }
        unsafe extern "system" fn GetNumOutputs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcResult_Impl::GetNumOutputs(this)
        }
        unsafe extern "system" fn GetOutputByIndex<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> DXC_OUT_KIND {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcResult_Impl::GetOutputByIndex(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn PrimaryOutput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DXC_OUT_KIND {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcResult_Impl::PrimaryOutput(this)
        }
        Self {
            base__: IDxcOperationResult_Vtbl::new::<Identity, Impl, OFFSET>(),
            HasOutput: HasOutput::<Identity, Impl, OFFSET>,
            GetOutput: GetOutput::<Identity, Impl, OFFSET>,
            GetNumOutputs: GetNumOutputs::<Identity, Impl, OFFSET>,
            GetOutputByIndex: GetOutputByIndex::<Identity, Impl, OFFSET>,
            PrimaryOutput: PrimaryOutput::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcResult as windows_core::Interface>::IID || iid == &<IDxcOperationResult as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDxcUtils_Impl: Sized {
    fn CreateBlobFromBlob(&self, pblob: Option<&IDxcBlob>, offset: u32, length: u32) -> windows_core::Result<IDxcBlob>;
    fn CreateBlobFromPinned(&self, pdata: *const core::ffi::c_void, size: u32, codepage: DXC_CP) -> windows_core::Result<IDxcBlobEncoding>;
    fn MoveToBlob(&self, pdata: *const core::ffi::c_void, pimalloc: Option<&super::super::super::System::Com::IMalloc>, size: u32, codepage: DXC_CP) -> windows_core::Result<IDxcBlobEncoding>;
    fn CreateBlob(&self, pdata: *const core::ffi::c_void, size: u32, codepage: DXC_CP) -> windows_core::Result<IDxcBlobEncoding>;
    fn LoadFile(&self, pfilename: &windows_core::PCWSTR, pcodepage: *const DXC_CP) -> windows_core::Result<IDxcBlobEncoding>;
    fn CreateReadOnlyStreamFromBlob(&self, pblob: Option<&IDxcBlob>) -> windows_core::Result<super::super::super::System::Com::IStream>;
    fn CreateDefaultIncludeHandler(&self) -> windows_core::Result<IDxcIncludeHandler>;
    fn GetBlobAsUtf8(&self, pblob: Option<&IDxcBlob>) -> windows_core::Result<IDxcBlobUtf8>;
    fn GetBlobAsUtf16(&self, pblob: Option<&IDxcBlob>) -> windows_core::Result<IDxcBlobUtf16>;
    fn GetDxilContainerPart(&self, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut core::ffi::c_void, ppartsizeinbytes: *mut u32) -> windows_core::Result<()>;
    fn CreateReflection(&self, pdata: *const DxcBuffer, iid: *const windows_core::GUID, ppvreflection: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn BuildArguments(&self, psourcename: &windows_core::PCWSTR, pentrypoint: &windows_core::PCWSTR, ptargetprofile: &windows_core::PCWSTR, parguments: *const windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32) -> windows_core::Result<IDxcCompilerArgs>;
    fn GetPDBContents(&self, ppdbblob: Option<&IDxcBlob>, pphash: *mut Option<IDxcBlob>, ppcontainer: *mut Option<IDxcBlob>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDxcUtils {}
#[cfg(feature = "Win32_System_Com")]
impl IDxcUtils_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>() -> IDxcUtils_Vtbl {
        unsafe extern "system" fn CreateBlobFromBlob<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, offset: u32, length: u32, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcUtils_Impl::CreateBlobFromBlob(this, windows_core::from_raw_borrowed(&pblob), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&length)) {
                Ok(ok__) => {
                    core::ptr::write(ppresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlobFromPinned<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcUtils_Impl::CreateBlobFromPinned(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&size), core::mem::transmute_copy(&codepage)) {
                Ok(ok__) => {
                    core::ptr::write(pblobencoding, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveToBlob<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const core::ffi::c_void, pimalloc: *mut core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcUtils_Impl::MoveToBlob(this, core::mem::transmute_copy(&pdata), windows_core::from_raw_borrowed(&pimalloc), core::mem::transmute_copy(&size), core::mem::transmute_copy(&codepage)) {
                Ok(ok__) => {
                    core::ptr::write(pblobencoding, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlob<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcUtils_Impl::CreateBlob(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&size), core::mem::transmute_copy(&codepage)) {
                Ok(ok__) => {
                    core::ptr::write(pblobencoding, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadFile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilename: windows_core::PCWSTR, pcodepage: *const DXC_CP, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcUtils_Impl::LoadFile(this, core::mem::transmute(&pfilename), core::mem::transmute_copy(&pcodepage)) {
                Ok(ok__) => {
                    core::ptr::write(pblobencoding, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReadOnlyStreamFromBlob<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcUtils_Impl::CreateReadOnlyStreamFromBlob(this, windows_core::from_raw_borrowed(&pblob)) {
                Ok(ok__) => {
                    core::ptr::write(ppstream, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDefaultIncludeHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcUtils_Impl::CreateDefaultIncludeHandler(this) {
                Ok(ok__) => {
                    core::ptr::write(ppresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBlobAsUtf8<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcUtils_Impl::GetBlobAsUtf8(this, windows_core::from_raw_borrowed(&pblob)) {
                Ok(ok__) => {
                    core::ptr::write(pblobencoding, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBlobAsUtf16<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *mut core::ffi::c_void, pblobencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcUtils_Impl::GetBlobAsUtf16(this, windows_core::from_raw_borrowed(&pblob)) {
                Ok(ok__) => {
                    core::ptr::write(pblobencoding, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDxilContainerPart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut core::ffi::c_void, ppartsizeinbytes: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcUtils_Impl::GetDxilContainerPart(this, core::mem::transmute_copy(&pshader), core::mem::transmute_copy(&dxcpart), core::mem::transmute_copy(&pppartdata), core::mem::transmute_copy(&ppartsizeinbytes)).into()
        }
        unsafe extern "system" fn CreateReflection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const DxcBuffer, iid: *const windows_core::GUID, ppvreflection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcUtils_Impl::CreateReflection(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppvreflection)).into()
        }
        unsafe extern "system" fn BuildArguments<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcename: windows_core::PCWSTR, pentrypoint: windows_core::PCWSTR, ptargetprofile: windows_core::PCWSTR, parguments: *const windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, ppargs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcUtils_Impl::BuildArguments(this, core::mem::transmute(&psourcename), core::mem::transmute(&pentrypoint), core::mem::transmute(&ptargetprofile), core::mem::transmute_copy(&parguments), core::mem::transmute_copy(&argcount), core::mem::transmute_copy(&pdefines), core::mem::transmute_copy(&definecount)) {
                Ok(ok__) => {
                    core::ptr::write(ppargs, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPDBContents<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdbblob: *mut core::ffi::c_void, pphash: *mut *mut core::ffi::c_void, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcUtils_Impl::GetPDBContents(this, windows_core::from_raw_borrowed(&ppdbblob), core::mem::transmute_copy(&pphash), core::mem::transmute_copy(&ppcontainer)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateBlobFromBlob: CreateBlobFromBlob::<Identity, Impl, OFFSET>,
            CreateBlobFromPinned: CreateBlobFromPinned::<Identity, Impl, OFFSET>,
            MoveToBlob: MoveToBlob::<Identity, Impl, OFFSET>,
            CreateBlob: CreateBlob::<Identity, Impl, OFFSET>,
            LoadFile: LoadFile::<Identity, Impl, OFFSET>,
            CreateReadOnlyStreamFromBlob: CreateReadOnlyStreamFromBlob::<Identity, Impl, OFFSET>,
            CreateDefaultIncludeHandler: CreateDefaultIncludeHandler::<Identity, Impl, OFFSET>,
            GetBlobAsUtf8: GetBlobAsUtf8::<Identity, Impl, OFFSET>,
            GetBlobAsUtf16: GetBlobAsUtf16::<Identity, Impl, OFFSET>,
            GetDxilContainerPart: GetDxilContainerPart::<Identity, Impl, OFFSET>,
            CreateReflection: CreateReflection::<Identity, Impl, OFFSET>,
            BuildArguments: BuildArguments::<Identity, Impl, OFFSET>,
            GetPDBContents: GetPDBContents::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcUtils as windows_core::Interface>::IID
    }
}
pub trait IDxcValidator_Impl: Sized {
    fn Validate(&self, pshader: Option<&IDxcBlob>, flags: u32) -> windows_core::Result<IDxcOperationResult>;
}
impl windows_core::RuntimeName for IDxcValidator {}
impl IDxcValidator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcValidator_Impl, const OFFSET: isize>() -> IDxcValidator_Vtbl {
        unsafe extern "system" fn Validate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcValidator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *mut core::ffi::c_void, flags: u32, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcValidator_Impl::Validate(this, windows_core::from_raw_borrowed(&pshader), core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    core::ptr::write(ppresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Validate: Validate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcValidator as windows_core::Interface>::IID
    }
}
pub trait IDxcValidator2_Impl: Sized + IDxcValidator_Impl {
    fn ValidateWithDebug(&self, pshader: Option<&IDxcBlob>, flags: u32, poptdebugbitcode: *const DxcBuffer) -> windows_core::Result<IDxcOperationResult>;
}
impl windows_core::RuntimeName for IDxcValidator2 {}
impl IDxcValidator2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcValidator2_Impl, const OFFSET: isize>() -> IDxcValidator2_Vtbl {
        unsafe extern "system" fn ValidateWithDebug<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcValidator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *mut core::ffi::c_void, flags: u32, poptdebugbitcode: *const DxcBuffer, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcValidator2_Impl::ValidateWithDebug(this, windows_core::from_raw_borrowed(&pshader), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&poptdebugbitcode)) {
                Ok(ok__) => {
                    core::ptr::write(ppresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IDxcValidator_Vtbl::new::<Identity, Impl, OFFSET>(), ValidateWithDebug: ValidateWithDebug::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcValidator2 as windows_core::Interface>::IID || iid == &<IDxcValidator as windows_core::Interface>::IID
    }
}
pub trait IDxcVersionInfo_Impl: Sized {
    fn GetVersion(&self, pmajor: *mut u32, pminor: *mut u32) -> windows_core::Result<()>;
    fn GetFlags(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IDxcVersionInfo {}
impl IDxcVersionInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcVersionInfo_Impl, const OFFSET: isize>() -> IDxcVersionInfo_Vtbl {
        unsafe extern "system" fn GetVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcVersionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmajor: *mut u32, pminor: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcVersionInfo_Impl::GetVersion(this, core::mem::transmute_copy(&pmajor), core::mem::transmute_copy(&pminor)).into()
        }
        unsafe extern "system" fn GetFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcVersionInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcVersionInfo_Impl::GetFlags(this) {
                Ok(ok__) => {
                    core::ptr::write(pflags, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcVersionInfo as windows_core::Interface>::IID
    }
}
pub trait IDxcVersionInfo2_Impl: Sized + IDxcVersionInfo_Impl {
    fn GetCommitInfo(&self, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDxcVersionInfo2 {}
impl IDxcVersionInfo2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcVersionInfo2_Impl, const OFFSET: isize>() -> IDxcVersionInfo2_Vtbl {
        unsafe extern "system" fn GetCommitInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcVersionInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDxcVersionInfo2_Impl::GetCommitInfo(this, core::mem::transmute_copy(&pcommitcount), core::mem::transmute_copy(&pcommithash)).into()
        }
        Self { base__: IDxcVersionInfo_Vtbl::new::<Identity, Impl, OFFSET>(), GetCommitInfo: GetCommitInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcVersionInfo2 as windows_core::Interface>::IID || iid == &<IDxcVersionInfo as windows_core::Interface>::IID
    }
}
pub trait IDxcVersionInfo3_Impl: Sized {
    fn GetCustomVersionString(&self) -> windows_core::Result<*mut i8>;
}
impl windows_core::RuntimeName for IDxcVersionInfo3 {}
impl IDxcVersionInfo3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcVersionInfo3_Impl, const OFFSET: isize>() -> IDxcVersionInfo3_Vtbl {
        unsafe extern "system" fn GetCustomVersionString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDxcVersionInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pversionstring: *mut *mut i8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDxcVersionInfo3_Impl::GetCustomVersionString(this) {
                Ok(ok__) => {
                    core::ptr::write(pversionstring, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCustomVersionString: GetCustomVersionString::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDxcVersionInfo3 as windows_core::Interface>::IID
    }
}
