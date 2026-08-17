// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Component object model defintions
use shared::minwindef::LPVOID;
use um::combaseapi::COINITBASE_MULTITHREADED;
use um::winnt::HRESULT;
ENUM!{enum COINIT {
    COINIT_APARTMENTTHREADED = 0x2,
    COINIT_MULTITHREADED = COINITBASE_MULTITHREADED,
    COINIT_DISABLE_OLE1DDE = 0x4,
    COINIT_SPEED_OVER_MEMORY = 0x8,
}}
    // pub fn CoBuildVersion();
extern "system" {
    pub fn CoInitialize(
        pvReserved: LPVOID,
    ) -> HRESULT;
}
    // pub fn CoRegisterMallocSpy();
    // pub fn CoRevokeMallocSpy();
    // pub fn CoRegisterInitializeSpy();
    // pub fn CoRevokeInitializeSpy();
    // pub fn CoGetSystemSecurityPermissions();
    // pub fn CoLoadLibrary();
    // pub fn CoFreeLibrary();
    // pub fn CoFreeAllLibraries();
    // pub fn CoGetInstanceFromFile();
    // pub fn CoGetInstanceFromIStorage();
    // pub fn CoAllowSetForegroundWindow();
    // pub fn DcomChannelSetHResult();
    // pub fn CoIsOle1Class();
    // pub fn CLSIDFromProgIDEx();
    // pub fn CoFileTimeToDosDateTime();
    // pub fn CoDosDateTimeToFileTime();
    // pub fn CoFileTimeNow();
    // pub fn CoRegisterMessageFilter();
    // pub fn CoRegisterChannelHook();
    // pub fn CoTreatAsClass();
    // pub fn CreateDataAdviseHolder();
    // pub fn CreateDataCache();
    // pub fn StgOpenAsyncDocfileOnIFillLockBytes();
    // pub fn StgGetIFillLockBytesOnILockBytes();
    // pub fn StgGetIFillLockBytesOnFile();
    // pub fn StgOpenLayoutDocfile();
    // pub fn CoInstall();
    // pub fn BindMoniker();
    // pub fn CoGetObject();
    // pub fn MkParseDisplayName();
    // pub fn MonikerRelativePathTo();
    // pub fn MonikerCommonPrefixWith();
    // pub fn CreateBindCtx();
    // pub fn CreateGenericComposite();
    // pub fn GetClassFile();
    // pub fn CreateClassMoniker();
    // pub fn CreateFileMoniker();
    // pub fn CreateItemMoniker();
    // pub fn CreateAntiMoniker();
    // pub fn CreatePointerMoniker();
    // pub fn CreateObjrefMoniker();
    // pub fn GetRunningObjectTable();
    // pub fn CreateStdProgressIndicator();
