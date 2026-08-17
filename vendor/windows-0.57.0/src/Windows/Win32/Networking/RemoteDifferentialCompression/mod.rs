windows_core::imp::define_interface!(IFindSimilarResults, IFindSimilarResults_Vtbl, 0x96236a81_9dbc_11da_9e3f_0011114ae311);
impl core::ops::Deref for IFindSimilarResults {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFindSimilarResults, windows_core::IUnknown);
impl IFindSimilarResults {
    pub unsafe fn GetSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetNextFileId(&self, numtraitsmatched: *mut u32, similarityfileid: *mut SimilarityFileId) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNextFileId)(windows_core::Interface::as_raw(self), numtraitsmatched, similarityfileid).ok()
    }
}
#[repr(C)]
pub struct IFindSimilarResults_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetNextFileId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut SimilarityFileId) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRdcComparator, IRdcComparator_Vtbl, 0x96236a77_9dbc_11da_9e3f_0011114ae311);
impl core::ops::Deref for IRdcComparator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRdcComparator, windows_core::IUnknown);
impl IRdcComparator {
    pub unsafe fn Process<P0>(&self, endofinput: P0, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, outputbuffer: *mut RdcNeedPointer, rdc_errorcode: *mut RDC_ErrorCode) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Process)(windows_core::Interface::as_raw(self), endofinput.param().abi(), endofoutput, inputbuffer, outputbuffer, rdc_errorcode).ok()
    }
}
#[repr(C)]
pub struct IRdcComparator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Process: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, *mut super::super::Foundation::BOOL, *mut RdcBufferPointer, *mut RdcNeedPointer, *mut RDC_ErrorCode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRdcFileReader, IRdcFileReader_Vtbl, 0x96236a74_9dbc_11da_9e3f_0011114ae311);
impl core::ops::Deref for IRdcFileReader {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRdcFileReader, windows_core::IUnknown);
impl IRdcFileReader {
    pub unsafe fn GetFileSize(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFileSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Read(&self, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), offsetfilestart, bytestoread, bytesactuallyread, buffer, eof).ok()
    }
    pub unsafe fn GetFilePosition(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFilePosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IRdcFileReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFileSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u32, *mut u32, *mut u8, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetFilePosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), offsetfilestart, bytestowrite, &mut result__).map(|| result__)
    }
    pub unsafe fn Truncate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Truncate)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DeleteOnClose(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteOnClose)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IRdcFileWriter_Vtbl {
    pub base__: IRdcFileReader_Vtbl,
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u32, *mut u8) -> windows_core::HRESULT,
    pub Truncate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteOnClose: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRdcGenerator, IRdcGenerator_Vtbl, 0x96236a73_9dbc_11da_9e3f_0011114ae311);
impl core::ops::Deref for IRdcGenerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRdcGenerator, windows_core::IUnknown);
impl IRdcGenerator {
    pub unsafe fn GetGeneratorParameters(&self, level: u32) -> windows_core::Result<IRdcGeneratorParameters> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGeneratorParameters)(windows_core::Interface::as_raw(self), level, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Process<P0>(&self, endofinput: P0, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, outputbuffers: &mut [*mut RdcBufferPointer], rdc_errorcode: *mut RDC_ErrorCode) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Process)(windows_core::Interface::as_raw(self), endofinput.param().abi(), endofoutput, inputbuffer, outputbuffers.len().try_into().unwrap(), core::mem::transmute(outputbuffers.as_ptr()), rdc_errorcode).ok()
    }
}
#[repr(C)]
pub struct IRdcGenerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetGeneratorParameters: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Process: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, *mut super::super::Foundation::BOOL, *mut RdcBufferPointer, u32, *mut *mut RdcBufferPointer, *mut RDC_ErrorCode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRdcGeneratorFilterMaxParameters, IRdcGeneratorFilterMaxParameters_Vtbl, 0x96236a72_9dbc_11da_9e3f_0011114ae311);
impl core::ops::Deref for IRdcGeneratorFilterMaxParameters {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRdcGeneratorFilterMaxParameters, windows_core::IUnknown);
impl IRdcGeneratorFilterMaxParameters {
    pub unsafe fn GetHorizonSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHorizonSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetHorizonSize(&self, horizonsize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetHorizonSize)(windows_core::Interface::as_raw(self), horizonsize).ok()
    }
    pub unsafe fn GetHashWindowSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHashWindowSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetHashWindowSize(&self, hashwindowsize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetHashWindowSize)(windows_core::Interface::as_raw(self), hashwindowsize).ok()
    }
}
#[repr(C)]
pub struct IRdcGeneratorFilterMaxParameters_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetHorizonSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetHorizonSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetHashWindowSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetHashWindowSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRdcGeneratorParameters, IRdcGeneratorParameters_Vtbl, 0x96236a71_9dbc_11da_9e3f_0011114ae311);
impl core::ops::Deref for IRdcGeneratorParameters {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRdcGeneratorParameters, windows_core::IUnknown);
impl IRdcGeneratorParameters {
    pub unsafe fn GetGeneratorParametersType(&self) -> windows_core::Result<GeneratorParametersType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGeneratorParametersType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetParametersVersion(&self, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetParametersVersion)(windows_core::Interface::as_raw(self), currentversion, minimumcompatibleappversion).ok()
    }
    pub unsafe fn GetSerializeSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSerializeSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Serialize(&self, size: u32, parametersblob: *mut u8, byteswritten: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Serialize)(windows_core::Interface::as_raw(self), size, parametersblob, byteswritten).ok()
    }
}
#[repr(C)]
pub struct IRdcGeneratorParameters_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetGeneratorParametersType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GeneratorParametersType) -> windows_core::HRESULT,
    pub GetParametersVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetSerializeSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Serialize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRdcLibrary, IRdcLibrary_Vtbl, 0x96236a78_9dbc_11da_9e3f_0011114ae311);
impl core::ops::Deref for IRdcLibrary {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRdcLibrary, windows_core::IUnknown);
impl IRdcLibrary {
    pub unsafe fn ComputeDefaultRecursionDepth(&self, filesize: u64) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ComputeDefaultRecursionDepth)(windows_core::Interface::as_raw(self), filesize, &mut result__).map(|| result__)
    }
    pub unsafe fn CreateGeneratorParameters(&self, parameterstype: GeneratorParametersType, level: u32) -> windows_core::Result<IRdcGeneratorParameters> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateGeneratorParameters)(windows_core::Interface::as_raw(self), parameterstype, level, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn OpenGeneratorParameters(&self, size: u32, parametersblob: *const u8) -> windows_core::Result<IRdcGeneratorParameters> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenGeneratorParameters)(windows_core::Interface::as_raw(self), size, parametersblob, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateGenerator(&self, igeneratorparametersarray: &[Option<IRdcGeneratorParameters>]) -> windows_core::Result<IRdcGenerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateGenerator)(windows_core::Interface::as_raw(self), igeneratorparametersarray.len().try_into().unwrap(), core::mem::transmute(igeneratorparametersarray.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateComparator<P0>(&self, iseedsignaturesfile: P0, comparatorbuffersize: u32) -> windows_core::Result<IRdcComparator>
    where
        P0: windows_core::Param<IRdcFileReader>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateComparator)(windows_core::Interface::as_raw(self), iseedsignaturesfile.param().abi(), comparatorbuffersize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateSignatureReader<P0>(&self, ifilereader: P0) -> windows_core::Result<IRdcSignatureReader>
    where
        P0: windows_core::Param<IRdcFileReader>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSignatureReader)(windows_core::Interface::as_raw(self), ifilereader.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetRDCVersion(&self, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRDCVersion)(windows_core::Interface::as_raw(self), currentversion, minimumcompatibleappversion).ok()
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IRdcSignatureReader, IRdcSignatureReader_Vtbl, 0x96236a76_9dbc_11da_9e3f_0011114ae311);
impl core::ops::Deref for IRdcSignatureReader {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRdcSignatureReader, windows_core::IUnknown);
impl IRdcSignatureReader {
    pub unsafe fn ReadHeader(&self) -> windows_core::Result<RDC_ErrorCode> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReadHeader)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ReadSignatures(&self, rdcsignaturepointer: *mut RdcSignaturePointer, endofoutput: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReadSignatures)(windows_core::Interface::as_raw(self), rdcsignaturepointer, endofoutput).ok()
    }
}
#[repr(C)]
pub struct IRdcSignatureReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReadHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RDC_ErrorCode) -> windows_core::HRESULT,
    pub ReadSignatures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RdcSignaturePointer, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRdcSimilarityGenerator, IRdcSimilarityGenerator_Vtbl, 0x96236a80_9dbc_11da_9e3f_0011114ae311);
impl core::ops::Deref for IRdcSimilarityGenerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRdcSimilarityGenerator, windows_core::IUnknown);
impl IRdcSimilarityGenerator {
    pub unsafe fn EnableSimilarity(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnableSimilarity)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Results(&self) -> windows_core::Result<SimilarityData> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Results)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IRdcSimilarityGenerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnableSimilarity: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Results: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SimilarityData) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISimilarity, ISimilarity_Vtbl, 0x96236a83_9dbc_11da_9e3f_0011114ae311);
impl core::ops::Deref for ISimilarity {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISimilarity, windows_core::IUnknown);
impl ISimilarity {
    pub unsafe fn CreateTable<P0, P1>(&self, path: P0, truncate: P1, securitydescriptor: *const u8, recordsize: u32) -> windows_core::Result<RdcCreatedTables>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTable)(windows_core::Interface::as_raw(self), path.param().abi(), truncate.param().abi(), securitydescriptor, recordsize, &mut result__).map(|| result__)
    }
    pub unsafe fn CreateTableIndirect<P0, P1, P2>(&self, mapping: P0, fileidfile: P1, truncate: P2, recordsize: u32) -> windows_core::Result<RdcCreatedTables>
    where
        P0: windows_core::Param<ISimilarityTraitsMapping>,
        P1: windows_core::Param<IRdcFileWriter>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTableIndirect)(windows_core::Interface::as_raw(self), mapping.param().abi(), fileidfile.param().abi(), truncate.param().abi(), recordsize, &mut result__).map(|| result__)
    }
    pub unsafe fn CloseTable<P0>(&self, isvalid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).CloseTable)(windows_core::Interface::as_raw(self), isvalid.param().abi()).ok()
    }
    pub unsafe fn Append(&self, similarityfileid: *const SimilarityFileId, similaritydata: *const SimilarityData) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), similarityfileid, similaritydata).ok()
    }
    pub unsafe fn FindSimilarFileId(&self, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, resultssize: u32) -> windows_core::Result<IFindSimilarResults> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindSimilarFileId)(windows_core::Interface::as_raw(self), similaritydata, numberofmatchesrequired, resultssize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CopyAndSwap<P0, P1>(&self, newsimilaritytables: P0, reportprogress: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISimilarity>,
        P1: windows_core::Param<ISimilarityReportProgress>,
    {
        (windows_core::Interface::vtable(self).CopyAndSwap)(windows_core::Interface::as_raw(self), newsimilaritytables.param().abi(), reportprogress.param().abi()).ok()
    }
    pub unsafe fn GetRecordCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRecordCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISimilarity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateTable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::super::Foundation::BOOL, *const u8, u32, *mut RdcCreatedTables) -> windows_core::HRESULT,
    pub CreateTableIndirect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, u32, *mut RdcCreatedTables) -> windows_core::HRESULT,
    pub CloseTable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *const SimilarityFileId, *const SimilarityData) -> windows_core::HRESULT,
    pub FindSimilarFileId: unsafe extern "system" fn(*mut core::ffi::c_void, *const SimilarityData, u16, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyAndSwap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRecordCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISimilarityFileIdTable, ISimilarityFileIdTable_Vtbl, 0x96236a7f_9dbc_11da_9e3f_0011114ae311);
impl core::ops::Deref for ISimilarityFileIdTable {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISimilarityFileIdTable, windows_core::IUnknown);
impl ISimilarityFileIdTable {
    pub unsafe fn CreateTable<P0, P1>(&self, path: P0, truncate: P1, securitydescriptor: *const u8, recordsize: u32) -> windows_core::Result<RdcCreatedTables>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTable)(windows_core::Interface::as_raw(self), path.param().abi(), truncate.param().abi(), securitydescriptor, recordsize, &mut result__).map(|| result__)
    }
    pub unsafe fn CreateTableIndirect<P0, P1>(&self, fileidfile: P0, truncate: P1, recordsize: u32) -> windows_core::Result<RdcCreatedTables>
    where
        P0: windows_core::Param<IRdcFileWriter>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTableIndirect)(windows_core::Interface::as_raw(self), fileidfile.param().abi(), truncate.param().abi(), recordsize, &mut result__).map(|| result__)
    }
    pub unsafe fn CloseTable<P0>(&self, isvalid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).CloseTable)(windows_core::Interface::as_raw(self), isvalid.param().abi()).ok()
    }
    pub unsafe fn Append(&self, similarityfileid: *const SimilarityFileId) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), similarityfileid, &mut result__).map(|| result__)
    }
    pub unsafe fn Lookup(&self, similarityfileindex: u32, similarityfileid: *mut SimilarityFileId) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Lookup)(windows_core::Interface::as_raw(self), similarityfileindex, similarityfileid).ok()
    }
    pub unsafe fn Invalidate(&self, similarityfileindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Invalidate)(windows_core::Interface::as_raw(self), similarityfileindex).ok()
    }
    pub unsafe fn GetRecordCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRecordCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISimilarityFileIdTable_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateTable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::super::Foundation::BOOL, *const u8, u32, *mut RdcCreatedTables) -> windows_core::HRESULT,
    pub CreateTableIndirect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, u32, *mut RdcCreatedTables) -> windows_core::HRESULT,
    pub CloseTable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *const SimilarityFileId, *mut u32) -> windows_core::HRESULT,
    pub Lookup: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut SimilarityFileId) -> windows_core::HRESULT,
    pub Invalidate: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetRecordCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISimilarityReportProgress, ISimilarityReportProgress_Vtbl, 0x96236a7a_9dbc_11da_9e3f_0011114ae311);
impl core::ops::Deref for ISimilarityReportProgress {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISimilarityReportProgress, windows_core::IUnknown);
impl ISimilarityReportProgress {
    pub unsafe fn ReportProgress(&self, percentcompleted: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReportProgress)(windows_core::Interface::as_raw(self), percentcompleted).ok()
    }
}
#[repr(C)]
pub struct ISimilarityReportProgress_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReportProgress: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISimilarityTableDumpState, ISimilarityTableDumpState_Vtbl, 0x96236a7b_9dbc_11da_9e3f_0011114ae311);
impl core::ops::Deref for ISimilarityTableDumpState {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISimilarityTableDumpState, windows_core::IUnknown);
impl ISimilarityTableDumpState {
    pub unsafe fn GetNextData(&self, resultssize: u32, resultsused: *mut u32, eof: *mut super::super::Foundation::BOOL, results: *mut SimilarityDumpData) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNextData)(windows_core::Interface::as_raw(self), resultssize, resultsused, eof, results).ok()
    }
}
#[repr(C)]
pub struct ISimilarityTableDumpState_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNextData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut super::super::Foundation::BOOL, *mut SimilarityDumpData) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISimilarityTraitsMappedView, ISimilarityTraitsMappedView_Vtbl, 0x96236a7c_9dbc_11da_9e3f_0011114ae311);
impl core::ops::Deref for ISimilarityTraitsMappedView {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISimilarityTraitsMappedView, windows_core::IUnknown);
impl ISimilarityTraitsMappedView {
    pub unsafe fn Flush(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Unmap(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Get<P0>(&self, index: u64, dirty: P0, numelements: u32) -> windows_core::Result<SimilarityMappedViewInfo>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), index, dirty.param().abi(), numelements, &mut result__).map(|| result__)
    }
    pub unsafe fn GetView(&self, mappedpagebegin: *mut *mut u8, mappedpageend: *mut *mut u8) {
        (windows_core::Interface::vtable(self).GetView)(windows_core::Interface::as_raw(self), mappedpagebegin, mappedpageend)
    }
}
#[repr(C)]
pub struct ISimilarityTraitsMappedView_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, u64, super::super::Foundation::BOOL, u32, *mut SimilarityMappedViewInfo) -> windows_core::HRESULT,
    pub GetView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut *mut u8),
}
windows_core::imp::define_interface!(ISimilarityTraitsMapping, ISimilarityTraitsMapping_Vtbl, 0x96236a7d_9dbc_11da_9e3f_0011114ae311);
impl core::ops::Deref for ISimilarityTraitsMapping {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISimilarityTraitsMapping, windows_core::IUnknown);
impl ISimilarityTraitsMapping {
    pub unsafe fn CloseMapping(&self) {
        (windows_core::Interface::vtable(self).CloseMapping)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetFileSize(&self, filesize: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFileSize)(windows_core::Interface::as_raw(self), filesize).ok()
    }
    pub unsafe fn GetFileSize(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFileSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn OpenMapping(&self, accessmode: RdcMappingAccessMode, begin: u64, end: u64) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenMapping)(windows_core::Interface::as_raw(self), accessmode, begin, end, &mut result__).map(|| result__)
    }
    pub unsafe fn ResizeMapping(&self, accessmode: RdcMappingAccessMode, begin: u64, end: u64) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResizeMapping)(windows_core::Interface::as_raw(self), accessmode, begin, end, &mut result__).map(|| result__)
    }
    pub unsafe fn GetPageSize(&self) -> u32 {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPageSize)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn CreateView(&self, minimummappedpages: u32, accessmode: RdcMappingAccessMode) -> windows_core::Result<ISimilarityTraitsMappedView> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateView)(windows_core::Interface::as_raw(self), minimummappedpages, accessmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(ISimilarityTraitsTable, ISimilarityTraitsTable_Vtbl, 0x96236a7e_9dbc_11da_9e3f_0011114ae311);
impl core::ops::Deref for ISimilarityTraitsTable {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISimilarityTraitsTable, windows_core::IUnknown);
impl ISimilarityTraitsTable {
    pub unsafe fn CreateTable<P0, P1>(&self, path: P0, truncate: P1, securitydescriptor: *const u8) -> windows_core::Result<RdcCreatedTables>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTable)(windows_core::Interface::as_raw(self), path.param().abi(), truncate.param().abi(), securitydescriptor, &mut result__).map(|| result__)
    }
    pub unsafe fn CreateTableIndirect<P0, P1>(&self, mapping: P0, truncate: P1) -> windows_core::Result<RdcCreatedTables>
    where
        P0: windows_core::Param<ISimilarityTraitsMapping>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTableIndirect)(windows_core::Interface::as_raw(self), mapping.param().abi(), truncate.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn CloseTable<P0>(&self, isvalid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).CloseTable)(windows_core::Interface::as_raw(self), isvalid.param().abi()).ok()
    }
    pub unsafe fn Append(&self, data: *const SimilarityData, fileindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), data, fileindex).ok()
    }
    pub unsafe fn FindSimilarFileIndex(&self, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, findsimilarfileindexresults: *mut FindSimilarFileIndexResults, resultssize: u32, resultsused: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FindSimilarFileIndex)(windows_core::Interface::as_raw(self), similaritydata, numberofmatchesrequired, findsimilarfileindexresults, resultssize, resultsused).ok()
    }
    pub unsafe fn BeginDump(&self) -> windows_core::Result<ISimilarityTableDumpState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BeginDump)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLastIndex(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLastIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISimilarityTraitsTable_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateTable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::super::Foundation::BOOL, *const u8, *mut RdcCreatedTables) -> windows_core::HRESULT,
    pub CreateTableIndirect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, *mut RdcCreatedTables) -> windows_core::HRESULT,
    pub CloseTable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *const SimilarityData, u32) -> windows_core::HRESULT,
    pub FindSimilarFileIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *const SimilarityData, u16, *mut FindSimilarFileIndexResults, u32, *mut u32) -> windows_core::HRESULT,
    pub BeginDump: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLastIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
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
pub const RDC_FileChecksumMismatch: RDC_ErrorCode = RDC_ErrorCode(7i32);
pub const RDC_HeaderMissingOrCorrupt: RDC_ErrorCode = RDC_ErrorCode(3i32);
pub const RDC_HeaderVersionNewer: RDC_ErrorCode = RDC_ErrorCode(1i32);
pub const RDC_HeaderVersionOlder: RDC_ErrorCode = RDC_ErrorCode(2i32);
pub const RDC_HeaderWrongType: RDC_ErrorCode = RDC_ErrorCode(4i32);
pub const RDC_NoError: RDC_ErrorCode = RDC_ErrorCode(0i32);
pub const RDC_Win32Error: RDC_ErrorCode = RDC_ErrorCode(10i32);
pub const SimilarityFileIdMaxSize: u32 = 32u32;
pub const SimilarityFileIdMinSize: u32 = 4u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GeneratorParametersType(pub i32);
impl windows_core::TypeKind for GeneratorParametersType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GeneratorParametersType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GeneratorParametersType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RDC_ErrorCode(pub i32);
impl windows_core::TypeKind for RDC_ErrorCode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RDC_ErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RDC_ErrorCode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RdcCreatedTables(pub i32);
impl windows_core::TypeKind for RdcCreatedTables {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RdcCreatedTables {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RdcCreatedTables").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RdcMappingAccessMode(pub i32);
impl windows_core::TypeKind for RdcMappingAccessMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RdcMappingAccessMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RdcMappingAccessMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RdcNeedType(pub i32);
impl windows_core::TypeKind for RdcNeedType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RdcNeedType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RdcNeedType").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FindSimilarFileIndexResults {
    pub m_FileIndex: u32,
    pub m_MatchCount: u32,
}
impl windows_core::TypeKind for FindSimilarFileIndexResults {
    type TypeKind = windows_core::CopyType;
}
impl Default for FindSimilarFileIndexResults {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FindSimilarResults: windows_core::GUID = windows_core::GUID::from_u128(0x96236a93_9dbc_11da_9e3f_0011114ae311);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RdcBufferPointer {
    pub m_Size: u32,
    pub m_Used: u32,
    pub m_Data: *mut u8,
}
impl windows_core::TypeKind for RdcBufferPointer {
    type TypeKind = windows_core::CopyType;
}
impl Default for RdcBufferPointer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RdcComparator: windows_core::GUID = windows_core::GUID::from_u128(0x96236a8b_9dbc_11da_9e3f_0011114ae311);
pub const RdcFileReader: windows_core::GUID = windows_core::GUID::from_u128(0x96236a89_9dbc_11da_9e3f_0011114ae311);
pub const RdcGenerator: windows_core::GUID = windows_core::GUID::from_u128(0x96236a88_9dbc_11da_9e3f_0011114ae311);
pub const RdcGeneratorFilterMaxParameters: windows_core::GUID = windows_core::GUID::from_u128(0x96236a87_9dbc_11da_9e3f_0011114ae311);
pub const RdcGeneratorParameters: windows_core::GUID = windows_core::GUID::from_u128(0x96236a86_9dbc_11da_9e3f_0011114ae311);
pub const RdcLibrary: windows_core::GUID = windows_core::GUID::from_u128(0x96236a85_9dbc_11da_9e3f_0011114ae311);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RdcNeed {
    pub m_BlockType: RdcNeedType,
    pub m_FileOffset: u64,
    pub m_BlockLength: u64,
}
impl windows_core::TypeKind for RdcNeed {
    type TypeKind = windows_core::CopyType;
}
impl Default for RdcNeed {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RdcNeedPointer {
    pub m_Size: u32,
    pub m_Used: u32,
    pub m_Data: *mut RdcNeed,
}
impl windows_core::TypeKind for RdcNeedPointer {
    type TypeKind = windows_core::CopyType;
}
impl Default for RdcNeedPointer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RdcSignature {
    pub m_Signature: [u8; 16],
    pub m_BlockLength: u16,
}
impl windows_core::TypeKind for RdcSignature {
    type TypeKind = windows_core::CopyType;
}
impl Default for RdcSignature {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RdcSignaturePointer {
    pub m_Size: u32,
    pub m_Used: u32,
    pub m_Data: *mut RdcSignature,
}
impl windows_core::TypeKind for RdcSignaturePointer {
    type TypeKind = windows_core::CopyType;
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SimilarityData {
    pub m_Data: [u8; 16],
}
impl windows_core::TypeKind for SimilarityData {
    type TypeKind = windows_core::CopyType;
}
impl Default for SimilarityData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SimilarityDumpData {
    pub m_FileIndex: u32,
    pub m_Data: SimilarityData,
}
impl windows_core::TypeKind for SimilarityDumpData {
    type TypeKind = windows_core::CopyType;
}
impl Default for SimilarityDumpData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SimilarityFileId {
    pub m_FileId: [u8; 32],
}
impl windows_core::TypeKind for SimilarityFileId {
    type TypeKind = windows_core::CopyType;
}
impl Default for SimilarityFileId {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SimilarityFileIdTable: windows_core::GUID = windows_core::GUID::from_u128(0x96236a90_9dbc_11da_9e3f_0011114ae311);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SimilarityMappedViewInfo {
    pub m_Data: *mut u8,
    pub m_Length: u32,
}
impl windows_core::TypeKind for SimilarityMappedViewInfo {
    type TypeKind = windows_core::CopyType;
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
#[cfg(feature = "implement")]
core::include!("impl.rs");
