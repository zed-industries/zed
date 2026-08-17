#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetAddColumnA<P0, P1>(sesid: P0, tableid: P1, szcolumnname: *const i8, pcolumndef: *const JET_COLUMNDEF, pvdefault: Option<*const core::ffi::c_void>, cbdefault: u32, pcolumnid: Option<*mut u32>) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetAddColumnA(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, szcolumnname : *const i8, pcolumndef : *const JET_COLUMNDEF, pvdefault : *const core::ffi::c_void, cbdefault : u32, pcolumnid : *mut u32) -> i32);
    JetAddColumnA(sesid.param().abi(), tableid.param().abi(), szcolumnname, pcolumndef, core::mem::transmute(pvdefault.unwrap_or(std::ptr::null())), cbdefault, core::mem::transmute(pcolumnid.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetAddColumnW<P0, P1>(sesid: P0, tableid: P1, szcolumnname: *const u16, pcolumndef: *const JET_COLUMNDEF, pvdefault: Option<*const core::ffi::c_void>, cbdefault: u32, pcolumnid: Option<*mut u32>) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetAddColumnW(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, szcolumnname : *const u16, pcolumndef : *const JET_COLUMNDEF, pvdefault : *const core::ffi::c_void, cbdefault : u32, pcolumnid : *mut u32) -> i32);
    JetAddColumnW(sesid.param().abi(), tableid.param().abi(), szcolumnname, pcolumndef, core::mem::transmute(pvdefault.unwrap_or(std::ptr::null())), cbdefault, core::mem::transmute(pcolumnid.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn JetAttachDatabase2A<P0>(sesid: P0, szfilename: *const i8, cpgdatabasesizemax: u32, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetAttachDatabase2A(sesid : JET_SESID, szfilename : *const i8, cpgdatabasesizemax : u32, grbit : u32) -> i32);
    JetAttachDatabase2A(sesid.param().abi(), szfilename, cpgdatabasesizemax, grbit)
}
#[inline]
pub unsafe fn JetAttachDatabase2W<P0>(sesid: P0, szfilename: *const u16, cpgdatabasesizemax: u32, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetAttachDatabase2W(sesid : JET_SESID, szfilename : *const u16, cpgdatabasesizemax : u32, grbit : u32) -> i32);
    JetAttachDatabase2W(sesid.param().abi(), szfilename, cpgdatabasesizemax, grbit)
}
#[inline]
pub unsafe fn JetAttachDatabaseA<P0>(sesid: P0, szfilename: *const i8, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetAttachDatabaseA(sesid : JET_SESID, szfilename : *const i8, grbit : u32) -> i32);
    JetAttachDatabaseA(sesid.param().abi(), szfilename, grbit)
}
#[inline]
pub unsafe fn JetAttachDatabaseW<P0>(sesid: P0, szfilename: *const u16, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetAttachDatabaseW(sesid : JET_SESID, szfilename : *const u16, grbit : u32) -> i32);
    JetAttachDatabaseW(sesid.param().abi(), szfilename, grbit)
}
#[inline]
pub unsafe fn JetBackupA(szbackuppath: *const i8, grbit: u32, pfnstatus: JET_PFNSTATUS) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetBackupA(szbackuppath : *const i8, grbit : u32, pfnstatus : JET_PFNSTATUS) -> i32);
    JetBackupA(szbackuppath, grbit, pfnstatus)
}
#[inline]
pub unsafe fn JetBackupInstanceA<P0>(instance: P0, szbackuppath: *const i8, grbit: u32, pfnstatus: JET_PFNSTATUS) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetBackupInstanceA(instance : JET_INSTANCE, szbackuppath : *const i8, grbit : u32, pfnstatus : JET_PFNSTATUS) -> i32);
    JetBackupInstanceA(instance.param().abi(), szbackuppath, grbit, pfnstatus)
}
#[inline]
pub unsafe fn JetBackupInstanceW<P0>(instance: P0, szbackuppath: *const u16, grbit: u32, pfnstatus: JET_PFNSTATUS) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetBackupInstanceW(instance : JET_INSTANCE, szbackuppath : *const u16, grbit : u32, pfnstatus : JET_PFNSTATUS) -> i32);
    JetBackupInstanceW(instance.param().abi(), szbackuppath, grbit, pfnstatus)
}
#[inline]
pub unsafe fn JetBackupW(szbackuppath: *const u16, grbit: u32, pfnstatus: JET_PFNSTATUS) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetBackupW(szbackuppath : *const u16, grbit : u32, pfnstatus : JET_PFNSTATUS) -> i32);
    JetBackupW(szbackuppath, grbit, pfnstatus)
}
#[inline]
pub unsafe fn JetBeginExternalBackup(grbit: u32) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetBeginExternalBackup(grbit : u32) -> i32);
    JetBeginExternalBackup(grbit)
}
#[inline]
pub unsafe fn JetBeginExternalBackupInstance<P0>(instance: P0, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetBeginExternalBackupInstance(instance : JET_INSTANCE, grbit : u32) -> i32);
    JetBeginExternalBackupInstance(instance.param().abi(), grbit)
}
#[inline]
pub unsafe fn JetBeginSessionA<P0>(instance: P0, psesid: *mut JET_SESID, szusername: Option<*const i8>, szpassword: Option<*const i8>) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetBeginSessionA(instance : JET_INSTANCE, psesid : *mut JET_SESID, szusername : *const i8, szpassword : *const i8) -> i32);
    JetBeginSessionA(instance.param().abi(), psesid, core::mem::transmute(szusername.unwrap_or(std::ptr::null())), core::mem::transmute(szpassword.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn JetBeginSessionW<P0>(instance: P0, psesid: *mut JET_SESID, szusername: Option<*const u16>, szpassword: Option<*const u16>) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetBeginSessionW(instance : JET_INSTANCE, psesid : *mut JET_SESID, szusername : *const u16, szpassword : *const u16) -> i32);
    JetBeginSessionW(instance.param().abi(), psesid, core::mem::transmute(szusername.unwrap_or(std::ptr::null())), core::mem::transmute(szpassword.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn JetBeginTransaction<P0>(sesid: P0) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetBeginTransaction(sesid : JET_SESID) -> i32);
    JetBeginTransaction(sesid.param().abi())
}
#[inline]
pub unsafe fn JetBeginTransaction2<P0>(sesid: P0, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetBeginTransaction2(sesid : JET_SESID, grbit : u32) -> i32);
    JetBeginTransaction2(sesid.param().abi(), grbit)
}
#[inline]
pub unsafe fn JetBeginTransaction3<P0>(sesid: P0, trxid: i64, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetBeginTransaction3(sesid : JET_SESID, trxid : i64, grbit : u32) -> i32);
    JetBeginTransaction3(sesid.param().abi(), trxid, grbit)
}
#[inline]
pub unsafe fn JetCloseDatabase<P0>(sesid: P0, dbid: u32, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCloseDatabase(sesid : JET_SESID, dbid : u32, grbit : u32) -> i32);
    JetCloseDatabase(sesid.param().abi(), dbid, grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCloseFile<P0>(hffile: P0) -> i32
where
    P0: windows_core::Param<super::StructuredStorage::JET_HANDLE>,
{
    windows_targets::link!("esent.dll" "system" fn JetCloseFile(hffile : super::StructuredStorage:: JET_HANDLE) -> i32);
    JetCloseFile(hffile.param().abi())
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCloseFileInstance<P0, P1>(instance: P0, hffile: P1) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
    P1: windows_core::Param<super::StructuredStorage::JET_HANDLE>,
{
    windows_targets::link!("esent.dll" "system" fn JetCloseFileInstance(instance : JET_INSTANCE, hffile : super::StructuredStorage:: JET_HANDLE) -> i32);
    JetCloseFileInstance(instance.param().abi(), hffile.param().abi())
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCloseTable<P0, P1>(sesid: P0, tableid: P1) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCloseTable(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID) -> i32);
    JetCloseTable(sesid.param().abi(), tableid.param().abi())
}
#[inline]
pub unsafe fn JetCommitTransaction<P0>(sesid: P0, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCommitTransaction(sesid : JET_SESID, grbit : u32) -> i32);
    JetCommitTransaction(sesid.param().abi(), grbit)
}
#[inline]
pub unsafe fn JetCommitTransaction2<P0>(sesid: P0, grbit: u32, cmsecdurablecommit: u32, pcommitid: Option<*mut JET_COMMIT_ID>) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCommitTransaction2(sesid : JET_SESID, grbit : u32, cmsecdurablecommit : u32, pcommitid : *mut JET_COMMIT_ID) -> i32);
    JetCommitTransaction2(sesid.param().abi(), grbit, cmsecdurablecommit, core::mem::transmute(pcommitid.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn JetCompactA<P0>(sesid: P0, szdatabasesrc: *const i8, szdatabasedest: *const i8, pfnstatus: JET_PFNSTATUS, pconvert: Option<*const JET_CONVERT_A>, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCompactA(sesid : JET_SESID, szdatabasesrc : *const i8, szdatabasedest : *const i8, pfnstatus : JET_PFNSTATUS, pconvert : *const JET_CONVERT_A, grbit : u32) -> i32);
    JetCompactA(sesid.param().abi(), szdatabasesrc, szdatabasedest, pfnstatus, core::mem::transmute(pconvert.unwrap_or(std::ptr::null())), grbit)
}
#[inline]
pub unsafe fn JetCompactW<P0>(sesid: P0, szdatabasesrc: *const u16, szdatabasedest: *const u16, pfnstatus: JET_PFNSTATUS, pconvert: Option<*const JET_CONVERT_W>, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCompactW(sesid : JET_SESID, szdatabasesrc : *const u16, szdatabasedest : *const u16, pfnstatus : JET_PFNSTATUS, pconvert : *const JET_CONVERT_W, grbit : u32) -> i32);
    JetCompactW(sesid.param().abi(), szdatabasesrc, szdatabasedest, pfnstatus, core::mem::transmute(pconvert.unwrap_or(std::ptr::null())), grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetComputeStats<P0, P1>(sesid: P0, tableid: P1) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetComputeStats(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID) -> i32);
    JetComputeStats(sesid.param().abi(), tableid.param().abi())
}
#[inline]
pub unsafe fn JetConfigureProcessForCrashDump(grbit: u32) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetConfigureProcessForCrashDump(grbit : u32) -> i32);
    JetConfigureProcessForCrashDump(grbit)
}
#[inline]
pub unsafe fn JetCreateDatabase2A<P0>(sesid: P0, szfilename: *const i8, cpgdatabasesizemax: u32, pdbid: *mut u32, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCreateDatabase2A(sesid : JET_SESID, szfilename : *const i8, cpgdatabasesizemax : u32, pdbid : *mut u32, grbit : u32) -> i32);
    JetCreateDatabase2A(sesid.param().abi(), szfilename, cpgdatabasesizemax, pdbid, grbit)
}
#[inline]
pub unsafe fn JetCreateDatabase2W<P0>(sesid: P0, szfilename: *const u16, cpgdatabasesizemax: u32, pdbid: *mut u32, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCreateDatabase2W(sesid : JET_SESID, szfilename : *const u16, cpgdatabasesizemax : u32, pdbid : *mut u32, grbit : u32) -> i32);
    JetCreateDatabase2W(sesid.param().abi(), szfilename, cpgdatabasesizemax, pdbid, grbit)
}
#[inline]
pub unsafe fn JetCreateDatabaseA<P0>(sesid: P0, szfilename: *const i8, szconnect: Option<*const i8>, pdbid: *mut u32, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCreateDatabaseA(sesid : JET_SESID, szfilename : *const i8, szconnect : *const i8, pdbid : *mut u32, grbit : u32) -> i32);
    JetCreateDatabaseA(sesid.param().abi(), szfilename, core::mem::transmute(szconnect.unwrap_or(std::ptr::null())), pdbid, grbit)
}
#[inline]
pub unsafe fn JetCreateDatabaseW<P0>(sesid: P0, szfilename: *const u16, szconnect: Option<*const u16>, pdbid: *mut u32, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCreateDatabaseW(sesid : JET_SESID, szfilename : *const u16, szconnect : *const u16, pdbid : *mut u32, grbit : u32) -> i32);
    JetCreateDatabaseW(sesid.param().abi(), szfilename, core::mem::transmute(szconnect.unwrap_or(std::ptr::null())), pdbid, grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateIndex2A<P0, P1>(sesid: P0, tableid: P1, pindexcreate: &[JET_INDEXCREATE_A]) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCreateIndex2A(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, pindexcreate : *const JET_INDEXCREATE_A, cindexcreate : u32) -> i32);
    JetCreateIndex2A(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(pindexcreate.as_ptr()), pindexcreate.len().try_into().unwrap())
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateIndex2W<P0, P1>(sesid: P0, tableid: P1, pindexcreate: &[JET_INDEXCREATE_W]) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCreateIndex2W(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, pindexcreate : *const JET_INDEXCREATE_W, cindexcreate : u32) -> i32);
    JetCreateIndex2W(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(pindexcreate.as_ptr()), pindexcreate.len().try_into().unwrap())
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateIndex3A<P0, P1>(sesid: P0, tableid: P1, pindexcreate: &[JET_INDEXCREATE2_A]) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCreateIndex3A(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, pindexcreate : *const JET_INDEXCREATE2_A, cindexcreate : u32) -> i32);
    JetCreateIndex3A(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(pindexcreate.as_ptr()), pindexcreate.len().try_into().unwrap())
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateIndex3W<P0, P1>(sesid: P0, tableid: P1, pindexcreate: &[JET_INDEXCREATE2_W]) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCreateIndex3W(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, pindexcreate : *const JET_INDEXCREATE2_W, cindexcreate : u32) -> i32);
    JetCreateIndex3W(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(pindexcreate.as_ptr()), pindexcreate.len().try_into().unwrap())
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateIndex4A<P0, P1>(sesid: P0, tableid: P1, pindexcreate: &[JET_INDEXCREATE3_A]) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCreateIndex4A(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, pindexcreate : *const JET_INDEXCREATE3_A, cindexcreate : u32) -> i32);
    JetCreateIndex4A(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(pindexcreate.as_ptr()), pindexcreate.len().try_into().unwrap())
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateIndex4W<P0, P1>(sesid: P0, tableid: P1, pindexcreate: &[JET_INDEXCREATE3_W]) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCreateIndex4W(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, pindexcreate : *const JET_INDEXCREATE3_W, cindexcreate : u32) -> i32);
    JetCreateIndex4W(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(pindexcreate.as_ptr()), pindexcreate.len().try_into().unwrap())
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateIndexA<P0, P1>(sesid: P0, tableid: P1, szindexname: *const i8, grbit: u32, szkey: &[u8], ldensity: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCreateIndexA(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, szindexname : *const i8, grbit : u32, szkey : windows_core::PCSTR, cbkey : u32, ldensity : u32) -> i32);
    JetCreateIndexA(sesid.param().abi(), tableid.param().abi(), szindexname, grbit, core::mem::transmute(szkey.as_ptr()), szkey.len().try_into().unwrap(), ldensity)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateIndexW<P0, P1, P2>(sesid: P0, tableid: P1, szindexname: *const u16, grbit: u32, szkey: P2, cbkey: u32, ldensity: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("esent.dll" "system" fn JetCreateIndexW(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, szindexname : *const u16, grbit : u32, szkey : windows_core::PCWSTR, cbkey : u32, ldensity : u32) -> i32);
    JetCreateIndexW(sesid.param().abi(), tableid.param().abi(), szindexname, grbit, szkey.param().abi(), cbkey, ldensity)
}
#[inline]
pub unsafe fn JetCreateInstance2A(pinstance: *mut JET_INSTANCE, szinstancename: Option<*const i8>, szdisplayname: Option<*const i8>, grbit: u32) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetCreateInstance2A(pinstance : *mut JET_INSTANCE, szinstancename : *const i8, szdisplayname : *const i8, grbit : u32) -> i32);
    JetCreateInstance2A(pinstance, core::mem::transmute(szinstancename.unwrap_or(std::ptr::null())), core::mem::transmute(szdisplayname.unwrap_or(std::ptr::null())), grbit)
}
#[inline]
pub unsafe fn JetCreateInstance2W(pinstance: *mut JET_INSTANCE, szinstancename: Option<*const u16>, szdisplayname: Option<*const u16>, grbit: u32) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetCreateInstance2W(pinstance : *mut JET_INSTANCE, szinstancename : *const u16, szdisplayname : *const u16, grbit : u32) -> i32);
    JetCreateInstance2W(pinstance, core::mem::transmute(szinstancename.unwrap_or(std::ptr::null())), core::mem::transmute(szdisplayname.unwrap_or(std::ptr::null())), grbit)
}
#[inline]
pub unsafe fn JetCreateInstanceA(pinstance: *mut JET_INSTANCE, szinstancename: Option<*const i8>) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetCreateInstanceA(pinstance : *mut JET_INSTANCE, szinstancename : *const i8) -> i32);
    JetCreateInstanceA(pinstance, core::mem::transmute(szinstancename.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn JetCreateInstanceW(pinstance: *mut JET_INSTANCE, szinstancename: Option<*const u16>) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetCreateInstanceW(pinstance : *mut JET_INSTANCE, szinstancename : *const u16) -> i32);
    JetCreateInstanceW(pinstance, core::mem::transmute(szinstancename.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateTableA<P0>(sesid: P0, dbid: u32, sztablename: *const i8, lpages: u32, ldensity: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCreateTableA(sesid : JET_SESID, dbid : u32, sztablename : *const i8, lpages : u32, ldensity : u32, ptableid : *mut super::StructuredStorage:: JET_TABLEID) -> i32);
    JetCreateTableA(sesid.param().abi(), dbid, sztablename, lpages, ldensity, ptableid)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateTableColumnIndex2A<P0>(sesid: P0, dbid: u32, ptablecreate: *mut JET_TABLECREATE2_A) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCreateTableColumnIndex2A(sesid : JET_SESID, dbid : u32, ptablecreate : *mut JET_TABLECREATE2_A) -> i32);
    JetCreateTableColumnIndex2A(sesid.param().abi(), dbid, ptablecreate)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateTableColumnIndex2W<P0>(sesid: P0, dbid: u32, ptablecreate: *mut JET_TABLECREATE2_W) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCreateTableColumnIndex2W(sesid : JET_SESID, dbid : u32, ptablecreate : *mut JET_TABLECREATE2_W) -> i32);
    JetCreateTableColumnIndex2W(sesid.param().abi(), dbid, ptablecreate)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateTableColumnIndex3A<P0>(sesid: P0, dbid: u32, ptablecreate: *mut JET_TABLECREATE3_A) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCreateTableColumnIndex3A(sesid : JET_SESID, dbid : u32, ptablecreate : *mut JET_TABLECREATE3_A) -> i32);
    JetCreateTableColumnIndex3A(sesid.param().abi(), dbid, ptablecreate)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateTableColumnIndex3W<P0>(sesid: P0, dbid: u32, ptablecreate: *mut JET_TABLECREATE3_W) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCreateTableColumnIndex3W(sesid : JET_SESID, dbid : u32, ptablecreate : *mut JET_TABLECREATE3_W) -> i32);
    JetCreateTableColumnIndex3W(sesid.param().abi(), dbid, ptablecreate)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateTableColumnIndex4A<P0>(sesid: P0, dbid: u32, ptablecreate: *mut JET_TABLECREATE4_A) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCreateTableColumnIndex4A(sesid : JET_SESID, dbid : u32, ptablecreate : *mut JET_TABLECREATE4_A) -> i32);
    JetCreateTableColumnIndex4A(sesid.param().abi(), dbid, ptablecreate)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateTableColumnIndex4W<P0>(sesid: P0, dbid: u32, ptablecreate: *mut JET_TABLECREATE4_W) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCreateTableColumnIndex4W(sesid : JET_SESID, dbid : u32, ptablecreate : *mut JET_TABLECREATE4_W) -> i32);
    JetCreateTableColumnIndex4W(sesid.param().abi(), dbid, ptablecreate)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateTableColumnIndexA<P0>(sesid: P0, dbid: u32, ptablecreate: *mut JET_TABLECREATE_A) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCreateTableColumnIndexA(sesid : JET_SESID, dbid : u32, ptablecreate : *mut JET_TABLECREATE_A) -> i32);
    JetCreateTableColumnIndexA(sesid.param().abi(), dbid, ptablecreate)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateTableColumnIndexW<P0>(sesid: P0, dbid: u32, ptablecreate: *mut JET_TABLECREATE_W) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCreateTableColumnIndexW(sesid : JET_SESID, dbid : u32, ptablecreate : *mut JET_TABLECREATE_W) -> i32);
    JetCreateTableColumnIndexW(sesid.param().abi(), dbid, ptablecreate)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateTableW<P0>(sesid: P0, dbid: u32, sztablename: *const u16, lpages: u32, ldensity: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetCreateTableW(sesid : JET_SESID, dbid : u32, sztablename : *const u16, lpages : u32, ldensity : u32, ptableid : *mut super::StructuredStorage:: JET_TABLEID) -> i32);
    JetCreateTableW(sesid.param().abi(), dbid, sztablename, lpages, ldensity, ptableid)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDefragment2A<P0>(sesid: P0, dbid: u32, sztablename: Option<*const i8>, pcpasses: Option<*mut u32>, pcseconds: Option<*mut u32>, callback: JET_CALLBACK, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetDefragment2A(sesid : JET_SESID, dbid : u32, sztablename : *const i8, pcpasses : *mut u32, pcseconds : *mut u32, callback : JET_CALLBACK, grbit : u32) -> i32);
    JetDefragment2A(sesid.param().abi(), dbid, core::mem::transmute(sztablename.unwrap_or(std::ptr::null())), core::mem::transmute(pcpasses.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcseconds.unwrap_or(std::ptr::null_mut())), callback, grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDefragment2W<P0>(sesid: P0, dbid: u32, sztablename: Option<*const u16>, pcpasses: Option<*mut u32>, pcseconds: Option<*mut u32>, callback: JET_CALLBACK, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetDefragment2W(sesid : JET_SESID, dbid : u32, sztablename : *const u16, pcpasses : *mut u32, pcseconds : *mut u32, callback : JET_CALLBACK, grbit : u32) -> i32);
    JetDefragment2W(sesid.param().abi(), dbid, core::mem::transmute(sztablename.unwrap_or(std::ptr::null())), core::mem::transmute(pcpasses.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcseconds.unwrap_or(std::ptr::null_mut())), callback, grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDefragment3A<P0>(sesid: P0, szdatabasename: *const i8, sztablename: Option<*const i8>, pcpasses: Option<*mut u32>, pcseconds: Option<*mut u32>, callback: JET_CALLBACK, pvcontext: *const core::ffi::c_void, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetDefragment3A(sesid : JET_SESID, szdatabasename : *const i8, sztablename : *const i8, pcpasses : *mut u32, pcseconds : *mut u32, callback : JET_CALLBACK, pvcontext : *const core::ffi::c_void, grbit : u32) -> i32);
    JetDefragment3A(sesid.param().abi(), szdatabasename, core::mem::transmute(sztablename.unwrap_or(std::ptr::null())), core::mem::transmute(pcpasses.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcseconds.unwrap_or(std::ptr::null_mut())), callback, pvcontext, grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDefragment3W<P0>(sesid: P0, szdatabasename: *const u16, sztablename: Option<*const u16>, pcpasses: Option<*mut u32>, pcseconds: Option<*mut u32>, callback: JET_CALLBACK, pvcontext: *const core::ffi::c_void, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetDefragment3W(sesid : JET_SESID, szdatabasename : *const u16, sztablename : *const u16, pcpasses : *mut u32, pcseconds : *mut u32, callback : JET_CALLBACK, pvcontext : *const core::ffi::c_void, grbit : u32) -> i32);
    JetDefragment3W(sesid.param().abi(), szdatabasename, core::mem::transmute(sztablename.unwrap_or(std::ptr::null())), core::mem::transmute(pcpasses.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcseconds.unwrap_or(std::ptr::null_mut())), callback, pvcontext, grbit)
}
#[inline]
pub unsafe fn JetDefragmentA<P0>(sesid: P0, dbid: u32, sztablename: Option<*const i8>, pcpasses: Option<*mut u32>, pcseconds: Option<*mut u32>, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetDefragmentA(sesid : JET_SESID, dbid : u32, sztablename : *const i8, pcpasses : *mut u32, pcseconds : *mut u32, grbit : u32) -> i32);
    JetDefragmentA(sesid.param().abi(), dbid, core::mem::transmute(sztablename.unwrap_or(std::ptr::null())), core::mem::transmute(pcpasses.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcseconds.unwrap_or(std::ptr::null_mut())), grbit)
}
#[inline]
pub unsafe fn JetDefragmentW<P0>(sesid: P0, dbid: u32, sztablename: Option<*const u16>, pcpasses: Option<*mut u32>, pcseconds: Option<*mut u32>, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetDefragmentW(sesid : JET_SESID, dbid : u32, sztablename : *const u16, pcpasses : *mut u32, pcseconds : *mut u32, grbit : u32) -> i32);
    JetDefragmentW(sesid.param().abi(), dbid, core::mem::transmute(sztablename.unwrap_or(std::ptr::null())), core::mem::transmute(pcpasses.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcseconds.unwrap_or(std::ptr::null_mut())), grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDelete<P0, P1>(sesid: P0, tableid: P1) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetDelete(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID) -> i32);
    JetDelete(sesid.param().abi(), tableid.param().abi())
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDeleteColumn2A<P0, P1>(sesid: P0, tableid: P1, szcolumnname: *const i8, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetDeleteColumn2A(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, szcolumnname : *const i8, grbit : u32) -> i32);
    JetDeleteColumn2A(sesid.param().abi(), tableid.param().abi(), szcolumnname, grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDeleteColumn2W<P0, P1>(sesid: P0, tableid: P1, szcolumnname: *const u16, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetDeleteColumn2W(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, szcolumnname : *const u16, grbit : u32) -> i32);
    JetDeleteColumn2W(sesid.param().abi(), tableid.param().abi(), szcolumnname, grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDeleteColumnA<P0, P1>(sesid: P0, tableid: P1, szcolumnname: *const i8) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetDeleteColumnA(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, szcolumnname : *const i8) -> i32);
    JetDeleteColumnA(sesid.param().abi(), tableid.param().abi(), szcolumnname)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDeleteColumnW<P0, P1>(sesid: P0, tableid: P1, szcolumnname: *const u16) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetDeleteColumnW(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, szcolumnname : *const u16) -> i32);
    JetDeleteColumnW(sesid.param().abi(), tableid.param().abi(), szcolumnname)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDeleteIndexA<P0, P1>(sesid: P0, tableid: P1, szindexname: *const i8) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetDeleteIndexA(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, szindexname : *const i8) -> i32);
    JetDeleteIndexA(sesid.param().abi(), tableid.param().abi(), szindexname)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDeleteIndexW<P0, P1>(sesid: P0, tableid: P1, szindexname: *const u16) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetDeleteIndexW(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, szindexname : *const u16) -> i32);
    JetDeleteIndexW(sesid.param().abi(), tableid.param().abi(), szindexname)
}
#[inline]
pub unsafe fn JetDeleteTableA<P0>(sesid: P0, dbid: u32, sztablename: *const i8) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetDeleteTableA(sesid : JET_SESID, dbid : u32, sztablename : *const i8) -> i32);
    JetDeleteTableA(sesid.param().abi(), dbid, sztablename)
}
#[inline]
pub unsafe fn JetDeleteTableW<P0>(sesid: P0, dbid: u32, sztablename: *const u16) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetDeleteTableW(sesid : JET_SESID, dbid : u32, sztablename : *const u16) -> i32);
    JetDeleteTableW(sesid.param().abi(), dbid, sztablename)
}
#[inline]
pub unsafe fn JetDetachDatabase2A<P0>(sesid: P0, szfilename: Option<*const i8>, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetDetachDatabase2A(sesid : JET_SESID, szfilename : *const i8, grbit : u32) -> i32);
    JetDetachDatabase2A(sesid.param().abi(), core::mem::transmute(szfilename.unwrap_or(std::ptr::null())), grbit)
}
#[inline]
pub unsafe fn JetDetachDatabase2W<P0>(sesid: P0, szfilename: Option<*const u16>, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetDetachDatabase2W(sesid : JET_SESID, szfilename : *const u16, grbit : u32) -> i32);
    JetDetachDatabase2W(sesid.param().abi(), core::mem::transmute(szfilename.unwrap_or(std::ptr::null())), grbit)
}
#[inline]
pub unsafe fn JetDetachDatabaseA<P0>(sesid: P0, szfilename: Option<*const i8>) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetDetachDatabaseA(sesid : JET_SESID, szfilename : *const i8) -> i32);
    JetDetachDatabaseA(sesid.param().abi(), core::mem::transmute(szfilename.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn JetDetachDatabaseW<P0>(sesid: P0, szfilename: Option<*const u16>) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetDetachDatabaseW(sesid : JET_SESID, szfilename : *const u16) -> i32);
    JetDetachDatabaseW(sesid.param().abi(), core::mem::transmute(szfilename.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDupCursor<P0, P1>(sesid: P0, tableid: P1, ptableid: *mut super::StructuredStorage::JET_TABLEID, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetDupCursor(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, ptableid : *mut super::StructuredStorage:: JET_TABLEID, grbit : u32) -> i32);
    JetDupCursor(sesid.param().abi(), tableid.param().abi(), ptableid, grbit)
}
#[inline]
pub unsafe fn JetDupSession<P0>(sesid: P0, psesid: *mut JET_SESID) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetDupSession(sesid : JET_SESID, psesid : *mut JET_SESID) -> i32);
    JetDupSession(sesid.param().abi(), psesid)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetEnableMultiInstanceA(psetsysparam: Option<&[JET_SETSYSPARAM_A]>, pcsetsucceed: Option<*mut u32>) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetEnableMultiInstanceA(psetsysparam : *const JET_SETSYSPARAM_A, csetsysparam : u32, pcsetsucceed : *mut u32) -> i32);
    JetEnableMultiInstanceA(core::mem::transmute(psetsysparam.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), psetsysparam.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcsetsucceed.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetEnableMultiInstanceW(psetsysparam: Option<&[JET_SETSYSPARAM_W]>, pcsetsucceed: Option<*mut u32>) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetEnableMultiInstanceW(psetsysparam : *const JET_SETSYSPARAM_W, csetsysparam : u32, pcsetsucceed : *mut u32) -> i32);
    JetEnableMultiInstanceW(core::mem::transmute(psetsysparam.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), psetsysparam.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcsetsucceed.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn JetEndExternalBackup() -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetEndExternalBackup() -> i32);
    JetEndExternalBackup()
}
#[inline]
pub unsafe fn JetEndExternalBackupInstance<P0>(instance: P0) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetEndExternalBackupInstance(instance : JET_INSTANCE) -> i32);
    JetEndExternalBackupInstance(instance.param().abi())
}
#[inline]
pub unsafe fn JetEndExternalBackupInstance2<P0>(instance: P0, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetEndExternalBackupInstance2(instance : JET_INSTANCE, grbit : u32) -> i32);
    JetEndExternalBackupInstance2(instance.param().abi(), grbit)
}
#[inline]
pub unsafe fn JetEndSession<P0>(sesid: P0, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetEndSession(sesid : JET_SESID, grbit : u32) -> i32);
    JetEndSession(sesid.param().abi(), grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetEnumerateColumns<P0, P1>(sesid: P0, tableid: P1, rgenumcolumnid: Option<&[JET_ENUMCOLUMNID]>, pcenumcolumn: *mut u32, prgenumcolumn: *mut *mut JET_ENUMCOLUMN, pfnrealloc: JET_PFNREALLOC, pvrealloccontext: Option<*const core::ffi::c_void>, cbdatamost: u32, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetEnumerateColumns(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, cenumcolumnid : u32, rgenumcolumnid : *const JET_ENUMCOLUMNID, pcenumcolumn : *mut u32, prgenumcolumn : *mut *mut JET_ENUMCOLUMN, pfnrealloc : JET_PFNREALLOC, pvrealloccontext : *const core::ffi::c_void, cbdatamost : u32, grbit : u32) -> i32);
    JetEnumerateColumns(sesid.param().abi(), tableid.param().abi(), rgenumcolumnid.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgenumcolumnid.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pcenumcolumn, prgenumcolumn, pfnrealloc, core::mem::transmute(pvrealloccontext.unwrap_or(std::ptr::null())), cbdatamost, grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetEscrowUpdate<P0, P1>(sesid: P0, tableid: P1, columnid: u32, pv: *const core::ffi::c_void, cbmax: u32, pvold: Option<*mut core::ffi::c_void>, cboldmax: u32, pcboldactual: Option<*mut u32>, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetEscrowUpdate(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, columnid : u32, pv : *const core::ffi::c_void, cbmax : u32, pvold : *mut core::ffi::c_void, cboldmax : u32, pcboldactual : *mut u32, grbit : u32) -> i32);
    JetEscrowUpdate(sesid.param().abi(), tableid.param().abi(), columnid, pv, cbmax, core::mem::transmute(pvold.unwrap_or(std::ptr::null_mut())), cboldmax, core::mem::transmute(pcboldactual.unwrap_or(std::ptr::null_mut())), grbit)
}
#[inline]
pub unsafe fn JetExternalRestore2A(szcheckpointfilepath: *const i8, szlogpath: *const i8, rgrstmap: Option<&[JET_RSTMAP_A]>, szbackuplogpath: *const i8, ploginfo: *mut JET_LOGINFO_A, sztargetinstancename: Option<*const i8>, sztargetinstancelogpath: Option<*const i8>, sztargetinstancecheckpointpath: Option<*const i8>, pfn: JET_PFNSTATUS) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetExternalRestore2A(szcheckpointfilepath : *const i8, szlogpath : *const i8, rgrstmap : *const JET_RSTMAP_A, crstfilemap : i32, szbackuplogpath : *const i8, ploginfo : *mut JET_LOGINFO_A, sztargetinstancename : *const i8, sztargetinstancelogpath : *const i8, sztargetinstancecheckpointpath : *const i8, pfn : JET_PFNSTATUS) -> i32);
    JetExternalRestore2A(szcheckpointfilepath, szlogpath, core::mem::transmute(rgrstmap.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), rgrstmap.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), szbackuplogpath, ploginfo, core::mem::transmute(sztargetinstancename.unwrap_or(std::ptr::null())), core::mem::transmute(sztargetinstancelogpath.unwrap_or(std::ptr::null())), core::mem::transmute(sztargetinstancecheckpointpath.unwrap_or(std::ptr::null())), pfn)
}
#[inline]
pub unsafe fn JetExternalRestore2W(szcheckpointfilepath: *const u16, szlogpath: *const u16, rgrstmap: Option<&[JET_RSTMAP_W]>, szbackuplogpath: *const u16, ploginfo: *mut JET_LOGINFO_W, sztargetinstancename: Option<*const u16>, sztargetinstancelogpath: Option<*const u16>, sztargetinstancecheckpointpath: Option<*const u16>, pfn: JET_PFNSTATUS) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetExternalRestore2W(szcheckpointfilepath : *const u16, szlogpath : *const u16, rgrstmap : *const JET_RSTMAP_W, crstfilemap : i32, szbackuplogpath : *const u16, ploginfo : *mut JET_LOGINFO_W, sztargetinstancename : *const u16, sztargetinstancelogpath : *const u16, sztargetinstancecheckpointpath : *const u16, pfn : JET_PFNSTATUS) -> i32);
    JetExternalRestore2W(szcheckpointfilepath, szlogpath, core::mem::transmute(rgrstmap.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), rgrstmap.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), szbackuplogpath, ploginfo, core::mem::transmute(sztargetinstancename.unwrap_or(std::ptr::null())), core::mem::transmute(sztargetinstancelogpath.unwrap_or(std::ptr::null())), core::mem::transmute(sztargetinstancecheckpointpath.unwrap_or(std::ptr::null())), pfn)
}
#[inline]
pub unsafe fn JetExternalRestoreA(szcheckpointfilepath: *const i8, szlogpath: *const i8, rgrstmap: Option<&[JET_RSTMAP_A]>, szbackuplogpath: *const i8, genlow: i32, genhigh: i32, pfn: JET_PFNSTATUS) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetExternalRestoreA(szcheckpointfilepath : *const i8, szlogpath : *const i8, rgrstmap : *const JET_RSTMAP_A, crstfilemap : i32, szbackuplogpath : *const i8, genlow : i32, genhigh : i32, pfn : JET_PFNSTATUS) -> i32);
    JetExternalRestoreA(szcheckpointfilepath, szlogpath, core::mem::transmute(rgrstmap.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), rgrstmap.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), szbackuplogpath, genlow, genhigh, pfn)
}
#[inline]
pub unsafe fn JetExternalRestoreW(szcheckpointfilepath: *const u16, szlogpath: *const u16, rgrstmap: Option<&[JET_RSTMAP_W]>, szbackuplogpath: *const u16, genlow: i32, genhigh: i32, pfn: JET_PFNSTATUS) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetExternalRestoreW(szcheckpointfilepath : *const u16, szlogpath : *const u16, rgrstmap : *const JET_RSTMAP_W, crstfilemap : i32, szbackuplogpath : *const u16, genlow : i32, genhigh : i32, pfn : JET_PFNSTATUS) -> i32);
    JetExternalRestoreW(szcheckpointfilepath, szlogpath, core::mem::transmute(rgrstmap.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), rgrstmap.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), szbackuplogpath, genlow, genhigh, pfn)
}
#[inline]
pub unsafe fn JetFreeBuffer<P0>(pbbuf: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("esent.dll" "system" fn JetFreeBuffer(pbbuf : windows_core::PCSTR) -> i32);
    JetFreeBuffer(pbbuf.param().abi())
}
#[inline]
pub unsafe fn JetGetAttachInfoA(szzdatabases: Option<&mut [u8]>, pcbactual: Option<*mut u32>) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetGetAttachInfoA(szzdatabases : *mut i8, cbmax : u32, pcbactual : *mut u32) -> i32);
    JetGetAttachInfoA(core::mem::transmute(szzdatabases.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szzdatabases.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcbactual.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn JetGetAttachInfoInstanceA<P0>(instance: P0, szzdatabases: Option<&mut [u8]>, pcbactual: Option<*mut u32>) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetAttachInfoInstanceA(instance : JET_INSTANCE, szzdatabases : *mut i8, cbmax : u32, pcbactual : *mut u32) -> i32);
    JetGetAttachInfoInstanceA(instance.param().abi(), core::mem::transmute(szzdatabases.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szzdatabases.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcbactual.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn JetGetAttachInfoInstanceW<P0>(instance: P0, szzdatabases: Option<*mut u16>, cbmax: u32, pcbactual: Option<*mut u32>) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetAttachInfoInstanceW(instance : JET_INSTANCE, szzdatabases : *mut u16, cbmax : u32, pcbactual : *mut u32) -> i32);
    JetGetAttachInfoInstanceW(instance.param().abi(), core::mem::transmute(szzdatabases.unwrap_or(std::ptr::null_mut())), cbmax, core::mem::transmute(pcbactual.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn JetGetAttachInfoW(wszzdatabases: Option<*mut u16>, cbmax: u32, pcbactual: Option<*mut u32>) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetGetAttachInfoW(wszzdatabases : *mut u16, cbmax : u32, pcbactual : *mut u32) -> i32);
    JetGetAttachInfoW(core::mem::transmute(wszzdatabases.unwrap_or(std::ptr::null_mut())), cbmax, core::mem::transmute(pcbactual.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetBookmark<P0, P1>(sesid: P0, tableid: P1, pvbookmark: Option<*mut core::ffi::c_void>, cbmax: u32, pcbactual: Option<*mut u32>) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetBookmark(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, pvbookmark : *mut core::ffi::c_void, cbmax : u32, pcbactual : *mut u32) -> i32);
    JetGetBookmark(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(pvbookmark.unwrap_or(std::ptr::null_mut())), cbmax, core::mem::transmute(pcbactual.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn JetGetColumnInfoA<P0>(sesid: P0, dbid: u32, sztablename: *const i8, pcolumnnameorid: Option<*const i8>, pvresult: *mut core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetColumnInfoA(sesid : JET_SESID, dbid : u32, sztablename : *const i8, pcolumnnameorid : *const i8, pvresult : *mut core::ffi::c_void, cbmax : u32, infolevel : u32) -> i32);
    JetGetColumnInfoA(sesid.param().abi(), dbid, sztablename, core::mem::transmute(pcolumnnameorid.unwrap_or(std::ptr::null())), pvresult, cbmax, infolevel)
}
#[inline]
pub unsafe fn JetGetColumnInfoW<P0>(sesid: P0, dbid: u32, sztablename: *const u16, pwcolumnnameorid: Option<*const u16>, pvresult: *mut core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetColumnInfoW(sesid : JET_SESID, dbid : u32, sztablename : *const u16, pwcolumnnameorid : *const u16, pvresult : *mut core::ffi::c_void, cbmax : u32, infolevel : u32) -> i32);
    JetGetColumnInfoW(sesid.param().abi(), dbid, sztablename, core::mem::transmute(pwcolumnnameorid.unwrap_or(std::ptr::null())), pvresult, cbmax, infolevel)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetCurrentIndexA<P0, P1>(sesid: P0, tableid: P1, szindexname: &mut [u8]) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetCurrentIndexA(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, szindexname : *mut i8, cbindexname : u32) -> i32);
    JetGetCurrentIndexA(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(szindexname.as_ptr()), szindexname.len().try_into().unwrap())
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetCurrentIndexW<P0, P1>(sesid: P0, tableid: P1, szindexname: *mut u16, cbindexname: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetCurrentIndexW(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, szindexname : *mut u16, cbindexname : u32) -> i32);
    JetGetCurrentIndexW(sesid.param().abi(), tableid.param().abi(), szindexname, cbindexname)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetCursorInfo<P0, P1>(sesid: P0, tableid: P1, pvresult: *mut core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetCursorInfo(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, pvresult : *mut core::ffi::c_void, cbmax : u32, infolevel : u32) -> i32);
    JetGetCursorInfo(sesid.param().abi(), tableid.param().abi(), pvresult, cbmax, infolevel)
}
#[inline]
pub unsafe fn JetGetDatabaseFileInfoA(szdatabasename: *const i8, pvresult: *mut core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetGetDatabaseFileInfoA(szdatabasename : *const i8, pvresult : *mut core::ffi::c_void, cbmax : u32, infolevel : u32) -> i32);
    JetGetDatabaseFileInfoA(szdatabasename, pvresult, cbmax, infolevel)
}
#[inline]
pub unsafe fn JetGetDatabaseFileInfoW(szdatabasename: *const u16, pvresult: *mut core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetGetDatabaseFileInfoW(szdatabasename : *const u16, pvresult : *mut core::ffi::c_void, cbmax : u32, infolevel : u32) -> i32);
    JetGetDatabaseFileInfoW(szdatabasename, pvresult, cbmax, infolevel)
}
#[inline]
pub unsafe fn JetGetDatabaseInfoA<P0>(sesid: P0, dbid: u32, pvresult: *mut core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetDatabaseInfoA(sesid : JET_SESID, dbid : u32, pvresult : *mut core::ffi::c_void, cbmax : u32, infolevel : u32) -> i32);
    JetGetDatabaseInfoA(sesid.param().abi(), dbid, pvresult, cbmax, infolevel)
}
#[inline]
pub unsafe fn JetGetDatabaseInfoW<P0>(sesid: P0, dbid: u32, pvresult: *mut core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetDatabaseInfoW(sesid : JET_SESID, dbid : u32, pvresult : *mut core::ffi::c_void, cbmax : u32, infolevel : u32) -> i32);
    JetGetDatabaseInfoW(sesid.param().abi(), dbid, pvresult, cbmax, infolevel)
}
#[inline]
pub unsafe fn JetGetErrorInfoW(pvcontext: Option<*const core::ffi::c_void>, pvresult: *mut core::ffi::c_void, cbmax: u32, infolevel: u32, grbit: u32) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetGetErrorInfoW(pvcontext : *const core::ffi::c_void, pvresult : *mut core::ffi::c_void, cbmax : u32, infolevel : u32, grbit : u32) -> i32);
    JetGetErrorInfoW(core::mem::transmute(pvcontext.unwrap_or(std::ptr::null())), pvresult, cbmax, infolevel, grbit)
}
#[inline]
pub unsafe fn JetGetIndexInfoA<P0>(sesid: P0, dbid: u32, sztablename: *const i8, szindexname: Option<*const i8>, pvresult: *mut core::ffi::c_void, cbresult: u32, infolevel: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetIndexInfoA(sesid : JET_SESID, dbid : u32, sztablename : *const i8, szindexname : *const i8, pvresult : *mut core::ffi::c_void, cbresult : u32, infolevel : u32) -> i32);
    JetGetIndexInfoA(sesid.param().abi(), dbid, sztablename, core::mem::transmute(szindexname.unwrap_or(std::ptr::null())), pvresult, cbresult, infolevel)
}
#[inline]
pub unsafe fn JetGetIndexInfoW<P0>(sesid: P0, dbid: u32, sztablename: *const u16, szindexname: Option<*const u16>, pvresult: *mut core::ffi::c_void, cbresult: u32, infolevel: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetIndexInfoW(sesid : JET_SESID, dbid : u32, sztablename : *const u16, szindexname : *const u16, pvresult : *mut core::ffi::c_void, cbresult : u32, infolevel : u32) -> i32);
    JetGetIndexInfoW(sesid.param().abi(), dbid, sztablename, core::mem::transmute(szindexname.unwrap_or(std::ptr::null())), pvresult, cbresult, infolevel)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetInstanceInfoA(pcinstanceinfo: *mut u32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_A) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetGetInstanceInfoA(pcinstanceinfo : *mut u32, painstanceinfo : *mut *mut JET_INSTANCE_INFO_A) -> i32);
    JetGetInstanceInfoA(pcinstanceinfo, painstanceinfo)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetInstanceInfoW(pcinstanceinfo: *mut u32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_W) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetGetInstanceInfoW(pcinstanceinfo : *mut u32, painstanceinfo : *mut *mut JET_INSTANCE_INFO_W) -> i32);
    JetGetInstanceInfoW(pcinstanceinfo, painstanceinfo)
}
#[inline]
pub unsafe fn JetGetInstanceMiscInfo<P0>(instance: P0, pvresult: *mut core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetInstanceMiscInfo(instance : JET_INSTANCE, pvresult : *mut core::ffi::c_void, cbmax : u32, infolevel : u32) -> i32);
    JetGetInstanceMiscInfo(instance.param().abi(), pvresult, cbmax, infolevel)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetLS<P0, P1>(sesid: P0, tableid: P1, pls: *mut JET_LS, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetLS(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, pls : *mut JET_LS, grbit : u32) -> i32);
    JetGetLS(sesid.param().abi(), tableid.param().abi(), pls, grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetLock<P0, P1>(sesid: P0, tableid: P1, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetLock(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, grbit : u32) -> i32);
    JetGetLock(sesid.param().abi(), tableid.param().abi(), grbit)
}
#[inline]
pub unsafe fn JetGetLogInfoA(szzlogs: Option<&mut [u8]>, pcbactual: Option<*mut u32>) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetGetLogInfoA(szzlogs : *mut i8, cbmax : u32, pcbactual : *mut u32) -> i32);
    JetGetLogInfoA(core::mem::transmute(szzlogs.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szzlogs.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcbactual.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn JetGetLogInfoInstance2A<P0>(instance: P0, szzlogs: Option<&mut [u8]>, pcbactual: Option<*mut u32>, ploginfo: Option<*mut JET_LOGINFO_A>) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetLogInfoInstance2A(instance : JET_INSTANCE, szzlogs : *mut i8, cbmax : u32, pcbactual : *mut u32, ploginfo : *mut JET_LOGINFO_A) -> i32);
    JetGetLogInfoInstance2A(instance.param().abi(), core::mem::transmute(szzlogs.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szzlogs.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcbactual.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ploginfo.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn JetGetLogInfoInstance2W<P0>(instance: P0, wszzlogs: Option<*mut u16>, cbmax: u32, pcbactual: Option<*mut u32>, ploginfo: Option<*mut JET_LOGINFO_W>) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetLogInfoInstance2W(instance : JET_INSTANCE, wszzlogs : *mut u16, cbmax : u32, pcbactual : *mut u32, ploginfo : *mut JET_LOGINFO_W) -> i32);
    JetGetLogInfoInstance2W(instance.param().abi(), core::mem::transmute(wszzlogs.unwrap_or(std::ptr::null_mut())), cbmax, core::mem::transmute(pcbactual.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ploginfo.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn JetGetLogInfoInstanceA<P0>(instance: P0, szzlogs: Option<&mut [u8]>, pcbactual: Option<*mut u32>) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetLogInfoInstanceA(instance : JET_INSTANCE, szzlogs : *mut i8, cbmax : u32, pcbactual : *mut u32) -> i32);
    JetGetLogInfoInstanceA(instance.param().abi(), core::mem::transmute(szzlogs.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szzlogs.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcbactual.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn JetGetLogInfoInstanceW<P0>(instance: P0, wszzlogs: Option<*mut u16>, cbmax: u32, pcbactual: Option<*mut u32>) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetLogInfoInstanceW(instance : JET_INSTANCE, wszzlogs : *mut u16, cbmax : u32, pcbactual : *mut u32) -> i32);
    JetGetLogInfoInstanceW(instance.param().abi(), core::mem::transmute(wszzlogs.unwrap_or(std::ptr::null_mut())), cbmax, core::mem::transmute(pcbactual.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn JetGetLogInfoW(szzlogs: Option<*mut u16>, cbmax: u32, pcbactual: Option<*mut u32>) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetGetLogInfoW(szzlogs : *mut u16, cbmax : u32, pcbactual : *mut u32) -> i32);
    JetGetLogInfoW(core::mem::transmute(szzlogs.unwrap_or(std::ptr::null_mut())), cbmax, core::mem::transmute(pcbactual.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn JetGetObjectInfoA<P0>(sesid: P0, dbid: u32, objtyp: u32, szcontainername: Option<*const i8>, szobjectname: Option<*const i8>, pvresult: *mut core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetObjectInfoA(sesid : JET_SESID, dbid : u32, objtyp : u32, szcontainername : *const i8, szobjectname : *const i8, pvresult : *mut core::ffi::c_void, cbmax : u32, infolevel : u32) -> i32);
    JetGetObjectInfoA(sesid.param().abi(), dbid, objtyp, core::mem::transmute(szcontainername.unwrap_or(std::ptr::null())), core::mem::transmute(szobjectname.unwrap_or(std::ptr::null())), pvresult, cbmax, infolevel)
}
#[inline]
pub unsafe fn JetGetObjectInfoW<P0>(sesid: P0, dbid: u32, objtyp: u32, szcontainername: Option<*const u16>, szobjectname: Option<*const u16>, pvresult: *mut core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetObjectInfoW(sesid : JET_SESID, dbid : u32, objtyp : u32, szcontainername : *const u16, szobjectname : *const u16, pvresult : *mut core::ffi::c_void, cbmax : u32, infolevel : u32) -> i32);
    JetGetObjectInfoW(sesid.param().abi(), dbid, objtyp, core::mem::transmute(szcontainername.unwrap_or(std::ptr::null())), core::mem::transmute(szobjectname.unwrap_or(std::ptr::null())), pvresult, cbmax, infolevel)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetRecordPosition<P0, P1>(sesid: P0, tableid: P1, precpos: *mut JET_RECPOS, cbrecpos: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetRecordPosition(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, precpos : *mut JET_RECPOS, cbrecpos : u32) -> i32);
    JetGetRecordPosition(sesid.param().abi(), tableid.param().abi(), precpos, cbrecpos)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetRecordSize<P0, P1>(sesid: P0, tableid: P1, precsize: *mut JET_RECSIZE, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetRecordSize(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, precsize : *mut JET_RECSIZE, grbit : u32) -> i32);
    JetGetRecordSize(sesid.param().abi(), tableid.param().abi(), precsize, grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetRecordSize2<P0, P1>(sesid: P0, tableid: P1, precsize: *mut JET_RECSIZE2, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetRecordSize2(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, precsize : *mut JET_RECSIZE2, grbit : u32) -> i32);
    JetGetRecordSize2(sesid.param().abi(), tableid.param().abi(), precsize, grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetSecondaryIndexBookmark<P0, P1>(sesid: P0, tableid: P1, pvsecondarykey: Option<*mut core::ffi::c_void>, cbsecondarykeymax: u32, pcbsecondarykeyactual: Option<*mut u32>, pvprimarybookmark: Option<*mut core::ffi::c_void>, cbprimarybookmarkmax: u32, pcbprimarybookmarkactual: Option<*mut u32>, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetSecondaryIndexBookmark(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, pvsecondarykey : *mut core::ffi::c_void, cbsecondarykeymax : u32, pcbsecondarykeyactual : *mut u32, pvprimarybookmark : *mut core::ffi::c_void, cbprimarybookmarkmax : u32, pcbprimarybookmarkactual : *mut u32, grbit : u32) -> i32);
    JetGetSecondaryIndexBookmark(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(pvsecondarykey.unwrap_or(std::ptr::null_mut())), cbsecondarykeymax, core::mem::transmute(pcbsecondarykeyactual.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pvprimarybookmark.unwrap_or(std::ptr::null_mut())), cbprimarybookmarkmax, core::mem::transmute(pcbprimarybookmarkactual.unwrap_or(std::ptr::null_mut())), grbit)
}
#[inline]
pub unsafe fn JetGetSessionParameter<P0>(sesid: P0, sesparamid: u32, pvparam: &mut [u8], pcbparamactual: Option<*mut u32>) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetSessionParameter(sesid : JET_SESID, sesparamid : u32, pvparam : *mut core::ffi::c_void, cbparammax : u32, pcbparamactual : *mut u32) -> i32);
    JetGetSessionParameter(sesid.param().abi(), sesparamid, core::mem::transmute(pvparam.as_ptr()), pvparam.len().try_into().unwrap(), core::mem::transmute(pcbparamactual.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetSystemParameterA<P0, P1>(instance: P0, sesid: P1, paramid: u32, plparam: Option<*mut super::StructuredStorage::JET_API_PTR>, szparam: Option<&mut [u8]>) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
    P1: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetSystemParameterA(instance : JET_INSTANCE, sesid : JET_SESID, paramid : u32, plparam : *mut super::StructuredStorage:: JET_API_PTR, szparam : *mut i8, cbmax : u32) -> i32);
    JetGetSystemParameterA(instance.param().abi(), sesid.param().abi(), paramid, core::mem::transmute(plparam.unwrap_or(std::ptr::null_mut())), core::mem::transmute(szparam.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szparam.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetSystemParameterW<P0, P1>(instance: P0, sesid: P1, paramid: u32, plparam: Option<*mut super::StructuredStorage::JET_API_PTR>, szparam: Option<*mut u16>, cbmax: u32) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
    P1: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetSystemParameterW(instance : JET_INSTANCE, sesid : JET_SESID, paramid : u32, plparam : *mut super::StructuredStorage:: JET_API_PTR, szparam : *mut u16, cbmax : u32) -> i32);
    JetGetSystemParameterW(instance.param().abi(), sesid.param().abi(), paramid, core::mem::transmute(plparam.unwrap_or(std::ptr::null_mut())), core::mem::transmute(szparam.unwrap_or(std::ptr::null_mut())), cbmax)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetTableColumnInfoA<P0, P1>(sesid: P0, tableid: P1, szcolumnname: Option<*const i8>, pvresult: *mut core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetTableColumnInfoA(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, szcolumnname : *const i8, pvresult : *mut core::ffi::c_void, cbmax : u32, infolevel : u32) -> i32);
    JetGetTableColumnInfoA(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(szcolumnname.unwrap_or(std::ptr::null())), pvresult, cbmax, infolevel)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetTableColumnInfoW<P0, P1>(sesid: P0, tableid: P1, szcolumnname: Option<*const u16>, pvresult: *mut core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetTableColumnInfoW(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, szcolumnname : *const u16, pvresult : *mut core::ffi::c_void, cbmax : u32, infolevel : u32) -> i32);
    JetGetTableColumnInfoW(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(szcolumnname.unwrap_or(std::ptr::null())), pvresult, cbmax, infolevel)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetTableIndexInfoA<P0, P1>(sesid: P0, tableid: P1, szindexname: Option<*const i8>, pvresult: *mut core::ffi::c_void, cbresult: u32, infolevel: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetTableIndexInfoA(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, szindexname : *const i8, pvresult : *mut core::ffi::c_void, cbresult : u32, infolevel : u32) -> i32);
    JetGetTableIndexInfoA(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(szindexname.unwrap_or(std::ptr::null())), pvresult, cbresult, infolevel)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetTableIndexInfoW<P0, P1>(sesid: P0, tableid: P1, szindexname: Option<*const u16>, pvresult: *mut core::ffi::c_void, cbresult: u32, infolevel: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetTableIndexInfoW(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, szindexname : *const u16, pvresult : *mut core::ffi::c_void, cbresult : u32, infolevel : u32) -> i32);
    JetGetTableIndexInfoW(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(szindexname.unwrap_or(std::ptr::null())), pvresult, cbresult, infolevel)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetTableInfoA<P0, P1>(sesid: P0, tableid: P1, pvresult: *mut core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetTableInfoA(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, pvresult : *mut core::ffi::c_void, cbmax : u32, infolevel : u32) -> i32);
    JetGetTableInfoA(sesid.param().abi(), tableid.param().abi(), pvresult, cbmax, infolevel)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetTableInfoW<P0, P1>(sesid: P0, tableid: P1, pvresult: *mut core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetTableInfoW(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, pvresult : *mut core::ffi::c_void, cbmax : u32, infolevel : u32) -> i32);
    JetGetTableInfoW(sesid.param().abi(), tableid.param().abi(), pvresult, cbmax, infolevel)
}
#[inline]
pub unsafe fn JetGetThreadStats(pvresult: *mut core::ffi::c_void, cbmax: u32) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetGetThreadStats(pvresult : *mut core::ffi::c_void, cbmax : u32) -> i32);
    JetGetThreadStats(pvresult, cbmax)
}
#[inline]
pub unsafe fn JetGetTruncateLogInfoInstanceA<P0>(instance: P0, szzlogs: Option<&mut [u8]>, pcbactual: Option<*mut u32>) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetTruncateLogInfoInstanceA(instance : JET_INSTANCE, szzlogs : *mut i8, cbmax : u32, pcbactual : *mut u32) -> i32);
    JetGetTruncateLogInfoInstanceA(instance.param().abi(), core::mem::transmute(szzlogs.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), szzlogs.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcbactual.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn JetGetTruncateLogInfoInstanceW<P0>(instance: P0, wszzlogs: Option<*mut u16>, cbmax: u32, pcbactual: Option<*mut u32>) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetTruncateLogInfoInstanceW(instance : JET_INSTANCE, wszzlogs : *mut u16, cbmax : u32, pcbactual : *mut u32) -> i32);
    JetGetTruncateLogInfoInstanceW(instance.param().abi(), core::mem::transmute(wszzlogs.unwrap_or(std::ptr::null_mut())), cbmax, core::mem::transmute(pcbactual.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn JetGetVersion<P0>(sesid: P0, pwversion: *mut u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGetVersion(sesid : JET_SESID, pwversion : *mut u32) -> i32);
    JetGetVersion(sesid.param().abi(), pwversion)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGotoBookmark<P0, P1>(sesid: P0, tableid: P1, pvbookmark: *const core::ffi::c_void, cbbookmark: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGotoBookmark(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, pvbookmark : *const core::ffi::c_void, cbbookmark : u32) -> i32);
    JetGotoBookmark(sesid.param().abi(), tableid.param().abi(), pvbookmark, cbbookmark)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGotoPosition<P0, P1>(sesid: P0, tableid: P1, precpos: *const JET_RECPOS) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGotoPosition(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, precpos : *const JET_RECPOS) -> i32);
    JetGotoPosition(sesid.param().abi(), tableid.param().abi(), precpos)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGotoSecondaryIndexBookmark<P0, P1>(sesid: P0, tableid: P1, pvsecondarykey: *const core::ffi::c_void, cbsecondarykey: u32, pvprimarybookmark: Option<*const core::ffi::c_void>, cbprimarybookmark: u32, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGotoSecondaryIndexBookmark(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, pvsecondarykey : *const core::ffi::c_void, cbsecondarykey : u32, pvprimarybookmark : *const core::ffi::c_void, cbprimarybookmark : u32, grbit : u32) -> i32);
    JetGotoSecondaryIndexBookmark(sesid.param().abi(), tableid.param().abi(), pvsecondarykey, cbsecondarykey, core::mem::transmute(pvprimarybookmark.unwrap_or(std::ptr::null())), cbprimarybookmark, grbit)
}
#[inline]
pub unsafe fn JetGrowDatabase<P0>(sesid: P0, dbid: u32, cpg: u32, pcpgreal: *const u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetGrowDatabase(sesid : JET_SESID, dbid : u32, cpg : u32, pcpgreal : *const u32) -> i32);
    JetGrowDatabase(sesid.param().abi(), dbid, cpg, pcpgreal)
}
#[inline]
pub unsafe fn JetIdle<P0>(sesid: P0, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetIdle(sesid : JET_SESID, grbit : u32) -> i32);
    JetIdle(sesid.param().abi(), grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetIndexRecordCount<P0, P1>(sesid: P0, tableid: P1, pcrec: *mut u32, crecmax: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetIndexRecordCount(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, pcrec : *mut u32, crecmax : u32) -> i32);
    JetIndexRecordCount(sesid.param().abi(), tableid.param().abi(), pcrec, crecmax)
}
#[inline]
pub unsafe fn JetInit(pinstance: Option<*mut JET_INSTANCE>) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetInit(pinstance : *mut JET_INSTANCE) -> i32);
    JetInit(core::mem::transmute(pinstance.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn JetInit2(pinstance: Option<*mut JET_INSTANCE>, grbit: u32) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetInit2(pinstance : *mut JET_INSTANCE, grbit : u32) -> i32);
    JetInit2(core::mem::transmute(pinstance.unwrap_or(std::ptr::null_mut())), grbit)
}
#[inline]
pub unsafe fn JetInit3A(pinstance: Option<*mut JET_INSTANCE>, prstinfo: Option<*const JET_RSTINFO_A>, grbit: u32) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetInit3A(pinstance : *mut JET_INSTANCE, prstinfo : *const JET_RSTINFO_A, grbit : u32) -> i32);
    JetInit3A(core::mem::transmute(pinstance.unwrap_or(std::ptr::null_mut())), core::mem::transmute(prstinfo.unwrap_or(std::ptr::null())), grbit)
}
#[inline]
pub unsafe fn JetInit3W(pinstance: Option<*mut JET_INSTANCE>, prstinfo: Option<*const JET_RSTINFO_W>, grbit: u32) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetInit3W(pinstance : *mut JET_INSTANCE, prstinfo : *const JET_RSTINFO_W, grbit : u32) -> i32);
    JetInit3W(core::mem::transmute(pinstance.unwrap_or(std::ptr::null_mut())), core::mem::transmute(prstinfo.unwrap_or(std::ptr::null())), grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetIntersectIndexes<P0>(sesid: P0, rgindexrange: &[JET_INDEXRANGE], precordlist: *mut JET_RECORDLIST, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetIntersectIndexes(sesid : JET_SESID, rgindexrange : *const JET_INDEXRANGE, cindexrange : u32, precordlist : *mut JET_RECORDLIST, grbit : u32) -> i32);
    JetIntersectIndexes(sesid.param().abi(), core::mem::transmute(rgindexrange.as_ptr()), rgindexrange.len().try_into().unwrap(), precordlist, grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetMakeKey<P0, P1>(sesid: P0, tableid: P1, pvdata: Option<*const core::ffi::c_void>, cbdata: u32, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetMakeKey(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, pvdata : *const core::ffi::c_void, cbdata : u32, grbit : u32) -> i32);
    JetMakeKey(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(pvdata.unwrap_or(std::ptr::null())), cbdata, grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetMove<P0, P1>(sesid: P0, tableid: P1, crow: i32, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetMove(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, crow : i32, grbit : u32) -> i32);
    JetMove(sesid.param().abi(), tableid.param().abi(), crow, grbit)
}
#[inline]
pub unsafe fn JetOSSnapshotAbort<P0>(snapid: P0, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_OSSNAPID>,
{
    windows_targets::link!("esent.dll" "system" fn JetOSSnapshotAbort(snapid : JET_OSSNAPID, grbit : u32) -> i32);
    JetOSSnapshotAbort(snapid.param().abi(), grbit)
}
#[inline]
pub unsafe fn JetOSSnapshotEnd<P0>(snapid: P0, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_OSSNAPID>,
{
    windows_targets::link!("esent.dll" "system" fn JetOSSnapshotEnd(snapid : JET_OSSNAPID, grbit : u32) -> i32);
    JetOSSnapshotEnd(snapid.param().abi(), grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOSSnapshotFreezeA<P0>(snapid: P0, pcinstanceinfo: *mut u32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_A, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_OSSNAPID>,
{
    windows_targets::link!("esent.dll" "system" fn JetOSSnapshotFreezeA(snapid : JET_OSSNAPID, pcinstanceinfo : *mut u32, painstanceinfo : *mut *mut JET_INSTANCE_INFO_A, grbit : u32) -> i32);
    JetOSSnapshotFreezeA(snapid.param().abi(), pcinstanceinfo, painstanceinfo, grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOSSnapshotFreezeW<P0>(snapid: P0, pcinstanceinfo: *mut u32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_W, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_OSSNAPID>,
{
    windows_targets::link!("esent.dll" "system" fn JetOSSnapshotFreezeW(snapid : JET_OSSNAPID, pcinstanceinfo : *mut u32, painstanceinfo : *mut *mut JET_INSTANCE_INFO_W, grbit : u32) -> i32);
    JetOSSnapshotFreezeW(snapid.param().abi(), pcinstanceinfo, painstanceinfo, grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOSSnapshotGetFreezeInfoA<P0>(snapid: P0, pcinstanceinfo: *mut u32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_A, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_OSSNAPID>,
{
    windows_targets::link!("esent.dll" "system" fn JetOSSnapshotGetFreezeInfoA(snapid : JET_OSSNAPID, pcinstanceinfo : *mut u32, painstanceinfo : *mut *mut JET_INSTANCE_INFO_A, grbit : u32) -> i32);
    JetOSSnapshotGetFreezeInfoA(snapid.param().abi(), pcinstanceinfo, painstanceinfo, grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOSSnapshotGetFreezeInfoW<P0>(snapid: P0, pcinstanceinfo: *mut u32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_W, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_OSSNAPID>,
{
    windows_targets::link!("esent.dll" "system" fn JetOSSnapshotGetFreezeInfoW(snapid : JET_OSSNAPID, pcinstanceinfo : *mut u32, painstanceinfo : *mut *mut JET_INSTANCE_INFO_W, grbit : u32) -> i32);
    JetOSSnapshotGetFreezeInfoW(snapid.param().abi(), pcinstanceinfo, painstanceinfo, grbit)
}
#[inline]
pub unsafe fn JetOSSnapshotPrepare(psnapid: *mut JET_OSSNAPID, grbit: u32) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetOSSnapshotPrepare(psnapid : *mut JET_OSSNAPID, grbit : u32) -> i32);
    JetOSSnapshotPrepare(psnapid, grbit)
}
#[inline]
pub unsafe fn JetOSSnapshotPrepareInstance<P0, P1>(snapid: P0, instance: P1, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_OSSNAPID>,
    P1: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetOSSnapshotPrepareInstance(snapid : JET_OSSNAPID, instance : JET_INSTANCE, grbit : u32) -> i32);
    JetOSSnapshotPrepareInstance(snapid.param().abi(), instance.param().abi(), grbit)
}
#[inline]
pub unsafe fn JetOSSnapshotThaw<P0>(snapid: P0, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_OSSNAPID>,
{
    windows_targets::link!("esent.dll" "system" fn JetOSSnapshotThaw(snapid : JET_OSSNAPID, grbit : u32) -> i32);
    JetOSSnapshotThaw(snapid.param().abi(), grbit)
}
#[inline]
pub unsafe fn JetOSSnapshotTruncateLog<P0>(snapid: P0, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_OSSNAPID>,
{
    windows_targets::link!("esent.dll" "system" fn JetOSSnapshotTruncateLog(snapid : JET_OSSNAPID, grbit : u32) -> i32);
    JetOSSnapshotTruncateLog(snapid.param().abi(), grbit)
}
#[inline]
pub unsafe fn JetOSSnapshotTruncateLogInstance<P0, P1>(snapid: P0, instance: P1, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_OSSNAPID>,
    P1: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetOSSnapshotTruncateLogInstance(snapid : JET_OSSNAPID, instance : JET_INSTANCE, grbit : u32) -> i32);
    JetOSSnapshotTruncateLogInstance(snapid.param().abi(), instance.param().abi(), grbit)
}
#[inline]
pub unsafe fn JetOpenDatabaseA<P0>(sesid: P0, szfilename: *const i8, szconnect: Option<*const i8>, pdbid: *mut u32, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetOpenDatabaseA(sesid : JET_SESID, szfilename : *const i8, szconnect : *const i8, pdbid : *mut u32, grbit : u32) -> i32);
    JetOpenDatabaseA(sesid.param().abi(), szfilename, core::mem::transmute(szconnect.unwrap_or(std::ptr::null())), pdbid, grbit)
}
#[inline]
pub unsafe fn JetOpenDatabaseW<P0>(sesid: P0, szfilename: *const u16, szconnect: Option<*const u16>, pdbid: *mut u32, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetOpenDatabaseW(sesid : JET_SESID, szfilename : *const u16, szconnect : *const u16, pdbid : *mut u32, grbit : u32) -> i32);
    JetOpenDatabaseW(sesid.param().abi(), szfilename, core::mem::transmute(szconnect.unwrap_or(std::ptr::null())), pdbid, grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOpenFileA(szfilename: *const i8, phffile: *mut super::StructuredStorage::JET_HANDLE, pulfilesizelow: *mut u32, pulfilesizehigh: *mut u32) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetOpenFileA(szfilename : *const i8, phffile : *mut super::StructuredStorage:: JET_HANDLE, pulfilesizelow : *mut u32, pulfilesizehigh : *mut u32) -> i32);
    JetOpenFileA(szfilename, phffile, pulfilesizelow, pulfilesizehigh)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOpenFileInstanceA<P0>(instance: P0, szfilename: *const i8, phffile: *mut super::StructuredStorage::JET_HANDLE, pulfilesizelow: *mut u32, pulfilesizehigh: *mut u32) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetOpenFileInstanceA(instance : JET_INSTANCE, szfilename : *const i8, phffile : *mut super::StructuredStorage:: JET_HANDLE, pulfilesizelow : *mut u32, pulfilesizehigh : *mut u32) -> i32);
    JetOpenFileInstanceA(instance.param().abi(), szfilename, phffile, pulfilesizelow, pulfilesizehigh)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOpenFileInstanceW<P0>(instance: P0, szfilename: *const u16, phffile: *mut super::StructuredStorage::JET_HANDLE, pulfilesizelow: *mut u32, pulfilesizehigh: *mut u32) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetOpenFileInstanceW(instance : JET_INSTANCE, szfilename : *const u16, phffile : *mut super::StructuredStorage:: JET_HANDLE, pulfilesizelow : *mut u32, pulfilesizehigh : *mut u32) -> i32);
    JetOpenFileInstanceW(instance.param().abi(), szfilename, phffile, pulfilesizelow, pulfilesizehigh)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOpenFileW(szfilename: *const u16, phffile: *mut super::StructuredStorage::JET_HANDLE, pulfilesizelow: *mut u32, pulfilesizehigh: *mut u32) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetOpenFileW(szfilename : *const u16, phffile : *mut super::StructuredStorage:: JET_HANDLE, pulfilesizelow : *mut u32, pulfilesizehigh : *mut u32) -> i32);
    JetOpenFileW(szfilename, phffile, pulfilesizelow, pulfilesizehigh)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOpenTableA<P0>(sesid: P0, dbid: u32, sztablename: *const i8, pvparameters: Option<*const core::ffi::c_void>, cbparameters: u32, grbit: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetOpenTableA(sesid : JET_SESID, dbid : u32, sztablename : *const i8, pvparameters : *const core::ffi::c_void, cbparameters : u32, grbit : u32, ptableid : *mut super::StructuredStorage:: JET_TABLEID) -> i32);
    JetOpenTableA(sesid.param().abi(), dbid, sztablename, core::mem::transmute(pvparameters.unwrap_or(std::ptr::null())), cbparameters, grbit, ptableid)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOpenTableW<P0>(sesid: P0, dbid: u32, sztablename: *const u16, pvparameters: Option<*const core::ffi::c_void>, cbparameters: u32, grbit: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetOpenTableW(sesid : JET_SESID, dbid : u32, sztablename : *const u16, pvparameters : *const core::ffi::c_void, cbparameters : u32, grbit : u32, ptableid : *mut super::StructuredStorage:: JET_TABLEID) -> i32);
    JetOpenTableW(sesid.param().abi(), dbid, sztablename, core::mem::transmute(pvparameters.unwrap_or(std::ptr::null())), cbparameters, grbit, ptableid)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOpenTempTable<P0>(sesid: P0, prgcolumndef: *const JET_COLUMNDEF, ccolumn: u32, grbit: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID, prgcolumnid: *mut u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetOpenTempTable(sesid : JET_SESID, prgcolumndef : *const JET_COLUMNDEF, ccolumn : u32, grbit : u32, ptableid : *mut super::StructuredStorage:: JET_TABLEID, prgcolumnid : *mut u32) -> i32);
    JetOpenTempTable(sesid.param().abi(), prgcolumndef, ccolumn, grbit, ptableid, prgcolumnid)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOpenTempTable2<P0>(sesid: P0, prgcolumndef: *const JET_COLUMNDEF, ccolumn: u32, lcid: u32, grbit: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID, prgcolumnid: *mut u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetOpenTempTable2(sesid : JET_SESID, prgcolumndef : *const JET_COLUMNDEF, ccolumn : u32, lcid : u32, grbit : u32, ptableid : *mut super::StructuredStorage:: JET_TABLEID, prgcolumnid : *mut u32) -> i32);
    JetOpenTempTable2(sesid.param().abi(), prgcolumndef, ccolumn, lcid, grbit, ptableid, prgcolumnid)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOpenTempTable3<P0>(sesid: P0, prgcolumndef: *const JET_COLUMNDEF, ccolumn: u32, pidxunicode: Option<*const JET_UNICODEINDEX>, grbit: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID, prgcolumnid: *mut u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetOpenTempTable3(sesid : JET_SESID, prgcolumndef : *const JET_COLUMNDEF, ccolumn : u32, pidxunicode : *const JET_UNICODEINDEX, grbit : u32, ptableid : *mut super::StructuredStorage:: JET_TABLEID, prgcolumnid : *mut u32) -> i32);
    JetOpenTempTable3(sesid.param().abi(), prgcolumndef, ccolumn, core::mem::transmute(pidxunicode.unwrap_or(std::ptr::null())), grbit, ptableid, prgcolumnid)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOpenTemporaryTable<P0>(sesid: P0, popentemporarytable: *const JET_OPENTEMPORARYTABLE) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetOpenTemporaryTable(sesid : JET_SESID, popentemporarytable : *const JET_OPENTEMPORARYTABLE) -> i32);
    JetOpenTemporaryTable(sesid.param().abi(), popentemporarytable)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOpenTemporaryTable2<P0>(sesid: P0, popentemporarytable: *const JET_OPENTEMPORARYTABLE2) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetOpenTemporaryTable2(sesid : JET_SESID, popentemporarytable : *const JET_OPENTEMPORARYTABLE2) -> i32);
    JetOpenTemporaryTable2(sesid.param().abi(), popentemporarytable)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetPrepareUpdate<P0, P1>(sesid: P0, tableid: P1, prep: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetPrepareUpdate(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, prep : u32) -> i32);
    JetPrepareUpdate(sesid.param().abi(), tableid.param().abi(), prep)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetPrereadIndexRanges<P0, P1>(sesid: P0, tableid: P1, rgindexranges: &[JET_INDEX_RANGE], pcrangespreread: Option<*mut u32>, rgcolumnidpreread: &[u32], grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetPrereadIndexRanges(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, rgindexranges : *const JET_INDEX_RANGE, cindexranges : u32, pcrangespreread : *mut u32, rgcolumnidpreread : *const u32, ccolumnidpreread : u32, grbit : u32) -> i32);
    JetPrereadIndexRanges(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(rgindexranges.as_ptr()), rgindexranges.len().try_into().unwrap(), core::mem::transmute(pcrangespreread.unwrap_or(std::ptr::null_mut())), core::mem::transmute(rgcolumnidpreread.as_ptr()), rgcolumnidpreread.len().try_into().unwrap(), grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetPrereadKeys<P0, P1>(sesid: P0, tableid: P1, rgpvkeys: *const *const core::ffi::c_void, rgcbkeys: *const u32, ckeys: i32, pckeyspreread: Option<*mut i32>, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetPrereadKeys(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, rgpvkeys : *const *const core::ffi::c_void, rgcbkeys : *const u32, ckeys : i32, pckeyspreread : *mut i32, grbit : u32) -> i32);
    JetPrereadKeys(sesid.param().abi(), tableid.param().abi(), rgpvkeys, rgcbkeys, ckeys, core::mem::transmute(pckeyspreread.unwrap_or(std::ptr::null_mut())), grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetReadFile<P0>(hffile: P0, pv: *mut core::ffi::c_void, cb: u32, pcbactual: Option<*mut u32>) -> i32
where
    P0: windows_core::Param<super::StructuredStorage::JET_HANDLE>,
{
    windows_targets::link!("esent.dll" "system" fn JetReadFile(hffile : super::StructuredStorage:: JET_HANDLE, pv : *mut core::ffi::c_void, cb : u32, pcbactual : *mut u32) -> i32);
    JetReadFile(hffile.param().abi(), pv, cb, core::mem::transmute(pcbactual.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetReadFileInstance<P0, P1>(instance: P0, hffile: P1, pv: *mut core::ffi::c_void, cb: u32, pcbactual: Option<*mut u32>) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
    P1: windows_core::Param<super::StructuredStorage::JET_HANDLE>,
{
    windows_targets::link!("esent.dll" "system" fn JetReadFileInstance(instance : JET_INSTANCE, hffile : super::StructuredStorage:: JET_HANDLE, pv : *mut core::ffi::c_void, cb : u32, pcbactual : *mut u32) -> i32);
    JetReadFileInstance(instance.param().abi(), hffile.param().abi(), pv, cb, core::mem::transmute(pcbactual.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetRegisterCallback<P0, P1>(sesid: P0, tableid: P1, cbtyp: u32, pcallback: JET_CALLBACK, pvcontext: Option<*const core::ffi::c_void>, phcallbackid: *const super::StructuredStorage::JET_HANDLE) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetRegisterCallback(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, cbtyp : u32, pcallback : JET_CALLBACK, pvcontext : *const core::ffi::c_void, phcallbackid : *const super::StructuredStorage:: JET_HANDLE) -> i32);
    JetRegisterCallback(sesid.param().abi(), tableid.param().abi(), cbtyp, pcallback, core::mem::transmute(pvcontext.unwrap_or(std::ptr::null())), phcallbackid)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetRenameColumnA<P0, P1>(sesid: P0, tableid: P1, szname: *const i8, sznamenew: *const i8, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetRenameColumnA(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, szname : *const i8, sznamenew : *const i8, grbit : u32) -> i32);
    JetRenameColumnA(sesid.param().abi(), tableid.param().abi(), szname, sznamenew, grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetRenameColumnW<P0, P1>(sesid: P0, tableid: P1, szname: *const u16, sznamenew: *const u16, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetRenameColumnW(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, szname : *const u16, sznamenew : *const u16, grbit : u32) -> i32);
    JetRenameColumnW(sesid.param().abi(), tableid.param().abi(), szname, sznamenew, grbit)
}
#[inline]
pub unsafe fn JetRenameTableA<P0>(sesid: P0, dbid: u32, szname: *const i8, sznamenew: *const i8) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetRenameTableA(sesid : JET_SESID, dbid : u32, szname : *const i8, sznamenew : *const i8) -> i32);
    JetRenameTableA(sesid.param().abi(), dbid, szname, sznamenew)
}
#[inline]
pub unsafe fn JetRenameTableW<P0>(sesid: P0, dbid: u32, szname: *const u16, sznamenew: *const u16) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetRenameTableW(sesid : JET_SESID, dbid : u32, szname : *const u16, sznamenew : *const u16) -> i32);
    JetRenameTableW(sesid.param().abi(), dbid, szname, sznamenew)
}
#[inline]
pub unsafe fn JetResetSessionContext<P0>(sesid: P0) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetResetSessionContext(sesid : JET_SESID) -> i32);
    JetResetSessionContext(sesid.param().abi())
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetResetTableSequential<P0, P1>(sesid: P0, tableid: P1, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetResetTableSequential(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, grbit : u32) -> i32);
    JetResetTableSequential(sesid.param().abi(), tableid.param().abi(), grbit)
}
#[inline]
pub unsafe fn JetResizeDatabase<P0>(sesid: P0, dbid: u32, cpgtarget: u32, pcpgactual: *mut u32, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetResizeDatabase(sesid : JET_SESID, dbid : u32, cpgtarget : u32, pcpgactual : *mut u32, grbit : u32) -> i32);
    JetResizeDatabase(sesid.param().abi(), dbid, cpgtarget, pcpgactual, grbit)
}
#[inline]
pub unsafe fn JetRestore2A(sz: *const i8, szdest: Option<*const i8>, pfn: JET_PFNSTATUS) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetRestore2A(sz : *const i8, szdest : *const i8, pfn : JET_PFNSTATUS) -> i32);
    JetRestore2A(sz, core::mem::transmute(szdest.unwrap_or(std::ptr::null())), pfn)
}
#[inline]
pub unsafe fn JetRestore2W(sz: *const u16, szdest: Option<*const u16>, pfn: JET_PFNSTATUS) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetRestore2W(sz : *const u16, szdest : *const u16, pfn : JET_PFNSTATUS) -> i32);
    JetRestore2W(sz, core::mem::transmute(szdest.unwrap_or(std::ptr::null())), pfn)
}
#[inline]
pub unsafe fn JetRestoreA(szsource: *const i8, pfn: JET_PFNSTATUS) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetRestoreA(szsource : *const i8, pfn : JET_PFNSTATUS) -> i32);
    JetRestoreA(szsource, pfn)
}
#[inline]
pub unsafe fn JetRestoreInstanceA<P0>(instance: P0, sz: *const i8, szdest: Option<*const i8>, pfn: JET_PFNSTATUS) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetRestoreInstanceA(instance : JET_INSTANCE, sz : *const i8, szdest : *const i8, pfn : JET_PFNSTATUS) -> i32);
    JetRestoreInstanceA(instance.param().abi(), sz, core::mem::transmute(szdest.unwrap_or(std::ptr::null())), pfn)
}
#[inline]
pub unsafe fn JetRestoreInstanceW<P0>(instance: P0, sz: *const u16, szdest: Option<*const u16>, pfn: JET_PFNSTATUS) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetRestoreInstanceW(instance : JET_INSTANCE, sz : *const u16, szdest : *const u16, pfn : JET_PFNSTATUS) -> i32);
    JetRestoreInstanceW(instance.param().abi(), sz, core::mem::transmute(szdest.unwrap_or(std::ptr::null())), pfn)
}
#[inline]
pub unsafe fn JetRestoreW(szsource: *const u16, pfn: JET_PFNSTATUS) -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetRestoreW(szsource : *const u16, pfn : JET_PFNSTATUS) -> i32);
    JetRestoreW(szsource, pfn)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetRetrieveColumn<P0, P1>(sesid: P0, tableid: P1, columnid: u32, pvdata: Option<*mut core::ffi::c_void>, cbdata: u32, pcbactual: Option<*mut u32>, grbit: u32, pretinfo: Option<*mut JET_RETINFO>) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetRetrieveColumn(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, columnid : u32, pvdata : *mut core::ffi::c_void, cbdata : u32, pcbactual : *mut u32, grbit : u32, pretinfo : *mut JET_RETINFO) -> i32);
    JetRetrieveColumn(sesid.param().abi(), tableid.param().abi(), columnid, core::mem::transmute(pvdata.unwrap_or(std::ptr::null_mut())), cbdata, core::mem::transmute(pcbactual.unwrap_or(std::ptr::null_mut())), grbit, core::mem::transmute(pretinfo.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetRetrieveColumns<P0, P1>(sesid: P0, tableid: P1, pretrievecolumn: Option<&mut [JET_RETRIEVECOLUMN]>) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetRetrieveColumns(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, pretrievecolumn : *mut JET_RETRIEVECOLUMN, cretrievecolumn : u32) -> i32);
    JetRetrieveColumns(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(pretrievecolumn.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pretrievecolumn.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetRetrieveKey<P0, P1>(sesid: P0, tableid: P1, pvkey: Option<*mut core::ffi::c_void>, cbmax: u32, pcbactual: Option<*mut u32>, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetRetrieveKey(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, pvkey : *mut core::ffi::c_void, cbmax : u32, pcbactual : *mut u32, grbit : u32) -> i32);
    JetRetrieveKey(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(pvkey.unwrap_or(std::ptr::null_mut())), cbmax, core::mem::transmute(pcbactual.unwrap_or(std::ptr::null_mut())), grbit)
}
#[inline]
pub unsafe fn JetRollback<P0>(sesid: P0, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetRollback(sesid : JET_SESID, grbit : u32) -> i32);
    JetRollback(sesid.param().abi(), grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSeek<P0, P1>(sesid: P0, tableid: P1, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetSeek(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, grbit : u32) -> i32);
    JetSeek(sesid.param().abi(), tableid.param().abi(), grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetColumn<P0, P1>(sesid: P0, tableid: P1, columnid: u32, pvdata: Option<*const core::ffi::c_void>, cbdata: u32, grbit: u32, psetinfo: Option<*const JET_SETINFO>) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetSetColumn(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, columnid : u32, pvdata : *const core::ffi::c_void, cbdata : u32, grbit : u32, psetinfo : *const JET_SETINFO) -> i32);
    JetSetColumn(sesid.param().abi(), tableid.param().abi(), columnid, core::mem::transmute(pvdata.unwrap_or(std::ptr::null())), cbdata, grbit, core::mem::transmute(psetinfo.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn JetSetColumnDefaultValueA<P0>(sesid: P0, dbid: u32, sztablename: *const i8, szcolumnname: *const i8, pvdata: *const core::ffi::c_void, cbdata: u32, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetSetColumnDefaultValueA(sesid : JET_SESID, dbid : u32, sztablename : *const i8, szcolumnname : *const i8, pvdata : *const core::ffi::c_void, cbdata : u32, grbit : u32) -> i32);
    JetSetColumnDefaultValueA(sesid.param().abi(), dbid, sztablename, szcolumnname, pvdata, cbdata, grbit)
}
#[inline]
pub unsafe fn JetSetColumnDefaultValueW<P0>(sesid: P0, dbid: u32, sztablename: *const u16, szcolumnname: *const u16, pvdata: *const core::ffi::c_void, cbdata: u32, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetSetColumnDefaultValueW(sesid : JET_SESID, dbid : u32, sztablename : *const u16, szcolumnname : *const u16, pvdata : *const core::ffi::c_void, cbdata : u32, grbit : u32) -> i32);
    JetSetColumnDefaultValueW(sesid.param().abi(), dbid, sztablename, szcolumnname, pvdata, cbdata, grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetColumns<P0, P1>(sesid: P0, tableid: P1, psetcolumn: Option<&[JET_SETCOLUMN]>) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetSetColumns(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, psetcolumn : *const JET_SETCOLUMN, csetcolumn : u32) -> i32);
    JetSetColumns(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(psetcolumn.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), psetcolumn.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetCurrentIndex2A<P0, P1>(sesid: P0, tableid: P1, szindexname: Option<*const i8>, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetSetCurrentIndex2A(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, szindexname : *const i8, grbit : u32) -> i32);
    JetSetCurrentIndex2A(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(szindexname.unwrap_or(std::ptr::null())), grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetCurrentIndex2W<P0, P1>(sesid: P0, tableid: P1, szindexname: Option<*const u16>, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetSetCurrentIndex2W(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, szindexname : *const u16, grbit : u32) -> i32);
    JetSetCurrentIndex2W(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(szindexname.unwrap_or(std::ptr::null())), grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetCurrentIndex3A<P0, P1>(sesid: P0, tableid: P1, szindexname: Option<*const i8>, grbit: u32, itagsequence: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetSetCurrentIndex3A(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, szindexname : *const i8, grbit : u32, itagsequence : u32) -> i32);
    JetSetCurrentIndex3A(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(szindexname.unwrap_or(std::ptr::null())), grbit, itagsequence)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetCurrentIndex3W<P0, P1>(sesid: P0, tableid: P1, szindexname: Option<*const u16>, grbit: u32, itagsequence: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetSetCurrentIndex3W(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, szindexname : *const u16, grbit : u32, itagsequence : u32) -> i32);
    JetSetCurrentIndex3W(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(szindexname.unwrap_or(std::ptr::null())), grbit, itagsequence)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetCurrentIndex4A<P0, P1>(sesid: P0, tableid: P1, szindexname: Option<*const i8>, pindexid: Option<*const JET_INDEXID>, grbit: u32, itagsequence: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetSetCurrentIndex4A(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, szindexname : *const i8, pindexid : *const JET_INDEXID, grbit : u32, itagsequence : u32) -> i32);
    JetSetCurrentIndex4A(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(szindexname.unwrap_or(std::ptr::null())), core::mem::transmute(pindexid.unwrap_or(std::ptr::null())), grbit, itagsequence)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetCurrentIndex4W<P0, P1>(sesid: P0, tableid: P1, szindexname: Option<*const u16>, pindexid: Option<*const JET_INDEXID>, grbit: u32, itagsequence: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetSetCurrentIndex4W(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, szindexname : *const u16, pindexid : *const JET_INDEXID, grbit : u32, itagsequence : u32) -> i32);
    JetSetCurrentIndex4W(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(szindexname.unwrap_or(std::ptr::null())), core::mem::transmute(pindexid.unwrap_or(std::ptr::null())), grbit, itagsequence)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetCurrentIndexA<P0, P1>(sesid: P0, tableid: P1, szindexname: Option<*const i8>) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetSetCurrentIndexA(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, szindexname : *const i8) -> i32);
    JetSetCurrentIndexA(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(szindexname.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetCurrentIndexW<P0, P1>(sesid: P0, tableid: P1, szindexname: Option<*const u16>) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetSetCurrentIndexW(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, szindexname : *const u16) -> i32);
    JetSetCurrentIndexW(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(szindexname.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetCursorFilter<P0, P1>(sesid: P0, tableid: P1, rgcolumnfilters: &[JET_INDEX_COLUMN], grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetSetCursorFilter(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, rgcolumnfilters : *const JET_INDEX_COLUMN, ccolumnfilters : u32, grbit : u32) -> i32);
    JetSetCursorFilter(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(rgcolumnfilters.as_ptr()), rgcolumnfilters.len().try_into().unwrap(), grbit)
}
#[inline]
pub unsafe fn JetSetDatabaseSizeA<P0>(sesid: P0, szdatabasename: *const i8, cpg: u32, pcpgreal: *mut u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetSetDatabaseSizeA(sesid : JET_SESID, szdatabasename : *const i8, cpg : u32, pcpgreal : *mut u32) -> i32);
    JetSetDatabaseSizeA(sesid.param().abi(), szdatabasename, cpg, pcpgreal)
}
#[inline]
pub unsafe fn JetSetDatabaseSizeW<P0>(sesid: P0, szdatabasename: *const u16, cpg: u32, pcpgreal: *mut u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetSetDatabaseSizeW(sesid : JET_SESID, szdatabasename : *const u16, cpg : u32, pcpgreal : *mut u32) -> i32);
    JetSetDatabaseSizeW(sesid.param().abi(), szdatabasename, cpg, pcpgreal)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetIndexRange<P0, P1>(sesid: P0, tableidsrc: P1, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetSetIndexRange(sesid : JET_SESID, tableidsrc : super::StructuredStorage:: JET_TABLEID, grbit : u32) -> i32);
    JetSetIndexRange(sesid.param().abi(), tableidsrc.param().abi(), grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetLS<P0, P1, P2>(sesid: P0, tableid: P1, ls: P2, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
    P2: windows_core::Param<JET_LS>,
{
    windows_targets::link!("esent.dll" "system" fn JetSetLS(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, ls : JET_LS, grbit : u32) -> i32);
    JetSetLS(sesid.param().abi(), tableid.param().abi(), ls.param().abi(), grbit)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetSessionContext<P0, P1>(sesid: P0, ulcontext: P1) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_API_PTR>,
{
    windows_targets::link!("esent.dll" "system" fn JetSetSessionContext(sesid : JET_SESID, ulcontext : super::StructuredStorage:: JET_API_PTR) -> i32);
    JetSetSessionContext(sesid.param().abi(), ulcontext.param().abi())
}
#[inline]
pub unsafe fn JetSetSessionParameter<P0>(sesid: P0, sesparamid: u32, pvparam: Option<*const core::ffi::c_void>, cbparam: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
{
    windows_targets::link!("esent.dll" "system" fn JetSetSessionParameter(sesid : JET_SESID, sesparamid : u32, pvparam : *const core::ffi::c_void, cbparam : u32) -> i32);
    JetSetSessionParameter(sesid.param().abi(), sesparamid, core::mem::transmute(pvparam.unwrap_or(std::ptr::null())), cbparam)
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetSystemParameterA<P0, P1>(pinstance: Option<*mut JET_INSTANCE>, sesid: P0, paramid: u32, lparam: P1, szparam: Option<*const i8>) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_API_PTR>,
{
    windows_targets::link!("esent.dll" "system" fn JetSetSystemParameterA(pinstance : *mut JET_INSTANCE, sesid : JET_SESID, paramid : u32, lparam : super::StructuredStorage:: JET_API_PTR, szparam : *const i8) -> i32);
    JetSetSystemParameterA(core::mem::transmute(pinstance.unwrap_or(std::ptr::null_mut())), sesid.param().abi(), paramid, lparam.param().abi(), core::mem::transmute(szparam.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetSystemParameterW<P0, P1>(pinstance: Option<*mut JET_INSTANCE>, sesid: P0, paramid: u32, lparam: P1, szparam: Option<*const u16>) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_API_PTR>,
{
    windows_targets::link!("esent.dll" "system" fn JetSetSystemParameterW(pinstance : *mut JET_INSTANCE, sesid : JET_SESID, paramid : u32, lparam : super::StructuredStorage:: JET_API_PTR, szparam : *const u16) -> i32);
    JetSetSystemParameterW(core::mem::transmute(pinstance.unwrap_or(std::ptr::null_mut())), sesid.param().abi(), paramid, lparam.param().abi(), core::mem::transmute(szparam.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetTableSequential<P0, P1>(sesid: P0, tableid: P1, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetSetTableSequential(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, grbit : u32) -> i32);
    JetSetTableSequential(sesid.param().abi(), tableid.param().abi(), grbit)
}
#[inline]
pub unsafe fn JetStopBackup() -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetStopBackup() -> i32);
    JetStopBackup()
}
#[inline]
pub unsafe fn JetStopBackupInstance<P0>(instance: P0) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetStopBackupInstance(instance : JET_INSTANCE) -> i32);
    JetStopBackupInstance(instance.param().abi())
}
#[inline]
pub unsafe fn JetStopService() -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetStopService() -> i32);
    JetStopService()
}
#[inline]
pub unsafe fn JetStopServiceInstance<P0>(instance: P0) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetStopServiceInstance(instance : JET_INSTANCE) -> i32);
    JetStopServiceInstance(instance.param().abi())
}
#[inline]
pub unsafe fn JetStopServiceInstance2<P0>(instance: P0, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetStopServiceInstance2(instance : JET_INSTANCE, grbit : u32) -> i32);
    JetStopServiceInstance2(instance.param().abi(), grbit)
}
#[inline]
pub unsafe fn JetTerm<P0>(instance: P0) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetTerm(instance : JET_INSTANCE) -> i32);
    JetTerm(instance.param().abi())
}
#[inline]
pub unsafe fn JetTerm2<P0>(instance: P0, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetTerm2(instance : JET_INSTANCE, grbit : u32) -> i32);
    JetTerm2(instance.param().abi(), grbit)
}
#[inline]
pub unsafe fn JetTruncateLog() -> i32 {
    windows_targets::link!("esent.dll" "system" fn JetTruncateLog() -> i32);
    JetTruncateLog()
}
#[inline]
pub unsafe fn JetTruncateLogInstance<P0>(instance: P0) -> i32
where
    P0: windows_core::Param<JET_INSTANCE>,
{
    windows_targets::link!("esent.dll" "system" fn JetTruncateLogInstance(instance : JET_INSTANCE) -> i32);
    JetTruncateLogInstance(instance.param().abi())
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetUnregisterCallback<P0, P1, P2>(sesid: P0, tableid: P1, cbtyp: u32, hcallbackid: P2) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
    P2: windows_core::Param<super::StructuredStorage::JET_HANDLE>,
{
    windows_targets::link!("esent.dll" "system" fn JetUnregisterCallback(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, cbtyp : u32, hcallbackid : super::StructuredStorage:: JET_HANDLE) -> i32);
    JetUnregisterCallback(sesid.param().abi(), tableid.param().abi(), cbtyp, hcallbackid.param().abi())
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetUpdate<P0, P1>(sesid: P0, tableid: P1, pvbookmark: Option<*mut core::ffi::c_void>, cbbookmark: u32, pcbactual: Option<*mut u32>) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetUpdate(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, pvbookmark : *mut core::ffi::c_void, cbbookmark : u32, pcbactual : *mut u32) -> i32);
    JetUpdate(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(pvbookmark.unwrap_or(std::ptr::null_mut())), cbbookmark, core::mem::transmute(pcbactual.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetUpdate2<P0, P1>(sesid: P0, tableid: P1, pvbookmark: Option<*mut core::ffi::c_void>, cbbookmark: u32, pcbactual: Option<*mut u32>, grbit: u32) -> i32
where
    P0: windows_core::Param<JET_SESID>,
    P1: windows_core::Param<super::StructuredStorage::JET_TABLEID>,
{
    windows_targets::link!("esent.dll" "system" fn JetUpdate2(sesid : JET_SESID, tableid : super::StructuredStorage:: JET_TABLEID, pvbookmark : *mut core::ffi::c_void, cbbookmark : u32, pcbactual : *mut u32, grbit : u32) -> i32);
    JetUpdate2(sesid.param().abi(), tableid.param().abi(), core::mem::transmute(pvbookmark.unwrap_or(std::ptr::null_mut())), cbbookmark, core::mem::transmute(pcbactual.unwrap_or(std::ptr::null_mut())), grbit)
}
pub const JET_BASE_NAME_LENGTH: u32 = 3u32;
pub const JET_ColInfoGrbitMinimalInfo: u32 = 1073741824u32;
pub const JET_ColInfoGrbitNonDerivedColumnsOnly: u32 = 2147483648u32;
pub const JET_ColInfoGrbitSortByColumnid: u32 = 536870912u32;
pub const JET_DbInfoCollate: u32 = 5u32;
pub const JET_DbInfoConnect: u32 = 1u32;
pub const JET_DbInfoCountry: u32 = 2u32;
pub const JET_DbInfoCp: u32 = 4u32;
pub const JET_DbInfoDBInUse: u32 = 15u32;
pub const JET_DbInfoFileType: u32 = 19u32;
pub const JET_DbInfoFilename: u32 = 0u32;
pub const JET_DbInfoFilesize: u32 = 10u32;
pub const JET_DbInfoFilesizeOnDisk: u32 = 21u32;
pub const JET_DbInfoIsam: u32 = 9u32;
pub const JET_DbInfoLCID: u32 = 3u32;
pub const JET_DbInfoLangid: u32 = 3u32;
pub const JET_DbInfoMisc: u32 = 14u32;
pub const JET_DbInfoOptions: u32 = 6u32;
pub const JET_DbInfoPageSize: u32 = 17u32;
pub const JET_DbInfoSpaceAvailable: u32 = 12u32;
pub const JET_DbInfoSpaceOwned: u32 = 11u32;
pub const JET_DbInfoTransactions: u32 = 7u32;
pub const JET_DbInfoUpgrade: u32 = 13u32;
pub const JET_DbInfoVersion: u32 = 8u32;
pub const JET_EventLoggingDisable: u32 = 0u32;
pub const JET_EventLoggingLevelHigh: u32 = 75u32;
pub const JET_EventLoggingLevelLow: u32 = 25u32;
pub const JET_EventLoggingLevelMax: u32 = 100u32;
pub const JET_EventLoggingLevelMedium: u32 = 50u32;
pub const JET_EventLoggingLevelMin: u32 = 1u32;
pub const JET_ExceptionFailFast: u32 = 4u32;
pub const JET_ExceptionMsgBox: u32 = 1u32;
pub const JET_ExceptionNone: u32 = 2u32;
pub const JET_IOPriorityLow: u32 = 1u32;
pub const JET_IOPriorityNormal: u32 = 0u32;
pub const JET_IndexCheckingDeferToOpenTable: JET_INDEXCHECKING = JET_INDEXCHECKING(2i32);
pub const JET_IndexCheckingMax: JET_INDEXCHECKING = JET_INDEXCHECKING(3i32);
pub const JET_IndexCheckingOff: JET_INDEXCHECKING = JET_INDEXCHECKING(0i32);
pub const JET_IndexCheckingOn: JET_INDEXCHECKING = JET_INDEXCHECKING(1i32);
pub const JET_MAX_COMPUTERNAME_LENGTH: u32 = 15u32;
pub const JET_MoveFirst: u32 = 2147483648u32;
pub const JET_MoveLast: u32 = 2147483647u32;
pub const JET_MovePrevious: i32 = -1i32;
pub const JET_OnlineDefragAll: u32 = 65535u32;
pub const JET_OnlineDefragAllOBSOLETE: u32 = 1u32;
pub const JET_OnlineDefragDatabases: u32 = 2u32;
pub const JET_OnlineDefragDisable: u32 = 0u32;
pub const JET_OnlineDefragSpaceTrees: u32 = 4u32;
pub const JET_VERSION: u32 = 1280u32;
pub const JET_bitAbortSnapshot: u32 = 1u32;
pub const JET_bitAllDatabasesSnapshot: u32 = 1u32;
pub const JET_bitBackupAtomic: u32 = 4u32;
pub const JET_bitBackupEndAbort: u32 = 2u32;
pub const JET_bitBackupEndNormal: u32 = 1u32;
pub const JET_bitBackupIncremental: u32 = 1u32;
pub const JET_bitBackupSnapshot: u32 = 16u32;
pub const JET_bitBackupTruncateDone: u32 = 256u32;
pub const JET_bitBookmarkPermitVirtualCurrency: u32 = 1u32;
pub const JET_bitCheckUniqueness: u32 = 64u32;
pub const JET_bitColumnAutoincrement: u32 = 16u32;
pub const JET_bitColumnCompressed: u32 = 524288u32;
pub const JET_bitColumnDeleteOnZero: u32 = 131072u32;
pub const JET_bitColumnEscrowUpdate: u32 = 2048u32;
pub const JET_bitColumnFinalize: u32 = 16384u32;
pub const JET_bitColumnFixed: u32 = 1u32;
pub const JET_bitColumnMaybeNull: u32 = 8192u32;
pub const JET_bitColumnMultiValued: u32 = 1024u32;
pub const JET_bitColumnNotNULL: u32 = 4u32;
pub const JET_bitColumnTTDescending: u32 = 128u32;
pub const JET_bitColumnTTKey: u32 = 64u32;
pub const JET_bitColumnTagged: u32 = 2u32;
pub const JET_bitColumnUnversioned: u32 = 4096u32;
pub const JET_bitColumnUpdatable: u32 = 32u32;
pub const JET_bitColumnUserDefinedDefault: u32 = 32768u32;
pub const JET_bitColumnVersion: u32 = 8u32;
pub const JET_bitCommitLazyFlush: u32 = 1u32;
pub const JET_bitCompactRepair: u32 = 64u32;
pub const JET_bitCompactStats: u32 = 32u32;
pub const JET_bitConfigStoreReadControlDefault: u32 = 0u32;
pub const JET_bitConfigStoreReadControlDisableAll: u32 = 2u32;
pub const JET_bitConfigStoreReadControlInhibitRead: u32 = 1u32;
pub const JET_bitContinueAfterThaw: u32 = 4u32;
pub const JET_bitCopySnapshot: u32 = 2u32;
pub const JET_bitCreateHintAppendSequential: u32 = 2u32;
pub const JET_bitCreateHintHotpointSequential: u32 = 4u32;
pub const JET_bitDbDeleteCorruptIndexes: u32 = 16u32;
pub const JET_bitDbDeleteUnicodeIndexes: u32 = 1024u32;
pub const JET_bitDbEnableBackgroundMaintenance: u32 = 2048u32;
pub const JET_bitDbExclusive: u32 = 2u32;
pub const JET_bitDbOverwriteExisting: u32 = 512u32;
pub const JET_bitDbPurgeCacheOnAttach: u32 = 4096u32;
pub const JET_bitDbReadOnly: u32 = 1u32;
pub const JET_bitDbRecoveryOff: u32 = 8u32;
pub const JET_bitDbShadowingOff: u32 = 128u32;
pub const JET_bitDbUpgrade: u32 = 512u32;
pub const JET_bitDefragmentAvailSpaceTreesOnly: u32 = 64u32;
pub const JET_bitDefragmentBTree: u32 = 256u32;
pub const JET_bitDefragmentBatchStart: u32 = 1u32;
pub const JET_bitDefragmentBatchStop: u32 = 2u32;
pub const JET_bitDefragmentNoPartialMerges: u32 = 128u32;
pub const JET_bitDeleteColumnIgnoreTemplateColumns: u32 = 1u32;
pub const JET_bitDeleteHintTableSequential: u32 = 256u32;
pub const JET_bitDumpCacheIncludeCachedPages: u32 = 32u32;
pub const JET_bitDumpCacheIncludeCorruptedPages: u32 = 64u32;
pub const JET_bitDumpCacheIncludeDirtyPages: u32 = 16u32;
pub const JET_bitDumpCacheMaximum: u32 = 8u32;
pub const JET_bitDumpCacheMinimum: u32 = 4u32;
pub const JET_bitDumpCacheNoDecommit: u32 = 128u32;
pub const JET_bitDumpMaximum: u32 = 2u32;
pub const JET_bitDumpMinimum: u32 = 1u32;
pub const JET_bitDurableCommitCallbackLogUnavailable: u32 = 1u32;
pub const JET_bitESE98FileNames: u32 = 1u32;
pub const JET_bitEightDotThreeSoftCompat: u32 = 2u32;
pub const JET_bitEnumerateCompressOutput: u32 = 524288u32;
pub const JET_bitEnumerateCopy: u32 = 1u32;
pub const JET_bitEnumerateIgnoreDefault: u32 = 32u32;
pub const JET_bitEnumerateIgnoreUserDefinedDefault: u32 = 1048576u32;
pub const JET_bitEnumerateInRecordOnly: u32 = 2097152u32;
pub const JET_bitEnumeratePresenceOnly: u32 = 131072u32;
pub const JET_bitEnumerateTaggedOnly: u32 = 262144u32;
pub const JET_bitEscrowNoRollback: u32 = 1u32;
pub const JET_bitExplicitPrepare: u32 = 8u32;
pub const JET_bitForceDetach: u32 = 1u32;
pub const JET_bitForceNewLog: u32 = 16u32;
pub const JET_bitFullColumnEndLimit: u32 = 512u32;
pub const JET_bitFullColumnStartLimit: u32 = 256u32;
pub const JET_bitHungIOEvent: u32 = 1u32;
pub const JET_bitIdleCompact: u32 = 2u32;
pub const JET_bitIdleFlushBuffers: u32 = 1u32;
pub const JET_bitIdleStatus: u32 = 4u32;
pub const JET_bitIncrementalSnapshot: u32 = 1u32;
pub const JET_bitIndexColumnMustBeNonNull: u32 = 2u32;
pub const JET_bitIndexColumnMustBeNull: u32 = 1u32;
pub const JET_bitIndexCrossProduct: u32 = 16384u32;
pub const JET_bitIndexDisallowNull: u32 = 4u32;
pub const JET_bitIndexDisallowTruncation: u32 = 65536u32;
pub const JET_bitIndexDotNetGuid: u32 = 262144u32;
pub const JET_bitIndexEmpty: u32 = 256u32;
pub const JET_bitIndexIgnoreAnyNull: u32 = 32u32;
pub const JET_bitIndexIgnoreFirstNull: u32 = 64u32;
pub const JET_bitIndexIgnoreNull: u32 = 8u32;
pub const JET_bitIndexImmutableStructure: u32 = 524288u32;
pub const JET_bitIndexKeyMost: u32 = 32768u32;
pub const JET_bitIndexLazyFlush: u32 = 128u32;
pub const JET_bitIndexNestedTable: u32 = 131072u32;
pub const JET_bitIndexPrimary: u32 = 2u32;
pub const JET_bitIndexSortNullsHigh: u32 = 1024u32;
pub const JET_bitIndexTupleLimits: u32 = 8192u32;
pub const JET_bitIndexTuples: u32 = 4096u32;
pub const JET_bitIndexUnicode: u32 = 2048u32;
pub const JET_bitIndexUnique: u32 = 1u32;
pub const JET_bitIndexUnversioned: u32 = 512u32;
pub const JET_bitKeepDbAttachedAtEndOfRecovery: u32 = 4096u32;
pub const JET_bitKeyAscending: u32 = 0u32;
pub const JET_bitKeyDataZeroLength: u32 = 16u32;
pub const JET_bitKeyDescending: u32 = 1u32;
pub const JET_bitLSCursor: u32 = 2u32;
pub const JET_bitLSReset: u32 = 1u32;
pub const JET_bitLSTable: u32 = 4u32;
pub const JET_bitLogStreamMustExist: u32 = 64u32;
pub const JET_bitMoveFirst: u32 = 0u32;
pub const JET_bitMoveKeyNE: u32 = 1u32;
pub const JET_bitNewKey: u32 = 1u32;
pub const JET_bitNoMove: u32 = 2u32;
pub const JET_bitNormalizedKey: u32 = 8u32;
pub const JET_bitObjectSystem: u32 = 2147483648u32;
pub const JET_bitObjectTableDerived: u32 = 268435456u32;
pub const JET_bitObjectTableFixedDDL: u32 = 1073741824u32;
pub const JET_bitObjectTableNoFixedVarColumnsInDerivedTables: u32 = 67108864u32;
pub const JET_bitObjectTableTemplate: u32 = 536870912u32;
pub const JET_bitPartialColumnEndLimit: u32 = 2048u32;
pub const JET_bitPartialColumnStartLimit: u32 = 1024u32;
pub const JET_bitPrereadBackward: u32 = 2u32;
pub const JET_bitPrereadFirstPage: u32 = 4u32;
pub const JET_bitPrereadForward: u32 = 1u32;
pub const JET_bitPrereadNormalizedKey: u32 = 8u32;
pub const JET_bitRangeInclusive: u32 = 1u32;
pub const JET_bitRangeInstantDuration: u32 = 4u32;
pub const JET_bitRangeRemove: u32 = 8u32;
pub const JET_bitRangeUpperLimit: u32 = 2u32;
pub const JET_bitReadLock: u32 = 1u32;
pub const JET_bitRecordInIndex: u32 = 1u32;
pub const JET_bitRecordNotInIndex: u32 = 2u32;
pub const JET_bitRecordSizeInCopyBuffer: u32 = 1u32;
pub const JET_bitRecordSizeLocal: u32 = 4u32;
pub const JET_bitRecordSizeRunningTotal: u32 = 2u32;
pub const JET_bitRecoveryWithoutUndo: u32 = 8u32;
pub const JET_bitReplayIgnoreLostLogs: u32 = 128u32;
pub const JET_bitReplayIgnoreMissingDB: u32 = 4u32;
pub const JET_bitReplayMissingMapEntryDB: u32 = 32u32;
pub const JET_bitResizeDatabaseOnlyGrow: u32 = 1u32;
pub const JET_bitResizeDatabaseOnlyShrink: u32 = 2u32;
pub const JET_bitRetrieveCopy: u32 = 1u32;
pub const JET_bitRetrieveFromIndex: u32 = 2u32;
pub const JET_bitRetrieveFromPrimaryBookmark: u32 = 4u32;
pub const JET_bitRetrieveHintReserve1: u32 = 8u32;
pub const JET_bitRetrieveHintReserve2: u32 = 64u32;
pub const JET_bitRetrieveHintReserve3: u32 = 128u32;
pub const JET_bitRetrieveHintTableScanBackward: u32 = 32u32;
pub const JET_bitRetrieveHintTableScanForward: u32 = 16u32;
pub const JET_bitRetrieveIgnoreDefault: u32 = 32u32;
pub const JET_bitRetrieveNull: u32 = 16u32;
pub const JET_bitRetrieveTag: u32 = 8u32;
pub const JET_bitRetrieveTuple: u32 = 2048u32;
pub const JET_bitRollbackAll: u32 = 1u32;
pub const JET_bitSeekEQ: u32 = 1u32;
pub const JET_bitSeekGE: u32 = 8u32;
pub const JET_bitSeekGT: u32 = 16u32;
pub const JET_bitSeekLE: u32 = 4u32;
pub const JET_bitSeekLT: u32 = 2u32;
pub const JET_bitSetAppendLV: u32 = 1u32;
pub const JET_bitSetCompressed: u32 = 131072u32;
pub const JET_bitSetContiguousLV: u32 = 262144u32;
pub const JET_bitSetIndexRange: u32 = 32u32;
pub const JET_bitSetIntrinsicLV: u32 = 1024u32;
pub const JET_bitSetOverwriteLV: u32 = 4u32;
pub const JET_bitSetRevertToDefaultValue: u32 = 512u32;
pub const JET_bitSetSeparateLV: u32 = 64u32;
pub const JET_bitSetSizeLV: u32 = 8u32;
pub const JET_bitSetUncompressed: u32 = 65536u32;
pub const JET_bitSetUniqueMultiValues: u32 = 128u32;
pub const JET_bitSetUniqueNormalizedMultiValues: u32 = 256u32;
pub const JET_bitSetZeroLength: u32 = 32u32;
pub const JET_bitShrinkDatabaseOff: u32 = 0u32;
pub const JET_bitShrinkDatabaseOn: u32 = 1u32;
pub const JET_bitShrinkDatabaseRealtime: u32 = 2u32;
pub const JET_bitShrinkDatabaseTrim: u32 = 1u32;
pub const JET_bitSpaceHintsUtilizeParentSpace: u32 = 1u32;
pub const JET_bitStopServiceAll: u32 = 0u32;
pub const JET_bitStopServiceBackgroundUserTasks: u32 = 2u32;
pub const JET_bitStopServiceQuiesceCaches: u32 = 4u32;
pub const JET_bitStopServiceResume: u32 = 2147483648u32;
pub const JET_bitStrLimit: u32 = 2u32;
pub const JET_bitSubStrLimit: u32 = 4u32;
pub const JET_bitTTDotNetGuid: u32 = 256u32;
pub const JET_bitTTErrorOnDuplicateInsertion: u32 = 32u32;
pub const JET_bitTTForceMaterialization: u32 = 32u32;
pub const JET_bitTTForwardOnly: u32 = 64u32;
pub const JET_bitTTIndexed: u32 = 1u32;
pub const JET_bitTTIntrinsicLVsOnly: u32 = 128u32;
pub const JET_bitTTScrollable: u32 = 8u32;
pub const JET_bitTTSortNullsHigh: u32 = 16u32;
pub const JET_bitTTUnique: u32 = 2u32;
pub const JET_bitTTUpdatable: u32 = 4u32;
pub const JET_bitTableClass1: u32 = 65536u32;
pub const JET_bitTableClass10: u32 = 655360u32;
pub const JET_bitTableClass11: u32 = 720896u32;
pub const JET_bitTableClass12: u32 = 786432u32;
pub const JET_bitTableClass13: u32 = 851968u32;
pub const JET_bitTableClass14: u32 = 917504u32;
pub const JET_bitTableClass15: u32 = 983040u32;
pub const JET_bitTableClass2: u32 = 131072u32;
pub const JET_bitTableClass3: u32 = 196608u32;
pub const JET_bitTableClass4: u32 = 262144u32;
pub const JET_bitTableClass5: u32 = 327680u32;
pub const JET_bitTableClass6: u32 = 393216u32;
pub const JET_bitTableClass7: u32 = 458752u32;
pub const JET_bitTableClass8: u32 = 524288u32;
pub const JET_bitTableClass9: u32 = 589824u32;
pub const JET_bitTableClassMask: u32 = 2031616u32;
pub const JET_bitTableClassNone: u32 = 0u32;
pub const JET_bitTableCreateFixedDDL: u32 = 1u32;
pub const JET_bitTableCreateImmutableStructure: u32 = 8u32;
pub const JET_bitTableCreateNoFixedVarColumnsInDerivedTables: u32 = 4u32;
pub const JET_bitTableCreateTemplateTable: u32 = 2u32;
pub const JET_bitTableDenyRead: u32 = 2u32;
pub const JET_bitTableDenyWrite: u32 = 1u32;
pub const JET_bitTableInfoBookmark: u32 = 2u32;
pub const JET_bitTableInfoRollback: u32 = 4u32;
pub const JET_bitTableInfoUpdatable: u32 = 1u32;
pub const JET_bitTableNoCache: u32 = 32u32;
pub const JET_bitTableOpportuneRead: u32 = 128u32;
pub const JET_bitTablePermitDDL: u32 = 16u32;
pub const JET_bitTablePreread: u32 = 64u32;
pub const JET_bitTableReadOnly: u32 = 4u32;
pub const JET_bitTableSequential: u32 = 32768u32;
pub const JET_bitTableUpdatable: u32 = 8u32;
pub const JET_bitTermAbrupt: u32 = 2u32;
pub const JET_bitTermComplete: u32 = 1u32;
pub const JET_bitTermDirty: u32 = 8u32;
pub const JET_bitTermStopBackup: u32 = 4u32;
pub const JET_bitTransactionReadOnly: u32 = 1u32;
pub const JET_bitTruncateLogsAfterRecovery: u32 = 16u32;
pub const JET_bitUpdateCheckESE97Compatibility: u32 = 1u32;
pub const JET_bitWaitAllLevel0Commit: u32 = 8u32;
pub const JET_bitWaitLastLevel0Commit: u32 = 2u32;
pub const JET_bitWriteLock: u32 = 2u32;
pub const JET_bitZeroLength: u32 = 1u32;
pub const JET_cbBookmarkMost: u32 = 256u32;
pub const JET_cbColumnLVPageOverhead: u32 = 82u32;
pub const JET_cbColumnMost: u32 = 255u32;
pub const JET_cbFullNameMost: u32 = 255u32;
pub const JET_cbKeyMost: u32 = 255u32;
pub const JET_cbKeyMost2KBytePage: u32 = 500u32;
pub const JET_cbKeyMost4KBytePage: u32 = 1000u32;
pub const JET_cbKeyMost8KBytePage: u32 = 2000u32;
pub const JET_cbKeyMostMin: u32 = 255u32;
pub const JET_cbLVColumnMost: u32 = 2147483647u32;
pub const JET_cbLVDefaultValueMost: u32 = 255u32;
pub const JET_cbLimitKeyMost: u32 = 256u32;
pub const JET_cbNameMost: u32 = 64u32;
pub const JET_cbPrimaryKeyMost: u32 = 255u32;
pub const JET_cbSecondaryKeyMost: u32 = 255u32;
pub const JET_cbtypAfterDelete: u32 = 64u32;
pub const JET_cbtypAfterInsert: u32 = 4u32;
pub const JET_cbtypAfterReplace: u32 = 16u32;
pub const JET_cbtypBeforeDelete: u32 = 32u32;
pub const JET_cbtypBeforeInsert: u32 = 2u32;
pub const JET_cbtypBeforeReplace: u32 = 8u32;
pub const JET_cbtypFinalize: u32 = 1u32;
pub const JET_cbtypFreeCursorLS: u32 = 512u32;
pub const JET_cbtypFreeTableLS: u32 = 1024u32;
pub const JET_cbtypNull: u32 = 0u32;
pub const JET_cbtypOnlineDefragCompleted: u32 = 256u32;
pub const JET_cbtypUserDefinedDefaultValue: u32 = 128u32;
pub const JET_ccolFixedMost: u32 = 127u32;
pub const JET_ccolKeyMost: u32 = 16u32;
pub const JET_ccolMost: u32 = 65248u32;
pub const JET_ccolTaggedMost: u32 = 64993u32;
pub const JET_ccolVarMost: u32 = 128u32;
pub const JET_coltypBinary: u32 = 9u32;
pub const JET_coltypBit: u32 = 1u32;
pub const JET_coltypCurrency: u32 = 5u32;
pub const JET_coltypDateTime: u32 = 8u32;
pub const JET_coltypGUID: u32 = 16u32;
pub const JET_coltypIEEEDouble: u32 = 7u32;
pub const JET_coltypIEEESingle: u32 = 6u32;
pub const JET_coltypLong: u32 = 4u32;
pub const JET_coltypLongBinary: u32 = 11u32;
pub const JET_coltypLongLong: u32 = 15u32;
pub const JET_coltypLongText: u32 = 12u32;
pub const JET_coltypMax: u32 = 13u32;
pub const JET_coltypNil: u32 = 0u32;
pub const JET_coltypSLV: u32 = 13u32;
pub const JET_coltypShort: u32 = 3u32;
pub const JET_coltypText: u32 = 10u32;
pub const JET_coltypUnsignedByte: u32 = 2u32;
pub const JET_coltypUnsignedLong: u32 = 14u32;
pub const JET_coltypUnsignedLongLong: u32 = 18u32;
pub const JET_coltypUnsignedShort: u32 = 17u32;
pub const JET_configDefault: u32 = 1u32;
pub const JET_configDynamicMediumMemory: u32 = 32u32;
pub const JET_configHighConcurrencyScaling: u32 = 1024u32;
pub const JET_configLowDiskFootprint: u32 = 4u32;
pub const JET_configLowMemory: u32 = 16u32;
pub const JET_configLowPower: u32 = 64u32;
pub const JET_configMediumDiskFootprint: u32 = 8u32;
pub const JET_configRemoveQuotas: u32 = 2u32;
pub const JET_configRunSilent: u32 = 256u32;
pub const JET_configSSDProfileIO: u32 = 128u32;
pub const JET_configUnthrottledMemory: u32 = 512u32;
pub const JET_dbstateBeingConverted: u32 = 4u32;
pub const JET_dbstateCleanShutdown: u32 = 3u32;
pub const JET_dbstateDirtyShutdown: u32 = 2u32;
pub const JET_dbstateForceDetach: u32 = 5u32;
pub const JET_dbstateJustCreated: u32 = 1u32;
pub const JET_errAccessDenied: i32 = -1907i32;
pub const JET_errAfterInitialization: i32 = -1850i32;
pub const JET_errAlreadyInitialized: i32 = -1030i32;
pub const JET_errAlreadyPrepared: i32 = -1607i32;
pub const JET_errAttachedDatabaseMismatch: i32 = -1216i32;
pub const JET_errBackupAbortByServer: i32 = -801i32;
pub const JET_errBackupDirectoryNotEmpty: i32 = -504i32;
pub const JET_errBackupInProgress: i32 = -505i32;
pub const JET_errBackupNotAllowedYet: i32 = -523i32;
pub const JET_errBadBackupDatabaseSize: i32 = -561i32;
pub const JET_errBadBookmark: i32 = -328i32;
pub const JET_errBadCheckpointSignature: i32 = -532i32;
pub const JET_errBadColumnId: i32 = -1517i32;
pub const JET_errBadDbSignature: i32 = -531i32;
pub const JET_errBadEmptyPage: i32 = -351i32;
pub const JET_errBadItagSequence: i32 = -1518i32;
pub const JET_errBadLineCount: i32 = -354i32;
pub const JET_errBadLogSignature: i32 = -530i32;
pub const JET_errBadLogVersion: i32 = -514i32;
pub const JET_errBadPageLink: i32 = -327i32;
pub const JET_errBadParentPageLink: i32 = -338i32;
pub const JET_errBadPatchPage: i32 = -535i32;
pub const JET_errBadRestoreTargetInstance: i32 = -577i32;
pub const JET_errBufferTooSmall: i32 = -1038i32;
pub const JET_errCallbackFailed: i32 = -2101i32;
pub const JET_errCallbackNotResolved: i32 = -2102i32;
pub const JET_errCannotAddFixedVarColumnToDerivedTable: i32 = -1330i32;
pub const JET_errCannotBeTagged: i32 = -1521i32;
pub const JET_errCannotDeleteSystemTable: i32 = -1318i32;
pub const JET_errCannotDeleteTempTable: i32 = -1317i32;
pub const JET_errCannotDeleteTemplateTable: i32 = -1319i32;
pub const JET_errCannotDisableVersioning: i32 = -1208i32;
pub const JET_errCannotIndex: i32 = -1071i32;
pub const JET_errCannotIndexOnEncryptedColumn: i32 = -1440i32;
pub const JET_errCannotLogDuringRecoveryRedo: i32 = -512i32;
pub const JET_errCannotMaterializeForwardOnlySort: i32 = -1113i32;
pub const JET_errCannotNestDDL: i32 = -1325i32;
pub const JET_errCannotSeparateIntrinsicLV: i32 = -416i32;
pub const JET_errCatalogCorrupted: i32 = -1220i32;
pub const JET_errCheckpointCorrupt: i32 = -533i32;
pub const JET_errCheckpointDepthTooDeep: i32 = -614i32;
pub const JET_errCheckpointFileNotFound: i32 = -542i32;
pub const JET_errClientRequestToStopJetService: i32 = -1329i32;
pub const JET_errColumnCannotBeCompressed: i32 = -1538i32;
pub const JET_errColumnCannotBeEncrypted: i32 = -1439i32;
pub const JET_errColumnDoesNotFit: i32 = -1503i32;
pub const JET_errColumnDuplicate: i32 = -1508i32;
pub const JET_errColumnInRelationship: i32 = -1519i32;
pub const JET_errColumnInUse: i32 = -1046i32;
pub const JET_errColumnIndexed: i32 = -1505i32;
pub const JET_errColumnLong: i32 = -1501i32;
pub const JET_errColumnNoChunk: i32 = -1502i32;
pub const JET_errColumnNoEncryptionKey: i32 = -1540i32;
pub const JET_errColumnNotFound: i32 = -1507i32;
pub const JET_errColumnNotUpdatable: i32 = -1048i32;
pub const JET_errColumnRedundant: i32 = -1510i32;
pub const JET_errColumnTooBig: i32 = -1506i32;
pub const JET_errCommittedLogFileCorrupt: i32 = -586i32;
pub const JET_errCommittedLogFilesMissing: i32 = -582i32;
pub const JET_errConsistentTimeMismatch: i32 = -551i32;
pub const JET_errContainerNotEmpty: i32 = -1043i32;
pub const JET_errDDLNotInheritable: i32 = -1326i32;
pub const JET_errDataHasChanged: i32 = -1611i32;
pub const JET_errDatabase200Format: i32 = -1210i32;
pub const JET_errDatabase400Format: i32 = -1211i32;
pub const JET_errDatabase500Format: i32 = -1212i32;
pub const JET_errDatabaseAlreadyRunningMaintenance: i32 = -2004i32;
pub const JET_errDatabaseAlreadyUpgraded: i32 = -562i32;
pub const JET_errDatabaseAttachedForRecovery: i32 = -1231i32;
pub const JET_errDatabaseBufferDependenciesCorrupted: i32 = -255i32;
pub const JET_errDatabaseCorrupted: i32 = -1206i32;
pub const JET_errDatabaseCorruptedNoRepair: i32 = -1224i32;
pub const JET_errDatabaseDirtyShutdown: i32 = -550i32;
pub const JET_errDatabaseDuplicate: i32 = -1201i32;
pub const JET_errDatabaseFileReadOnly: i32 = -1008i32;
pub const JET_errDatabaseIdInUse: i32 = -1218i32;
pub const JET_errDatabaseInUse: i32 = -1202i32;
pub const JET_errDatabaseIncompleteUpgrade: i32 = -563i32;
pub const JET_errDatabaseInconsistent: i32 = -550i32;
pub const JET_errDatabaseInvalidName: i32 = -1204i32;
pub const JET_errDatabaseInvalidPages: i32 = -1205i32;
pub const JET_errDatabaseInvalidPath: i32 = -1217i32;
pub const JET_errDatabaseLeakInSpace: i32 = -348i32;
pub const JET_errDatabaseLocked: i32 = -1207i32;
pub const JET_errDatabaseLogSetMismatch: i32 = -539i32;
pub const JET_errDatabaseNotFound: i32 = -1203i32;
pub const JET_errDatabaseNotReady: i32 = -1230i32;
pub const JET_errDatabasePatchFileMismatch: i32 = -552i32;
pub const JET_errDatabaseSharingViolation: i32 = -1215i32;
pub const JET_errDatabaseSignInUse: i32 = -1222i32;
pub const JET_errDatabaseStreamingFileMismatch: i32 = -540i32;
pub const JET_errDatabaseUnavailable: i32 = -1091i32;
pub const JET_errDatabasesNotFromSameSnapshot: i32 = -580i32;
pub const JET_errDbTimeBeyondMaxRequired: i32 = -625i32;
pub const JET_errDbTimeCorrupted: i32 = -344i32;
pub const JET_errDbTimeTooNew: i32 = -567i32;
pub const JET_errDbTimeTooOld: i32 = -566i32;
pub const JET_errDecompressionFailed: i32 = -1620i32;
pub const JET_errDecryptionFailed: i32 = -1622i32;
pub const JET_errDefaultValueTooBig: i32 = -1524i32;
pub const JET_errDeleteBackupFileFail: i32 = -524i32;
pub const JET_errDensityInvalid: i32 = -1307i32;
pub const JET_errDerivedColumnCorruption: i32 = -1529i32;
pub const JET_errDirtyShutdown: i32 = -1116i32;
pub const JET_errDisabledFunctionality: i32 = -112i32;
pub const JET_errDiskFull: i32 = -1808i32;
pub const JET_errDiskIO: i32 = -1022i32;
pub const JET_errDiskReadVerificationFailure: i32 = -1021i32;
pub const JET_errEncryptionBadItag: i32 = -1623i32;
pub const JET_errEndingRestoreLogTooLow: i32 = -553i32;
pub const JET_errEngineFormatVersionNoLongerSupportedTooLow: i32 = -619i32;
pub const JET_errEngineFormatVersionNotYetImplementedTooHigh: i32 = -620i32;
pub const JET_errEngineFormatVersionParamTooLowForRequestedFeature: i32 = -621i32;
pub const JET_errEngineFormatVersionSpecifiedTooLowForDatabaseVersion: i32 = -623i32;
pub const JET_errEngineFormatVersionSpecifiedTooLowForLogVersion: i32 = -622i32;
pub const JET_errEntryPointNotFound: i32 = -1911i32;
pub const JET_errExclusiveTableLockRequired: i32 = -1322i32;
pub const JET_errExistingLogFileHasBadSignature: i32 = -610i32;
pub const JET_errExistingLogFileIsNotContiguous: i32 = -611i32;
pub const JET_errFeatureNotAvailable: i32 = -1001i32;
pub const JET_errFileAccessDenied: i32 = -1032i32;
pub const JET_errFileAlreadyExists: i32 = -1814i32;
pub const JET_errFileClose: i32 = -102i32;
pub const JET_errFileCompressed: i32 = -4005i32;
pub const JET_errFileIOAbort: i32 = -4002i32;
pub const JET_errFileIOBeyondEOF: i32 = -4001i32;
pub const JET_errFileIOFail: i32 = -4004i32;
pub const JET_errFileIORetry: i32 = -4003i32;
pub const JET_errFileIOSparse: i32 = -4000i32;
pub const JET_errFileInvalidType: i32 = -1812i32;
pub const JET_errFileNotFound: i32 = -1811i32;
pub const JET_errFileSystemCorruption: i32 = -1121i32;
pub const JET_errFilteredMoveNotSupported: i32 = -1124i32;
pub const JET_errFixedDDL: i32 = -1323i32;
pub const JET_errFixedInheritedDDL: i32 = -1324i32;
pub const JET_errFlushMapDatabaseMismatch: i32 = -1919i32;
pub const JET_errFlushMapUnrecoverable: i32 = -1920i32;
pub const JET_errFlushMapVersionUnsupported: i32 = -1918i32;
pub const JET_errForceDetachNotAllowed: i32 = -1219i32;
pub const JET_errGivenLogFileHasBadSignature: i32 = -555i32;
pub const JET_errGivenLogFileIsNotContiguous: i32 = -556i32;
pub const JET_errIllegalOperation: i32 = -1312i32;
pub const JET_errInTransaction: i32 = -1108i32;
pub const JET_errIndexBuildCorrupted: i32 = -1412i32;
pub const JET_errIndexCantBuild: i32 = -1401i32;
pub const JET_errIndexDuplicate: i32 = -1403i32;
pub const JET_errIndexHasPrimary: i32 = -1402i32;
pub const JET_errIndexInUse: i32 = -1051i32;
pub const JET_errIndexInvalidDef: i32 = -1406i32;
pub const JET_errIndexMustStay: i32 = -1405i32;
pub const JET_errIndexNotFound: i32 = -1404i32;
pub const JET_errIndexTuplesCannotRetrieveFromIndex: i32 = -1436i32;
pub const JET_errIndexTuplesInvalidLimits: i32 = -1435i32;
pub const JET_errIndexTuplesKeyTooSmall: i32 = -1437i32;
pub const JET_errIndexTuplesNonUniqueOnly: i32 = -1432i32;
pub const JET_errIndexTuplesOneColumnOnly: i32 = -1431i32;
pub const JET_errIndexTuplesSecondaryIndexOnly: i32 = -1430i32;
pub const JET_errIndexTuplesTextBinaryColumnsOnly: i32 = -1433i32;
pub const JET_errIndexTuplesTextColumnsOnly: i32 = -1433i32;
pub const JET_errIndexTuplesTooManyColumns: i32 = -1431i32;
pub const JET_errIndexTuplesVarSegMacNotAllowed: i32 = -1434i32;
pub const JET_errInitInProgress: i32 = -1031i32;
pub const JET_errInstanceNameInUse: i32 = -1086i32;
pub const JET_errInstanceUnavailable: i32 = -1090i32;
pub const JET_errInstanceUnavailableDueToFatalLogDiskFull: i32 = -1092i32;
pub const JET_errInternalError: i32 = -107i32;
pub const JET_errInvalidBackup: i32 = -526i32;
pub const JET_errInvalidBackupSequence: i32 = -521i32;
pub const JET_errInvalidBookmark: i32 = -1045i32;
pub const JET_errInvalidBufferSize: i32 = -1047i32;
pub const JET_errInvalidCodePage: i32 = -1063i32;
pub const JET_errInvalidColumnType: i32 = -1511i32;
pub const JET_errInvalidCountry: i32 = -1061i32;
pub const JET_errInvalidCreateDbVersion: i32 = -1225i32;
pub const JET_errInvalidCreateIndex: i32 = -1409i32;
pub const JET_errInvalidDatabase: i32 = -1028i32;
pub const JET_errInvalidDatabaseId: i32 = -1010i32;
pub const JET_errInvalidDatabaseVersion: i32 = -1209i32;
pub const JET_errInvalidDbparamId: i32 = -1095i32;
pub const JET_errInvalidFilename: i32 = -1044i32;
pub const JET_errInvalidGrbit: i32 = -900i32;
pub const JET_errInvalidIndexId: i32 = -1416i32;
pub const JET_errInvalidInstance: i32 = -1115i32;
pub const JET_errInvalidLCMapStringFlags: i32 = -1064i32;
pub const JET_errInvalidLVChunkSize: i32 = -1438i32;
pub const JET_errInvalidLanguageId: i32 = -1062i32;
pub const JET_errInvalidLogDirectory: i32 = -1025i32;
pub const JET_errInvalidLogSequence: i32 = -515i32;
pub const JET_errInvalidLoggedOperation: i32 = -500i32;
pub const JET_errInvalidName: i32 = -1002i32;
pub const JET_errInvalidObject: i32 = -1316i32;
pub const JET_errInvalidOnSort: i32 = -1702i32;
pub const JET_errInvalidOperation: i32 = -1906i32;
pub const JET_errInvalidParameter: i32 = -1003i32;
pub const JET_errInvalidPath: i32 = -1023i32;
pub const JET_errInvalidPlaceholderColumn: i32 = -1530i32;
pub const JET_errInvalidPreread: i32 = -424i32;
pub const JET_errInvalidSesid: i32 = -1104i32;
pub const JET_errInvalidSesparamId: i32 = -1093i32;
pub const JET_errInvalidSettings: i32 = -1328i32;
pub const JET_errInvalidSystemPath: i32 = -1024i32;
pub const JET_errInvalidTableId: i32 = -1310i32;
pub const JET_errKeyBoundary: i32 = -324i32;
pub const JET_errKeyDuplicate: i32 = -1605i32;
pub const JET_errKeyIsMade: i32 = -1516i32;
pub const JET_errKeyNotMade: i32 = -1608i32;
pub const JET_errKeyTooBig: i32 = -408i32;
pub const JET_errKeyTruncated: i32 = -346i32;
pub const JET_errLSAlreadySet: i32 = -3001i32;
pub const JET_errLSCallbackNotSpecified: i32 = -3000i32;
pub const JET_errLSNotSet: i32 = -3002i32;
pub const JET_errLVCorrupted: i32 = -1526i32;
pub const JET_errLanguageNotSupported: i32 = -1619i32;
pub const JET_errLinkNotSupported: i32 = -1052i32;
pub const JET_errLogBufferTooSmall: i32 = -517i32;
pub const JET_errLogCorruptDuringHardRecovery: i32 = -574i32;
pub const JET_errLogCorruptDuringHardRestore: i32 = -573i32;
pub const JET_errLogCorrupted: i32 = -1852i32;
pub const JET_errLogDisabledDueToRecoveryFailure: i32 = -511i32;
pub const JET_errLogDiskFull: i32 = -529i32;
pub const JET_errLogFileCorrupt: i32 = -501i32;
pub const JET_errLogFileNotCopied: i32 = -616i32;
pub const JET_errLogFilePathInUse: i32 = -1084i32;
pub const JET_errLogFileSizeMismatch: i32 = -541i32;
pub const JET_errLogFileSizeMismatchDatabasesConsistent: i32 = -545i32;
pub const JET_errLogGenerationMismatch: i32 = -513i32;
pub const JET_errLogReadVerifyFailure: i32 = -612i32;
pub const JET_errLogSectorSizeMismatch: i32 = -546i32;
pub const JET_errLogSectorSizeMismatchDatabasesConsistent: i32 = -547i32;
pub const JET_errLogSequenceChecksumMismatch: i32 = -590i32;
pub const JET_errLogSequenceEnd: i32 = -519i32;
pub const JET_errLogSequenceEndDatabasesConsistent: i32 = -548i32;
pub const JET_errLogTornWriteDuringHardRecovery: i32 = -571i32;
pub const JET_errLogTornWriteDuringHardRestore: i32 = -570i32;
pub const JET_errLogWriteFail: i32 = -510i32;
pub const JET_errLoggingDisabled: i32 = -516i32;
pub const JET_errMakeBackupDirectoryFail: i32 = -525i32;
pub const JET_errMissingCurrentLogFiles: i32 = -565i32;
pub const JET_errMissingFileToBackup: i32 = -569i32;
pub const JET_errMissingFullBackup: i32 = -560i32;
pub const JET_errMissingLogFile: i32 = -528i32;
pub const JET_errMissingPatchPage: i32 = -534i32;
pub const JET_errMissingPreviousLogFile: i32 = -509i32;
pub const JET_errMissingRestoreLogFiles: i32 = -557i32;
pub const JET_errMultiValuedColumnMustBeTagged: i32 = -1509i32;
pub const JET_errMultiValuedDuplicate: i32 = -1525i32;
pub const JET_errMultiValuedDuplicateAfterTruncation: i32 = -1528i32;
pub const JET_errMultiValuedIndexViolation: i32 = -1411i32;
pub const JET_errMustBeSeparateLongValue: i32 = -423i32;
pub const JET_errMustDisableLoggingForDbUpgrade: i32 = -575i32;
pub const JET_errMustRollback: i32 = -1057i32;
pub const JET_errNTSystemCallFailed: i32 = -334i32;
pub const JET_errNoBackup: i32 = -520i32;
pub const JET_errNoBackupDirectory: i32 = -503i32;
pub const JET_errNoCurrentIndex: i32 = -1515i32;
pub const JET_errNoCurrentRecord: i32 = -1603i32;
pub const JET_errNodeCorrupted: i32 = -358i32;
pub const JET_errNotInTransaction: i32 = -1054i32;
pub const JET_errNotInitialized: i32 = -1029i32;
pub const JET_errNullInvalid: i32 = -1504i32;
pub const JET_errNullKeyDisallowed: i32 = -1053i32;
pub const JET_errOSSnapshotInvalidSequence: i32 = -2401i32;
pub const JET_errOSSnapshotInvalidSnapId: i32 = -2404i32;
pub const JET_errOSSnapshotNotAllowed: i32 = -2403i32;
pub const JET_errOSSnapshotTimeOut: i32 = -2402i32;
pub const JET_errObjectDuplicate: i32 = -1314i32;
pub const JET_errObjectNotFound: i32 = -1305i32;
pub const JET_errOneDatabasePerSession: i32 = -1916i32;
pub const JET_errOutOfAutoincrementValues: i32 = -1076i32;
pub const JET_errOutOfBuffers: i32 = -1014i32;
pub const JET_errOutOfCursors: i32 = -1013i32;
pub const JET_errOutOfDatabaseSpace: i32 = -1012i32;
pub const JET_errOutOfDbtimeValues: i32 = -1077i32;
pub const JET_errOutOfFileHandles: i32 = -1020i32;
pub const JET_errOutOfLongValueIDs: i32 = -1075i32;
pub const JET_errOutOfMemory: i32 = -1011i32;
pub const JET_errOutOfObjectIDs: i32 = -1074i32;
pub const JET_errOutOfSequentialIndexValues: i32 = -1078i32;
pub const JET_errOutOfSessions: i32 = -1101i32;
pub const JET_errOutOfThreads: i32 = -103i32;
pub const JET_errPageBoundary: i32 = -323i32;
pub const JET_errPageInitializedMismatch: i32 = -596i32;
pub const JET_errPageNotInitialized: i32 = -1019i32;
pub const JET_errPageSizeMismatch: i32 = -1213i32;
pub const JET_errPageTagCorrupted: i32 = -357i32;
pub const JET_errPartiallyAttachedDB: i32 = -1221i32;
pub const JET_errPatchFileMissing: i32 = -538i32;
pub const JET_errPermissionDenied: i32 = -1809i32;
pub const JET_errPreviousVersion: i32 = -322i32;
pub const JET_errPrimaryIndexCorrupted: i32 = -1413i32;
pub const JET_errReadLostFlushVerifyFailure: i32 = -1119i32;
pub const JET_errReadPgnoVerifyFailure: i32 = -1118i32;
pub const JET_errReadVerifyFailure: i32 = -1018i32;
pub const JET_errRecordDeleted: i32 = -1017i32;
pub const JET_errRecordFormatConversionFailed: i32 = -1915i32;
pub const JET_errRecordNoCopy: i32 = -1602i32;
pub const JET_errRecordNotDeleted: i32 = -1072i32;
pub const JET_errRecordNotFound: i32 = -1601i32;
pub const JET_errRecordPrimaryChanged: i32 = -1604i32;
pub const JET_errRecordTooBig: i32 = -1026i32;
pub const JET_errRecordTooBigForBackwardCompatibility: i32 = -1112i32;
pub const JET_errRecoveredWithErrors: i32 = -527i32;
pub const JET_errRecoveredWithoutUndo: i32 = -579i32;
pub const JET_errRecoveredWithoutUndoDatabasesConsistent: i32 = -584i32;
pub const JET_errRecoveryVerifyFailure: i32 = -1123i32;
pub const JET_errRedoAbruptEnded: i32 = -536i32;
pub const JET_errRequiredLogFilesMissing: i32 = -543i32;
pub const JET_errRestoreInProgress: i32 = -506i32;
pub const JET_errRestoreOfNonBackupDatabase: i32 = -615i32;
pub const JET_errRfsFailure: i32 = -100i32;
pub const JET_errRfsNotArmed: i32 = -101i32;
pub const JET_errRollbackError: i32 = -1917i32;
pub const JET_errRollbackRequired: i32 = -1109i32;
pub const JET_errRunningInMultiInstanceMode: i32 = -1081i32;
pub const JET_errRunningInOneInstanceMode: i32 = -1080i32;
pub const JET_errSPAvailExtCacheOutOfMemory: i32 = -342i32;
pub const JET_errSPAvailExtCacheOutOfSync: i32 = -340i32;
pub const JET_errSPAvailExtCorrupted: i32 = -341i32;
pub const JET_errSPOwnExtCorrupted: i32 = -343i32;
pub const JET_errSecondaryIndexCorrupted: i32 = -1414i32;
pub const JET_errSectorSizeNotSupported: i32 = -583i32;
pub const JET_errSeparatedLongValue: i32 = -421i32;
pub const JET_errSesidTableIdMismatch: i32 = -1114i32;
pub const JET_errSessionContextAlreadySet: i32 = -1912i32;
pub const JET_errSessionContextNotSetByThisThread: i32 = -1913i32;
pub const JET_errSessionInUse: i32 = -1914i32;
pub const JET_errSessionSharingViolation: i32 = -1910i32;
pub const JET_errSessionWriteConflict: i32 = -1111i32;
pub const JET_errSoftRecoveryOnBackupDatabase: i32 = -544i32;
pub const JET_errSoftRecoveryOnSnapshot: i32 = -581i32;
pub const JET_errSpaceHintsInvalid: i32 = -2103i32;
pub const JET_errStartingRestoreLogTooHigh: i32 = -554i32;
pub const JET_errStreamingDataNotLogged: i32 = -549i32;
pub const JET_errSuccess: i32 = 0i32;
pub const JET_errSystemParameterConflict: i32 = -1087i32;
pub const JET_errSystemParamsAlreadySet: i32 = -1082i32;
pub const JET_errSystemPathInUse: i32 = -1083i32;
pub const JET_errTableDuplicate: i32 = -1303i32;
pub const JET_errTableInUse: i32 = -1304i32;
pub const JET_errTableLocked: i32 = -1302i32;
pub const JET_errTableNotEmpty: i32 = -1308i32;
pub const JET_errTaggedNotNULL: i32 = -1514i32;
pub const JET_errTaskDropped: i32 = -106i32;
pub const JET_errTempFileOpenError: i32 = -1803i32;
pub const JET_errTempPathInUse: i32 = -1085i32;
pub const JET_errTermInProgress: i32 = -1000i32;
pub const JET_errTooManyActiveUsers: i32 = -1059i32;
pub const JET_errTooManyAttachedDatabases: i32 = -1805i32;
pub const JET_errTooManyColumns: i32 = -1040i32;
pub const JET_errTooManyIO: i32 = -105i32;
pub const JET_errTooManyIndexes: i32 = -1015i32;
pub const JET_errTooManyInstances: i32 = -1214i32;
pub const JET_errTooManyKeys: i32 = -1016i32;
pub const JET_errTooManyMempoolEntries: i32 = -1073i32;
pub const JET_errTooManyOpenDatabases: i32 = -1027i32;
pub const JET_errTooManyOpenIndexes: i32 = -1410i32;
pub const JET_errTooManyOpenTables: i32 = -1311i32;
pub const JET_errTooManyOpenTablesAndCleanupTimedOut: i32 = -1313i32;
pub const JET_errTooManyRecords: i32 = -1094i32;
pub const JET_errTooManySorts: i32 = -1701i32;
pub const JET_errTooManySplits: i32 = -1909i32;
pub const JET_errTransReadOnly: i32 = -1110i32;
pub const JET_errTransTooDeep: i32 = -1103i32;
pub const JET_errTransactionTooLong: i32 = -618i32;
pub const JET_errTransactionsNotReadyDuringRecovery: i32 = -1232i32;
pub const JET_errUnicodeLanguageValidationFailure: i32 = -604i32;
pub const JET_errUnicodeNormalizationNotSupported: i32 = -603i32;
pub const JET_errUnicodeTranslationBufferTooSmall: i32 = -601i32;
pub const JET_errUnicodeTranslationFail: i32 = -602i32;
pub const JET_errUnloadableOSFunctionality: i32 = -113i32;
pub const JET_errUpdateMustVersion: i32 = -1621i32;
pub const JET_errUpdateNotPrepared: i32 = -1609i32;
pub const JET_errVersionStoreEntryTooBig: i32 = -1065i32;
pub const JET_errVersionStoreOutOfMemory: i32 = -1069i32;
pub const JET_errVersionStoreOutOfMemoryAndCleanupTimedOut: i32 = -1066i32;
pub const JET_errWriteConflict: i32 = -1102i32;
pub const JET_errWriteConflictPrimaryIndex: i32 = -1105i32;
pub const JET_errcatApi: JET_ERRCAT = JET_ERRCAT(13i32);
pub const JET_errcatCorruption: JET_ERRCAT = JET_ERRCAT(10i32);
pub const JET_errcatData: JET_ERRCAT = JET_ERRCAT(9i32);
pub const JET_errcatDisk: JET_ERRCAT = JET_ERRCAT(8i32);
pub const JET_errcatError: JET_ERRCAT = JET_ERRCAT(1i32);
pub const JET_errcatFatal: JET_ERRCAT = JET_ERRCAT(3i32);
pub const JET_errcatFragmentation: JET_ERRCAT = JET_ERRCAT(12i32);
pub const JET_errcatIO: JET_ERRCAT = JET_ERRCAT(4i32);
pub const JET_errcatInconsistent: JET_ERRCAT = JET_ERRCAT(11i32);
pub const JET_errcatMax: JET_ERRCAT = JET_ERRCAT(17i32);
pub const JET_errcatMemory: JET_ERRCAT = JET_ERRCAT(6i32);
pub const JET_errcatObsolete: JET_ERRCAT = JET_ERRCAT(16i32);
pub const JET_errcatOperation: JET_ERRCAT = JET_ERRCAT(2i32);
pub const JET_errcatQuota: JET_ERRCAT = JET_ERRCAT(7i32);
pub const JET_errcatResource: JET_ERRCAT = JET_ERRCAT(5i32);
pub const JET_errcatState: JET_ERRCAT = JET_ERRCAT(15i32);
pub const JET_errcatUnknown: JET_ERRCAT = JET_ERRCAT(0i32);
pub const JET_errcatUsage: JET_ERRCAT = JET_ERRCAT(14i32);
pub const JET_filetypeCheckpoint: u32 = 4u32;
pub const JET_filetypeDatabase: u32 = 1u32;
pub const JET_filetypeFlushMap: u32 = 7u32;
pub const JET_filetypeLog: u32 = 3u32;
pub const JET_filetypeTempDatabase: u32 = 5u32;
pub const JET_filetypeUnknown: u32 = 0u32;
pub const JET_objtypNil: u32 = 0u32;
pub const JET_objtypTable: u32 = 1u32;
pub const JET_paramAccessDeniedRetryPeriod: u32 = 53u32;
pub const JET_paramAlternateDatabaseRecoveryPath: u32 = 113u32;
pub const JET_paramBaseName: u32 = 3u32;
pub const JET_paramBatchIOBufferMax: u32 = 22u32;
pub const JET_paramCachePriority: u32 = 177u32;
pub const JET_paramCacheSize: u32 = 41u32;
pub const JET_paramCacheSizeMax: u32 = 23u32;
pub const JET_paramCacheSizeMin: u32 = 60u32;
pub const JET_paramCachedClosedTables: u32 = 125u32;
pub const JET_paramCheckFormatWhenOpenFail: u32 = 44u32;
pub const JET_paramCheckpointDepthMax: u32 = 24u32;
pub const JET_paramCheckpointIOMax: u32 = 135u32;
pub const JET_paramCircularLog: u32 = 17u32;
pub const JET_paramCleanupMismatchedLogFiles: u32 = 77u32;
pub const JET_paramCommitDefault: u32 = 16u32;
pub const JET_paramConfigStoreSpec: u32 = 189u32;
pub const JET_paramConfiguration: u32 = 129u32;
pub const JET_paramCreatePathIfNotExist: u32 = 100u32;
pub const JET_paramDatabasePageSize: u32 = 64u32;
pub const JET_paramDbExtensionSize: u32 = 18u32;
pub const JET_paramDbScanIntervalMaxSec: u32 = 172u32;
pub const JET_paramDbScanIntervalMinSec: u32 = 171u32;
pub const JET_paramDbScanThrottle: u32 = 170u32;
pub const JET_paramDefragmentSequentialBTrees: u32 = 160u32;
pub const JET_paramDefragmentSequentialBTreesDensityCheckFrequency: u32 = 161u32;
pub const JET_paramDeleteOldLogs: u32 = 48u32;
pub const JET_paramDeleteOutOfRangeLogs: u32 = 52u32;
pub const JET_paramDisableCallbacks: u32 = 65u32;
pub const JET_paramDisablePerfmon: u32 = 107u32;
pub const JET_paramDurableCommitCallback: u32 = 187u32;
pub const JET_paramEnableAdvanced: u32 = 130u32;
pub const JET_paramEnableDBScanInRecovery: u32 = 169u32;
pub const JET_paramEnableDBScanSerialization: u32 = 180u32;
pub const JET_paramEnableFileCache: u32 = 126u32;
pub const JET_paramEnableIndexChecking: u32 = 45u32;
pub const JET_paramEnableIndexCleanup: u32 = 54u32;
pub const JET_paramEnableOnlineDefrag: u32 = 35u32;
pub const JET_paramEnablePersistedCallbacks: u32 = 156u32;
pub const JET_paramEnableRBS: u32 = 215u32;
pub const JET_paramEnableShrinkDatabase: u32 = 184u32;
pub const JET_paramEnableSqm: u32 = 188u32;
pub const JET_paramEnableTempTableVersioning: u32 = 46u32;
pub const JET_paramEnableViewCache: u32 = 127u32;
pub const JET_paramErrorToString: u32 = 70u32;
pub const JET_paramEventLogCache: u32 = 99u32;
pub const JET_paramEventLoggingLevel: u32 = 51u32;
pub const JET_paramEventSource: u32 = 4u32;
pub const JET_paramEventSourceKey: u32 = 49u32;
pub const JET_paramExceptionAction: u32 = 98u32;
pub const JET_paramGlobalMinVerPages: u32 = 81u32;
pub const JET_paramHungIOActions: u32 = 182u32;
pub const JET_paramHungIOThreshold: u32 = 181u32;
pub const JET_paramIOPriority: u32 = 152u32;
pub const JET_paramIOThrottlingTimeQuanta: u32 = 162u32;
pub const JET_paramIgnoreLogVersion: u32 = 47u32;
pub const JET_paramIndexTupleIncrement: u32 = 132u32;
pub const JET_paramIndexTupleStart: u32 = 133u32;
pub const JET_paramIndexTuplesLengthMax: u32 = 111u32;
pub const JET_paramIndexTuplesLengthMin: u32 = 110u32;
pub const JET_paramIndexTuplesToIndexMax: u32 = 112u32;
pub const JET_paramKeyMost: u32 = 134u32;
pub const JET_paramLRUKCorrInterval: u32 = 25u32;
pub const JET_paramLRUKHistoryMax: u32 = 26u32;
pub const JET_paramLRUKPolicy: u32 = 27u32;
pub const JET_paramLRUKTimeout: u32 = 28u32;
pub const JET_paramLRUKTrxCorrInterval: u32 = 29u32;
pub const JET_paramLVChunkSizeMost: u32 = 163u32;
pub const JET_paramLegacyFileNames: u32 = 136u32;
pub const JET_paramLogBuffers: u32 = 12u32;
pub const JET_paramLogCheckpointPeriod: u32 = 14u32;
pub const JET_paramLogFileCreateAsynch: u32 = 69u32;
pub const JET_paramLogFilePath: u32 = 2u32;
pub const JET_paramLogFileSize: u32 = 11u32;
pub const JET_paramLogWaitingUserMax: u32 = 15u32;
pub const JET_paramMaxCoalesceReadGapSize: u32 = 166u32;
pub const JET_paramMaxCoalesceReadSize: u32 = 164u32;
pub const JET_paramMaxCoalesceWriteGapSize: u32 = 167u32;
pub const JET_paramMaxCoalesceWriteSize: u32 = 165u32;
pub const JET_paramMaxColtyp: u32 = 131u32;
pub const JET_paramMaxCursors: u32 = 8u32;
pub const JET_paramMaxInstances: u32 = 104u32;
pub const JET_paramMaxOpenTables: u32 = 6u32;
pub const JET_paramMaxSessions: u32 = 5u32;
pub const JET_paramMaxTemporaryTables: u32 = 10u32;
pub const JET_paramMaxTransactionSize: u32 = 178u32;
pub const JET_paramMaxValueInvalid: u32 = 218u32;
pub const JET_paramMaxVerPages: u32 = 9u32;
pub const JET_paramMinDataForXpress: u32 = 183u32;
pub const JET_paramNoInformationEvent: u32 = 50u32;
pub const JET_paramOSSnapshotTimeout: u32 = 82u32;
pub const JET_paramOneDatabasePerSession: u32 = 102u32;
pub const JET_paramOutstandingIOMax: u32 = 30u32;
pub const JET_paramPageFragment: u32 = 20u32;
pub const JET_paramPageHintCacheSize: u32 = 101u32;
pub const JET_paramPageTempDBMin: u32 = 19u32;
pub const JET_paramPerfmonRefreshInterval: u32 = 217u32;
pub const JET_paramPreferredMaxOpenTables: u32 = 7u32;
pub const JET_paramPreferredVerPages: u32 = 63u32;
pub const JET_paramPrereadIOMax: u32 = 179u32;
pub const JET_paramProcessFriendlyName: u32 = 186u32;
pub const JET_paramRBSFilePath: u32 = 216u32;
pub const JET_paramRecordUpgradeDirtyLevel: u32 = 78u32;
pub const JET_paramRecovery: u32 = 34u32;
pub const JET_paramRuntimeCallback: u32 = 73u32;
pub const JET_paramStartFlushThreshold: u32 = 31u32;
pub const JET_paramStopFlushThreshold: u32 = 32u32;
pub const JET_paramSystemPath: u32 = 0u32;
pub const JET_paramTableClass10Name: u32 = 146u32;
pub const JET_paramTableClass11Name: u32 = 147u32;
pub const JET_paramTableClass12Name: u32 = 148u32;
pub const JET_paramTableClass13Name: u32 = 149u32;
pub const JET_paramTableClass14Name: u32 = 150u32;
pub const JET_paramTableClass15Name: u32 = 151u32;
pub const JET_paramTableClass1Name: u32 = 137u32;
pub const JET_paramTableClass2Name: u32 = 138u32;
pub const JET_paramTableClass3Name: u32 = 139u32;
pub const JET_paramTableClass4Name: u32 = 140u32;
pub const JET_paramTableClass5Name: u32 = 141u32;
pub const JET_paramTableClass6Name: u32 = 142u32;
pub const JET_paramTableClass7Name: u32 = 143u32;
pub const JET_paramTableClass8Name: u32 = 144u32;
pub const JET_paramTableClass9Name: u32 = 145u32;
pub const JET_paramTempPath: u32 = 1u32;
pub const JET_paramUnicodeIndexDefault: u32 = 72u32;
pub const JET_paramUseFlushForWriteDurability: u32 = 214u32;
pub const JET_paramVerPageSize: u32 = 128u32;
pub const JET_paramVersionStoreTaskQueueMax: u32 = 105u32;
pub const JET_paramWaitLogFlush: u32 = 13u32;
pub const JET_paramWaypointLatency: u32 = 153u32;
pub const JET_paramZeroDatabaseDuringBackup: u32 = 71u32;
pub const JET_prepCancel: u32 = 3u32;
pub const JET_prepInsert: u32 = 0u32;
pub const JET_prepInsertCopy: u32 = 5u32;
pub const JET_prepInsertCopyDeleteOriginal: u32 = 7u32;
pub const JET_prepInsertCopyReplaceOriginal: u32 = 9u32;
pub const JET_prepReplace: u32 = 2u32;
pub const JET_prepReplaceNoLock: u32 = 4u32;
pub const JET_relopBitmaskEqualsZero: JET_RELOP = JET_RELOP(7i32);
pub const JET_relopBitmaskNotEqualsZero: JET_RELOP = JET_RELOP(8i32);
pub const JET_relopEquals: JET_RELOP = JET_RELOP(0i32);
pub const JET_relopGreaterThan: JET_RELOP = JET_RELOP(6i32);
pub const JET_relopGreaterThanOrEqual: JET_RELOP = JET_RELOP(5i32);
pub const JET_relopLessThan: JET_RELOP = JET_RELOP(4i32);
pub const JET_relopLessThanOrEqual: JET_RELOP = JET_RELOP(3i32);
pub const JET_relopNotEquals: JET_RELOP = JET_RELOP(2i32);
pub const JET_relopPrefixEquals: JET_RELOP = JET_RELOP(1i32);
pub const JET_sesparamCommitDefault: u32 = 4097u32;
pub const JET_sesparamCorrelationID: u32 = 4101u32;
pub const JET_sesparamMaxValueInvalid: u32 = 4111u32;
pub const JET_sesparamOperationContext: u32 = 4100u32;
pub const JET_sesparamTransactionLevel: u32 = 4099u32;
pub const JET_snpBackup: u32 = 9u32;
pub const JET_snpCompact: u32 = 4u32;
pub const JET_snpRepair: u32 = 2u32;
pub const JET_snpRestore: u32 = 8u32;
pub const JET_snpScrub: u32 = 11u32;
pub const JET_snpUpgrade: u32 = 10u32;
pub const JET_snpUpgradeRecordFormat: u32 = 12u32;
pub const JET_sntBegin: u32 = 5u32;
pub const JET_sntComplete: u32 = 6u32;
pub const JET_sntFail: u32 = 3u32;
pub const JET_sntProgress: u32 = 0u32;
pub const JET_sntRequirements: u32 = 7u32;
pub const JET_sqmDisable: u32 = 0u32;
pub const JET_sqmEnable: u32 = 1u32;
pub const JET_sqmFromCEIP: u32 = 2u32;
pub const JET_wrnBufferTruncated: u32 = 1006u32;
pub const JET_wrnCallbackNotRegistered: u32 = 2100u32;
pub const JET_wrnColumnDefault: u32 = 1537u32;
pub const JET_wrnColumnMaxTruncated: u32 = 1512u32;
pub const JET_wrnColumnMoreTags: u32 = 1533u32;
pub const JET_wrnColumnNotInRecord: u32 = 1539u32;
pub const JET_wrnColumnNotLocal: u32 = 1532u32;
pub const JET_wrnColumnNull: u32 = 1004u32;
pub const JET_wrnColumnPresent: u32 = 1535u32;
pub const JET_wrnColumnReference: u32 = 1541u32;
pub const JET_wrnColumnSetNull: u32 = 1068u32;
pub const JET_wrnColumnSingleValue: u32 = 1536u32;
pub const JET_wrnColumnSkipped: u32 = 1531u32;
pub const JET_wrnColumnTruncated: u32 = 1534u32;
pub const JET_wrnCommittedLogFilesLost: u32 = 585u32;
pub const JET_wrnCommittedLogFilesRemoved: u32 = 587u32;
pub const JET_wrnCopyLongValue: u32 = 1520u32;
pub const JET_wrnCorruptIndexDeleted: u32 = 1415u32;
pub const JET_wrnDataHasChanged: u32 = 1610u32;
pub const JET_wrnDatabaseAttached: u32 = 1007u32;
pub const JET_wrnDatabaseRepaired: u32 = 595u32;
pub const JET_wrnDefragAlreadyRunning: u32 = 2000u32;
pub const JET_wrnDefragNotRunning: u32 = 2001u32;
pub const JET_wrnExistingLogFileHasBadSignature: u32 = 558u32;
pub const JET_wrnExistingLogFileIsNotContiguous: u32 = 559u32;
pub const JET_wrnFileOpenReadOnly: u32 = 1813u32;
pub const JET_wrnFinishWithUndo: u32 = 588u32;
pub const JET_wrnIdleFull: u32 = 1908u32;
pub const JET_wrnKeyChanged: u32 = 1618u32;
pub const JET_wrnNoErrorInfo: u32 = 1055u32;
pub const JET_wrnNoIdleActivity: u32 = 1058u32;
pub const JET_wrnNoWriteLock: u32 = 1067u32;
pub const JET_wrnNyi: i32 = -1i32;
pub const JET_wrnPrimaryIndexOutOfDate: u32 = 1417u32;
pub const JET_wrnRemainingVersions: u32 = 321u32;
pub const JET_wrnSecondaryIndexOutOfDate: u32 = 1418u32;
pub const JET_wrnSeekNotEqual: u32 = 1039u32;
pub const JET_wrnSeparateLongValue: u32 = 406u32;
pub const JET_wrnShrinkNotPossible: u32 = 1122u32;
pub const JET_wrnSkipThisRecord: u32 = 564u32;
pub const JET_wrnSortOverflow: u32 = 1009u32;
pub const JET_wrnTableEmpty: u32 = 1301u32;
pub const JET_wrnTableInUseBySystem: u32 = 1327u32;
pub const JET_wrnTargetInstanceRunning: u32 = 578u32;
pub const JET_wrnUniqueKey: u32 = 345u32;
pub const JET_wszConfigStoreReadControl: windows_core::PCWSTR = windows_core::w!("CsReadControl");
pub const JET_wszConfigStoreRelPathSysParamDefault: windows_core::PCWSTR = windows_core::w!("SysParamDefault");
pub const JET_wszConfigStoreRelPathSysParamOverride: windows_core::PCWSTR = windows_core::w!("SysParamOverride");
pub const cColumnInfoCols: u32 = 14u32;
pub const cIndexInfoCols: u32 = 15u32;
pub const cObjectInfoCols: u32 = 9u32;
pub const wrnBTNotVisibleAccumulated: u32 = 353u32;
pub const wrnBTNotVisibleRejected: u32 = 352u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct JET_ERRCAT(pub i32);
impl windows_core::TypeKind for JET_ERRCAT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for JET_ERRCAT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("JET_ERRCAT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct JET_INDEXCHECKING(pub i32);
impl windows_core::TypeKind for JET_INDEXCHECKING {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for JET_INDEXCHECKING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("JET_INDEXCHECKING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct JET_RELOP(pub i32);
impl windows_core::TypeKind for JET_RELOP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for JET_RELOP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("JET_RELOP").field(&self.0).finish()
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct JET_BKINFO {
    pub lgposMark: JET_LGPOS,
    pub Anonymous: JET_BKINFO_0,
    pub genLow: u32,
    pub genHigh: u32,
}
impl windows_core::TypeKind for JET_BKINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_BKINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_BKINFO_0 {
    pub logtimeMark: JET_LOGTIME,
    pub bklogtimeMark: JET_BKLOGTIME,
}
impl windows_core::TypeKind for JET_BKINFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_BKINFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_BKLOGTIME {
    pub bSeconds: i8,
    pub bMinutes: i8,
    pub bHours: i8,
    pub bDay: i8,
    pub bMonth: i8,
    pub bYear: i8,
    pub Anonymous1: JET_BKLOGTIME_0,
    pub Anonymous2: JET_BKLOGTIME_1,
}
impl windows_core::TypeKind for JET_BKLOGTIME {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_BKLOGTIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_BKLOGTIME_0 {
    pub bFiller1: i8,
    pub Anonymous: JET_BKLOGTIME_0_0,
}
impl windows_core::TypeKind for JET_BKLOGTIME_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_BKLOGTIME_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_BKLOGTIME_0_0 {
    pub _bitfield: u8,
}
impl windows_core::TypeKind for JET_BKLOGTIME_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_BKLOGTIME_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_BKLOGTIME_1 {
    pub bFiller2: i8,
    pub Anonymous: JET_BKLOGTIME_1_0,
}
impl windows_core::TypeKind for JET_BKLOGTIME_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_BKLOGTIME_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_BKLOGTIME_1_0 {
    pub _bitfield: u8,
}
impl windows_core::TypeKind for JET_BKLOGTIME_1_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_BKLOGTIME_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_COLUMNBASE_A {
    pub cbStruct: u32,
    pub columnid: u32,
    pub coltyp: u32,
    pub wCountry: u16,
    pub langid: u16,
    pub cp: u16,
    pub wFiller: u16,
    pub cbMax: u32,
    pub grbit: u32,
    pub szBaseTableName: [i8; 256],
    pub szBaseColumnName: [i8; 256],
}
impl windows_core::TypeKind for JET_COLUMNBASE_A {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_COLUMNBASE_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_COLUMNBASE_W {
    pub cbStruct: u32,
    pub columnid: u32,
    pub coltyp: u32,
    pub wCountry: u16,
    pub langid: u16,
    pub cp: u16,
    pub wFiller: u16,
    pub cbMax: u32,
    pub grbit: u32,
    pub szBaseTableName: [u16; 256],
    pub szBaseColumnName: [u16; 256],
}
impl windows_core::TypeKind for JET_COLUMNBASE_W {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_COLUMNBASE_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_COLUMNCREATE_A {
    pub cbStruct: u32,
    pub szColumnName: windows_core::PSTR,
    pub coltyp: u32,
    pub cbMax: u32,
    pub grbit: u32,
    pub pvDefault: *mut core::ffi::c_void,
    pub cbDefault: u32,
    pub cp: u32,
    pub columnid: u32,
    pub err: i32,
}
impl windows_core::TypeKind for JET_COLUMNCREATE_A {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_COLUMNCREATE_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_COLUMNCREATE_W {
    pub cbStruct: u32,
    pub szColumnName: windows_core::PWSTR,
    pub coltyp: u32,
    pub cbMax: u32,
    pub grbit: u32,
    pub pvDefault: *mut core::ffi::c_void,
    pub cbDefault: u32,
    pub cp: u32,
    pub columnid: u32,
    pub err: i32,
}
impl windows_core::TypeKind for JET_COLUMNCREATE_W {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_COLUMNCREATE_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_COLUMNDEF {
    pub cbStruct: u32,
    pub columnid: u32,
    pub coltyp: u32,
    pub wCountry: u16,
    pub langid: u16,
    pub cp: u16,
    pub wCollate: u16,
    pub cbMax: u32,
    pub grbit: u32,
}
impl windows_core::TypeKind for JET_COLUMNDEF {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_COLUMNDEF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_COLUMNLIST {
    pub cbStruct: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
    pub cRecord: u32,
    pub columnidPresentationOrder: u32,
    pub columnidcolumnname: u32,
    pub columnidcolumnid: u32,
    pub columnidcoltyp: u32,
    pub columnidCountry: u32,
    pub columnidLangid: u32,
    pub columnidCp: u32,
    pub columnidCollate: u32,
    pub columnidcbMax: u32,
    pub columnidgrbit: u32,
    pub columnidDefault: u32,
    pub columnidBaseTableName: u32,
    pub columnidBaseColumnName: u32,
    pub columnidDefinitionName: u32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl windows_core::TypeKind for JET_COLUMNLIST {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl Default for JET_COLUMNLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct JET_COMMIT_ID {
    pub signLog: JET_SIGNATURE,
    pub reserved: i32,
    pub commitId: i64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for JET_COMMIT_ID {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for JET_COMMIT_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct JET_COMMIT_ID {
    pub signLog: JET_SIGNATURE,
    pub reserved: i32,
    pub commitId: i64,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for JET_COMMIT_ID {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for JET_COMMIT_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_CONDITIONALCOLUMN_A {
    pub cbStruct: u32,
    pub szColumnName: windows_core::PSTR,
    pub grbit: u32,
}
impl windows_core::TypeKind for JET_CONDITIONALCOLUMN_A {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_CONDITIONALCOLUMN_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_CONDITIONALCOLUMN_W {
    pub cbStruct: u32,
    pub szColumnName: windows_core::PWSTR,
    pub grbit: u32,
}
impl windows_core::TypeKind for JET_CONDITIONALCOLUMN_W {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_CONDITIONALCOLUMN_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_CONVERT_A {
    pub szOldDll: windows_core::PSTR,
    pub Anonymous: JET_CONVERT_A_0,
}
impl windows_core::TypeKind for JET_CONVERT_A {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_CONVERT_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_CONVERT_A_0 {
    pub fFlags: u32,
    pub Anonymous: JET_CONVERT_A_0_0,
}
impl windows_core::TypeKind for JET_CONVERT_A_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_CONVERT_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_CONVERT_A_0_0 {
    pub _bitfield: u32,
}
impl windows_core::TypeKind for JET_CONVERT_A_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_CONVERT_A_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_CONVERT_W {
    pub szOldDll: windows_core::PWSTR,
    pub Anonymous: JET_CONVERT_W_0,
}
impl windows_core::TypeKind for JET_CONVERT_W {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_CONVERT_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_CONVERT_W_0 {
    pub fFlags: u32,
    pub Anonymous: JET_CONVERT_W_0_0,
}
impl windows_core::TypeKind for JET_CONVERT_W_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_CONVERT_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_CONVERT_W_0_0 {
    pub _bitfield: u32,
}
impl windows_core::TypeKind for JET_CONVERT_W_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_CONVERT_W_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_DBINFOMISC {
    pub ulVersion: u32,
    pub ulUpdate: u32,
    pub signDb: JET_SIGNATURE,
    pub dbstate: u32,
    pub lgposConsistent: JET_LGPOS,
    pub logtimeConsistent: JET_LOGTIME,
    pub logtimeAttach: JET_LOGTIME,
    pub lgposAttach: JET_LGPOS,
    pub logtimeDetach: JET_LOGTIME,
    pub lgposDetach: JET_LGPOS,
    pub signLog: JET_SIGNATURE,
    pub bkinfoFullPrev: JET_BKINFO,
    pub bkinfoIncPrev: JET_BKINFO,
    pub bkinfoFullCur: JET_BKINFO,
    pub fShadowingDisabled: u32,
    pub fUpgradeDb: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub lSPNumber: i32,
    pub cbPageSize: u32,
}
impl windows_core::TypeKind for JET_DBINFOMISC {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_DBINFOMISC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_DBINFOMISC2 {
    pub ulVersion: u32,
    pub ulUpdate: u32,
    pub signDb: JET_SIGNATURE,
    pub dbstate: u32,
    pub lgposConsistent: JET_LGPOS,
    pub logtimeConsistent: JET_LOGTIME,
    pub logtimeAttach: JET_LOGTIME,
    pub lgposAttach: JET_LGPOS,
    pub logtimeDetach: JET_LOGTIME,
    pub lgposDetach: JET_LGPOS,
    pub signLog: JET_SIGNATURE,
    pub bkinfoFullPrev: JET_BKINFO,
    pub bkinfoIncPrev: JET_BKINFO,
    pub bkinfoFullCur: JET_BKINFO,
    pub fShadowingDisabled: u32,
    pub fUpgradeDb: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub lSPNumber: i32,
    pub cbPageSize: u32,
    pub genMinRequired: u32,
    pub genMaxRequired: u32,
    pub logtimeGenMaxCreate: JET_LOGTIME,
    pub ulRepairCount: u32,
    pub logtimeRepair: JET_LOGTIME,
    pub ulRepairCountOld: u32,
    pub ulECCFixSuccess: u32,
    pub logtimeECCFixSuccess: JET_LOGTIME,
    pub ulECCFixSuccessOld: u32,
    pub ulECCFixFail: u32,
    pub logtimeECCFixFail: JET_LOGTIME,
    pub ulECCFixFailOld: u32,
    pub ulBadChecksum: u32,
    pub logtimeBadChecksum: JET_LOGTIME,
    pub ulBadChecksumOld: u32,
}
impl windows_core::TypeKind for JET_DBINFOMISC2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_DBINFOMISC2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_DBINFOMISC3 {
    pub ulVersion: u32,
    pub ulUpdate: u32,
    pub signDb: JET_SIGNATURE,
    pub dbstate: u32,
    pub lgposConsistent: JET_LGPOS,
    pub logtimeConsistent: JET_LOGTIME,
    pub logtimeAttach: JET_LOGTIME,
    pub lgposAttach: JET_LGPOS,
    pub logtimeDetach: JET_LOGTIME,
    pub lgposDetach: JET_LGPOS,
    pub signLog: JET_SIGNATURE,
    pub bkinfoFullPrev: JET_BKINFO,
    pub bkinfoIncPrev: JET_BKINFO,
    pub bkinfoFullCur: JET_BKINFO,
    pub fShadowingDisabled: u32,
    pub fUpgradeDb: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub lSPNumber: i32,
    pub cbPageSize: u32,
    pub genMinRequired: u32,
    pub genMaxRequired: u32,
    pub logtimeGenMaxCreate: JET_LOGTIME,
    pub ulRepairCount: u32,
    pub logtimeRepair: JET_LOGTIME,
    pub ulRepairCountOld: u32,
    pub ulECCFixSuccess: u32,
    pub logtimeECCFixSuccess: JET_LOGTIME,
    pub ulECCFixSuccessOld: u32,
    pub ulECCFixFail: u32,
    pub logtimeECCFixFail: JET_LOGTIME,
    pub ulECCFixFailOld: u32,
    pub ulBadChecksum: u32,
    pub logtimeBadChecksum: JET_LOGTIME,
    pub ulBadChecksumOld: u32,
    pub genCommitted: u32,
}
impl windows_core::TypeKind for JET_DBINFOMISC3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_DBINFOMISC3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_DBINFOMISC4 {
    pub ulVersion: u32,
    pub ulUpdate: u32,
    pub signDb: JET_SIGNATURE,
    pub dbstate: u32,
    pub lgposConsistent: JET_LGPOS,
    pub logtimeConsistent: JET_LOGTIME,
    pub logtimeAttach: JET_LOGTIME,
    pub lgposAttach: JET_LGPOS,
    pub logtimeDetach: JET_LOGTIME,
    pub lgposDetach: JET_LGPOS,
    pub signLog: JET_SIGNATURE,
    pub bkinfoFullPrev: JET_BKINFO,
    pub bkinfoIncPrev: JET_BKINFO,
    pub bkinfoFullCur: JET_BKINFO,
    pub fShadowingDisabled: u32,
    pub fUpgradeDb: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub lSPNumber: i32,
    pub cbPageSize: u32,
    pub genMinRequired: u32,
    pub genMaxRequired: u32,
    pub logtimeGenMaxCreate: JET_LOGTIME,
    pub ulRepairCount: u32,
    pub logtimeRepair: JET_LOGTIME,
    pub ulRepairCountOld: u32,
    pub ulECCFixSuccess: u32,
    pub logtimeECCFixSuccess: JET_LOGTIME,
    pub ulECCFixSuccessOld: u32,
    pub ulECCFixFail: u32,
    pub logtimeECCFixFail: JET_LOGTIME,
    pub ulECCFixFailOld: u32,
    pub ulBadChecksum: u32,
    pub logtimeBadChecksum: JET_LOGTIME,
    pub ulBadChecksumOld: u32,
    pub genCommitted: u32,
    pub bkinfoCopyPrev: JET_BKINFO,
    pub bkinfoDiffPrev: JET_BKINFO,
}
impl windows_core::TypeKind for JET_DBINFOMISC4 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_DBINFOMISC4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_DBINFOUPGRADE {
    pub cbStruct: u32,
    pub cbFilesizeLow: u32,
    pub cbFilesizeHigh: u32,
    pub cbFreeSpaceRequiredLow: u32,
    pub cbFreeSpaceRequiredHigh: u32,
    pub csecToUpgrade: u32,
    pub Anonymous: JET_DBINFOUPGRADE_0,
}
impl windows_core::TypeKind for JET_DBINFOUPGRADE {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_DBINFOUPGRADE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_DBINFOUPGRADE_0 {
    pub ulFlags: u32,
    pub Anonymous: JET_DBINFOUPGRADE_0_0,
}
impl windows_core::TypeKind for JET_DBINFOUPGRADE_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_DBINFOUPGRADE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_DBINFOUPGRADE_0_0 {
    pub _bitfield: u32,
}
impl windows_core::TypeKind for JET_DBINFOUPGRADE_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_DBINFOUPGRADE_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_ENUMCOLUMN {
    pub columnid: u32,
    pub err: i32,
    pub Anonymous: JET_ENUMCOLUMN_0,
}
impl windows_core::TypeKind for JET_ENUMCOLUMN {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_ENUMCOLUMN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_ENUMCOLUMN_0 {
    pub Anonymous1: JET_ENUMCOLUMN_0_0,
    pub Anonymous2: JET_ENUMCOLUMN_0_1,
}
impl windows_core::TypeKind for JET_ENUMCOLUMN_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_ENUMCOLUMN_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_ENUMCOLUMN_0_0 {
    pub cEnumColumnValue: u32,
    pub rgEnumColumnValue: *mut JET_ENUMCOLUMNVALUE,
}
impl windows_core::TypeKind for JET_ENUMCOLUMN_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_ENUMCOLUMN_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_ENUMCOLUMN_0_1 {
    pub cbData: u32,
    pub pvData: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for JET_ENUMCOLUMN_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_ENUMCOLUMN_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_ENUMCOLUMNID {
    pub columnid: u32,
    pub ctagSequence: u32,
    pub rgtagSequence: *mut u32,
}
impl windows_core::TypeKind for JET_ENUMCOLUMNID {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_ENUMCOLUMNID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_ENUMCOLUMNVALUE {
    pub itagSequence: u32,
    pub err: i32,
    pub cbData: u32,
    pub pvData: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for JET_ENUMCOLUMNVALUE {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_ENUMCOLUMNVALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_ERRINFOBASIC_W {
    pub cbStruct: u32,
    pub errValue: i32,
    pub errcatMostSpecific: JET_ERRCAT,
    pub rgCategoricalHierarchy: [u8; 8],
    pub lSourceLine: u32,
    pub rgszSourceFile: [u16; 64],
}
impl windows_core::TypeKind for JET_ERRINFOBASIC_W {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_ERRINFOBASIC_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_INDEXCREATE2_A {
    pub cbStruct: u32,
    pub szIndexName: windows_core::PSTR,
    pub szKey: windows_core::PSTR,
    pub cbKey: u32,
    pub grbit: u32,
    pub ulDensity: u32,
    pub Anonymous1: JET_INDEXCREATE2_A_0,
    pub Anonymous2: JET_INDEXCREATE2_A_1,
    pub rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_A,
    pub cConditionalColumn: u32,
    pub err: i32,
    pub cbKeyMost: u32,
    pub pSpacehints: *mut JET_SPACEHINTS,
}
impl windows_core::TypeKind for JET_INDEXCREATE2_A {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_INDEXCREATE2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_INDEXCREATE2_A_0 {
    pub lcid: u32,
    pub pidxunicode: *mut JET_UNICODEINDEX,
}
impl windows_core::TypeKind for JET_INDEXCREATE2_A_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_INDEXCREATE2_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_INDEXCREATE2_A_1 {
    pub cbVarSegMac: u32,
    pub ptuplelimits: *mut JET_TUPLELIMITS,
}
impl windows_core::TypeKind for JET_INDEXCREATE2_A_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_INDEXCREATE2_A_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_INDEXCREATE2_W {
    pub cbStruct: u32,
    pub szIndexName: windows_core::PWSTR,
    pub szKey: windows_core::PWSTR,
    pub cbKey: u32,
    pub grbit: u32,
    pub ulDensity: u32,
    pub Anonymous1: JET_INDEXCREATE2_W_0,
    pub Anonymous2: JET_INDEXCREATE2_W_1,
    pub rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_W,
    pub cConditionalColumn: u32,
    pub err: i32,
    pub cbKeyMost: u32,
    pub pSpacehints: *mut JET_SPACEHINTS,
}
impl windows_core::TypeKind for JET_INDEXCREATE2_W {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_INDEXCREATE2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_INDEXCREATE2_W_0 {
    pub lcid: u32,
    pub pidxunicode: *mut JET_UNICODEINDEX,
}
impl windows_core::TypeKind for JET_INDEXCREATE2_W_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_INDEXCREATE2_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_INDEXCREATE2_W_1 {
    pub cbVarSegMac: u32,
    pub ptuplelimits: *mut JET_TUPLELIMITS,
}
impl windows_core::TypeKind for JET_INDEXCREATE2_W_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_INDEXCREATE2_W_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_INDEXCREATE3_A {
    pub cbStruct: u32,
    pub szIndexName: windows_core::PSTR,
    pub szKey: windows_core::PSTR,
    pub cbKey: u32,
    pub grbit: u32,
    pub ulDensity: u32,
    pub pidxunicode: *mut JET_UNICODEINDEX2,
    pub Anonymous: JET_INDEXCREATE3_A_0,
    pub rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_A,
    pub cConditionalColumn: u32,
    pub err: i32,
    pub cbKeyMost: u32,
    pub pSpacehints: *mut JET_SPACEHINTS,
}
impl windows_core::TypeKind for JET_INDEXCREATE3_A {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_INDEXCREATE3_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_INDEXCREATE3_A_0 {
    pub cbVarSegMac: u32,
    pub ptuplelimits: *mut JET_TUPLELIMITS,
}
impl windows_core::TypeKind for JET_INDEXCREATE3_A_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_INDEXCREATE3_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_INDEXCREATE3_W {
    pub cbStruct: u32,
    pub szIndexName: windows_core::PWSTR,
    pub szKey: windows_core::PWSTR,
    pub cbKey: u32,
    pub grbit: u32,
    pub ulDensity: u32,
    pub pidxunicode: *mut JET_UNICODEINDEX2,
    pub Anonymous: JET_INDEXCREATE3_W_0,
    pub rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_W,
    pub cConditionalColumn: u32,
    pub err: i32,
    pub cbKeyMost: u32,
    pub pSpacehints: *mut JET_SPACEHINTS,
}
impl windows_core::TypeKind for JET_INDEXCREATE3_W {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_INDEXCREATE3_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_INDEXCREATE3_W_0 {
    pub cbVarSegMac: u32,
    pub ptuplelimits: *mut JET_TUPLELIMITS,
}
impl windows_core::TypeKind for JET_INDEXCREATE3_W_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_INDEXCREATE3_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_INDEXCREATE_A {
    pub cbStruct: u32,
    pub szIndexName: windows_core::PSTR,
    pub szKey: windows_core::PSTR,
    pub cbKey: u32,
    pub grbit: u32,
    pub ulDensity: u32,
    pub Anonymous1: JET_INDEXCREATE_A_0,
    pub Anonymous2: JET_INDEXCREATE_A_1,
    pub rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_A,
    pub cConditionalColumn: u32,
    pub err: i32,
    pub cbKeyMost: u32,
}
impl windows_core::TypeKind for JET_INDEXCREATE_A {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_INDEXCREATE_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_INDEXCREATE_A_0 {
    pub lcid: u32,
    pub pidxunicode: *mut JET_UNICODEINDEX,
}
impl windows_core::TypeKind for JET_INDEXCREATE_A_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_INDEXCREATE_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_INDEXCREATE_A_1 {
    pub cbVarSegMac: u32,
    pub ptuplelimits: *mut JET_TUPLELIMITS,
}
impl windows_core::TypeKind for JET_INDEXCREATE_A_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_INDEXCREATE_A_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_INDEXCREATE_W {
    pub cbStruct: u32,
    pub szIndexName: windows_core::PWSTR,
    pub szKey: windows_core::PWSTR,
    pub cbKey: u32,
    pub grbit: u32,
    pub ulDensity: u32,
    pub Anonymous1: JET_INDEXCREATE_W_0,
    pub Anonymous2: JET_INDEXCREATE_W_1,
    pub rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_W,
    pub cConditionalColumn: u32,
    pub err: i32,
    pub cbKeyMost: u32,
}
impl windows_core::TypeKind for JET_INDEXCREATE_W {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_INDEXCREATE_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_INDEXCREATE_W_0 {
    pub lcid: u32,
    pub pidxunicode: *mut JET_UNICODEINDEX,
}
impl windows_core::TypeKind for JET_INDEXCREATE_W_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_INDEXCREATE_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_INDEXCREATE_W_1 {
    pub cbVarSegMac: u32,
    pub ptuplelimits: *mut JET_TUPLELIMITS,
}
impl windows_core::TypeKind for JET_INDEXCREATE_W_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_INDEXCREATE_W_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_INDEXID {
    pub cbStruct: u32,
    pub rgbIndexId: [u8; 16],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for JET_INDEXID {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for JET_INDEXID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_INDEXID {
    pub cbStruct: u32,
    pub rgbIndexId: [u8; 12],
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for JET_INDEXID {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for JET_INDEXID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_INDEXLIST {
    pub cbStruct: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
    pub cRecord: u32,
    pub columnidindexname: u32,
    pub columnidgrbitIndex: u32,
    pub columnidcKey: u32,
    pub columnidcEntry: u32,
    pub columnidcPage: u32,
    pub columnidcColumn: u32,
    pub columnidiColumn: u32,
    pub columnidcolumnid: u32,
    pub columnidcoltyp: u32,
    pub columnidCountry: u32,
    pub columnidLangid: u32,
    pub columnidCp: u32,
    pub columnidCollate: u32,
    pub columnidgrbitColumn: u32,
    pub columnidcolumnname: u32,
    pub columnidLCMapFlags: u32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl windows_core::TypeKind for JET_INDEXLIST {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl Default for JET_INDEXLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_INDEXRANGE {
    pub cbStruct: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
    pub grbit: u32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl windows_core::TypeKind for JET_INDEXRANGE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl Default for JET_INDEXRANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_INDEX_COLUMN {
    pub columnid: u32,
    pub relop: JET_RELOP,
    pub pv: *mut core::ffi::c_void,
    pub cb: u32,
    pub grbit: u32,
}
impl windows_core::TypeKind for JET_INDEX_COLUMN {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_INDEX_COLUMN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_INDEX_RANGE {
    pub rgStartColumns: *mut JET_INDEX_COLUMN,
    pub cStartColumns: u32,
    pub rgEndColumns: *mut JET_INDEX_COLUMN,
    pub cEndColumns: u32,
}
impl windows_core::TypeKind for JET_INDEX_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_INDEX_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JET_INSTANCE(pub usize);
impl JET_INSTANCE {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl windows_core::Free for JET_INSTANCE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = JetTerm(*self);
        }
    }
}
impl Default for JET_INSTANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for JET_INSTANCE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_INSTANCE_INFO_A {
    pub hInstanceId: JET_INSTANCE,
    pub szInstanceName: windows_core::PSTR,
    pub cDatabases: super::StructuredStorage::JET_API_PTR,
    pub szDatabaseFileName: *mut *mut i8,
    pub szDatabaseDisplayName: *mut *mut i8,
    pub szDatabaseSLVFileName_Obsolete: *mut *mut i8,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl windows_core::TypeKind for JET_INSTANCE_INFO_A {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl Default for JET_INSTANCE_INFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_INSTANCE_INFO_W {
    pub hInstanceId: JET_INSTANCE,
    pub szInstanceName: windows_core::PWSTR,
    pub cDatabases: super::StructuredStorage::JET_API_PTR,
    pub szDatabaseFileName: *mut *mut u16,
    pub szDatabaseDisplayName: *mut *mut u16,
    pub szDatabaseSLVFileName_Obsolete: *mut *mut u16,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl windows_core::TypeKind for JET_INSTANCE_INFO_W {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl Default for JET_INSTANCE_INFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct JET_LGPOS {
    pub ib: u16,
    pub isec: u16,
    pub lGeneration: i32,
}
impl windows_core::TypeKind for JET_LGPOS {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_LGPOS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_LOGINFO_A {
    pub cbSize: u32,
    pub ulGenLow: u32,
    pub ulGenHigh: u32,
    pub szBaseName: [i8; 4],
}
impl windows_core::TypeKind for JET_LOGINFO_A {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_LOGINFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_LOGINFO_W {
    pub cbSize: u32,
    pub ulGenLow: u32,
    pub ulGenHigh: u32,
    pub szBaseName: [u16; 4],
}
impl windows_core::TypeKind for JET_LOGINFO_W {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_LOGINFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_LOGTIME {
    pub bSeconds: i8,
    pub bMinutes: i8,
    pub bHours: i8,
    pub bDay: i8,
    pub bMonth: i8,
    pub bYear: i8,
    pub Anonymous1: JET_LOGTIME_0,
    pub Anonymous2: JET_LOGTIME_1,
}
impl windows_core::TypeKind for JET_LOGTIME {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_LOGTIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_LOGTIME_0 {
    pub bFiller1: i8,
    pub Anonymous: JET_LOGTIME_0_0,
}
impl windows_core::TypeKind for JET_LOGTIME_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_LOGTIME_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_LOGTIME_0_0 {
    pub _bitfield: u8,
}
impl windows_core::TypeKind for JET_LOGTIME_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_LOGTIME_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JET_LOGTIME_1 {
    pub bFiller2: i8,
    pub Anonymous: JET_LOGTIME_1_0,
}
impl windows_core::TypeKind for JET_LOGTIME_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_LOGTIME_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_LOGTIME_1_0 {
    pub _bitfield: u8,
}
impl windows_core::TypeKind for JET_LOGTIME_1_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_LOGTIME_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JET_LS(pub usize);
impl JET_LS {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl Default for JET_LS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for JET_LS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct JET_OBJECTINFO {
    pub cbStruct: u32,
    pub objtyp: u32,
    pub dtCreate: f64,
    pub dtUpdate: f64,
    pub grbit: u32,
    pub flags: u32,
    pub cRecord: u32,
    pub cPage: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for JET_OBJECTINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for JET_OBJECTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct JET_OBJECTINFO {
    pub cbStruct: u32,
    pub objtyp: u32,
    pub dtCreate: f64,
    pub dtUpdate: f64,
    pub grbit: u32,
    pub flags: u32,
    pub cRecord: u32,
    pub cPage: u32,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for JET_OBJECTINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for JET_OBJECTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_OBJECTLIST {
    pub cbStruct: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
    pub cRecord: u32,
    pub columnidcontainername: u32,
    pub columnidobjectname: u32,
    pub columnidobjtyp: u32,
    pub columniddtCreate: u32,
    pub columniddtUpdate: u32,
    pub columnidgrbit: u32,
    pub columnidflags: u32,
    pub columnidcRecord: u32,
    pub columnidcPage: u32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl windows_core::TypeKind for JET_OBJECTLIST {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl Default for JET_OBJECTLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_OPENTEMPORARYTABLE {
    pub cbStruct: u32,
    pub prgcolumndef: *const JET_COLUMNDEF,
    pub ccolumn: u32,
    pub pidxunicode: *mut JET_UNICODEINDEX,
    pub grbit: u32,
    pub prgcolumnid: *mut u32,
    pub cbKeyMost: u32,
    pub cbVarSegMac: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl windows_core::TypeKind for JET_OPENTEMPORARYTABLE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl Default for JET_OPENTEMPORARYTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_OPENTEMPORARYTABLE2 {
    pub cbStruct: u32,
    pub prgcolumndef: *const JET_COLUMNDEF,
    pub ccolumn: u32,
    pub pidxunicode: *mut JET_UNICODEINDEX2,
    pub grbit: u32,
    pub prgcolumnid: *mut u32,
    pub cbKeyMost: u32,
    pub cbVarSegMac: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl windows_core::TypeKind for JET_OPENTEMPORARYTABLE2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl Default for JET_OPENTEMPORARYTABLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_OPERATIONCONTEXT {
    pub ulUserID: u32,
    pub nOperationID: u8,
    pub nOperationType: u8,
    pub nClientType: u8,
    pub fFlags: u8,
}
impl windows_core::TypeKind for JET_OPERATIONCONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_OPERATIONCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JET_OSSNAPID(pub usize);
impl JET_OSSNAPID {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl Default for JET_OSSNAPID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for JET_OSSNAPID {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_RECORDLIST {
    pub cbStruct: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
    pub cRecord: u32,
    pub columnidBookmark: u32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl windows_core::TypeKind for JET_RECORDLIST {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl Default for JET_RECORDLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_RECPOS {
    pub cbStruct: u32,
    pub centriesLT: u32,
    pub centriesInRange: u32,
    pub centriesTotal: u32,
}
impl windows_core::TypeKind for JET_RECPOS {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_RECPOS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct JET_RECPOS2 {
    pub cbStruct: u32,
    pub centriesLTDeprecated: u32,
    pub centriesInRangeDeprecated: u32,
    pub centriesTotalDeprecated: u32,
    pub centriesLT: u64,
    pub centriesTotal: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for JET_RECPOS2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for JET_RECPOS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct JET_RECPOS2 {
    pub cbStruct: u32,
    pub centriesLTDeprecated: u32,
    pub centriesInRangeDeprecated: u32,
    pub centriesTotalDeprecated: u32,
    pub centriesLT: u64,
    pub centriesTotal: u64,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for JET_RECPOS2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for JET_RECPOS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct JET_RECSIZE {
    pub cbData: u64,
    pub cbLongValueData: u64,
    pub cbOverhead: u64,
    pub cbLongValueOverhead: u64,
    pub cNonTaggedColumns: u64,
    pub cTaggedColumns: u64,
    pub cLongValues: u64,
    pub cMultiValues: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for JET_RECSIZE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for JET_RECSIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct JET_RECSIZE {
    pub cbData: u64,
    pub cbLongValueData: u64,
    pub cbOverhead: u64,
    pub cbLongValueOverhead: u64,
    pub cNonTaggedColumns: u64,
    pub cTaggedColumns: u64,
    pub cLongValues: u64,
    pub cMultiValues: u64,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for JET_RECSIZE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for JET_RECSIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct JET_RECSIZE2 {
    pub cbData: u64,
    pub cbLongValueData: u64,
    pub cbOverhead: u64,
    pub cbLongValueOverhead: u64,
    pub cNonTaggedColumns: u64,
    pub cTaggedColumns: u64,
    pub cLongValues: u64,
    pub cMultiValues: u64,
    pub cCompressedColumns: u64,
    pub cbDataCompressed: u64,
    pub cbLongValueDataCompressed: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for JET_RECSIZE2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for JET_RECSIZE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct JET_RECSIZE2 {
    pub cbData: u64,
    pub cbLongValueData: u64,
    pub cbOverhead: u64,
    pub cbLongValueOverhead: u64,
    pub cNonTaggedColumns: u64,
    pub cTaggedColumns: u64,
    pub cLongValues: u64,
    pub cMultiValues: u64,
    pub cCompressedColumns: u64,
    pub cbDataCompressed: u64,
    pub cbLongValueDataCompressed: u64,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for JET_RECSIZE2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for JET_RECSIZE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_RETINFO {
    pub cbStruct: u32,
    pub ibLongValue: u32,
    pub itagSequence: u32,
    pub columnidNextTagged: u32,
}
impl windows_core::TypeKind for JET_RETINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_RETINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_RETRIEVECOLUMN {
    pub columnid: u32,
    pub pvData: *mut core::ffi::c_void,
    pub cbData: u32,
    pub cbActual: u32,
    pub grbit: u32,
    pub ibLongValue: u32,
    pub itagSequence: u32,
    pub columnidNextTagged: u32,
    pub err: i32,
}
impl windows_core::TypeKind for JET_RETRIEVECOLUMN {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_RETRIEVECOLUMN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_RSTINFO_A {
    pub cbStruct: u32,
    pub rgrstmap: *mut JET_RSTMAP_A,
    pub crstmap: i32,
    pub lgposStop: JET_LGPOS,
    pub logtimeStop: JET_LOGTIME,
    pub pfnStatus: JET_PFNSTATUS,
}
impl windows_core::TypeKind for JET_RSTINFO_A {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_RSTINFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JET_RSTINFO_W {
    pub cbStruct: u32,
    pub rgrstmap: *mut JET_RSTMAP_W,
    pub crstmap: i32,
    pub lgposStop: JET_LGPOS,
    pub logtimeStop: JET_LOGTIME,
    pub pfnStatus: JET_PFNSTATUS,
}
impl windows_core::TypeKind for JET_RSTINFO_W {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_RSTINFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_RSTMAP_A {
    pub szDatabaseName: windows_core::PSTR,
    pub szNewDatabaseName: windows_core::PSTR,
}
impl windows_core::TypeKind for JET_RSTMAP_A {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_RSTMAP_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_RSTMAP_W {
    pub szDatabaseName: windows_core::PWSTR,
    pub szNewDatabaseName: windows_core::PWSTR,
}
impl windows_core::TypeKind for JET_RSTMAP_W {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_RSTMAP_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JET_SESID(pub usize);
impl JET_SESID {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl windows_core::Free for JET_SESID {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = JetEndSession(*self, 0);
        }
    }
}
impl Default for JET_SESID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for JET_SESID {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_SETCOLUMN {
    pub columnid: u32,
    pub pvData: *const core::ffi::c_void,
    pub cbData: u32,
    pub grbit: u32,
    pub ibLongValue: u32,
    pub itagSequence: u32,
    pub err: i32,
}
impl windows_core::TypeKind for JET_SETCOLUMN {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_SETCOLUMN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_SETINFO {
    pub cbStruct: u32,
    pub ibLongValue: u32,
    pub itagSequence: u32,
}
impl windows_core::TypeKind for JET_SETINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_SETINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_SETSYSPARAM_A {
    pub paramid: u32,
    pub lParam: super::StructuredStorage::JET_API_PTR,
    pub sz: windows_core::PCSTR,
    pub err: i32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl windows_core::TypeKind for JET_SETSYSPARAM_A {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl Default for JET_SETSYSPARAM_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_SETSYSPARAM_W {
    pub paramid: u32,
    pub lParam: super::StructuredStorage::JET_API_PTR,
    pub sz: windows_core::PCWSTR,
    pub err: i32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl windows_core::TypeKind for JET_SETSYSPARAM_W {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl Default for JET_SETSYSPARAM_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct JET_SIGNATURE {
    pub ulRandom: u32,
    pub logtimeCreate: JET_LOGTIME,
    pub szComputerName: [i8; 16],
}
impl windows_core::TypeKind for JET_SIGNATURE {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_SIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_SNPROG {
    pub cbStruct: u32,
    pub cunitDone: u32,
    pub cunitTotal: u32,
}
impl windows_core::TypeKind for JET_SNPROG {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_SNPROG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_SPACEHINTS {
    pub cbStruct: u32,
    pub ulInitialDensity: u32,
    pub cbInitial: u32,
    pub grbit: u32,
    pub ulMaintDensity: u32,
    pub ulGrowth: u32,
    pub cbMinExtent: u32,
    pub cbMaxExtent: u32,
}
impl windows_core::TypeKind for JET_SPACEHINTS {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_SPACEHINTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_TABLECREATE2_A {
    pub cbStruct: u32,
    pub szTableName: windows_core::PSTR,
    pub szTemplateTableName: windows_core::PSTR,
    pub ulPages: u32,
    pub ulDensity: u32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_A,
    pub cColumns: u32,
    pub rgindexcreate: *mut JET_INDEXCREATE_A,
    pub cIndexes: u32,
    pub szCallback: windows_core::PSTR,
    pub cbtyp: u32,
    pub grbit: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
    pub cCreated: u32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl windows_core::TypeKind for JET_TABLECREATE2_A {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl Default for JET_TABLECREATE2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_TABLECREATE2_W {
    pub cbStruct: u32,
    pub szTableName: windows_core::PWSTR,
    pub szTemplateTableName: windows_core::PWSTR,
    pub ulPages: u32,
    pub ulDensity: u32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_W,
    pub cColumns: u32,
    pub rgindexcreate: *mut JET_INDEXCREATE_W,
    pub cIndexes: u32,
    pub szCallback: windows_core::PWSTR,
    pub cbtyp: u32,
    pub grbit: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
    pub cCreated: u32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl windows_core::TypeKind for JET_TABLECREATE2_W {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl Default for JET_TABLECREATE2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_TABLECREATE3_A {
    pub cbStruct: u32,
    pub szTableName: windows_core::PSTR,
    pub szTemplateTableName: windows_core::PSTR,
    pub ulPages: u32,
    pub ulDensity: u32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_A,
    pub cColumns: u32,
    pub rgindexcreate: *mut JET_INDEXCREATE2_A,
    pub cIndexes: u32,
    pub szCallback: windows_core::PSTR,
    pub cbtyp: u32,
    pub grbit: u32,
    pub pSeqSpacehints: *mut JET_SPACEHINTS,
    pub pLVSpacehints: *mut JET_SPACEHINTS,
    pub cbSeparateLV: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
    pub cCreated: u32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl windows_core::TypeKind for JET_TABLECREATE3_A {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl Default for JET_TABLECREATE3_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_TABLECREATE3_W {
    pub cbStruct: u32,
    pub szTableName: windows_core::PWSTR,
    pub szTemplateTableName: windows_core::PWSTR,
    pub ulPages: u32,
    pub ulDensity: u32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_W,
    pub cColumns: u32,
    pub rgindexcreate: *mut JET_INDEXCREATE2_W,
    pub cIndexes: u32,
    pub szCallback: windows_core::PWSTR,
    pub cbtyp: u32,
    pub grbit: u32,
    pub pSeqSpacehints: *mut JET_SPACEHINTS,
    pub pLVSpacehints: *mut JET_SPACEHINTS,
    pub cbSeparateLV: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
    pub cCreated: u32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl windows_core::TypeKind for JET_TABLECREATE3_W {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl Default for JET_TABLECREATE3_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_TABLECREATE4_A {
    pub cbStruct: u32,
    pub szTableName: windows_core::PSTR,
    pub szTemplateTableName: windows_core::PSTR,
    pub ulPages: u32,
    pub ulDensity: u32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_A,
    pub cColumns: u32,
    pub rgindexcreate: *mut JET_INDEXCREATE3_A,
    pub cIndexes: u32,
    pub szCallback: windows_core::PSTR,
    pub cbtyp: u32,
    pub grbit: u32,
    pub pSeqSpacehints: *mut JET_SPACEHINTS,
    pub pLVSpacehints: *mut JET_SPACEHINTS,
    pub cbSeparateLV: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
    pub cCreated: u32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl windows_core::TypeKind for JET_TABLECREATE4_A {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl Default for JET_TABLECREATE4_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_TABLECREATE4_W {
    pub cbStruct: u32,
    pub szTableName: windows_core::PWSTR,
    pub szTemplateTableName: windows_core::PWSTR,
    pub ulPages: u32,
    pub ulDensity: u32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_W,
    pub cColumns: u32,
    pub rgindexcreate: *mut JET_INDEXCREATE3_W,
    pub cIndexes: u32,
    pub szCallback: windows_core::PWSTR,
    pub cbtyp: u32,
    pub grbit: u32,
    pub pSeqSpacehints: *mut JET_SPACEHINTS,
    pub pLVSpacehints: *mut JET_SPACEHINTS,
    pub cbSeparateLV: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
    pub cCreated: u32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl windows_core::TypeKind for JET_TABLECREATE4_W {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl Default for JET_TABLECREATE4_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_TABLECREATE_A {
    pub cbStruct: u32,
    pub szTableName: windows_core::PSTR,
    pub szTemplateTableName: windows_core::PSTR,
    pub ulPages: u32,
    pub ulDensity: u32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_A,
    pub cColumns: u32,
    pub rgindexcreate: *mut JET_INDEXCREATE_A,
    pub cIndexes: u32,
    pub grbit: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
    pub cCreated: u32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl windows_core::TypeKind for JET_TABLECREATE_A {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl Default for JET_TABLECREATE_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_TABLECREATE_W {
    pub cbStruct: u32,
    pub szTableName: windows_core::PWSTR,
    pub szTemplateTableName: windows_core::PWSTR,
    pub ulPages: u32,
    pub ulDensity: u32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_W,
    pub cColumns: u32,
    pub rgindexcreate: *mut JET_INDEXCREATE_W,
    pub cIndexes: u32,
    pub grbit: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
    pub cCreated: u32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl windows_core::TypeKind for JET_TABLECREATE_W {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl Default for JET_TABLECREATE_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_THREADSTATS {
    pub cbStruct: u32,
    pub cPageReferenced: u32,
    pub cPageRead: u32,
    pub cPagePreread: u32,
    pub cPageDirtied: u32,
    pub cPageRedirtied: u32,
    pub cLogRecord: u32,
    pub cbLogRecord: u32,
}
impl windows_core::TypeKind for JET_THREADSTATS {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_THREADSTATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct JET_THREADSTATS2 {
    pub cbStruct: u32,
    pub cPageReferenced: u32,
    pub cPageRead: u32,
    pub cPagePreread: u32,
    pub cPageDirtied: u32,
    pub cPageRedirtied: u32,
    pub cLogRecord: u32,
    pub cbLogRecord: u32,
    pub cusecPageCacheMiss: u64,
    pub cPageCacheMiss: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for JET_THREADSTATS2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for JET_THREADSTATS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct JET_THREADSTATS2 {
    pub cbStruct: u32,
    pub cPageReferenced: u32,
    pub cPageRead: u32,
    pub cPagePreread: u32,
    pub cPageDirtied: u32,
    pub cPageRedirtied: u32,
    pub cLogRecord: u32,
    pub cbLogRecord: u32,
    pub cusecPageCacheMiss: u64,
    pub cPageCacheMiss: u32,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for JET_THREADSTATS2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for JET_THREADSTATS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_TUPLELIMITS {
    pub chLengthMin: u32,
    pub chLengthMax: u32,
    pub chToIndexMax: u32,
    pub cchIncrement: u32,
    pub ichStart: u32,
}
impl windows_core::TypeKind for JET_TUPLELIMITS {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_TUPLELIMITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_UNICODEINDEX {
    pub lcid: u32,
    pub dwMapFlags: u32,
}
impl windows_core::TypeKind for JET_UNICODEINDEX {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_UNICODEINDEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_UNICODEINDEX2 {
    pub szLocaleName: windows_core::PWSTR,
    pub dwMapFlags: u32,
}
impl windows_core::TypeKind for JET_UNICODEINDEX2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_UNICODEINDEX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_USERDEFINEDDEFAULT_A {
    pub szCallback: windows_core::PSTR,
    pub pbUserData: *mut u8,
    pub cbUserData: u32,
    pub szDependantColumns: windows_core::PSTR,
}
impl windows_core::TypeKind for JET_USERDEFINEDDEFAULT_A {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_USERDEFINEDDEFAULT_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JET_USERDEFINEDDEFAULT_W {
    pub szCallback: windows_core::PWSTR,
    pub pbUserData: *mut u8,
    pub cbUserData: u32,
    pub szDependantColumns: windows_core::PWSTR,
}
impl windows_core::TypeKind for JET_USERDEFINEDDEFAULT_W {
    type TypeKind = windows_core::CopyType;
}
impl Default for JET_USERDEFINEDDEFAULT_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
pub type JET_CALLBACK = Option<unsafe extern "system" fn(sesid: JET_SESID, dbid: u32, tableid: super::StructuredStorage::JET_TABLEID, cbtyp: u32, pvarg1: *mut core::ffi::c_void, pvarg2: *mut core::ffi::c_void, pvcontext: *const core::ffi::c_void, ulunused: super::StructuredStorage::JET_API_PTR) -> i32>;
pub type JET_PFNDURABLECOMMITCALLBACK = Option<unsafe extern "system" fn(instance: JET_INSTANCE, pcommitidseen: *const JET_COMMIT_ID, grbit: u32) -> i32>;
pub type JET_PFNREALLOC = Option<unsafe extern "system" fn(pvcontext: *const core::ffi::c_void, pv: *const core::ffi::c_void, cb: u32) -> *mut core::ffi::c_void>;
pub type JET_PFNSTATUS = Option<unsafe extern "system" fn(sesid: JET_SESID, snp: u32, snt: u32, pv: *const core::ffi::c_void) -> i32>;
