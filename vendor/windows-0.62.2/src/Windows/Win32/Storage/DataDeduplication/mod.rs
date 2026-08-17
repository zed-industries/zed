#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DDP_FILE_EXTENT {
    pub Length: i64,
    pub Offset: i64,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEDUP_BACKUP_SUPPORT_PARAM_TYPE(pub i32);
pub const DEDUP_CHUNKLIB_MAX_CHUNKS_ENUM: u32 = 1024u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEDUP_CHUNK_INFO_HASH32 {
    pub ChunkFlags: u32,
    pub ChunkOffsetInStream: u64,
    pub ChunkSize: u64,
    pub HashVal: [u8; 32],
}
impl Default for DEDUP_CHUNK_INFO_HASH32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEDUP_CONTAINER_EXTENT {
    pub ContainerIndex: u32,
    pub StartOffset: i64,
    pub Length: i64,
}
pub const DEDUP_PT_AvgChunkSizeBytes: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(3i32);
pub const DEDUP_PT_DisableStrongHashComputation: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(5i32);
pub const DEDUP_PT_InvariantChunking: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(4i32);
pub const DEDUP_PT_MaxChunkSizeBytes: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(2i32);
pub const DEDUP_PT_MinChunkSizeBytes: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(1i32);
pub const DEDUP_RECONSTRUCT_OPTIMIZED: DEDUP_BACKUP_SUPPORT_PARAM_TYPE = DEDUP_BACKUP_SUPPORT_PARAM_TYPE(2i32);
pub const DEDUP_RECONSTRUCT_UNOPTIMIZED: DEDUP_BACKUP_SUPPORT_PARAM_TYPE = DEDUP_BACKUP_SUPPORT_PARAM_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEDUP_SET_PARAM_TYPE(pub i32);
pub const DedupBackupSupport: windows_core::GUID = windows_core::GUID::from_u128(0x73d6b2ad_2984_4715_b2e3_924c149744dd);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DedupChunk {
    pub Hash: DedupHash,
    pub Flags: DedupChunkFlags,
    pub LogicalSize: u32,
    pub DataSize: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DedupChunkFlags(pub i32);
pub const DedupChunkFlags_Compressed: DedupChunkFlags = DedupChunkFlags(1i32);
pub const DedupChunkFlags_None: DedupChunkFlags = DedupChunkFlags(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DedupChunkingAlgorithm(pub i32);
pub const DedupChunkingAlgorithm_Unknonwn: DedupChunkingAlgorithm = DedupChunkingAlgorithm(0i32);
pub const DedupChunkingAlgorithm_V1: DedupChunkingAlgorithm = DedupChunkingAlgorithm(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DedupCompressionAlgorithm(pub i32);
pub const DedupCompressionAlgorithm_Unknonwn: DedupCompressionAlgorithm = DedupCompressionAlgorithm(0i32);
pub const DedupCompressionAlgorithm_Xpress: DedupCompressionAlgorithm = DedupCompressionAlgorithm(1i32);
pub const DedupDataPort: windows_core::GUID = windows_core::GUID::from_u128(0x8f107207_1829_48b2_a64b_e61f8e0d9acb);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DedupDataPortManagerOption(pub i32);
pub const DedupDataPortManagerOption_AutoStart: DedupDataPortManagerOption = DedupDataPortManagerOption(1i32);
pub const DedupDataPortManagerOption_None: DedupDataPortManagerOption = DedupDataPortManagerOption(0i32);
pub const DedupDataPortManagerOption_SkipReconciliation: DedupDataPortManagerOption = DedupDataPortManagerOption(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DedupDataPortRequestStatus(pub i32);
pub const DedupDataPortRequestStatus_Complete: DedupDataPortRequestStatus = DedupDataPortRequestStatus(4i32);
pub const DedupDataPortRequestStatus_Failed: DedupDataPortRequestStatus = DedupDataPortRequestStatus(5i32);
pub const DedupDataPortRequestStatus_Partial: DedupDataPortRequestStatus = DedupDataPortRequestStatus(3i32);
pub const DedupDataPortRequestStatus_Processing: DedupDataPortRequestStatus = DedupDataPortRequestStatus(2i32);
pub const DedupDataPortRequestStatus_Queued: DedupDataPortRequestStatus = DedupDataPortRequestStatus(1i32);
pub const DedupDataPortRequestStatus_Unknown: DedupDataPortRequestStatus = DedupDataPortRequestStatus(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DedupDataPortVolumeStatus(pub i32);
pub const DedupDataPortVolumeStatus_Initializing: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(3i32);
pub const DedupDataPortVolumeStatus_Maintenance: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(5i32);
pub const DedupDataPortVolumeStatus_NotAvailable: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(2i32);
pub const DedupDataPortVolumeStatus_NotEnabled: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(1i32);
pub const DedupDataPortVolumeStatus_Ready: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(4i32);
pub const DedupDataPortVolumeStatus_Shutdown: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(6i32);
pub const DedupDataPortVolumeStatus_Unknown: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DedupHash {
    pub Hash: [u8; 32],
}
impl Default for DedupHash {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DedupHashingAlgorithm(pub i32);
pub const DedupHashingAlgorithm_Unknonwn: DedupHashingAlgorithm = DedupHashingAlgorithm(0i32);
pub const DedupHashingAlgorithm_V1: DedupHashingAlgorithm = DedupHashingAlgorithm(1i32);
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DedupStream {
    pub Path: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub Offset: u64,
    pub Length: u64,
    pub ChunkCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DedupStreamEntry {
    pub Hash: DedupHash,
    pub LogicalSize: u32,
    pub Offset: u64,
}
windows_core::imp::define_interface!(IDedupBackupSupport, IDedupBackupSupport_Vtbl, 0xc719d963_2b2d_415e_acf7_7eb7ca596ff4);
windows_core::imp::interface_hierarchy!(IDedupBackupSupport, windows_core::IUnknown);
impl IDedupBackupSupport {
    pub unsafe fn RestoreFiles<P2>(&self, numberoffiles: u32, filefullpaths: *const windows_core::BSTR, store: P2, flags: u32, fileresults: *mut windows_core::HRESULT) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IDedupReadFileCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).RestoreFiles)(windows_core::Interface::as_raw(self), numberoffiles, core::mem::transmute(filefullpaths), store.param().abi(), flags, fileresults as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupBackupSupport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RestoreFiles: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait IDedupBackupSupport_Impl: windows_core::IUnknownImpl {
    fn RestoreFiles(&self, numberoffiles: u32, filefullpaths: *const windows_core::BSTR, store: windows_core::Ref<IDedupReadFileCallback>, flags: u32, fileresults: *mut windows_core::HRESULT) -> windows_core::Result<()>;
}
impl IDedupBackupSupport_Vtbl {
    pub const fn new<Identity: IDedupBackupSupport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RestoreFiles<Identity: IDedupBackupSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numberoffiles: u32, filefullpaths: *const *mut core::ffi::c_void, store: *mut core::ffi::c_void, flags: u32, fileresults: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDedupBackupSupport_Impl::RestoreFiles(this, core::mem::transmute_copy(&numberoffiles), core::mem::transmute_copy(&filefullpaths), core::mem::transmute_copy(&store), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&fileresults)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RestoreFiles: RestoreFiles::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDedupBackupSupport as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDedupBackupSupport {}
windows_core::imp::define_interface!(IDedupChunkLibrary, IDedupChunkLibrary_Vtbl, 0xbb5144d7_2720_4dcc_8777_78597416ec23);
windows_core::imp::interface_hierarchy!(IDedupChunkLibrary, windows_core::IUnknown);
impl IDedupChunkLibrary {
    pub unsafe fn InitializeForPushBuffers(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InitializeForPushBuffers)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Uninitialize(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Uninitialize)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetParameter(&self, dwparamtype: u32, vparamvalue: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetParameter)(windows_core::Interface::as_raw(self), dwparamtype, core::mem::transmute_copy(vparamvalue)).ok() }
    }
    pub unsafe fn StartChunking(&self, iiditeratorinterfaceid: windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartChunking)(windows_core::Interface::as_raw(self), core::mem::transmute(iiditeratorinterfaceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupChunkLibrary_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitializeForPushBuffers: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Uninitialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetParameter: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetParameter: usize,
    pub StartChunking: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDedupChunkLibrary_Impl: windows_core::IUnknownImpl {
    fn InitializeForPushBuffers(&self) -> windows_core::Result<()>;
    fn Uninitialize(&self) -> windows_core::Result<()>;
    fn SetParameter(&self, dwparamtype: u32, vparamvalue: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn StartChunking(&self, iiditeratorinterfaceid: &windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDedupChunkLibrary_Vtbl {
    pub const fn new<Identity: IDedupChunkLibrary_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeForPushBuffers<Identity: IDedupChunkLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDedupChunkLibrary_Impl::InitializeForPushBuffers(this).into()
            }
        }
        unsafe extern "system" fn Uninitialize<Identity: IDedupChunkLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDedupChunkLibrary_Impl::Uninitialize(this).into()
            }
        }
        unsafe extern "system" fn SetParameter<Identity: IDedupChunkLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwparamtype: u32, vparamvalue: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDedupChunkLibrary_Impl::SetParameter(this, core::mem::transmute_copy(&dwparamtype), core::mem::transmute(&vparamvalue)).into()
            }
        }
        unsafe extern "system" fn StartChunking<Identity: IDedupChunkLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iiditeratorinterfaceid: windows_core::GUID, ppchunksenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDedupChunkLibrary_Impl::StartChunking(this, core::mem::transmute(&iiditeratorinterfaceid)) {
                    Ok(ok__) => {
                        ppchunksenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDedupChunkLibrary {}
windows_core::imp::define_interface!(IDedupDataPort, IDedupDataPort_Vtbl, 0x7963d734_40a9_4ea3_bbf6_5a89d26f7ae8);
windows_core::imp::interface_hierarchy!(IDedupDataPort, windows_core::IUnknown);
impl IDedupDataPort {
    pub unsafe fn GetStatus(&self, pstatus: *mut DedupDataPortVolumeStatus, pdataheadroommb: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), pstatus as _, pdataheadroommb as _).ok() }
    }
    pub unsafe fn LookupChunks(&self, phashes: &[DedupHash]) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LookupChunks)(windows_core::Interface::as_raw(self), phashes.len().try_into().unwrap(), core::mem::transmute(phashes.as_ptr()), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InsertChunks(&self, pchunkmetadata: &[DedupChunk], pchunkdata: &[u8]) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InsertChunks)(windows_core::Interface::as_raw(self), pchunkmetadata.len().try_into().unwrap(), core::mem::transmute(pchunkmetadata.as_ptr()), pchunkdata.len().try_into().unwrap(), core::mem::transmute(pchunkdata.as_ptr()), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertChunksWithStream<P3>(&self, pchunkmetadata: &[DedupChunk], databytecount: u32, pchunkdatastream: P3) -> windows_core::Result<windows_core::GUID>
    where
        P3: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InsertChunksWithStream)(windows_core::Interface::as_raw(self), pchunkmetadata.len().try_into().unwrap(), core::mem::transmute(pchunkmetadata.as_ptr()), databytecount, pchunkdatastream.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CommitStreams(&self, pstreams: &[DedupStream], pentries: &[DedupStreamEntry]) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommitStreams)(windows_core::Interface::as_raw(self), pstreams.len().try_into().unwrap(), core::mem::transmute(pstreams.as_ptr()), pentries.len().try_into().unwrap(), core::mem::transmute(pentries.as_ptr()), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommitStreamsWithStream<P3>(&self, pstreams: &[DedupStream], entrycount: u32, pentriesstream: P3) -> windows_core::Result<windows_core::GUID>
    where
        P3: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommitStreamsWithStream)(windows_core::Interface::as_raw(self), pstreams.len().try_into().unwrap(), core::mem::transmute(pstreams.as_ptr()), entrycount, pentriesstream.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStreams(&self, pstreampaths: &[windows_core::BSTR]) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreams)(windows_core::Interface::as_raw(self), pstreampaths.len().try_into().unwrap(), core::mem::transmute(pstreampaths.as_ptr()), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStreamsResults(&self, requestid: windows_core::GUID, maxwaitms: u32, streamentryindex: u32, pstreamcount: *mut u32, ppstreams: *mut *mut DedupStream, pentrycount: *mut u32, ppentries: *mut *mut DedupStreamEntry, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStreamsResults)(windows_core::Interface::as_raw(self), core::mem::transmute(requestid), maxwaitms, streamentryindex, pstreamcount as _, ppstreams as _, pentrycount as _, ppentries as _, pstatus as _, ppitemresults as _).ok() }
    }
    pub unsafe fn GetChunks(&self, phashes: &[DedupHash]) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChunks)(windows_core::Interface::as_raw(self), phashes.len().try_into().unwrap(), core::mem::transmute(phashes.as_ptr()), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetChunksResults(&self, requestid: windows_core::GUID, maxwaitms: u32, chunkindex: u32, pchunkcount: *mut u32, ppchunkmetadata: *mut *mut DedupChunk, pdatabytecount: *mut u32, ppchunkdata: *mut *mut u8, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetChunksResults)(windows_core::Interface::as_raw(self), core::mem::transmute(requestid), maxwaitms, chunkindex, pchunkcount as _, ppchunkmetadata as _, pdatabytecount as _, ppchunkdata as _, pstatus as _, ppitemresults as _).ok() }
    }
    pub unsafe fn GetRequestStatus(&self, requestid: windows_core::GUID) -> windows_core::Result<DedupDataPortRequestStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRequestStatus)(windows_core::Interface::as_raw(self), core::mem::transmute(requestid), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRequestResults(&self, requestid: windows_core::GUID, maxwaitms: u32, pbatchresult: *mut windows_core::HRESULT, pbatchcount: *mut u32, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRequestResults)(windows_core::Interface::as_raw(self), core::mem::transmute(requestid), maxwaitms, pbatchresult as _, pbatchcount as _, pstatus as _, ppitemresults as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupDataPort_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DedupDataPortVolumeStatus, *mut u32) -> windows_core::HRESULT,
    pub LookupChunks: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DedupHash, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub InsertChunks: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DedupChunk, u32, *const u8, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertChunksWithStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DedupChunk, u32, *mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertChunksWithStream: usize,
    pub CommitStreams: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DedupStream, u32, *const DedupStreamEntry, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CommitStreamsWithStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DedupStream, u32, *mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommitStreamsWithStream: usize,
    pub GetStreams: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetStreamsResults: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32, u32, *mut u32, *mut *mut DedupStream, *mut u32, *mut *mut DedupStreamEntry, *mut DedupDataPortRequestStatus, *mut *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetChunks: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DedupHash, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetChunksResults: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32, u32, *mut u32, *mut *mut DedupChunk, *mut u32, *mut *mut u8, *mut DedupDataPortRequestStatus, *mut *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetRequestStatus: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut DedupDataPortRequestStatus) -> windows_core::HRESULT,
    pub GetRequestResults: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32, *mut windows_core::HRESULT, *mut u32, *mut DedupDataPortRequestStatus, *mut *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDedupDataPort_Impl: windows_core::IUnknownImpl {
    fn GetStatus(&self, pstatus: *mut DedupDataPortVolumeStatus, pdataheadroommb: *mut u32) -> windows_core::Result<()>;
    fn LookupChunks(&self, count: u32, phashes: *const DedupHash) -> windows_core::Result<windows_core::GUID>;
    fn InsertChunks(&self, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdata: *const u8) -> windows_core::Result<windows_core::GUID>;
    fn InsertChunksWithStream(&self, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdatastream: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<windows_core::GUID>;
    fn CommitStreams(&self, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentries: *const DedupStreamEntry) -> windows_core::Result<windows_core::GUID>;
    fn CommitStreamsWithStream(&self, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentriesstream: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<windows_core::GUID>;
    fn GetStreams(&self, streamcount: u32, pstreampaths: *const windows_core::BSTR) -> windows_core::Result<windows_core::GUID>;
    fn GetStreamsResults(&self, requestid: &windows_core::GUID, maxwaitms: u32, streamentryindex: u32, pstreamcount: *mut u32, ppstreams: *mut *mut DedupStream, pentrycount: *mut u32, ppentries: *mut *mut DedupStreamEntry, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetChunks(&self, count: u32, phashes: *const DedupHash) -> windows_core::Result<windows_core::GUID>;
    fn GetChunksResults(&self, requestid: &windows_core::GUID, maxwaitms: u32, chunkindex: u32, pchunkcount: *mut u32, ppchunkmetadata: *mut *mut DedupChunk, pdatabytecount: *mut u32, ppchunkdata: *mut *mut u8, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetRequestStatus(&self, requestid: &windows_core::GUID) -> windows_core::Result<DedupDataPortRequestStatus>;
    fn GetRequestResults(&self, requestid: &windows_core::GUID, maxwaitms: u32, pbatchresult: *mut windows_core::HRESULT, pbatchcount: *mut u32, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut windows_core::HRESULT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IDedupDataPort_Vtbl {
    pub const fn new<Identity: IDedupDataPort_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStatus<Identity: IDedupDataPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut DedupDataPortVolumeStatus, pdataheadroommb: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDedupDataPort_Impl::GetStatus(this, core::mem::transmute_copy(&pstatus), core::mem::transmute_copy(&pdataheadroommb)).into()
            }
        }
        unsafe extern "system" fn LookupChunks<Identity: IDedupDataPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, phashes: *const DedupHash, prequestid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDedupDataPort_Impl::LookupChunks(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&phashes)) {
                    Ok(ok__) => {
                        prequestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertChunks<Identity: IDedupDataPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdata: *const u8, prequestid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDedupDataPort_Impl::InsertChunks(this, core::mem::transmute_copy(&chunkcount), core::mem::transmute_copy(&pchunkmetadata), core::mem::transmute_copy(&databytecount), core::mem::transmute_copy(&pchunkdata)) {
                    Ok(ok__) => {
                        prequestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertChunksWithStream<Identity: IDedupDataPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdatastream: *mut core::ffi::c_void, prequestid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDedupDataPort_Impl::InsertChunksWithStream(this, core::mem::transmute_copy(&chunkcount), core::mem::transmute_copy(&pchunkmetadata), core::mem::transmute_copy(&databytecount), core::mem::transmute_copy(&pchunkdatastream)) {
                    Ok(ok__) => {
                        prequestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CommitStreams<Identity: IDedupDataPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentries: *const DedupStreamEntry, prequestid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDedupDataPort_Impl::CommitStreams(this, core::mem::transmute_copy(&streamcount), core::mem::transmute_copy(&pstreams), core::mem::transmute_copy(&entrycount), core::mem::transmute_copy(&pentries)) {
                    Ok(ok__) => {
                        prequestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CommitStreamsWithStream<Identity: IDedupDataPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentriesstream: *mut core::ffi::c_void, prequestid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDedupDataPort_Impl::CommitStreamsWithStream(this, core::mem::transmute_copy(&streamcount), core::mem::transmute_copy(&pstreams), core::mem::transmute_copy(&entrycount), core::mem::transmute_copy(&pentriesstream)) {
                    Ok(ok__) => {
                        prequestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStreams<Identity: IDedupDataPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamcount: u32, pstreampaths: *const *mut core::ffi::c_void, prequestid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDedupDataPort_Impl::GetStreams(this, core::mem::transmute_copy(&streamcount), core::mem::transmute_copy(&pstreampaths)) {
                    Ok(ok__) => {
                        prequestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStreamsResults<Identity: IDedupDataPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: windows_core::GUID, maxwaitms: u32, streamentryindex: u32, pstreamcount: *mut u32, ppstreams: *mut *mut DedupStream, pentrycount: *mut u32, ppentries: *mut *mut DedupStreamEntry, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDedupDataPort_Impl::GetStreamsResults(this, core::mem::transmute(&requestid), core::mem::transmute_copy(&maxwaitms), core::mem::transmute_copy(&streamentryindex), core::mem::transmute_copy(&pstreamcount), core::mem::transmute_copy(&ppstreams), core::mem::transmute_copy(&pentrycount), core::mem::transmute_copy(&ppentries), core::mem::transmute_copy(&pstatus), core::mem::transmute_copy(&ppitemresults)).into()
            }
        }
        unsafe extern "system" fn GetChunks<Identity: IDedupDataPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, phashes: *const DedupHash, prequestid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDedupDataPort_Impl::GetChunks(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&phashes)) {
                    Ok(ok__) => {
                        prequestid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetChunksResults<Identity: IDedupDataPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: windows_core::GUID, maxwaitms: u32, chunkindex: u32, pchunkcount: *mut u32, ppchunkmetadata: *mut *mut DedupChunk, pdatabytecount: *mut u32, ppchunkdata: *mut *mut u8, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDedupDataPort_Impl::GetChunksResults(this, core::mem::transmute(&requestid), core::mem::transmute_copy(&maxwaitms), core::mem::transmute_copy(&chunkindex), core::mem::transmute_copy(&pchunkcount), core::mem::transmute_copy(&ppchunkmetadata), core::mem::transmute_copy(&pdatabytecount), core::mem::transmute_copy(&ppchunkdata), core::mem::transmute_copy(&pstatus), core::mem::transmute_copy(&ppitemresults)).into()
            }
        }
        unsafe extern "system" fn GetRequestStatus<Identity: IDedupDataPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: windows_core::GUID, pstatus: *mut DedupDataPortRequestStatus) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDedupDataPort_Impl::GetRequestStatus(this, core::mem::transmute(&requestid)) {
                    Ok(ok__) => {
                        pstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRequestResults<Identity: IDedupDataPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: windows_core::GUID, maxwaitms: u32, pbatchresult: *mut windows_core::HRESULT, pbatchcount: *mut u32, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDedupDataPort_Impl::GetRequestResults(this, core::mem::transmute(&requestid), core::mem::transmute_copy(&maxwaitms), core::mem::transmute_copy(&pbatchresult), core::mem::transmute_copy(&pbatchcount), core::mem::transmute_copy(&pstatus), core::mem::transmute_copy(&ppitemresults)).into()
            }
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
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDedupDataPort {}
windows_core::imp::define_interface!(IDedupDataPortManager, IDedupDataPortManager_Vtbl, 0x44677452_b90a_445e_8192_cdcfe81511fb);
windows_core::imp::interface_hierarchy!(IDedupDataPortManager, windows_core::IUnknown);
impl IDedupDataPortManager {
    pub unsafe fn GetConfiguration(&self, pminchunksize: *mut u32, pmaxchunksize: *mut u32, pchunkingalgorithm: *mut DedupChunkingAlgorithm, phashingalgorithm: *mut DedupHashingAlgorithm, pcompressionalgorithm: *mut DedupCompressionAlgorithm) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetConfiguration)(windows_core::Interface::as_raw(self), pminchunksize as _, pmaxchunksize as _, pchunkingalgorithm as _, phashingalgorithm as _, pcompressionalgorithm as _).ok() }
    }
    pub unsafe fn GetVolumeStatus(&self, options: u32, path: &windows_core::BSTR) -> windows_core::Result<DedupDataPortVolumeStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVolumeStatus)(windows_core::Interface::as_raw(self), options, core::mem::transmute_copy(path), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetVolumeDataPort(&self, options: u32, path: &windows_core::BSTR) -> windows_core::Result<IDedupDataPort> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVolumeDataPort)(windows_core::Interface::as_raw(self), options, core::mem::transmute_copy(path), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupDataPortManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut DedupChunkingAlgorithm, *mut DedupHashingAlgorithm, *mut DedupCompressionAlgorithm) -> windows_core::HRESULT,
    pub GetVolumeStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut DedupDataPortVolumeStatus) -> windows_core::HRESULT,
    pub GetVolumeDataPort: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDedupDataPortManager_Impl: windows_core::IUnknownImpl {
    fn GetConfiguration(&self, pminchunksize: *mut u32, pmaxchunksize: *mut u32, pchunkingalgorithm: *mut DedupChunkingAlgorithm, phashingalgorithm: *mut DedupHashingAlgorithm, pcompressionalgorithm: *mut DedupCompressionAlgorithm) -> windows_core::Result<()>;
    fn GetVolumeStatus(&self, options: u32, path: &windows_core::BSTR) -> windows_core::Result<DedupDataPortVolumeStatus>;
    fn GetVolumeDataPort(&self, options: u32, path: &windows_core::BSTR) -> windows_core::Result<IDedupDataPort>;
}
impl IDedupDataPortManager_Vtbl {
    pub const fn new<Identity: IDedupDataPortManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetConfiguration<Identity: IDedupDataPortManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pminchunksize: *mut u32, pmaxchunksize: *mut u32, pchunkingalgorithm: *mut DedupChunkingAlgorithm, phashingalgorithm: *mut DedupHashingAlgorithm, pcompressionalgorithm: *mut DedupCompressionAlgorithm) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDedupDataPortManager_Impl::GetConfiguration(this, core::mem::transmute_copy(&pminchunksize), core::mem::transmute_copy(&pmaxchunksize), core::mem::transmute_copy(&pchunkingalgorithm), core::mem::transmute_copy(&phashingalgorithm), core::mem::transmute_copy(&pcompressionalgorithm)).into()
            }
        }
        unsafe extern "system" fn GetVolumeStatus<Identity: IDedupDataPortManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: u32, path: *mut core::ffi::c_void, pstatus: *mut DedupDataPortVolumeStatus) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDedupDataPortManager_Impl::GetVolumeStatus(this, core::mem::transmute_copy(&options), core::mem::transmute(&path)) {
                    Ok(ok__) => {
                        pstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVolumeDataPort<Identity: IDedupDataPortManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: u32, path: *mut core::ffi::c_void, ppdataport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDedupDataPortManager_Impl::GetVolumeDataPort(this, core::mem::transmute_copy(&options), core::mem::transmute(&path)) {
                    Ok(ok__) => {
                        ppdataport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for IDedupDataPortManager {}
windows_core::imp::define_interface!(IDedupIterateChunksHash32, IDedupIterateChunksHash32_Vtbl, 0x90b584d3_72aa_400f_9767_cad866a5a2d8);
windows_core::imp::interface_hierarchy!(IDedupIterateChunksHash32, windows_core::IUnknown);
impl IDedupIterateChunksHash32 {
    pub unsafe fn PushBuffer(&self, pbuffer: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PushBuffer)(windows_core::Interface::as_raw(self), core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn Next(&self, parrchunks: &mut [DEDUP_CHUNK_INFO_HASH32], pulfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), parrchunks.len().try_into().unwrap(), core::mem::transmute(parrchunks.as_ptr()), pulfetched as _).ok() }
    }
    pub unsafe fn Drain(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Drain)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupIterateChunksHash32_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PushBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DEDUP_CHUNK_INFO_HASH32, *mut u32) -> windows_core::HRESULT,
    pub Drain: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDedupIterateChunksHash32_Impl: windows_core::IUnknownImpl {
    fn PushBuffer(&self, pbuffer: *const u8, ulbufferlength: u32) -> windows_core::Result<()>;
    fn Next(&self, ulmaxchunks: u32, parrchunks: *mut DEDUP_CHUNK_INFO_HASH32, pulfetched: *mut u32) -> windows_core::Result<()>;
    fn Drain(&self) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
}
impl IDedupIterateChunksHash32_Vtbl {
    pub const fn new<Identity: IDedupIterateChunksHash32_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PushBuffer<Identity: IDedupIterateChunksHash32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *const u8, ulbufferlength: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDedupIterateChunksHash32_Impl::PushBuffer(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&ulbufferlength)).into()
            }
        }
        unsafe extern "system" fn Next<Identity: IDedupIterateChunksHash32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulmaxchunks: u32, parrchunks: *mut DEDUP_CHUNK_INFO_HASH32, pulfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDedupIterateChunksHash32_Impl::Next(this, core::mem::transmute_copy(&ulmaxchunks), core::mem::transmute_copy(&parrchunks), core::mem::transmute_copy(&pulfetched)).into()
            }
        }
        unsafe extern "system" fn Drain<Identity: IDedupIterateChunksHash32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDedupIterateChunksHash32_Impl::Drain(this).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IDedupIterateChunksHash32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDedupIterateChunksHash32_Impl::Reset(this).into()
            }
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
impl windows_core::RuntimeName for IDedupIterateChunksHash32 {}
windows_core::imp::define_interface!(IDedupReadFileCallback, IDedupReadFileCallback_Vtbl, 0x7bacc67a_2f1d_42d0_897e_6ff62dd533bb);
windows_core::imp::interface_hierarchy!(IDedupReadFileCallback, windows_core::IUnknown);
impl IDedupReadFileCallback {
    pub unsafe fn ReadBackupFile(&self, filefullpath: &windows_core::BSTR, fileoffset: i64, filebuffer: &mut [u8], returnedsize: *mut u32, flags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReadBackupFile)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filefullpath), fileoffset, filebuffer.len().try_into().unwrap(), core::mem::transmute(filebuffer.as_ptr()), returnedsize as _, flags).ok() }
    }
    pub unsafe fn OrderContainersRestore(&self, containerpaths: &[windows_core::BSTR], readplanentries: *mut u32, readplan: *mut *mut DEDUP_CONTAINER_EXTENT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OrderContainersRestore)(windows_core::Interface::as_raw(self), containerpaths.len().try_into().unwrap(), core::mem::transmute(containerpaths.as_ptr()), readplanentries as _, readplan as _).ok() }
    }
    pub unsafe fn PreviewContainerRead(&self, filefullpath: &windows_core::BSTR, readoffsets: &[DDP_FILE_EXTENT]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PreviewContainerRead)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filefullpath), readoffsets.len().try_into().unwrap(), core::mem::transmute(readoffsets.as_ptr())).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupReadFileCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReadBackupFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i64, u32, *mut u8, *mut u32, u32) -> windows_core::HRESULT,
    pub OrderContainersRestore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, *mut u32, *mut *mut DEDUP_CONTAINER_EXTENT) -> windows_core::HRESULT,
    pub PreviewContainerRead: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const DDP_FILE_EXTENT) -> windows_core::HRESULT,
}
pub trait IDedupReadFileCallback_Impl: windows_core::IUnknownImpl {
    fn ReadBackupFile(&self, filefullpath: &windows_core::BSTR, fileoffset: i64, sizetoread: u32, filebuffer: *mut u8, returnedsize: *mut u32, flags: u32) -> windows_core::Result<()>;
    fn OrderContainersRestore(&self, numberofcontainers: u32, containerpaths: *const windows_core::BSTR, readplanentries: *mut u32, readplan: *mut *mut DEDUP_CONTAINER_EXTENT) -> windows_core::Result<()>;
    fn PreviewContainerRead(&self, filefullpath: &windows_core::BSTR, numberofreads: u32, readoffsets: *const DDP_FILE_EXTENT) -> windows_core::Result<()>;
}
impl IDedupReadFileCallback_Vtbl {
    pub const fn new<Identity: IDedupReadFileCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReadBackupFile<Identity: IDedupReadFileCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filefullpath: *mut core::ffi::c_void, fileoffset: i64, sizetoread: u32, filebuffer: *mut u8, returnedsize: *mut u32, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDedupReadFileCallback_Impl::ReadBackupFile(this, core::mem::transmute(&filefullpath), core::mem::transmute_copy(&fileoffset), core::mem::transmute_copy(&sizetoread), core::mem::transmute_copy(&filebuffer), core::mem::transmute_copy(&returnedsize), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn OrderContainersRestore<Identity: IDedupReadFileCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numberofcontainers: u32, containerpaths: *const *mut core::ffi::c_void, readplanentries: *mut u32, readplan: *mut *mut DEDUP_CONTAINER_EXTENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDedupReadFileCallback_Impl::OrderContainersRestore(this, core::mem::transmute_copy(&numberofcontainers), core::mem::transmute_copy(&containerpaths), core::mem::transmute_copy(&readplanentries), core::mem::transmute_copy(&readplan)).into()
            }
        }
        unsafe extern "system" fn PreviewContainerRead<Identity: IDedupReadFileCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filefullpath: *mut core::ffi::c_void, numberofreads: u32, readoffsets: *const DDP_FILE_EXTENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDedupReadFileCallback_Impl::PreviewContainerRead(this, core::mem::transmute(&filefullpath), core::mem::transmute_copy(&numberofreads), core::mem::transmute_copy(&readoffsets)).into()
            }
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
impl windows_core::RuntimeName for IDedupReadFileCallback {}
