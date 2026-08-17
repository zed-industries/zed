#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FindSimilarFileIndexResults {
    pub m_FileIndex: u32,
    pub m_MatchCount: u32,
}
pub const FindSimilarResults: windows_core::GUID = windows_core::GUID::from_u128(0x96236a93_9dbc_11da_9e3f_0011114ae311);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GeneratorParametersType(pub i32);
windows_core::imp::define_interface!(IFindSimilarResults, IFindSimilarResults_Vtbl, 0x96236a81_9dbc_11da_9e3f_0011114ae311);
windows_core::imp::interface_hierarchy!(IFindSimilarResults, windows_core::IUnknown);
impl IFindSimilarResults {
    pub unsafe fn GetSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetNextFileId(&self, numtraitsmatched: *mut u32, similarityfileid: *mut SimilarityFileId) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetNextFileId)(windows_core::Interface::as_raw(self), numtraitsmatched as _, similarityfileid as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindSimilarResults_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetNextFileId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut SimilarityFileId) -> windows_core::HRESULT,
}
pub trait IFindSimilarResults_Impl: windows_core::IUnknownImpl {
    fn GetSize(&self) -> windows_core::Result<u32>;
    fn GetNextFileId(&self, numtraitsmatched: *mut u32, similarityfileid: *mut SimilarityFileId) -> windows_core::Result<()>;
}
impl IFindSimilarResults_Vtbl {
    pub const fn new<Identity: IFindSimilarResults_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSize<Identity: IFindSimilarResults_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFindSimilarResults_Impl::GetSize(this) {
                    Ok(ok__) => {
                        size.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNextFileId<Identity: IFindSimilarResults_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numtraitsmatched: *mut u32, similarityfileid: *mut SimilarityFileId) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFindSimilarResults_Impl::GetNextFileId(this, core::mem::transmute_copy(&numtraitsmatched), core::mem::transmute_copy(&similarityfileid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSize: GetSize::<Identity, OFFSET>,
            GetNextFileId: GetNextFileId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFindSimilarResults as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IFindSimilarResults {}
windows_core::imp::define_interface!(IRdcComparator, IRdcComparator_Vtbl, 0x96236a77_9dbc_11da_9e3f_0011114ae311);
windows_core::imp::interface_hierarchy!(IRdcComparator, windows_core::IUnknown);
impl IRdcComparator {
    pub unsafe fn Process(&self, endofinput: bool, endofoutput: *mut windows_core::BOOL, inputbuffer: *mut RdcBufferPointer, outputbuffer: *mut RdcNeedPointer, rdc_errorcode: *mut RDC_ErrorCode) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Process)(windows_core::Interface::as_raw(self), endofinput.into(), endofoutput as _, inputbuffer as _, outputbuffer as _, rdc_errorcode as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcComparator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Process: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut windows_core::BOOL, *mut RdcBufferPointer, *mut RdcNeedPointer, *mut RDC_ErrorCode) -> windows_core::HRESULT,
}
pub trait IRdcComparator_Impl: windows_core::IUnknownImpl {
    fn Process(&self, endofinput: windows_core::BOOL, endofoutput: *mut windows_core::BOOL, inputbuffer: *mut RdcBufferPointer, outputbuffer: *mut RdcNeedPointer, rdc_errorcode: *mut RDC_ErrorCode) -> windows_core::Result<()>;
}
impl IRdcComparator_Vtbl {
    pub const fn new<Identity: IRdcComparator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Process<Identity: IRdcComparator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endofinput: windows_core::BOOL, endofoutput: *mut windows_core::BOOL, inputbuffer: *mut RdcBufferPointer, outputbuffer: *mut RdcNeedPointer, rdc_errorcode: *mut RDC_ErrorCode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRdcComparator_Impl::Process(this, core::mem::transmute_copy(&endofinput), core::mem::transmute_copy(&endofoutput), core::mem::transmute_copy(&inputbuffer), core::mem::transmute_copy(&outputbuffer), core::mem::transmute_copy(&rdc_errorcode)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Process: Process::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRdcComparator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRdcComparator {}
windows_core::imp::define_interface!(IRdcFileReader, IRdcFileReader_Vtbl, 0x96236a74_9dbc_11da_9e3f_0011114ae311);
windows_core::imp::interface_hierarchy!(IRdcFileReader, windows_core::IUnknown);
impl IRdcFileReader {
    pub unsafe fn GetFileSize(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Read(&self, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), offsetfilestart, bytestoread, bytesactuallyread as _, buffer as _, eof as _).ok() }
    }
    pub unsafe fn GetFilePosition(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFilePosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcFileReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFileSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u32, *mut u32, *mut u8, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetFilePosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
pub trait IRdcFileReader_Impl: windows_core::IUnknownImpl {
    fn GetFileSize(&self) -> windows_core::Result<u64>;
    fn Read(&self, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetFilePosition(&self) -> windows_core::Result<u64>;
}
impl IRdcFileReader_Vtbl {
    pub const fn new<Identity: IRdcFileReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFileSize<Identity: IRdcFileReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesize: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRdcFileReader_Impl::GetFileSize(this) {
                    Ok(ok__) => {
                        filesize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Read<Identity: IRdcFileReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRdcFileReader_Impl::Read(this, core::mem::transmute_copy(&offsetfilestart), core::mem::transmute_copy(&bytestoread), core::mem::transmute_copy(&bytesactuallyread), core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&eof)).into()
            }
        }
        unsafe extern "system" fn GetFilePosition<Identity: IRdcFileReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsetfromstart: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRdcFileReader_Impl::GetFilePosition(this) {
                    Ok(ok__) => {
                        offsetfromstart.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFileSize: GetFileSize::<Identity, OFFSET>,
            Read: Read::<Identity, OFFSET>,
            GetFilePosition: GetFilePosition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRdcFileReader as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRdcFileReader {}
windows_core::imp::define_interface!(IRdcFileWriter, IRdcFileWriter_Vtbl, 0x96236a75_9dbc_11da_9e3f_0011114ae311);
impl core::ops::Deref for IRdcFileWriter {
    type Target = IRdcFileReader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRdcFileWriter, windows_core::IUnknown, IRdcFileReader);
impl IRdcFileWriter {
    pub unsafe fn Write(&self, offsetfilestart: u64, bytestowrite: u32) -> windows_core::Result<u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), offsetfilestart, bytestowrite, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Truncate(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Truncate)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn DeleteOnClose(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteOnClose)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcFileWriter_Vtbl {
    pub base__: IRdcFileReader_Vtbl,
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u32, *mut u8) -> windows_core::HRESULT,
    pub Truncate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteOnClose: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRdcFileWriter_Impl: IRdcFileReader_Impl {
    fn Write(&self, offsetfilestart: u64, bytestowrite: u32) -> windows_core::Result<u8>;
    fn Truncate(&self) -> windows_core::Result<()>;
    fn DeleteOnClose(&self) -> windows_core::Result<()>;
}
impl IRdcFileWriter_Vtbl {
    pub const fn new<Identity: IRdcFileWriter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Write<Identity: IRdcFileWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsetfilestart: u64, bytestowrite: u32, buffer: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRdcFileWriter_Impl::Write(this, core::mem::transmute_copy(&offsetfilestart), core::mem::transmute_copy(&bytestowrite)) {
                    Ok(ok__) => {
                        buffer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Truncate<Identity: IRdcFileWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRdcFileWriter_Impl::Truncate(this).into()
            }
        }
        unsafe extern "system" fn DeleteOnClose<Identity: IRdcFileWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRdcFileWriter_Impl::DeleteOnClose(this).into()
            }
        }
        Self {
            base__: IRdcFileReader_Vtbl::new::<Identity, OFFSET>(),
            Write: Write::<Identity, OFFSET>,
            Truncate: Truncate::<Identity, OFFSET>,
            DeleteOnClose: DeleteOnClose::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRdcFileWriter as windows_core::Interface>::IID || iid == &<IRdcFileReader as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRdcFileWriter {}
windows_core::imp::define_interface!(IRdcGenerator, IRdcGenerator_Vtbl, 0x96236a73_9dbc_11da_9e3f_0011114ae311);
windows_core::imp::interface_hierarchy!(IRdcGenerator, windows_core::IUnknown);
impl IRdcGenerator {
    pub unsafe fn GetGeneratorParameters(&self, level: u32) -> windows_core::Result<IRdcGeneratorParameters> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGeneratorParameters)(windows_core::Interface::as_raw(self), level, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Process(&self, endofinput: bool, endofoutput: *mut windows_core::BOOL, inputbuffer: *mut RdcBufferPointer, outputbuffers: &mut [*mut RdcBufferPointer], rdc_errorcode: *mut RDC_ErrorCode) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Process)(windows_core::Interface::as_raw(self), endofinput.into(), endofoutput as _, inputbuffer as _, outputbuffers.len().try_into().unwrap(), core::mem::transmute(outputbuffers.as_ptr()), rdc_errorcode as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcGenerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetGeneratorParameters: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Process: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut windows_core::BOOL, *mut RdcBufferPointer, u32, *mut *mut RdcBufferPointer, *mut RDC_ErrorCode) -> windows_core::HRESULT,
}
pub trait IRdcGenerator_Impl: windows_core::IUnknownImpl {
    fn GetGeneratorParameters(&self, level: u32) -> windows_core::Result<IRdcGeneratorParameters>;
    fn Process(&self, endofinput: windows_core::BOOL, endofoutput: *mut windows_core::BOOL, inputbuffer: *mut RdcBufferPointer, depth: u32, outputbuffers: *mut *mut RdcBufferPointer, rdc_errorcode: *mut RDC_ErrorCode) -> windows_core::Result<()>;
}
impl IRdcGenerator_Vtbl {
    pub const fn new<Identity: IRdcGenerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGeneratorParameters<Identity: IRdcGenerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32, igeneratorparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRdcGenerator_Impl::GetGeneratorParameters(this, core::mem::transmute_copy(&level)) {
                    Ok(ok__) => {
                        igeneratorparameters.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Process<Identity: IRdcGenerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endofinput: windows_core::BOOL, endofoutput: *mut windows_core::BOOL, inputbuffer: *mut RdcBufferPointer, depth: u32, outputbuffers: *mut *mut RdcBufferPointer, rdc_errorcode: *mut RDC_ErrorCode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRdcGenerator_Impl::Process(this, core::mem::transmute_copy(&endofinput), core::mem::transmute_copy(&endofoutput), core::mem::transmute_copy(&inputbuffer), core::mem::transmute_copy(&depth), core::mem::transmute_copy(&outputbuffers), core::mem::transmute_copy(&rdc_errorcode)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGeneratorParameters: GetGeneratorParameters::<Identity, OFFSET>,
            Process: Process::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRdcGenerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRdcGenerator {}
windows_core::imp::define_interface!(IRdcGeneratorFilterMaxParameters, IRdcGeneratorFilterMaxParameters_Vtbl, 0x96236a72_9dbc_11da_9e3f_0011114ae311);
windows_core::imp::interface_hierarchy!(IRdcGeneratorFilterMaxParameters, windows_core::IUnknown);
impl IRdcGeneratorFilterMaxParameters {
    pub unsafe fn GetHorizonSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHorizonSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHorizonSize(&self, horizonsize: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHorizonSize)(windows_core::Interface::as_raw(self), horizonsize).ok() }
    }
    pub unsafe fn GetHashWindowSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHashWindowSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHashWindowSize(&self, hashwindowsize: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHashWindowSize)(windows_core::Interface::as_raw(self), hashwindowsize).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcGeneratorFilterMaxParameters_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetHorizonSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetHorizonSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetHashWindowSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetHashWindowSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IRdcGeneratorFilterMaxParameters_Impl: windows_core::IUnknownImpl {
    fn GetHorizonSize(&self) -> windows_core::Result<u32>;
    fn SetHorizonSize(&self, horizonsize: u32) -> windows_core::Result<()>;
    fn GetHashWindowSize(&self) -> windows_core::Result<u32>;
    fn SetHashWindowSize(&self, hashwindowsize: u32) -> windows_core::Result<()>;
}
impl IRdcGeneratorFilterMaxParameters_Vtbl {
    pub const fn new<Identity: IRdcGeneratorFilterMaxParameters_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetHorizonSize<Identity: IRdcGeneratorFilterMaxParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizonsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRdcGeneratorFilterMaxParameters_Impl::GetHorizonSize(this) {
                    Ok(ok__) => {
                        horizonsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHorizonSize<Identity: IRdcGeneratorFilterMaxParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizonsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRdcGeneratorFilterMaxParameters_Impl::SetHorizonSize(this, core::mem::transmute_copy(&horizonsize)).into()
            }
        }
        unsafe extern "system" fn GetHashWindowSize<Identity: IRdcGeneratorFilterMaxParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hashwindowsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRdcGeneratorFilterMaxParameters_Impl::GetHashWindowSize(this) {
                    Ok(ok__) => {
                        hashwindowsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHashWindowSize<Identity: IRdcGeneratorFilterMaxParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hashwindowsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRdcGeneratorFilterMaxParameters_Impl::SetHashWindowSize(this, core::mem::transmute_copy(&hashwindowsize)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetHorizonSize: GetHorizonSize::<Identity, OFFSET>,
            SetHorizonSize: SetHorizonSize::<Identity, OFFSET>,
            GetHashWindowSize: GetHashWindowSize::<Identity, OFFSET>,
            SetHashWindowSize: SetHashWindowSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRdcGeneratorFilterMaxParameters as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRdcGeneratorFilterMaxParameters {}
windows_core::imp::define_interface!(IRdcGeneratorParameters, IRdcGeneratorParameters_Vtbl, 0x96236a71_9dbc_11da_9e3f_0011114ae311);
windows_core::imp::interface_hierarchy!(IRdcGeneratorParameters, windows_core::IUnknown);
impl IRdcGeneratorParameters {
    pub unsafe fn GetGeneratorParametersType(&self) -> windows_core::Result<GeneratorParametersType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGeneratorParametersType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetParametersVersion(&self, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetParametersVersion)(windows_core::Interface::as_raw(self), currentversion as _, minimumcompatibleappversion as _).ok() }
    }
    pub unsafe fn GetSerializeSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSerializeSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Serialize(&self, size: u32, parametersblob: *mut u8, byteswritten: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Serialize)(windows_core::Interface::as_raw(self), size, parametersblob as _, byteswritten as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcGeneratorParameters_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetGeneratorParametersType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GeneratorParametersType) -> windows_core::HRESULT,
    pub GetParametersVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetSerializeSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Serialize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
}
pub trait IRdcGeneratorParameters_Impl: windows_core::IUnknownImpl {
    fn GetGeneratorParametersType(&self) -> windows_core::Result<GeneratorParametersType>;
    fn GetParametersVersion(&self, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> windows_core::Result<()>;
    fn GetSerializeSize(&self) -> windows_core::Result<u32>;
    fn Serialize(&self, size: u32, parametersblob: *mut u8, byteswritten: *mut u32) -> windows_core::Result<()>;
}
impl IRdcGeneratorParameters_Vtbl {
    pub const fn new<Identity: IRdcGeneratorParameters_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGeneratorParametersType<Identity: IRdcGeneratorParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterstype: *mut GeneratorParametersType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRdcGeneratorParameters_Impl::GetGeneratorParametersType(this) {
                    Ok(ok__) => {
                        parameterstype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetParametersVersion<Identity: IRdcGeneratorParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRdcGeneratorParameters_Impl::GetParametersVersion(this, core::mem::transmute_copy(&currentversion), core::mem::transmute_copy(&minimumcompatibleappversion)).into()
            }
        }
        unsafe extern "system" fn GetSerializeSize<Identity: IRdcGeneratorParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRdcGeneratorParameters_Impl::GetSerializeSize(this) {
                    Ok(ok__) => {
                        size.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Serialize<Identity: IRdcGeneratorParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: u32, parametersblob: *mut u8, byteswritten: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRdcGeneratorParameters_Impl::Serialize(this, core::mem::transmute_copy(&size), core::mem::transmute_copy(&parametersblob), core::mem::transmute_copy(&byteswritten)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGeneratorParametersType: GetGeneratorParametersType::<Identity, OFFSET>,
            GetParametersVersion: GetParametersVersion::<Identity, OFFSET>,
            GetSerializeSize: GetSerializeSize::<Identity, OFFSET>,
            Serialize: Serialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRdcGeneratorParameters as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRdcGeneratorParameters {}
windows_core::imp::define_interface!(IRdcLibrary, IRdcLibrary_Vtbl, 0x96236a78_9dbc_11da_9e3f_0011114ae311);
windows_core::imp::interface_hierarchy!(IRdcLibrary, windows_core::IUnknown);
impl IRdcLibrary {
    pub unsafe fn ComputeDefaultRecursionDepth(&self, filesize: u64) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComputeDefaultRecursionDepth)(windows_core::Interface::as_raw(self), filesize, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateGeneratorParameters(&self, parameterstype: GeneratorParametersType, level: u32) -> windows_core::Result<IRdcGeneratorParameters> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGeneratorParameters)(windows_core::Interface::as_raw(self), parameterstype, level, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OpenGeneratorParameters(&self, size: u32, parametersblob: *const u8) -> windows_core::Result<IRdcGeneratorParameters> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenGeneratorParameters)(windows_core::Interface::as_raw(self), size, parametersblob, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateGenerator(&self, igeneratorparametersarray: &[Option<IRdcGeneratorParameters>]) -> windows_core::Result<IRdcGenerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGenerator)(windows_core::Interface::as_raw(self), igeneratorparametersarray.len().try_into().unwrap(), core::mem::transmute(igeneratorparametersarray.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateComparator<P0>(&self, iseedsignaturesfile: P0, comparatorbuffersize: u32) -> windows_core::Result<IRdcComparator>
    where
        P0: windows_core::Param<IRdcFileReader>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateComparator)(windows_core::Interface::as_raw(self), iseedsignaturesfile.param().abi(), comparatorbuffersize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateSignatureReader<P0>(&self, ifilereader: P0) -> windows_core::Result<IRdcSignatureReader>
    where
        P0: windows_core::Param<IRdcFileReader>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSignatureReader)(windows_core::Interface::as_raw(self), ifilereader.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRDCVersion(&self, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRDCVersion)(windows_core::Interface::as_raw(self), currentversion as _, minimumcompatibleappversion as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcLibrary_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ComputeDefaultRecursionDepth: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut u32) -> windows_core::HRESULT,
    pub CreateGeneratorParameters: unsafe extern "system" fn(*mut core::ffi::c_void, GeneratorParametersType, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenGeneratorParameters: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateGenerator: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateComparator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSignatureReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRDCVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IRdcLibrary_Impl: windows_core::IUnknownImpl {
    fn ComputeDefaultRecursionDepth(&self, filesize: u64) -> windows_core::Result<u32>;
    fn CreateGeneratorParameters(&self, parameterstype: GeneratorParametersType, level: u32) -> windows_core::Result<IRdcGeneratorParameters>;
    fn OpenGeneratorParameters(&self, size: u32, parametersblob: *const u8) -> windows_core::Result<IRdcGeneratorParameters>;
    fn CreateGenerator(&self, depth: u32, igeneratorparametersarray: *const Option<IRdcGeneratorParameters>) -> windows_core::Result<IRdcGenerator>;
    fn CreateComparator(&self, iseedsignaturesfile: windows_core::Ref<IRdcFileReader>, comparatorbuffersize: u32) -> windows_core::Result<IRdcComparator>;
    fn CreateSignatureReader(&self, ifilereader: windows_core::Ref<IRdcFileReader>) -> windows_core::Result<IRdcSignatureReader>;
    fn GetRDCVersion(&self, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> windows_core::Result<()>;
}
impl IRdcLibrary_Vtbl {
    pub const fn new<Identity: IRdcLibrary_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ComputeDefaultRecursionDepth<Identity: IRdcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesize: u64, depth: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRdcLibrary_Impl::ComputeDefaultRecursionDepth(this, core::mem::transmute_copy(&filesize)) {
                    Ok(ok__) => {
                        depth.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateGeneratorParameters<Identity: IRdcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterstype: GeneratorParametersType, level: u32, igeneratorparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRdcLibrary_Impl::CreateGeneratorParameters(this, core::mem::transmute_copy(&parameterstype), core::mem::transmute_copy(&level)) {
                    Ok(ok__) => {
                        igeneratorparameters.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenGeneratorParameters<Identity: IRdcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: u32, parametersblob: *const u8, igeneratorparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRdcLibrary_Impl::OpenGeneratorParameters(this, core::mem::transmute_copy(&size), core::mem::transmute_copy(&parametersblob)) {
                    Ok(ok__) => {
                        igeneratorparameters.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateGenerator<Identity: IRdcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, depth: u32, igeneratorparametersarray: *const *mut core::ffi::c_void, igenerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRdcLibrary_Impl::CreateGenerator(this, core::mem::transmute_copy(&depth), core::mem::transmute_copy(&igeneratorparametersarray)) {
                    Ok(ok__) => {
                        igenerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateComparator<Identity: IRdcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iseedsignaturesfile: *mut core::ffi::c_void, comparatorbuffersize: u32, icomparator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRdcLibrary_Impl::CreateComparator(this, core::mem::transmute_copy(&iseedsignaturesfile), core::mem::transmute_copy(&comparatorbuffersize)) {
                    Ok(ok__) => {
                        icomparator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSignatureReader<Identity: IRdcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ifilereader: *mut core::ffi::c_void, isignaturereader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRdcLibrary_Impl::CreateSignatureReader(this, core::mem::transmute_copy(&ifilereader)) {
                    Ok(ok__) => {
                        isignaturereader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRDCVersion<Identity: IRdcLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRdcLibrary_Impl::GetRDCVersion(this, core::mem::transmute_copy(&currentversion), core::mem::transmute_copy(&minimumcompatibleappversion)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ComputeDefaultRecursionDepth: ComputeDefaultRecursionDepth::<Identity, OFFSET>,
            CreateGeneratorParameters: CreateGeneratorParameters::<Identity, OFFSET>,
            OpenGeneratorParameters: OpenGeneratorParameters::<Identity, OFFSET>,
            CreateGenerator: CreateGenerator::<Identity, OFFSET>,
            CreateComparator: CreateComparator::<Identity, OFFSET>,
            CreateSignatureReader: CreateSignatureReader::<Identity, OFFSET>,
            GetRDCVersion: GetRDCVersion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRdcLibrary as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRdcLibrary {}
windows_core::imp::define_interface!(IRdcSignatureReader, IRdcSignatureReader_Vtbl, 0x96236a76_9dbc_11da_9e3f_0011114ae311);
windows_core::imp::interface_hierarchy!(IRdcSignatureReader, windows_core::IUnknown);
impl IRdcSignatureReader {
    pub unsafe fn ReadHeader(&self) -> windows_core::Result<RDC_ErrorCode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadHeader)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReadSignatures(&self, rdcsignaturepointer: *mut RdcSignaturePointer, endofoutput: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReadSignatures)(windows_core::Interface::as_raw(self), rdcsignaturepointer as _, endofoutput as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcSignatureReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReadHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RDC_ErrorCode) -> windows_core::HRESULT,
    pub ReadSignatures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RdcSignaturePointer, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IRdcSignatureReader_Impl: windows_core::IUnknownImpl {
    fn ReadHeader(&self) -> windows_core::Result<RDC_ErrorCode>;
    fn ReadSignatures(&self, rdcsignaturepointer: *mut RdcSignaturePointer, endofoutput: *mut windows_core::BOOL) -> windows_core::Result<()>;
}
impl IRdcSignatureReader_Vtbl {
    pub const fn new<Identity: IRdcSignatureReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReadHeader<Identity: IRdcSignatureReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rdc_errorcode: *mut RDC_ErrorCode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRdcSignatureReader_Impl::ReadHeader(this) {
                    Ok(ok__) => {
                        rdc_errorcode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReadSignatures<Identity: IRdcSignatureReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rdcsignaturepointer: *mut RdcSignaturePointer, endofoutput: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRdcSignatureReader_Impl::ReadSignatures(this, core::mem::transmute_copy(&rdcsignaturepointer), core::mem::transmute_copy(&endofoutput)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReadHeader: ReadHeader::<Identity, OFFSET>,
            ReadSignatures: ReadSignatures::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRdcSignatureReader as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRdcSignatureReader {}
windows_core::imp::define_interface!(IRdcSimilarityGenerator, IRdcSimilarityGenerator_Vtbl, 0x96236a80_9dbc_11da_9e3f_0011114ae311);
windows_core::imp::interface_hierarchy!(IRdcSimilarityGenerator, windows_core::IUnknown);
impl IRdcSimilarityGenerator {
    pub unsafe fn EnableSimilarity(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnableSimilarity)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Results(&self) -> windows_core::Result<SimilarityData> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Results)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcSimilarityGenerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnableSimilarity: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Results: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SimilarityData) -> windows_core::HRESULT,
}
pub trait IRdcSimilarityGenerator_Impl: windows_core::IUnknownImpl {
    fn EnableSimilarity(&self) -> windows_core::Result<()>;
    fn Results(&self) -> windows_core::Result<SimilarityData>;
}
impl IRdcSimilarityGenerator_Vtbl {
    pub const fn new<Identity: IRdcSimilarityGenerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnableSimilarity<Identity: IRdcSimilarityGenerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRdcSimilarityGenerator_Impl::EnableSimilarity(this).into()
            }
        }
        unsafe extern "system" fn Results<Identity: IRdcSimilarityGenerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, similaritydata: *mut SimilarityData) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRdcSimilarityGenerator_Impl::Results(this) {
                    Ok(ok__) => {
                        similaritydata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnableSimilarity: EnableSimilarity::<Identity, OFFSET>,
            Results: Results::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRdcSimilarityGenerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRdcSimilarityGenerator {}
windows_core::imp::define_interface!(ISimilarity, ISimilarity_Vtbl, 0x96236a83_9dbc_11da_9e3f_0011114ae311);
windows_core::imp::interface_hierarchy!(ISimilarity, windows_core::IUnknown);
impl ISimilarity {
    pub unsafe fn CreateTable<P0>(&self, path: P0, truncate: bool, securitydescriptor: *const u8, recordsize: u32) -> windows_core::Result<RdcCreatedTables>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTable)(windows_core::Interface::as_raw(self), path.param().abi(), truncate.into(), securitydescriptor, recordsize, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateTableIndirect<P0, P1>(&self, mapping: P0, fileidfile: P1, truncate: bool, recordsize: u32) -> windows_core::Result<RdcCreatedTables>
    where
        P0: windows_core::Param<ISimilarityTraitsMapping>,
        P1: windows_core::Param<IRdcFileWriter>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTableIndirect)(windows_core::Interface::as_raw(self), mapping.param().abi(), fileidfile.param().abi(), truncate.into(), recordsize, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CloseTable(&self, isvalid: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CloseTable)(windows_core::Interface::as_raw(self), isvalid.into()).ok() }
    }
    pub unsafe fn Append(&self, similarityfileid: *const SimilarityFileId, similaritydata: *const SimilarityData) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), similarityfileid, similaritydata).ok() }
    }
    pub unsafe fn FindSimilarFileId(&self, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, resultssize: u32) -> windows_core::Result<IFindSimilarResults> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindSimilarFileId)(windows_core::Interface::as_raw(self), similaritydata, numberofmatchesrequired, resultssize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CopyAndSwap<P0, P1>(&self, newsimilaritytables: P0, reportprogress: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISimilarity>,
        P1: windows_core::Param<ISimilarityReportProgress>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyAndSwap)(windows_core::Interface::as_raw(self), newsimilaritytables.param().abi(), reportprogress.param().abi()).ok() }
    }
    pub unsafe fn GetRecordCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRecordCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateTable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::BOOL, *const u8, u32, *mut RdcCreatedTables) -> windows_core::HRESULT,
    pub CreateTableIndirect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, u32, *mut RdcCreatedTables) -> windows_core::HRESULT,
    pub CloseTable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *const SimilarityFileId, *const SimilarityData) -> windows_core::HRESULT,
    pub FindSimilarFileId: unsafe extern "system" fn(*mut core::ffi::c_void, *const SimilarityData, u16, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyAndSwap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRecordCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait ISimilarity_Impl: windows_core::IUnknownImpl {
    fn CreateTable(&self, path: &windows_core::PCWSTR, truncate: windows_core::BOOL, securitydescriptor: *const u8, recordsize: u32) -> windows_core::Result<RdcCreatedTables>;
    fn CreateTableIndirect(&self, mapping: windows_core::Ref<ISimilarityTraitsMapping>, fileidfile: windows_core::Ref<IRdcFileWriter>, truncate: windows_core::BOOL, recordsize: u32) -> windows_core::Result<RdcCreatedTables>;
    fn CloseTable(&self, isvalid: windows_core::BOOL) -> windows_core::Result<()>;
    fn Append(&self, similarityfileid: *const SimilarityFileId, similaritydata: *const SimilarityData) -> windows_core::Result<()>;
    fn FindSimilarFileId(&self, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, resultssize: u32) -> windows_core::Result<IFindSimilarResults>;
    fn CopyAndSwap(&self, newsimilaritytables: windows_core::Ref<ISimilarity>, reportprogress: windows_core::Ref<ISimilarityReportProgress>) -> windows_core::Result<()>;
    fn GetRecordCount(&self) -> windows_core::Result<u32>;
}
impl ISimilarity_Vtbl {
    pub const fn new<Identity: ISimilarity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateTable<Identity: ISimilarity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: windows_core::PCWSTR, truncate: windows_core::BOOL, securitydescriptor: *const u8, recordsize: u32, isnew: *mut RdcCreatedTables) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISimilarity_Impl::CreateTable(this, core::mem::transmute(&path), core::mem::transmute_copy(&truncate), core::mem::transmute_copy(&securitydescriptor), core::mem::transmute_copy(&recordsize)) {
                    Ok(ok__) => {
                        isnew.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTableIndirect<Identity: ISimilarity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mapping: *mut core::ffi::c_void, fileidfile: *mut core::ffi::c_void, truncate: windows_core::BOOL, recordsize: u32, isnew: *mut RdcCreatedTables) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISimilarity_Impl::CreateTableIndirect(this, core::mem::transmute_copy(&mapping), core::mem::transmute_copy(&fileidfile), core::mem::transmute_copy(&truncate), core::mem::transmute_copy(&recordsize)) {
                    Ok(ok__) => {
                        isnew.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CloseTable<Identity: ISimilarity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isvalid: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISimilarity_Impl::CloseTable(this, core::mem::transmute_copy(&isvalid)).into()
            }
        }
        unsafe extern "system" fn Append<Identity: ISimilarity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, similarityfileid: *const SimilarityFileId, similaritydata: *const SimilarityData) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISimilarity_Impl::Append(this, core::mem::transmute_copy(&similarityfileid), core::mem::transmute_copy(&similaritydata)).into()
            }
        }
        unsafe extern "system" fn FindSimilarFileId<Identity: ISimilarity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, resultssize: u32, findsimilarresults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISimilarity_Impl::FindSimilarFileId(this, core::mem::transmute_copy(&similaritydata), core::mem::transmute_copy(&numberofmatchesrequired), core::mem::transmute_copy(&resultssize)) {
                    Ok(ok__) => {
                        findsimilarresults.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CopyAndSwap<Identity: ISimilarity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newsimilaritytables: *mut core::ffi::c_void, reportprogress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISimilarity_Impl::CopyAndSwap(this, core::mem::transmute_copy(&newsimilaritytables), core::mem::transmute_copy(&reportprogress)).into()
            }
        }
        unsafe extern "system" fn GetRecordCount<Identity: ISimilarity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, recordcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISimilarity_Impl::GetRecordCount(this) {
                    Ok(ok__) => {
                        recordcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTable: CreateTable::<Identity, OFFSET>,
            CreateTableIndirect: CreateTableIndirect::<Identity, OFFSET>,
            CloseTable: CloseTable::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
            FindSimilarFileId: FindSimilarFileId::<Identity, OFFSET>,
            CopyAndSwap: CopyAndSwap::<Identity, OFFSET>,
            GetRecordCount: GetRecordCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISimilarity as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISimilarity {}
windows_core::imp::define_interface!(ISimilarityFileIdTable, ISimilarityFileIdTable_Vtbl, 0x96236a7f_9dbc_11da_9e3f_0011114ae311);
windows_core::imp::interface_hierarchy!(ISimilarityFileIdTable, windows_core::IUnknown);
impl ISimilarityFileIdTable {
    pub unsafe fn CreateTable<P0>(&self, path: P0, truncate: bool, securitydescriptor: *const u8, recordsize: u32) -> windows_core::Result<RdcCreatedTables>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTable)(windows_core::Interface::as_raw(self), path.param().abi(), truncate.into(), securitydescriptor, recordsize, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateTableIndirect<P0>(&self, fileidfile: P0, truncate: bool, recordsize: u32) -> windows_core::Result<RdcCreatedTables>
    where
        P0: windows_core::Param<IRdcFileWriter>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTableIndirect)(windows_core::Interface::as_raw(self), fileidfile.param().abi(), truncate.into(), recordsize, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CloseTable(&self, isvalid: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CloseTable)(windows_core::Interface::as_raw(self), isvalid.into()).ok() }
    }
    pub unsafe fn Append(&self, similarityfileid: *const SimilarityFileId) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), similarityfileid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Lookup(&self, similarityfileindex: u32, similarityfileid: *mut SimilarityFileId) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Lookup)(windows_core::Interface::as_raw(self), similarityfileindex, similarityfileid as _).ok() }
    }
    pub unsafe fn Invalidate(&self, similarityfileindex: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Invalidate)(windows_core::Interface::as_raw(self), similarityfileindex).ok() }
    }
    pub unsafe fn GetRecordCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRecordCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarityFileIdTable_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateTable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::BOOL, *const u8, u32, *mut RdcCreatedTables) -> windows_core::HRESULT,
    pub CreateTableIndirect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, u32, *mut RdcCreatedTables) -> windows_core::HRESULT,
    pub CloseTable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *const SimilarityFileId, *mut u32) -> windows_core::HRESULT,
    pub Lookup: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut SimilarityFileId) -> windows_core::HRESULT,
    pub Invalidate: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetRecordCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait ISimilarityFileIdTable_Impl: windows_core::IUnknownImpl {
    fn CreateTable(&self, path: &windows_core::PCWSTR, truncate: windows_core::BOOL, securitydescriptor: *const u8, recordsize: u32) -> windows_core::Result<RdcCreatedTables>;
    fn CreateTableIndirect(&self, fileidfile: windows_core::Ref<IRdcFileWriter>, truncate: windows_core::BOOL, recordsize: u32) -> windows_core::Result<RdcCreatedTables>;
    fn CloseTable(&self, isvalid: windows_core::BOOL) -> windows_core::Result<()>;
    fn Append(&self, similarityfileid: *const SimilarityFileId) -> windows_core::Result<u32>;
    fn Lookup(&self, similarityfileindex: u32, similarityfileid: *mut SimilarityFileId) -> windows_core::Result<()>;
    fn Invalidate(&self, similarityfileindex: u32) -> windows_core::Result<()>;
    fn GetRecordCount(&self) -> windows_core::Result<u32>;
}
impl ISimilarityFileIdTable_Vtbl {
    pub const fn new<Identity: ISimilarityFileIdTable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateTable<Identity: ISimilarityFileIdTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: windows_core::PCWSTR, truncate: windows_core::BOOL, securitydescriptor: *const u8, recordsize: u32, isnew: *mut RdcCreatedTables) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISimilarityFileIdTable_Impl::CreateTable(this, core::mem::transmute(&path), core::mem::transmute_copy(&truncate), core::mem::transmute_copy(&securitydescriptor), core::mem::transmute_copy(&recordsize)) {
                    Ok(ok__) => {
                        isnew.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTableIndirect<Identity: ISimilarityFileIdTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fileidfile: *mut core::ffi::c_void, truncate: windows_core::BOOL, recordsize: u32, isnew: *mut RdcCreatedTables) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISimilarityFileIdTable_Impl::CreateTableIndirect(this, core::mem::transmute_copy(&fileidfile), core::mem::transmute_copy(&truncate), core::mem::transmute_copy(&recordsize)) {
                    Ok(ok__) => {
                        isnew.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CloseTable<Identity: ISimilarityFileIdTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isvalid: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISimilarityFileIdTable_Impl::CloseTable(this, core::mem::transmute_copy(&isvalid)).into()
            }
        }
        unsafe extern "system" fn Append<Identity: ISimilarityFileIdTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, similarityfileid: *const SimilarityFileId, similarityfileindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISimilarityFileIdTable_Impl::Append(this, core::mem::transmute_copy(&similarityfileid)) {
                    Ok(ok__) => {
                        similarityfileindex.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Lookup<Identity: ISimilarityFileIdTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, similarityfileindex: u32, similarityfileid: *mut SimilarityFileId) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISimilarityFileIdTable_Impl::Lookup(this, core::mem::transmute_copy(&similarityfileindex), core::mem::transmute_copy(&similarityfileid)).into()
            }
        }
        unsafe extern "system" fn Invalidate<Identity: ISimilarityFileIdTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, similarityfileindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISimilarityFileIdTable_Impl::Invalidate(this, core::mem::transmute_copy(&similarityfileindex)).into()
            }
        }
        unsafe extern "system" fn GetRecordCount<Identity: ISimilarityFileIdTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, recordcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISimilarityFileIdTable_Impl::GetRecordCount(this) {
                    Ok(ok__) => {
                        recordcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTable: CreateTable::<Identity, OFFSET>,
            CreateTableIndirect: CreateTableIndirect::<Identity, OFFSET>,
            CloseTable: CloseTable::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
            Lookup: Lookup::<Identity, OFFSET>,
            Invalidate: Invalidate::<Identity, OFFSET>,
            GetRecordCount: GetRecordCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISimilarityFileIdTable as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISimilarityFileIdTable {}
windows_core::imp::define_interface!(ISimilarityReportProgress, ISimilarityReportProgress_Vtbl, 0x96236a7a_9dbc_11da_9e3f_0011114ae311);
windows_core::imp::interface_hierarchy!(ISimilarityReportProgress, windows_core::IUnknown);
impl ISimilarityReportProgress {
    pub unsafe fn ReportProgress(&self, percentcompleted: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReportProgress)(windows_core::Interface::as_raw(self), percentcompleted).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarityReportProgress_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReportProgress: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ISimilarityReportProgress_Impl: windows_core::IUnknownImpl {
    fn ReportProgress(&self, percentcompleted: u32) -> windows_core::Result<()>;
}
impl ISimilarityReportProgress_Vtbl {
    pub const fn new<Identity: ISimilarityReportProgress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReportProgress<Identity: ISimilarityReportProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, percentcompleted: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISimilarityReportProgress_Impl::ReportProgress(this, core::mem::transmute_copy(&percentcompleted)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ReportProgress: ReportProgress::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISimilarityReportProgress as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISimilarityReportProgress {}
windows_core::imp::define_interface!(ISimilarityTableDumpState, ISimilarityTableDumpState_Vtbl, 0x96236a7b_9dbc_11da_9e3f_0011114ae311);
windows_core::imp::interface_hierarchy!(ISimilarityTableDumpState, windows_core::IUnknown);
impl ISimilarityTableDumpState {
    pub unsafe fn GetNextData(&self, resultssize: u32, resultsused: *mut u32, eof: *mut windows_core::BOOL, results: *mut SimilarityDumpData) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetNextData)(windows_core::Interface::as_raw(self), resultssize, resultsused as _, eof as _, results as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarityTableDumpState_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNextData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut windows_core::BOOL, *mut SimilarityDumpData) -> windows_core::HRESULT,
}
pub trait ISimilarityTableDumpState_Impl: windows_core::IUnknownImpl {
    fn GetNextData(&self, resultssize: u32, resultsused: *mut u32, eof: *mut windows_core::BOOL, results: *mut SimilarityDumpData) -> windows_core::Result<()>;
}
impl ISimilarityTableDumpState_Vtbl {
    pub const fn new<Identity: ISimilarityTableDumpState_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNextData<Identity: ISimilarityTableDumpState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resultssize: u32, resultsused: *mut u32, eof: *mut windows_core::BOOL, results: *mut SimilarityDumpData) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISimilarityTableDumpState_Impl::GetNextData(this, core::mem::transmute_copy(&resultssize), core::mem::transmute_copy(&resultsused), core::mem::transmute_copy(&eof), core::mem::transmute_copy(&results)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetNextData: GetNextData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISimilarityTableDumpState as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISimilarityTableDumpState {}
windows_core::imp::define_interface!(ISimilarityTraitsMappedView, ISimilarityTraitsMappedView_Vtbl, 0x96236a7c_9dbc_11da_9e3f_0011114ae311);
windows_core::imp::interface_hierarchy!(ISimilarityTraitsMappedView, windows_core::IUnknown);
impl ISimilarityTraitsMappedView {
    pub unsafe fn Flush(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Unmap(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Get(&self, index: u64, dirty: bool, numelements: u32) -> windows_core::Result<SimilarityMappedViewInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), index, dirty.into(), numelements, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetView(&self, mappedpagebegin: *mut *mut u8, mappedpageend: *mut *mut u8) {
        unsafe { (windows_core::Interface::vtable(self).GetView)(windows_core::Interface::as_raw(self), mappedpagebegin as _, mappedpageend as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarityTraitsMappedView_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, u64, windows_core::BOOL, u32, *mut SimilarityMappedViewInfo) -> windows_core::HRESULT,
    pub GetView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut *mut u8),
}
pub trait ISimilarityTraitsMappedView_Impl: windows_core::IUnknownImpl {
    fn Flush(&self) -> windows_core::Result<()>;
    fn Unmap(&self) -> windows_core::Result<()>;
    fn Get(&self, index: u64, dirty: windows_core::BOOL, numelements: u32) -> windows_core::Result<SimilarityMappedViewInfo>;
    fn GetView(&self, mappedpagebegin: *mut *mut u8, mappedpageend: *mut *mut u8);
}
impl ISimilarityTraitsMappedView_Vtbl {
    pub const fn new<Identity: ISimilarityTraitsMappedView_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Flush<Identity: ISimilarityTraitsMappedView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISimilarityTraitsMappedView_Impl::Flush(this).into()
            }
        }
        unsafe extern "system" fn Unmap<Identity: ISimilarityTraitsMappedView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISimilarityTraitsMappedView_Impl::Unmap(this).into()
            }
        }
        unsafe extern "system" fn Get<Identity: ISimilarityTraitsMappedView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u64, dirty: windows_core::BOOL, numelements: u32, viewinfo: *mut SimilarityMappedViewInfo) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISimilarityTraitsMappedView_Impl::Get(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&dirty), core::mem::transmute_copy(&numelements)) {
                    Ok(ok__) => {
                        viewinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetView<Identity: ISimilarityTraitsMappedView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mappedpagebegin: *mut *mut u8, mappedpageend: *mut *mut u8) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISimilarityTraitsMappedView_Impl::GetView(this, core::mem::transmute_copy(&mappedpagebegin), core::mem::transmute_copy(&mappedpageend))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Flush: Flush::<Identity, OFFSET>,
            Unmap: Unmap::<Identity, OFFSET>,
            Get: Get::<Identity, OFFSET>,
            GetView: GetView::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISimilarityTraitsMappedView as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISimilarityTraitsMappedView {}
windows_core::imp::define_interface!(ISimilarityTraitsMapping, ISimilarityTraitsMapping_Vtbl, 0x96236a7d_9dbc_11da_9e3f_0011114ae311);
windows_core::imp::interface_hierarchy!(ISimilarityTraitsMapping, windows_core::IUnknown);
impl ISimilarityTraitsMapping {
    pub unsafe fn CloseMapping(&self) {
        unsafe { (windows_core::Interface::vtable(self).CloseMapping)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetFileSize(&self, filesize: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFileSize)(windows_core::Interface::as_raw(self), filesize).ok() }
    }
    pub unsafe fn GetFileSize(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn OpenMapping(&self, accessmode: RdcMappingAccessMode, begin: u64, end: u64) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenMapping)(windows_core::Interface::as_raw(self), accessmode, begin, end, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ResizeMapping(&self, accessmode: RdcMappingAccessMode, begin: u64, end: u64) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResizeMapping)(windows_core::Interface::as_raw(self), accessmode, begin, end, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPageSize(&self) -> u32 {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPageSize)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn CreateView(&self, minimummappedpages: u32, accessmode: RdcMappingAccessMode) -> windows_core::Result<ISimilarityTraitsMappedView> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateView)(windows_core::Interface::as_raw(self), minimummappedpages, accessmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarityTraitsMapping_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CloseMapping: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub SetFileSize: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub GetFileSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub OpenMapping: unsafe extern "system" fn(*mut core::ffi::c_void, RdcMappingAccessMode, u64, u64, *mut u64) -> windows_core::HRESULT,
    pub ResizeMapping: unsafe extern "system" fn(*mut core::ffi::c_void, RdcMappingAccessMode, u64, u64, *mut u64) -> windows_core::HRESULT,
    pub GetPageSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32),
    pub CreateView: unsafe extern "system" fn(*mut core::ffi::c_void, u32, RdcMappingAccessMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISimilarityTraitsMapping_Impl: windows_core::IUnknownImpl {
    fn CloseMapping(&self);
    fn SetFileSize(&self, filesize: u64) -> windows_core::Result<()>;
    fn GetFileSize(&self) -> windows_core::Result<u64>;
    fn OpenMapping(&self, accessmode: RdcMappingAccessMode, begin: u64, end: u64) -> windows_core::Result<u64>;
    fn ResizeMapping(&self, accessmode: RdcMappingAccessMode, begin: u64, end: u64) -> windows_core::Result<u64>;
    fn GetPageSize(&self, pagesize: *mut u32);
    fn CreateView(&self, minimummappedpages: u32, accessmode: RdcMappingAccessMode) -> windows_core::Result<ISimilarityTraitsMappedView>;
}
impl ISimilarityTraitsMapping_Vtbl {
    pub const fn new<Identity: ISimilarityTraitsMapping_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CloseMapping<Identity: ISimilarityTraitsMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISimilarityTraitsMapping_Impl::CloseMapping(this)
            }
        }
        unsafe extern "system" fn SetFileSize<Identity: ISimilarityTraitsMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesize: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISimilarityTraitsMapping_Impl::SetFileSize(this, core::mem::transmute_copy(&filesize)).into()
            }
        }
        unsafe extern "system" fn GetFileSize<Identity: ISimilarityTraitsMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesize: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISimilarityTraitsMapping_Impl::GetFileSize(this) {
                    Ok(ok__) => {
                        filesize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenMapping<Identity: ISimilarityTraitsMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, accessmode: RdcMappingAccessMode, begin: u64, end: u64, actualend: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISimilarityTraitsMapping_Impl::OpenMapping(this, core::mem::transmute_copy(&accessmode), core::mem::transmute_copy(&begin), core::mem::transmute_copy(&end)) {
                    Ok(ok__) => {
                        actualend.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResizeMapping<Identity: ISimilarityTraitsMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, accessmode: RdcMappingAccessMode, begin: u64, end: u64, actualend: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISimilarityTraitsMapping_Impl::ResizeMapping(this, core::mem::transmute_copy(&accessmode), core::mem::transmute_copy(&begin), core::mem::transmute_copy(&end)) {
                    Ok(ok__) => {
                        actualend.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPageSize<Identity: ISimilarityTraitsMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pagesize: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISimilarityTraitsMapping_Impl::GetPageSize(this, core::mem::transmute_copy(&pagesize))
            }
        }
        unsafe extern "system" fn CreateView<Identity: ISimilarityTraitsMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minimummappedpages: u32, accessmode: RdcMappingAccessMode, mappedview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISimilarityTraitsMapping_Impl::CreateView(this, core::mem::transmute_copy(&minimummappedpages), core::mem::transmute_copy(&accessmode)) {
                    Ok(ok__) => {
                        mappedview.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CloseMapping: CloseMapping::<Identity, OFFSET>,
            SetFileSize: SetFileSize::<Identity, OFFSET>,
            GetFileSize: GetFileSize::<Identity, OFFSET>,
            OpenMapping: OpenMapping::<Identity, OFFSET>,
            ResizeMapping: ResizeMapping::<Identity, OFFSET>,
            GetPageSize: GetPageSize::<Identity, OFFSET>,
            CreateView: CreateView::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISimilarityTraitsMapping as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISimilarityTraitsMapping {}
windows_core::imp::define_interface!(ISimilarityTraitsTable, ISimilarityTraitsTable_Vtbl, 0x96236a7e_9dbc_11da_9e3f_0011114ae311);
windows_core::imp::interface_hierarchy!(ISimilarityTraitsTable, windows_core::IUnknown);
impl ISimilarityTraitsTable {
    pub unsafe fn CreateTable<P0>(&self, path: P0, truncate: bool, securitydescriptor: *const u8) -> windows_core::Result<RdcCreatedTables>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTable)(windows_core::Interface::as_raw(self), path.param().abi(), truncate.into(), securitydescriptor, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateTableIndirect<P0>(&self, mapping: P0, truncate: bool) -> windows_core::Result<RdcCreatedTables>
    where
        P0: windows_core::Param<ISimilarityTraitsMapping>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTableIndirect)(windows_core::Interface::as_raw(self), mapping.param().abi(), truncate.into(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CloseTable(&self, isvalid: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CloseTable)(windows_core::Interface::as_raw(self), isvalid.into()).ok() }
    }
    pub unsafe fn Append(&self, data: *const SimilarityData, fileindex: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), data, fileindex).ok() }
    }
    pub unsafe fn FindSimilarFileIndex(&self, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, findsimilarfileindexresults: *mut FindSimilarFileIndexResults, resultssize: u32, resultsused: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FindSimilarFileIndex)(windows_core::Interface::as_raw(self), similaritydata, numberofmatchesrequired, findsimilarfileindexresults as _, resultssize, resultsused as _).ok() }
    }
    pub unsafe fn BeginDump(&self) -> windows_core::Result<ISimilarityTableDumpState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginDump)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetLastIndex(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarityTraitsTable_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateTable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::BOOL, *const u8, *mut RdcCreatedTables) -> windows_core::HRESULT,
    pub CreateTableIndirect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *mut RdcCreatedTables) -> windows_core::HRESULT,
    pub CloseTable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *const SimilarityData, u32) -> windows_core::HRESULT,
    pub FindSimilarFileIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *const SimilarityData, u16, *mut FindSimilarFileIndexResults, u32, *mut u32) -> windows_core::HRESULT,
    pub BeginDump: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLastIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait ISimilarityTraitsTable_Impl: windows_core::IUnknownImpl {
    fn CreateTable(&self, path: &windows_core::PCWSTR, truncate: windows_core::BOOL, securitydescriptor: *const u8) -> windows_core::Result<RdcCreatedTables>;
    fn CreateTableIndirect(&self, mapping: windows_core::Ref<ISimilarityTraitsMapping>, truncate: windows_core::BOOL) -> windows_core::Result<RdcCreatedTables>;
    fn CloseTable(&self, isvalid: windows_core::BOOL) -> windows_core::Result<()>;
    fn Append(&self, data: *const SimilarityData, fileindex: u32) -> windows_core::Result<()>;
    fn FindSimilarFileIndex(&self, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, findsimilarfileindexresults: *mut FindSimilarFileIndexResults, resultssize: u32, resultsused: *mut u32) -> windows_core::Result<()>;
    fn BeginDump(&self) -> windows_core::Result<ISimilarityTableDumpState>;
    fn GetLastIndex(&self) -> windows_core::Result<u32>;
}
impl ISimilarityTraitsTable_Vtbl {
    pub const fn new<Identity: ISimilarityTraitsTable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateTable<Identity: ISimilarityTraitsTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: windows_core::PCWSTR, truncate: windows_core::BOOL, securitydescriptor: *const u8, isnew: *mut RdcCreatedTables) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISimilarityTraitsTable_Impl::CreateTable(this, core::mem::transmute(&path), core::mem::transmute_copy(&truncate), core::mem::transmute_copy(&securitydescriptor)) {
                    Ok(ok__) => {
                        isnew.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTableIndirect<Identity: ISimilarityTraitsTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mapping: *mut core::ffi::c_void, truncate: windows_core::BOOL, isnew: *mut RdcCreatedTables) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISimilarityTraitsTable_Impl::CreateTableIndirect(this, core::mem::transmute_copy(&mapping), core::mem::transmute_copy(&truncate)) {
                    Ok(ok__) => {
                        isnew.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CloseTable<Identity: ISimilarityTraitsTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isvalid: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISimilarityTraitsTable_Impl::CloseTable(this, core::mem::transmute_copy(&isvalid)).into()
            }
        }
        unsafe extern "system" fn Append<Identity: ISimilarityTraitsTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *const SimilarityData, fileindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISimilarityTraitsTable_Impl::Append(this, core::mem::transmute_copy(&data), core::mem::transmute_copy(&fileindex)).into()
            }
        }
        unsafe extern "system" fn FindSimilarFileIndex<Identity: ISimilarityTraitsTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, findsimilarfileindexresults: *mut FindSimilarFileIndexResults, resultssize: u32, resultsused: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISimilarityTraitsTable_Impl::FindSimilarFileIndex(this, core::mem::transmute_copy(&similaritydata), core::mem::transmute_copy(&numberofmatchesrequired), core::mem::transmute_copy(&findsimilarfileindexresults), core::mem::transmute_copy(&resultssize), core::mem::transmute_copy(&resultsused)).into()
            }
        }
        unsafe extern "system" fn BeginDump<Identity: ISimilarityTraitsTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, similaritytabledumpstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISimilarityTraitsTable_Impl::BeginDump(this) {
                    Ok(ok__) => {
                        similaritytabledumpstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLastIndex<Identity: ISimilarityTraitsTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fileindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISimilarityTraitsTable_Impl::GetLastIndex(this) {
                    Ok(ok__) => {
                        fileindex.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTable: CreateTable::<Identity, OFFSET>,
            CreateTableIndirect: CreateTableIndirect::<Identity, OFFSET>,
            CloseTable: CloseTable::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
            FindSimilarFileIndex: FindSimilarFileIndex::<Identity, OFFSET>,
            BeginDump: BeginDump::<Identity, OFFSET>,
            GetLastIndex: GetLastIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISimilarityTraitsTable as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISimilarityTraitsTable {}
pub const MSRDC_DEFAULT_COMPAREBUFFER: u32 = 3200000u32;
pub const MSRDC_DEFAULT_HASHWINDOWSIZE_1: u32 = 48u32;
pub const MSRDC_DEFAULT_HASHWINDOWSIZE_N: u32 = 2u32;
pub const MSRDC_DEFAULT_HORIZONSIZE_1: u32 = 1024u32;
pub const MSRDC_DEFAULT_HORIZONSIZE_N: u32 = 128u32;
pub const MSRDC_MAXIMUM_COMPAREBUFFER: u32 = 1073741824u32;
pub const MSRDC_MAXIMUM_DEPTH: u32 = 8u32;
pub const MSRDC_MAXIMUM_HASHWINDOWSIZE: u32 = 96u32;
pub const MSRDC_MAXIMUM_HORIZONSIZE: u32 = 16384u32;
pub const MSRDC_MAXIMUM_MATCHESREQUIRED: u32 = 16u32;
pub const MSRDC_MAXIMUM_TRAITVALUE: u32 = 63u32;
pub const MSRDC_MINIMUM_COMPAREBUFFER: u32 = 100000u32;
pub const MSRDC_MINIMUM_COMPATIBLE_APP_VERSION: u32 = 65536u32;
pub const MSRDC_MINIMUM_DEPTH: u32 = 1u32;
pub const MSRDC_MINIMUM_HASHWINDOWSIZE: u32 = 2u32;
pub const MSRDC_MINIMUM_HORIZONSIZE: u32 = 128u32;
pub const MSRDC_MINIMUM_INPUTBUFFERSIZE: u32 = 1024u32;
pub const MSRDC_MINIMUM_MATCHESREQUIRED: u32 = 1u32;
pub const MSRDC_SIGNATURE_HASHSIZE: u32 = 16u32;
pub const MSRDC_VERSION: u32 = 65536u32;
pub const RDCE_TABLE_CORRUPT: u32 = 2147745794u32;
pub const RDCE_TABLE_FULL: u32 = 2147745793u32;
pub const RDCGENTYPE_FilterMax: GeneratorParametersType = GeneratorParametersType(1i32);
pub const RDCGENTYPE_Unused: GeneratorParametersType = GeneratorParametersType(0i32);
pub const RDCMAPPING_ReadOnly: RdcMappingAccessMode = RdcMappingAccessMode(1i32);
pub const RDCMAPPING_ReadWrite: RdcMappingAccessMode = RdcMappingAccessMode(2i32);
pub const RDCMAPPING_Undefined: RdcMappingAccessMode = RdcMappingAccessMode(0i32);
pub const RDCNEED_SEED: RdcNeedType = RdcNeedType(2i32);
pub const RDCNEED_SEED_MAX: RdcNeedType = RdcNeedType(255i32);
pub const RDCNEED_SOURCE: RdcNeedType = RdcNeedType(0i32);
pub const RDCNEED_TARGET: RdcNeedType = RdcNeedType(1i32);
pub const RDCTABLE_Existing: RdcCreatedTables = RdcCreatedTables(1i32);
pub const RDCTABLE_InvalidOrUnknown: RdcCreatedTables = RdcCreatedTables(0i32);
pub const RDCTABLE_New: RdcCreatedTables = RdcCreatedTables(2i32);
pub const RDC_Aborted: RDC_ErrorCode = RDC_ErrorCode(9i32);
pub const RDC_ApplicationError: RDC_ErrorCode = RDC_ErrorCode(8i32);
pub const RDC_DataMissingOrCorrupt: RDC_ErrorCode = RDC_ErrorCode(5i32);
pub const RDC_DataTooManyRecords: RDC_ErrorCode = RDC_ErrorCode(6i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RDC_ErrorCode(pub i32);
pub const RDC_FileChecksumMismatch: RDC_ErrorCode = RDC_ErrorCode(7i32);
pub const RDC_HeaderMissingOrCorrupt: RDC_ErrorCode = RDC_ErrorCode(3i32);
pub const RDC_HeaderVersionNewer: RDC_ErrorCode = RDC_ErrorCode(1i32);
pub const RDC_HeaderVersionOlder: RDC_ErrorCode = RDC_ErrorCode(2i32);
pub const RDC_HeaderWrongType: RDC_ErrorCode = RDC_ErrorCode(4i32);
pub const RDC_NoError: RDC_ErrorCode = RDC_ErrorCode(0i32);
pub const RDC_Win32Error: RDC_ErrorCode = RDC_ErrorCode(10i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RdcBufferPointer {
    pub m_Size: u32,
    pub m_Used: u32,
    pub m_Data: *mut u8,
}
impl Default for RdcBufferPointer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RdcComparator: windows_core::GUID = windows_core::GUID::from_u128(0x96236a8b_9dbc_11da_9e3f_0011114ae311);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RdcCreatedTables(pub i32);
pub const RdcFileReader: windows_core::GUID = windows_core::GUID::from_u128(0x96236a89_9dbc_11da_9e3f_0011114ae311);
pub const RdcGenerator: windows_core::GUID = windows_core::GUID::from_u128(0x96236a88_9dbc_11da_9e3f_0011114ae311);
pub const RdcGeneratorFilterMaxParameters: windows_core::GUID = windows_core::GUID::from_u128(0x96236a87_9dbc_11da_9e3f_0011114ae311);
pub const RdcGeneratorParameters: windows_core::GUID = windows_core::GUID::from_u128(0x96236a86_9dbc_11da_9e3f_0011114ae311);
pub const RdcLibrary: windows_core::GUID = windows_core::GUID::from_u128(0x96236a85_9dbc_11da_9e3f_0011114ae311);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RdcMappingAccessMode(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RdcNeed {
    pub m_BlockType: RdcNeedType,
    pub m_FileOffset: u64,
    pub m_BlockLength: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RdcNeedPointer {
    pub m_Size: u32,
    pub m_Used: u32,
    pub m_Data: *mut RdcNeed,
}
impl Default for RdcNeedPointer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RdcNeedType(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RdcSignature {
    pub m_Signature: [u8; 16],
    pub m_BlockLength: u16,
}
impl Default for RdcSignature {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RdcSignaturePointer {
    pub m_Size: u32,
    pub m_Used: u32,
    pub m_Data: *mut RdcSignature,
}
impl Default for RdcSignaturePointer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RdcSignatureReader: windows_core::GUID = windows_core::GUID::from_u128(0x96236a8a_9dbc_11da_9e3f_0011114ae311);
pub const RdcSimilarityGenerator: windows_core::GUID = windows_core::GUID::from_u128(0x96236a92_9dbc_11da_9e3f_0011114ae311);
pub const Similarity: windows_core::GUID = windows_core::GUID::from_u128(0x96236a91_9dbc_11da_9e3f_0011114ae311);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimilarityData {
    pub m_Data: [u8; 16],
}
impl Default for SimilarityData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SimilarityDumpData {
    pub m_FileIndex: u32,
    pub m_Data: SimilarityData,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimilarityFileId {
    pub m_FileId: [u8; 32],
}
impl Default for SimilarityFileId {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SimilarityFileIdMaxSize: u32 = 32u32;
pub const SimilarityFileIdMinSize: u32 = 4u32;
pub const SimilarityFileIdTable: windows_core::GUID = windows_core::GUID::from_u128(0x96236a90_9dbc_11da_9e3f_0011114ae311);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimilarityMappedViewInfo {
    pub m_Data: *mut u8,
    pub m_Length: u32,
}
impl Default for SimilarityMappedViewInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SimilarityReportProgress: windows_core::GUID = windows_core::GUID::from_u128(0x96236a8d_9dbc_11da_9e3f_0011114ae311);
pub const SimilarityTableDumpState: windows_core::GUID = windows_core::GUID::from_u128(0x96236a8e_9dbc_11da_9e3f_0011114ae311);
pub const SimilarityTraitsMappedView: windows_core::GUID = windows_core::GUID::from_u128(0x96236a95_9dbc_11da_9e3f_0011114ae311);
pub const SimilarityTraitsMapping: windows_core::GUID = windows_core::GUID::from_u128(0x96236a94_9dbc_11da_9e3f_0011114ae311);
pub const SimilarityTraitsTable: windows_core::GUID = windows_core::GUID::from_u128(0x96236a8f_9dbc_11da_9e3f_0011114ae311);
