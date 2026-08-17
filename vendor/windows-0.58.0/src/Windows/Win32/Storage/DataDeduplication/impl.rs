pub trait IDedupBackupSupport_Impl: Sized {
    fn RestoreFiles(&self, numberoffiles: u32, filefullpaths: *const windows_core::BSTR, store: Option<&IDedupReadFileCallback>, flags: u32, fileresults: *mut windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDedupBackupSupport {}
impl IDedupBackupSupport_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDedupBackupSupport_Vtbl
    where
        Identity: IDedupBackupSupport_Impl,
    {
        unsafe extern "system" fn RestoreFiles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, numberoffiles: u32, filefullpaths: *const core::mem::MaybeUninit<windows_core::BSTR>, store: *mut core::ffi::c_void, flags: u32, fileresults: *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IDedupBackupSupport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDedupBackupSupport_Impl::RestoreFiles(this, core::mem::transmute_copy(&numberoffiles), core::mem::transmute_copy(&filefullpaths), windows_core::from_raw_borrowed(&store), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&fileresults)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RestoreFiles: RestoreFiles::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDedupBackupSupport as windows_core::Interface>::IID
    }
}
pub trait IDedupChunkLibrary_Impl: Sized {
    fn InitializeForPushBuffers(&self) -> windows_core::Result<()>;
    fn Uninitialize(&self) -> windows_core::Result<()>;
    fn SetParameter(&self, dwparamtype: u32, vparamvalue: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn StartChunking(&self, iiditeratorinterfaceid: &windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for IDedupChunkLibrary {}
impl IDedupChunkLibrary_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDedupChunkLibrary_Vtbl
    where
        Identity: IDedupChunkLibrary_Impl,
    {
        unsafe extern "system" fn InitializeForPushBuffers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDedupChunkLibrary_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDedupChunkLibrary_Impl::InitializeForPushBuffers(this).into()
        }
        unsafe extern "system" fn Uninitialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDedupChunkLibrary_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDedupChunkLibrary_Impl::Uninitialize(this).into()
        }
        unsafe extern "system" fn SetParameter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwparamtype: u32, vparamvalue: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IDedupChunkLibrary_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDedupChunkLibrary_Impl::SetParameter(this, core::mem::transmute_copy(&dwparamtype), core::mem::transmute(&vparamvalue)).into()
        }
        unsafe extern "system" fn StartChunking<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iiditeratorinterfaceid: windows_core::GUID, ppchunksenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDedupChunkLibrary_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDedupChunkLibrary_Impl::StartChunking(this, core::mem::transmute(&iiditeratorinterfaceid)) {
                Ok(ok__) => {
                    ppchunksenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializeForPushBuffers: InitializeForPushBuffers::<Identity, OFFSET>,
            Uninitialize: Uninitialize::<Identity, OFFSET>,
            SetParameter: SetParameter::<Identity, OFFSET>,
            StartChunking: StartChunking::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDedupChunkLibrary as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDedupDataPort_Impl: Sized {
    fn GetStatus(&self, pstatus: *mut DedupDataPortVolumeStatus, pdataheadroommb: *mut u32) -> windows_core::Result<()>;
    fn LookupChunks(&self, count: u32, phashes: *const DedupHash) -> windows_core::Result<windows_core::GUID>;
    fn InsertChunks(&self, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdata: *const u8) -> windows_core::Result<windows_core::GUID>;
    fn InsertChunksWithStream(&self, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdatastream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<windows_core::GUID>;
    fn CommitStreams(&self, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentries: *const DedupStreamEntry) -> windows_core::Result<windows_core::GUID>;
    fn CommitStreamsWithStream(&self, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentriesstream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<windows_core::GUID>;
    fn GetStreams(&self, streamcount: u32, pstreampaths: *const windows_core::BSTR) -> windows_core::Result<windows_core::GUID>;
    fn GetStreamsResults(&self, requestid: &windows_core::GUID, maxwaitms: u32, streamentryindex: u32, pstreamcount: *mut u32, ppstreams: *mut *mut DedupStream, pentrycount: *mut u32, ppentries: *mut *mut DedupStreamEntry, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetChunks(&self, count: u32, phashes: *const DedupHash) -> windows_core::Result<windows_core::GUID>;
    fn GetChunksResults(&self, requestid: &windows_core::GUID, maxwaitms: u32, chunkindex: u32, pchunkcount: *mut u32, ppchunkmetadata: *mut *mut DedupChunk, pdatabytecount: *mut u32, ppchunkdata: *mut *mut u8, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetRequestStatus(&self, requestid: &windows_core::GUID) -> windows_core::Result<DedupDataPortRequestStatus>;
    fn GetRequestResults(&self, requestid: &windows_core::GUID, maxwaitms: u32, pbatchresult: *mut windows_core::HRESULT, pbatchcount: *mut u32, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut windows_core::HRESULT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDedupDataPort {}
#[cfg(feature = "Win32_System_Com")]
impl IDedupDataPort_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDedupDataPort_Vtbl
    where
        Identity: IDedupDataPort_Impl,
    {
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut DedupDataPortVolumeStatus, pdataheadroommb: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDedupDataPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDedupDataPort_Impl::GetStatus(this, core::mem::transmute_copy(&pstatus), core::mem::transmute_copy(&pdataheadroommb)).into()
        }
        unsafe extern "system" fn LookupChunks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, phashes: *const DedupHash, prequestid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDedupDataPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDedupDataPort_Impl::LookupChunks(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&phashes)) {
                Ok(ok__) => {
                    prequestid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertChunks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdata: *const u8, prequestid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDedupDataPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDedupDataPort_Impl::InsertChunks(this, core::mem::transmute_copy(&chunkcount), core::mem::transmute_copy(&pchunkmetadata), core::mem::transmute_copy(&databytecount), core::mem::transmute_copy(&pchunkdata)) {
                Ok(ok__) => {
                    prequestid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertChunksWithStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdatastream: *mut core::ffi::c_void, prequestid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDedupDataPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDedupDataPort_Impl::InsertChunksWithStream(this, core::mem::transmute_copy(&chunkcount), core::mem::transmute_copy(&pchunkmetadata), core::mem::transmute_copy(&databytecount), windows_core::from_raw_borrowed(&pchunkdatastream)) {
                Ok(ok__) => {
                    prequestid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitStreams<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentries: *const DedupStreamEntry, prequestid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDedupDataPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDedupDataPort_Impl::CommitStreams(this, core::mem::transmute_copy(&streamcount), core::mem::transmute_copy(&pstreams), core::mem::transmute_copy(&entrycount), core::mem::transmute_copy(&pentries)) {
                Ok(ok__) => {
                    prequestid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitStreamsWithStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentriesstream: *mut core::ffi::c_void, prequestid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDedupDataPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDedupDataPort_Impl::CommitStreamsWithStream(this, core::mem::transmute_copy(&streamcount), core::mem::transmute_copy(&pstreams), core::mem::transmute_copy(&entrycount), windows_core::from_raw_borrowed(&pentriesstream)) {
                Ok(ok__) => {
                    prequestid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreams<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamcount: u32, pstreampaths: *const core::mem::MaybeUninit<windows_core::BSTR>, prequestid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDedupDataPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDedupDataPort_Impl::GetStreams(this, core::mem::transmute_copy(&streamcount), core::mem::transmute_copy(&pstreampaths)) {
                Ok(ok__) => {
                    prequestid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamsResults<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: windows_core::GUID, maxwaitms: u32, streamentryindex: u32, pstreamcount: *mut u32, ppstreams: *mut *mut DedupStream, pentrycount: *mut u32, ppentries: *mut *mut DedupStreamEntry, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IDedupDataPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDedupDataPort_Impl::GetStreamsResults(this, core::mem::transmute(&requestid), core::mem::transmute_copy(&maxwaitms), core::mem::transmute_copy(&streamentryindex), core::mem::transmute_copy(&pstreamcount), core::mem::transmute_copy(&ppstreams), core::mem::transmute_copy(&pentrycount), core::mem::transmute_copy(&ppentries), core::mem::transmute_copy(&pstatus), core::mem::transmute_copy(&ppitemresults)).into()
        }
        unsafe extern "system" fn GetChunks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, phashes: *const DedupHash, prequestid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDedupDataPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDedupDataPort_Impl::GetChunks(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&phashes)) {
                Ok(ok__) => {
                    prequestid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChunksResults<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: windows_core::GUID, maxwaitms: u32, chunkindex: u32, pchunkcount: *mut u32, ppchunkmetadata: *mut *mut DedupChunk, pdatabytecount: *mut u32, ppchunkdata: *mut *mut u8, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IDedupDataPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDedupDataPort_Impl::GetChunksResults(this, core::mem::transmute(&requestid), core::mem::transmute_copy(&maxwaitms), core::mem::transmute_copy(&chunkindex), core::mem::transmute_copy(&pchunkcount), core::mem::transmute_copy(&ppchunkmetadata), core::mem::transmute_copy(&pdatabytecount), core::mem::transmute_copy(&ppchunkdata), core::mem::transmute_copy(&pstatus), core::mem::transmute_copy(&ppitemresults)).into()
        }
        unsafe extern "system" fn GetRequestStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: windows_core::GUID, pstatus: *mut DedupDataPortRequestStatus) -> windows_core::HRESULT
        where
            Identity: IDedupDataPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDedupDataPort_Impl::GetRequestStatus(this, core::mem::transmute(&requestid)) {
                Ok(ok__) => {
                    pstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRequestResults<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: windows_core::GUID, maxwaitms: u32, pbatchresult: *mut windows_core::HRESULT, pbatchcount: *mut u32, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IDedupDataPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDedupDataPort_Impl::GetRequestResults(this, core::mem::transmute(&requestid), core::mem::transmute_copy(&maxwaitms), core::mem::transmute_copy(&pbatchresult), core::mem::transmute_copy(&pbatchcount), core::mem::transmute_copy(&pstatus), core::mem::transmute_copy(&ppitemresults)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStatus: GetStatus::<Identity, OFFSET>,
            LookupChunks: LookupChunks::<Identity, OFFSET>,
            InsertChunks: InsertChunks::<Identity, OFFSET>,
            InsertChunksWithStream: InsertChunksWithStream::<Identity, OFFSET>,
            CommitStreams: CommitStreams::<Identity, OFFSET>,
            CommitStreamsWithStream: CommitStreamsWithStream::<Identity, OFFSET>,
            GetStreams: GetStreams::<Identity, OFFSET>,
            GetStreamsResults: GetStreamsResults::<Identity, OFFSET>,
            GetChunks: GetChunks::<Identity, OFFSET>,
            GetChunksResults: GetChunksResults::<Identity, OFFSET>,
            GetRequestStatus: GetRequestStatus::<Identity, OFFSET>,
            GetRequestResults: GetRequestResults::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDedupDataPort as windows_core::Interface>::IID
    }
}
pub trait IDedupDataPortManager_Impl: Sized {
    fn GetConfiguration(&self, pminchunksize: *mut u32, pmaxchunksize: *mut u32, pchunkingalgorithm: *mut DedupChunkingAlgorithm, phashingalgorithm: *mut DedupHashingAlgorithm, pcompressionalgorithm: *mut DedupCompressionAlgorithm) -> windows_core::Result<()>;
    fn GetVolumeStatus(&self, options: u32, path: &windows_core::BSTR) -> windows_core::Result<DedupDataPortVolumeStatus>;
    fn GetVolumeDataPort(&self, options: u32, path: &windows_core::BSTR) -> windows_core::Result<IDedupDataPort>;
}
impl windows_core::RuntimeName for IDedupDataPortManager {}
impl IDedupDataPortManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDedupDataPortManager_Vtbl
    where
        Identity: IDedupDataPortManager_Impl,
    {
        unsafe extern "system" fn GetConfiguration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pminchunksize: *mut u32, pmaxchunksize: *mut u32, pchunkingalgorithm: *mut DedupChunkingAlgorithm, phashingalgorithm: *mut DedupHashingAlgorithm, pcompressionalgorithm: *mut DedupCompressionAlgorithm) -> windows_core::HRESULT
        where
            Identity: IDedupDataPortManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDedupDataPortManager_Impl::GetConfiguration(this, core::mem::transmute_copy(&pminchunksize), core::mem::transmute_copy(&pmaxchunksize), core::mem::transmute_copy(&pchunkingalgorithm), core::mem::transmute_copy(&phashingalgorithm), core::mem::transmute_copy(&pcompressionalgorithm)).into()
        }
        unsafe extern "system" fn GetVolumeStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: u32, path: core::mem::MaybeUninit<windows_core::BSTR>, pstatus: *mut DedupDataPortVolumeStatus) -> windows_core::HRESULT
        where
            Identity: IDedupDataPortManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDedupDataPortManager_Impl::GetVolumeStatus(this, core::mem::transmute_copy(&options), core::mem::transmute(&path)) {
                Ok(ok__) => {
                    pstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVolumeDataPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: u32, path: core::mem::MaybeUninit<windows_core::BSTR>, ppdataport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDedupDataPortManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDedupDataPortManager_Impl::GetVolumeDataPort(this, core::mem::transmute_copy(&options), core::mem::transmute(&path)) {
                Ok(ok__) => {
                    ppdataport.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetConfiguration: GetConfiguration::<Identity, OFFSET>,
            GetVolumeStatus: GetVolumeStatus::<Identity, OFFSET>,
            GetVolumeDataPort: GetVolumeDataPort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDedupDataPortManager as windows_core::Interface>::IID
    }
}
pub trait IDedupIterateChunksHash32_Impl: Sized {
    fn PushBuffer(&self, pbuffer: *const u8, ulbufferlength: u32) -> windows_core::Result<()>;
    fn Next(&self, ulmaxchunks: u32, parrchunks: *mut DEDUP_CHUNK_INFO_HASH32, pulfetched: *mut u32) -> windows_core::Result<()>;
    fn Drain(&self) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDedupIterateChunksHash32 {}
impl IDedupIterateChunksHash32_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDedupIterateChunksHash32_Vtbl
    where
        Identity: IDedupIterateChunksHash32_Impl,
    {
        unsafe extern "system" fn PushBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *const u8, ulbufferlength: u32) -> windows_core::HRESULT
        where
            Identity: IDedupIterateChunksHash32_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDedupIterateChunksHash32_Impl::PushBuffer(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&ulbufferlength)).into()
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulmaxchunks: u32, parrchunks: *mut DEDUP_CHUNK_INFO_HASH32, pulfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDedupIterateChunksHash32_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDedupIterateChunksHash32_Impl::Next(this, core::mem::transmute_copy(&ulmaxchunks), core::mem::transmute_copy(&parrchunks), core::mem::transmute_copy(&pulfetched)).into()
        }
        unsafe extern "system" fn Drain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDedupIterateChunksHash32_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDedupIterateChunksHash32_Impl::Drain(this).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDedupIterateChunksHash32_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDedupIterateChunksHash32_Impl::Reset(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PushBuffer: PushBuffer::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Drain: Drain::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDedupIterateChunksHash32 as windows_core::Interface>::IID
    }
}
pub trait IDedupReadFileCallback_Impl: Sized {
    fn ReadBackupFile(&self, filefullpath: &windows_core::BSTR, fileoffset: i64, sizetoread: u32, filebuffer: *mut u8, returnedsize: *mut u32, flags: u32) -> windows_core::Result<()>;
    fn OrderContainersRestore(&self, numberofcontainers: u32, containerpaths: *const windows_core::BSTR, readplanentries: *mut u32, readplan: *mut *mut DEDUP_CONTAINER_EXTENT) -> windows_core::Result<()>;
    fn PreviewContainerRead(&self, filefullpath: &windows_core::BSTR, numberofreads: u32, readoffsets: *const DDP_FILE_EXTENT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDedupReadFileCallback {}
impl IDedupReadFileCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDedupReadFileCallback_Vtbl
    where
        Identity: IDedupReadFileCallback_Impl,
    {
        unsafe extern "system" fn ReadBackupFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filefullpath: core::mem::MaybeUninit<windows_core::BSTR>, fileoffset: i64, sizetoread: u32, filebuffer: *mut u8, returnedsize: *mut u32, flags: u32) -> windows_core::HRESULT
        where
            Identity: IDedupReadFileCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDedupReadFileCallback_Impl::ReadBackupFile(this, core::mem::transmute(&filefullpath), core::mem::transmute_copy(&fileoffset), core::mem::transmute_copy(&sizetoread), core::mem::transmute_copy(&filebuffer), core::mem::transmute_copy(&returnedsize), core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn OrderContainersRestore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, numberofcontainers: u32, containerpaths: *const core::mem::MaybeUninit<windows_core::BSTR>, readplanentries: *mut u32, readplan: *mut *mut DEDUP_CONTAINER_EXTENT) -> windows_core::HRESULT
        where
            Identity: IDedupReadFileCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDedupReadFileCallback_Impl::OrderContainersRestore(this, core::mem::transmute_copy(&numberofcontainers), core::mem::transmute_copy(&containerpaths), core::mem::transmute_copy(&readplanentries), core::mem::transmute_copy(&readplan)).into()
        }
        unsafe extern "system" fn PreviewContainerRead<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filefullpath: core::mem::MaybeUninit<windows_core::BSTR>, numberofreads: u32, readoffsets: *const DDP_FILE_EXTENT) -> windows_core::HRESULT
        where
            Identity: IDedupReadFileCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDedupReadFileCallback_Impl::PreviewContainerRead(this, core::mem::transmute(&filefullpath), core::mem::transmute_copy(&numberofreads), core::mem::transmute_copy(&readoffsets)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReadBackupFile: ReadBackupFile::<Identity, OFFSET>,
            OrderContainersRestore: OrderContainersRestore::<Identity, OFFSET>,
            PreviewContainerRead: PreviewContainerRead::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDedupReadFileCallback as windows_core::Interface>::IID
    }
}
