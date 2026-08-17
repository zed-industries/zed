pub trait IFindSimilarResults_Impl: Sized {
    fn GetSize(&self) -> windows_core::Result<u32>;
    fn GetNextFileId(&self, numtraitsmatched: *mut u32, similarityfileid: *mut SimilarityFileId) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IFindSimilarResults {}
impl IFindSimilarResults_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFindSimilarResults_Impl, const OFFSET: isize>() -> IFindSimilarResults_Vtbl {
        unsafe extern "system" fn GetSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFindSimilarResults_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFindSimilarResults_Impl::GetSize(this) {
                Ok(ok__) => {
                    core::ptr::write(size, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextFileId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFindSimilarResults_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numtraitsmatched: *mut u32, similarityfileid: *mut SimilarityFileId) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFindSimilarResults_Impl::GetNextFileId(this, core::mem::transmute_copy(&numtraitsmatched), core::mem::transmute_copy(&similarityfileid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetNextFileId: GetNextFileId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFindSimilarResults as windows_core::Interface>::IID
    }
}
pub trait IRdcComparator_Impl: Sized {
    fn Process(&self, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, outputbuffer: *mut RdcNeedPointer, rdc_errorcode: *mut RDC_ErrorCode) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRdcComparator {}
impl IRdcComparator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcComparator_Impl, const OFFSET: isize>() -> IRdcComparator_Vtbl {
        unsafe extern "system" fn Process<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcComparator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, outputbuffer: *mut RdcNeedPointer, rdc_errorcode: *mut RDC_ErrorCode) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRdcComparator_Impl::Process(this, core::mem::transmute_copy(&endofinput), core::mem::transmute_copy(&endofoutput), core::mem::transmute_copy(&inputbuffer), core::mem::transmute_copy(&outputbuffer), core::mem::transmute_copy(&rdc_errorcode)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Process: Process::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRdcComparator as windows_core::Interface>::IID
    }
}
pub trait IRdcFileReader_Impl: Sized {
    fn GetFileSize(&self) -> windows_core::Result<u64>;
    fn Read(&self, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetFilePosition(&self) -> windows_core::Result<u64>;
}
impl windows_core::RuntimeName for IRdcFileReader {}
impl IRdcFileReader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcFileReader_Impl, const OFFSET: isize>() -> IRdcFileReader_Vtbl {
        unsafe extern "system" fn GetFileSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcFileReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesize: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRdcFileReader_Impl::GetFileSize(this) {
                Ok(ok__) => {
                    core::ptr::write(filesize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Read<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcFileReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRdcFileReader_Impl::Read(this, core::mem::transmute_copy(&offsetfilestart), core::mem::transmute_copy(&bytestoread), core::mem::transmute_copy(&bytesactuallyread), core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&eof)).into()
        }
        unsafe extern "system" fn GetFilePosition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcFileReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsetfromstart: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRdcFileReader_Impl::GetFilePosition(this) {
                Ok(ok__) => {
                    core::ptr::write(offsetfromstart, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFileSize: GetFileSize::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            GetFilePosition: GetFilePosition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRdcFileReader as windows_core::Interface>::IID
    }
}
pub trait IRdcFileWriter_Impl: Sized + IRdcFileReader_Impl {
    fn Write(&self, offsetfilestart: u64, bytestowrite: u32) -> windows_core::Result<u8>;
    fn Truncate(&self) -> windows_core::Result<()>;
    fn DeleteOnClose(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRdcFileWriter {}
impl IRdcFileWriter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcFileWriter_Impl, const OFFSET: isize>() -> IRdcFileWriter_Vtbl {
        unsafe extern "system" fn Write<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcFileWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsetfilestart: u64, bytestowrite: u32, buffer: *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRdcFileWriter_Impl::Write(this, core::mem::transmute_copy(&offsetfilestart), core::mem::transmute_copy(&bytestowrite)) {
                Ok(ok__) => {
                    core::ptr::write(buffer, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Truncate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcFileWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRdcFileWriter_Impl::Truncate(this).into()
        }
        unsafe extern "system" fn DeleteOnClose<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcFileWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRdcFileWriter_Impl::DeleteOnClose(this).into()
        }
        Self {
            base__: IRdcFileReader_Vtbl::new::<Identity, Impl, OFFSET>(),
            Write: Write::<Identity, Impl, OFFSET>,
            Truncate: Truncate::<Identity, Impl, OFFSET>,
            DeleteOnClose: DeleteOnClose::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRdcFileWriter as windows_core::Interface>::IID || iid == &<IRdcFileReader as windows_core::Interface>::IID
    }
}
pub trait IRdcGenerator_Impl: Sized {
    fn GetGeneratorParameters(&self, level: u32) -> windows_core::Result<IRdcGeneratorParameters>;
    fn Process(&self, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, depth: u32, outputbuffers: *mut *mut RdcBufferPointer, rdc_errorcode: *mut RDC_ErrorCode) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRdcGenerator {}
impl IRdcGenerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcGenerator_Impl, const OFFSET: isize>() -> IRdcGenerator_Vtbl {
        unsafe extern "system" fn GetGeneratorParameters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcGenerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32, igeneratorparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRdcGenerator_Impl::GetGeneratorParameters(this, core::mem::transmute_copy(&level)) {
                Ok(ok__) => {
                    core::ptr::write(igeneratorparameters, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Process<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcGenerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, depth: u32, outputbuffers: *mut *mut RdcBufferPointer, rdc_errorcode: *mut RDC_ErrorCode) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRdcGenerator_Impl::Process(this, core::mem::transmute_copy(&endofinput), core::mem::transmute_copy(&endofoutput), core::mem::transmute_copy(&inputbuffer), core::mem::transmute_copy(&depth), core::mem::transmute_copy(&outputbuffers), core::mem::transmute_copy(&rdc_errorcode)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGeneratorParameters: GetGeneratorParameters::<Identity, Impl, OFFSET>,
            Process: Process::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRdcGenerator as windows_core::Interface>::IID
    }
}
pub trait IRdcGeneratorFilterMaxParameters_Impl: Sized {
    fn GetHorizonSize(&self) -> windows_core::Result<u32>;
    fn SetHorizonSize(&self, horizonsize: u32) -> windows_core::Result<()>;
    fn GetHashWindowSize(&self) -> windows_core::Result<u32>;
    fn SetHashWindowSize(&self, hashwindowsize: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRdcGeneratorFilterMaxParameters {}
impl IRdcGeneratorFilterMaxParameters_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcGeneratorFilterMaxParameters_Impl, const OFFSET: isize>() -> IRdcGeneratorFilterMaxParameters_Vtbl {
        unsafe extern "system" fn GetHorizonSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcGeneratorFilterMaxParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizonsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRdcGeneratorFilterMaxParameters_Impl::GetHorizonSize(this) {
                Ok(ok__) => {
                    core::ptr::write(horizonsize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizonSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcGeneratorFilterMaxParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizonsize: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRdcGeneratorFilterMaxParameters_Impl::SetHorizonSize(this, core::mem::transmute_copy(&horizonsize)).into()
        }
        unsafe extern "system" fn GetHashWindowSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcGeneratorFilterMaxParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hashwindowsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRdcGeneratorFilterMaxParameters_Impl::GetHashWindowSize(this) {
                Ok(ok__) => {
                    core::ptr::write(hashwindowsize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashWindowSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcGeneratorFilterMaxParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hashwindowsize: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRdcGeneratorFilterMaxParameters_Impl::SetHashWindowSize(this, core::mem::transmute_copy(&hashwindowsize)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetHorizonSize: GetHorizonSize::<Identity, Impl, OFFSET>,
            SetHorizonSize: SetHorizonSize::<Identity, Impl, OFFSET>,
            GetHashWindowSize: GetHashWindowSize::<Identity, Impl, OFFSET>,
            SetHashWindowSize: SetHashWindowSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRdcGeneratorFilterMaxParameters as windows_core::Interface>::IID
    }
}
pub trait IRdcGeneratorParameters_Impl: Sized {
    fn GetGeneratorParametersType(&self) -> windows_core::Result<GeneratorParametersType>;
    fn GetParametersVersion(&self, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> windows_core::Result<()>;
    fn GetSerializeSize(&self) -> windows_core::Result<u32>;
    fn Serialize(&self, size: u32, parametersblob: *mut u8, byteswritten: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRdcGeneratorParameters {}
impl IRdcGeneratorParameters_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcGeneratorParameters_Impl, const OFFSET: isize>() -> IRdcGeneratorParameters_Vtbl {
        unsafe extern "system" fn GetGeneratorParametersType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcGeneratorParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterstype: *mut GeneratorParametersType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRdcGeneratorParameters_Impl::GetGeneratorParametersType(this) {
                Ok(ok__) => {
                    core::ptr::write(parameterstype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParametersVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcGeneratorParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRdcGeneratorParameters_Impl::GetParametersVersion(this, core::mem::transmute_copy(&currentversion), core::mem::transmute_copy(&minimumcompatibleappversion)).into()
        }
        unsafe extern "system" fn GetSerializeSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcGeneratorParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRdcGeneratorParameters_Impl::GetSerializeSize(this) {
                Ok(ok__) => {
                    core::ptr::write(size, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Serialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcGeneratorParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: u32, parametersblob: *mut u8, byteswritten: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRdcGeneratorParameters_Impl::Serialize(this, core::mem::transmute_copy(&size), core::mem::transmute_copy(&parametersblob), core::mem::transmute_copy(&byteswritten)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGeneratorParametersType: GetGeneratorParametersType::<Identity, Impl, OFFSET>,
            GetParametersVersion: GetParametersVersion::<Identity, Impl, OFFSET>,
            GetSerializeSize: GetSerializeSize::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRdcGeneratorParameters as windows_core::Interface>::IID
    }
}
pub trait IRdcLibrary_Impl: Sized {
    fn ComputeDefaultRecursionDepth(&self, filesize: u64) -> windows_core::Result<u32>;
    fn CreateGeneratorParameters(&self, parameterstype: GeneratorParametersType, level: u32) -> windows_core::Result<IRdcGeneratorParameters>;
    fn OpenGeneratorParameters(&self, size: u32, parametersblob: *const u8) -> windows_core::Result<IRdcGeneratorParameters>;
    fn CreateGenerator(&self, depth: u32, igeneratorparametersarray: *const Option<IRdcGeneratorParameters>) -> windows_core::Result<IRdcGenerator>;
    fn CreateComparator(&self, iseedsignaturesfile: Option<&IRdcFileReader>, comparatorbuffersize: u32) -> windows_core::Result<IRdcComparator>;
    fn CreateSignatureReader(&self, ifilereader: Option<&IRdcFileReader>) -> windows_core::Result<IRdcSignatureReader>;
    fn GetRDCVersion(&self, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRdcLibrary {}
impl IRdcLibrary_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcLibrary_Impl, const OFFSET: isize>() -> IRdcLibrary_Vtbl {
        unsafe extern "system" fn ComputeDefaultRecursionDepth<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesize: u64, depth: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRdcLibrary_Impl::ComputeDefaultRecursionDepth(this, core::mem::transmute_copy(&filesize)) {
                Ok(ok__) => {
                    core::ptr::write(depth, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeneratorParameters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterstype: GeneratorParametersType, level: u32, igeneratorparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRdcLibrary_Impl::CreateGeneratorParameters(this, core::mem::transmute_copy(&parameterstype), core::mem::transmute_copy(&level)) {
                Ok(ok__) => {
                    core::ptr::write(igeneratorparameters, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenGeneratorParameters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: u32, parametersblob: *const u8, igeneratorparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRdcLibrary_Impl::OpenGeneratorParameters(this, core::mem::transmute_copy(&size), core::mem::transmute_copy(&parametersblob)) {
                Ok(ok__) => {
                    core::ptr::write(igeneratorparameters, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGenerator<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, depth: u32, igeneratorparametersarray: *const *mut core::ffi::c_void, igenerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRdcLibrary_Impl::CreateGenerator(this, core::mem::transmute_copy(&depth), core::mem::transmute_copy(&igeneratorparametersarray)) {
                Ok(ok__) => {
                    core::ptr::write(igenerator, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateComparator<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iseedsignaturesfile: *mut core::ffi::c_void, comparatorbuffersize: u32, icomparator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRdcLibrary_Impl::CreateComparator(this, windows_core::from_raw_borrowed(&iseedsignaturesfile), core::mem::transmute_copy(&comparatorbuffersize)) {
                Ok(ok__) => {
                    core::ptr::write(icomparator, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSignatureReader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ifilereader: *mut core::ffi::c_void, isignaturereader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRdcLibrary_Impl::CreateSignatureReader(this, windows_core::from_raw_borrowed(&ifilereader)) {
                Ok(ok__) => {
                    core::ptr::write(isignaturereader, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRDCVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRdcLibrary_Impl::GetRDCVersion(this, core::mem::transmute_copy(&currentversion), core::mem::transmute_copy(&minimumcompatibleappversion)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ComputeDefaultRecursionDepth: ComputeDefaultRecursionDepth::<Identity, Impl, OFFSET>,
            CreateGeneratorParameters: CreateGeneratorParameters::<Identity, Impl, OFFSET>,
            OpenGeneratorParameters: OpenGeneratorParameters::<Identity, Impl, OFFSET>,
            CreateGenerator: CreateGenerator::<Identity, Impl, OFFSET>,
            CreateComparator: CreateComparator::<Identity, Impl, OFFSET>,
            CreateSignatureReader: CreateSignatureReader::<Identity, Impl, OFFSET>,
            GetRDCVersion: GetRDCVersion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRdcLibrary as windows_core::Interface>::IID
    }
}
pub trait IRdcSignatureReader_Impl: Sized {
    fn ReadHeader(&self) -> windows_core::Result<RDC_ErrorCode>;
    fn ReadSignatures(&self, rdcsignaturepointer: *mut RdcSignaturePointer, endofoutput: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRdcSignatureReader {}
impl IRdcSignatureReader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcSignatureReader_Impl, const OFFSET: isize>() -> IRdcSignatureReader_Vtbl {
        unsafe extern "system" fn ReadHeader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcSignatureReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rdc_errorcode: *mut RDC_ErrorCode) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRdcSignatureReader_Impl::ReadHeader(this) {
                Ok(ok__) => {
                    core::ptr::write(rdc_errorcode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadSignatures<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcSignatureReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rdcsignaturepointer: *mut RdcSignaturePointer, endofoutput: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRdcSignatureReader_Impl::ReadSignatures(this, core::mem::transmute_copy(&rdcsignaturepointer), core::mem::transmute_copy(&endofoutput)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReadHeader: ReadHeader::<Identity, Impl, OFFSET>,
            ReadSignatures: ReadSignatures::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRdcSignatureReader as windows_core::Interface>::IID
    }
}
pub trait IRdcSimilarityGenerator_Impl: Sized {
    fn EnableSimilarity(&self) -> windows_core::Result<()>;
    fn Results(&self) -> windows_core::Result<SimilarityData>;
}
impl windows_core::RuntimeName for IRdcSimilarityGenerator {}
impl IRdcSimilarityGenerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcSimilarityGenerator_Impl, const OFFSET: isize>() -> IRdcSimilarityGenerator_Vtbl {
        unsafe extern "system" fn EnableSimilarity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcSimilarityGenerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRdcSimilarityGenerator_Impl::EnableSimilarity(this).into()
        }
        unsafe extern "system" fn Results<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRdcSimilarityGenerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, similaritydata: *mut SimilarityData) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRdcSimilarityGenerator_Impl::Results(this) {
                Ok(ok__) => {
                    core::ptr::write(similaritydata, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnableSimilarity: EnableSimilarity::<Identity, Impl, OFFSET>,
            Results: Results::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRdcSimilarityGenerator as windows_core::Interface>::IID
    }
}
pub trait ISimilarity_Impl: Sized {
    fn CreateTable(&self, path: &windows_core::PCWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32) -> windows_core::Result<RdcCreatedTables>;
    fn CreateTableIndirect(&self, mapping: Option<&ISimilarityTraitsMapping>, fileidfile: Option<&IRdcFileWriter>, truncate: super::super::Foundation::BOOL, recordsize: u32) -> windows_core::Result<RdcCreatedTables>;
    fn CloseTable(&self, isvalid: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Append(&self, similarityfileid: *const SimilarityFileId, similaritydata: *const SimilarityData) -> windows_core::Result<()>;
    fn FindSimilarFileId(&self, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, resultssize: u32) -> windows_core::Result<IFindSimilarResults>;
    fn CopyAndSwap(&self, newsimilaritytables: Option<&ISimilarity>, reportprogress: Option<&ISimilarityReportProgress>) -> windows_core::Result<()>;
    fn GetRecordCount(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for ISimilarity {}
impl ISimilarity_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarity_Impl, const OFFSET: isize>() -> ISimilarity_Vtbl {
        unsafe extern "system" fn CreateTable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: windows_core::PCWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32, isnew: *mut RdcCreatedTables) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISimilarity_Impl::CreateTable(this, core::mem::transmute(&path), core::mem::transmute_copy(&truncate), core::mem::transmute_copy(&securitydescriptor), core::mem::transmute_copy(&recordsize)) {
                Ok(ok__) => {
                    core::ptr::write(isnew, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTableIndirect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mapping: *mut core::ffi::c_void, fileidfile: *mut core::ffi::c_void, truncate: super::super::Foundation::BOOL, recordsize: u32, isnew: *mut RdcCreatedTables) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISimilarity_Impl::CreateTableIndirect(this, windows_core::from_raw_borrowed(&mapping), windows_core::from_raw_borrowed(&fileidfile), core::mem::transmute_copy(&truncate), core::mem::transmute_copy(&recordsize)) {
                Ok(ok__) => {
                    core::ptr::write(isnew, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseTable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isvalid: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISimilarity_Impl::CloseTable(this, core::mem::transmute_copy(&isvalid)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, similarityfileid: *const SimilarityFileId, similaritydata: *const SimilarityData) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISimilarity_Impl::Append(this, core::mem::transmute_copy(&similarityfileid), core::mem::transmute_copy(&similaritydata)).into()
        }
        unsafe extern "system" fn FindSimilarFileId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, resultssize: u32, findsimilarresults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISimilarity_Impl::FindSimilarFileId(this, core::mem::transmute_copy(&similaritydata), core::mem::transmute_copy(&numberofmatchesrequired), core::mem::transmute_copy(&resultssize)) {
                Ok(ok__) => {
                    core::ptr::write(findsimilarresults, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyAndSwap<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newsimilaritytables: *mut core::ffi::c_void, reportprogress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISimilarity_Impl::CopyAndSwap(this, windows_core::from_raw_borrowed(&newsimilaritytables), windows_core::from_raw_borrowed(&reportprogress)).into()
        }
        unsafe extern "system" fn GetRecordCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, recordcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISimilarity_Impl::GetRecordCount(this) {
                Ok(ok__) => {
                    core::ptr::write(recordcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTable: CreateTable::<Identity, Impl, OFFSET>,
            CreateTableIndirect: CreateTableIndirect::<Identity, Impl, OFFSET>,
            CloseTable: CloseTable::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            FindSimilarFileId: FindSimilarFileId::<Identity, Impl, OFFSET>,
            CopyAndSwap: CopyAndSwap::<Identity, Impl, OFFSET>,
            GetRecordCount: GetRecordCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISimilarity as windows_core::Interface>::IID
    }
}
pub trait ISimilarityFileIdTable_Impl: Sized {
    fn CreateTable(&self, path: &windows_core::PCWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32) -> windows_core::Result<RdcCreatedTables>;
    fn CreateTableIndirect(&self, fileidfile: Option<&IRdcFileWriter>, truncate: super::super::Foundation::BOOL, recordsize: u32) -> windows_core::Result<RdcCreatedTables>;
    fn CloseTable(&self, isvalid: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Append(&self, similarityfileid: *const SimilarityFileId) -> windows_core::Result<u32>;
    fn Lookup(&self, similarityfileindex: u32, similarityfileid: *mut SimilarityFileId) -> windows_core::Result<()>;
    fn Invalidate(&self, similarityfileindex: u32) -> windows_core::Result<()>;
    fn GetRecordCount(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for ISimilarityFileIdTable {}
impl ISimilarityFileIdTable_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityFileIdTable_Impl, const OFFSET: isize>() -> ISimilarityFileIdTable_Vtbl {
        unsafe extern "system" fn CreateTable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityFileIdTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: windows_core::PCWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32, isnew: *mut RdcCreatedTables) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISimilarityFileIdTable_Impl::CreateTable(this, core::mem::transmute(&path), core::mem::transmute_copy(&truncate), core::mem::transmute_copy(&securitydescriptor), core::mem::transmute_copy(&recordsize)) {
                Ok(ok__) => {
                    core::ptr::write(isnew, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTableIndirect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityFileIdTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fileidfile: *mut core::ffi::c_void, truncate: super::super::Foundation::BOOL, recordsize: u32, isnew: *mut RdcCreatedTables) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISimilarityFileIdTable_Impl::CreateTableIndirect(this, windows_core::from_raw_borrowed(&fileidfile), core::mem::transmute_copy(&truncate), core::mem::transmute_copy(&recordsize)) {
                Ok(ok__) => {
                    core::ptr::write(isnew, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseTable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityFileIdTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isvalid: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISimilarityFileIdTable_Impl::CloseTable(this, core::mem::transmute_copy(&isvalid)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityFileIdTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, similarityfileid: *const SimilarityFileId, similarityfileindex: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISimilarityFileIdTable_Impl::Append(this, core::mem::transmute_copy(&similarityfileid)) {
                Ok(ok__) => {
                    core::ptr::write(similarityfileindex, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Lookup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityFileIdTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, similarityfileindex: u32, similarityfileid: *mut SimilarityFileId) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISimilarityFileIdTable_Impl::Lookup(this, core::mem::transmute_copy(&similarityfileindex), core::mem::transmute_copy(&similarityfileid)).into()
        }
        unsafe extern "system" fn Invalidate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityFileIdTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, similarityfileindex: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISimilarityFileIdTable_Impl::Invalidate(this, core::mem::transmute_copy(&similarityfileindex)).into()
        }
        unsafe extern "system" fn GetRecordCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityFileIdTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, recordcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISimilarityFileIdTable_Impl::GetRecordCount(this) {
                Ok(ok__) => {
                    core::ptr::write(recordcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTable: CreateTable::<Identity, Impl, OFFSET>,
            CreateTableIndirect: CreateTableIndirect::<Identity, Impl, OFFSET>,
            CloseTable: CloseTable::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            Lookup: Lookup::<Identity, Impl, OFFSET>,
            Invalidate: Invalidate::<Identity, Impl, OFFSET>,
            GetRecordCount: GetRecordCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISimilarityFileIdTable as windows_core::Interface>::IID
    }
}
pub trait ISimilarityReportProgress_Impl: Sized {
    fn ReportProgress(&self, percentcompleted: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISimilarityReportProgress {}
impl ISimilarityReportProgress_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityReportProgress_Impl, const OFFSET: isize>() -> ISimilarityReportProgress_Vtbl {
        unsafe extern "system" fn ReportProgress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityReportProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, percentcompleted: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISimilarityReportProgress_Impl::ReportProgress(this, core::mem::transmute_copy(&percentcompleted)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ReportProgress: ReportProgress::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISimilarityReportProgress as windows_core::Interface>::IID
    }
}
pub trait ISimilarityTableDumpState_Impl: Sized {
    fn GetNextData(&self, resultssize: u32, resultsused: *mut u32, eof: *mut super::super::Foundation::BOOL, results: *mut SimilarityDumpData) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISimilarityTableDumpState {}
impl ISimilarityTableDumpState_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTableDumpState_Impl, const OFFSET: isize>() -> ISimilarityTableDumpState_Vtbl {
        unsafe extern "system" fn GetNextData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTableDumpState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resultssize: u32, resultsused: *mut u32, eof: *mut super::super::Foundation::BOOL, results: *mut SimilarityDumpData) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISimilarityTableDumpState_Impl::GetNextData(this, core::mem::transmute_copy(&resultssize), core::mem::transmute_copy(&resultsused), core::mem::transmute_copy(&eof), core::mem::transmute_copy(&results)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetNextData: GetNextData::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISimilarityTableDumpState as windows_core::Interface>::IID
    }
}
pub trait ISimilarityTraitsMappedView_Impl: Sized {
    fn Flush(&self) -> windows_core::Result<()>;
    fn Unmap(&self) -> windows_core::Result<()>;
    fn Get(&self, index: u64, dirty: super::super::Foundation::BOOL, numelements: u32) -> windows_core::Result<SimilarityMappedViewInfo>;
    fn GetView(&self, mappedpagebegin: *mut *mut u8, mappedpageend: *mut *mut u8);
}
impl windows_core::RuntimeName for ISimilarityTraitsMappedView {}
impl ISimilarityTraitsMappedView_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsMappedView_Impl, const OFFSET: isize>() -> ISimilarityTraitsMappedView_Vtbl {
        unsafe extern "system" fn Flush<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsMappedView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISimilarityTraitsMappedView_Impl::Flush(this).into()
        }
        unsafe extern "system" fn Unmap<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsMappedView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISimilarityTraitsMappedView_Impl::Unmap(this).into()
        }
        unsafe extern "system" fn Get<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsMappedView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u64, dirty: super::super::Foundation::BOOL, numelements: u32, viewinfo: *mut SimilarityMappedViewInfo) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISimilarityTraitsMappedView_Impl::Get(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&dirty), core::mem::transmute_copy(&numelements)) {
                Ok(ok__) => {
                    core::ptr::write(viewinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsMappedView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mappedpagebegin: *mut *mut u8, mappedpageend: *mut *mut u8) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISimilarityTraitsMappedView_Impl::GetView(this, core::mem::transmute_copy(&mappedpagebegin), core::mem::transmute_copy(&mappedpageend))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Flush: Flush::<Identity, Impl, OFFSET>,
            Unmap: Unmap::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            GetView: GetView::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISimilarityTraitsMappedView as windows_core::Interface>::IID
    }
}
pub trait ISimilarityTraitsMapping_Impl: Sized {
    fn CloseMapping(&self);
    fn SetFileSize(&self, filesize: u64) -> windows_core::Result<()>;
    fn GetFileSize(&self) -> windows_core::Result<u64>;
    fn OpenMapping(&self, accessmode: RdcMappingAccessMode, begin: u64, end: u64) -> windows_core::Result<u64>;
    fn ResizeMapping(&self, accessmode: RdcMappingAccessMode, begin: u64, end: u64) -> windows_core::Result<u64>;
    fn GetPageSize(&self, pagesize: *mut u32);
    fn CreateView(&self, minimummappedpages: u32, accessmode: RdcMappingAccessMode) -> windows_core::Result<ISimilarityTraitsMappedView>;
}
impl windows_core::RuntimeName for ISimilarityTraitsMapping {}
impl ISimilarityTraitsMapping_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsMapping_Impl, const OFFSET: isize>() -> ISimilarityTraitsMapping_Vtbl {
        unsafe extern "system" fn CloseMapping<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISimilarityTraitsMapping_Impl::CloseMapping(this)
        }
        unsafe extern "system" fn SetFileSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesize: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISimilarityTraitsMapping_Impl::SetFileSize(this, core::mem::transmute_copy(&filesize)).into()
        }
        unsafe extern "system" fn GetFileSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesize: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISimilarityTraitsMapping_Impl::GetFileSize(this) {
                Ok(ok__) => {
                    core::ptr::write(filesize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenMapping<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, accessmode: RdcMappingAccessMode, begin: u64, end: u64, actualend: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISimilarityTraitsMapping_Impl::OpenMapping(this, core::mem::transmute_copy(&accessmode), core::mem::transmute_copy(&begin), core::mem::transmute_copy(&end)) {
                Ok(ok__) => {
                    core::ptr::write(actualend, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResizeMapping<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, accessmode: RdcMappingAccessMode, begin: u64, end: u64, actualend: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISimilarityTraitsMapping_Impl::ResizeMapping(this, core::mem::transmute_copy(&accessmode), core::mem::transmute_copy(&begin), core::mem::transmute_copy(&end)) {
                Ok(ok__) => {
                    core::ptr::write(actualend, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagesize: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISimilarityTraitsMapping_Impl::GetPageSize(this, core::mem::transmute_copy(&pagesize))
        }
        unsafe extern "system" fn CreateView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minimummappedpages: u32, accessmode: RdcMappingAccessMode, mappedview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISimilarityTraitsMapping_Impl::CreateView(this, core::mem::transmute_copy(&minimummappedpages), core::mem::transmute_copy(&accessmode)) {
                Ok(ok__) => {
                    core::ptr::write(mappedview, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CloseMapping: CloseMapping::<Identity, Impl, OFFSET>,
            SetFileSize: SetFileSize::<Identity, Impl, OFFSET>,
            GetFileSize: GetFileSize::<Identity, Impl, OFFSET>,
            OpenMapping: OpenMapping::<Identity, Impl, OFFSET>,
            ResizeMapping: ResizeMapping::<Identity, Impl, OFFSET>,
            GetPageSize: GetPageSize::<Identity, Impl, OFFSET>,
            CreateView: CreateView::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISimilarityTraitsMapping as windows_core::Interface>::IID
    }
}
pub trait ISimilarityTraitsTable_Impl: Sized {
    fn CreateTable(&self, path: &windows_core::PCWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8) -> windows_core::Result<RdcCreatedTables>;
    fn CreateTableIndirect(&self, mapping: Option<&ISimilarityTraitsMapping>, truncate: super::super::Foundation::BOOL) -> windows_core::Result<RdcCreatedTables>;
    fn CloseTable(&self, isvalid: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Append(&self, data: *const SimilarityData, fileindex: u32) -> windows_core::Result<()>;
    fn FindSimilarFileIndex(&self, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, findsimilarfileindexresults: *mut FindSimilarFileIndexResults, resultssize: u32, resultsused: *mut u32) -> windows_core::Result<()>;
    fn BeginDump(&self) -> windows_core::Result<ISimilarityTableDumpState>;
    fn GetLastIndex(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for ISimilarityTraitsTable {}
impl ISimilarityTraitsTable_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsTable_Impl, const OFFSET: isize>() -> ISimilarityTraitsTable_Vtbl {
        unsafe extern "system" fn CreateTable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: windows_core::PCWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, isnew: *mut RdcCreatedTables) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISimilarityTraitsTable_Impl::CreateTable(this, core::mem::transmute(&path), core::mem::transmute_copy(&truncate), core::mem::transmute_copy(&securitydescriptor)) {
                Ok(ok__) => {
                    core::ptr::write(isnew, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTableIndirect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mapping: *mut core::ffi::c_void, truncate: super::super::Foundation::BOOL, isnew: *mut RdcCreatedTables) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISimilarityTraitsTable_Impl::CreateTableIndirect(this, windows_core::from_raw_borrowed(&mapping), core::mem::transmute_copy(&truncate)) {
                Ok(ok__) => {
                    core::ptr::write(isnew, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseTable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isvalid: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISimilarityTraitsTable_Impl::CloseTable(this, core::mem::transmute_copy(&isvalid)).into()
        }
        unsafe extern "system" fn Append<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *const SimilarityData, fileindex: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISimilarityTraitsTable_Impl::Append(this, core::mem::transmute_copy(&data), core::mem::transmute_copy(&fileindex)).into()
        }
        unsafe extern "system" fn FindSimilarFileIndex<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, findsimilarfileindexresults: *mut FindSimilarFileIndexResults, resultssize: u32, resultsused: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISimilarityTraitsTable_Impl::FindSimilarFileIndex(this, core::mem::transmute_copy(&similaritydata), core::mem::transmute_copy(&numberofmatchesrequired), core::mem::transmute_copy(&findsimilarfileindexresults), core::mem::transmute_copy(&resultssize), core::mem::transmute_copy(&resultsused)).into()
        }
        unsafe extern "system" fn BeginDump<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, similaritytabledumpstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISimilarityTraitsTable_Impl::BeginDump(this) {
                Ok(ok__) => {
                    core::ptr::write(similaritytabledumpstate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastIndex<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fileindex: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISimilarityTraitsTable_Impl::GetLastIndex(this) {
                Ok(ok__) => {
                    core::ptr::write(fileindex, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTable: CreateTable::<Identity, Impl, OFFSET>,
            CreateTableIndirect: CreateTableIndirect::<Identity, Impl, OFFSET>,
            CloseTable: CloseTable::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            FindSimilarFileIndex: FindSimilarFileIndex::<Identity, Impl, OFFSET>,
            BeginDump: BeginDump::<Identity, Impl, OFFSET>,
            GetLastIndex: GetLastIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISimilarityTraitsTable as windows_core::Interface>::IID
    }
}
