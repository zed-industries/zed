windows_targets::link!("mapi32.dll" "system" fn BuildDisplayTable(lpallocatebuffer : LPALLOCATEBUFFER, lpallocatemore : LPALLOCATEMORE, lpfreebuffer : LPFREEBUFFER, lpmalloc : * mut core::ffi::c_void, hinstance : super::super::Foundation:: HINSTANCE, cpages : u32, lppage : *mut DTPAGE, ulflags : u32, lpptable : *mut * mut core::ffi::c_void, lpptbldata : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("mapi32.dll" "system" fn ChangeIdleRoutine(ftg : *mut core::ffi::c_void, lpfnidle : PFNIDLE, lpvidleparam : *mut core::ffi::c_void, priidle : i16, csecidle : u32, iroidle : u16, ircidle : u16));
windows_targets::link!("mapi32.dll" "system" fn CreateIProp(lpinterface : *mut windows_sys::core::GUID, lpallocatebuffer : LPALLOCATEBUFFER, lpallocatemore : LPALLOCATEMORE, lpfreebuffer : LPFREEBUFFER, lpvreserved : *mut core::ffi::c_void, lpppropdata : *mut * mut core::ffi::c_void) -> i32);
windows_targets::link!("rtm.dll" "system" fn CreateTable(lpinterface : *mut windows_sys::core::GUID, lpallocatebuffer : LPALLOCATEBUFFER, lpallocatemore : LPALLOCATEMORE, lpfreebuffer : LPFREEBUFFER, lpvreserved : *mut core::ffi::c_void, ultabletype : u32, ulproptagindexcolumn : u32, lpsproptagarraycolumns : *mut SPropTagArray, lpptabledata : *mut * mut core::ffi::c_void) -> i32);
windows_targets::link!("mapi32.dll" "system" fn DeinitMapiUtil());
windows_targets::link!("mapi32.dll" "system" fn DeregisterIdleRoutine(ftg : *mut core::ffi::c_void));
windows_targets::link!("mapi32.dll" "system" fn EnableIdleRoutine(ftg : *mut core::ffi::c_void, fenable : super::super::Foundation:: BOOL));
windows_targets::link!("mapi32.dll" "system" fn FEqualNames(lpname1 : *mut MAPINAMEID, lpname2 : *mut MAPINAMEID) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("mapi32.dll" "system" fn FPropCompareProp(lpspropvalue1 : *mut SPropValue, ulrelop : u32, lpspropvalue2 : *mut SPropValue) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("mapi32.dll" "system" fn FPropContainsProp(lpspropvaluedst : *mut SPropValue, lpspropvaluesrc : *mut SPropValue, ulfuzzylevel : u32) -> super::super::Foundation:: BOOL);
windows_targets::link!("mapi32.dll" "system" fn FPropExists(lpmapiprop : * mut core::ffi::c_void, ulproptag : u32) -> super::super::Foundation:: BOOL);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("mapi32.dll" "system" fn FreePadrlist(lpadrlist : *mut ADRLIST));
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("mapi32.dll" "system" fn FreeProws(lprows : *mut SRowSet));
windows_targets::link!("mapi32.dll" "system" fn FtAddFt(ftaddend1 : super::super::Foundation:: FILETIME, ftaddend2 : super::super::Foundation:: FILETIME) -> super::super::Foundation:: FILETIME);
windows_targets::link!("mapi32.dll" "system" fn FtMulDw(ftmultiplier : u32, ftmultiplicand : super::super::Foundation:: FILETIME) -> super::super::Foundation:: FILETIME);
windows_targets::link!("mapi32.dll" "system" fn FtMulDwDw(ftmultiplicand : u32, ftmultiplier : u32) -> super::super::Foundation:: FILETIME);
windows_targets::link!("mapi32.dll" "system" fn FtNegFt(ft : super::super::Foundation:: FILETIME) -> super::super::Foundation:: FILETIME);
windows_targets::link!("mapi32.dll" "system" fn FtSubFt(ftminuend : super::super::Foundation:: FILETIME, ftsubtrahend : super::super::Foundation:: FILETIME) -> super::super::Foundation:: FILETIME);
windows_targets::link!("mapi32.dll" "system" fn FtgRegisterIdleRoutine(lpfnidle : PFNIDLE, lpvidleparam : *mut core::ffi::c_void, priidle : i16, csecidle : u32, iroidle : u16) -> *mut core::ffi::c_void);
windows_targets::link!("mapi32.dll" "system" fn HrAddColumns(lptbl : * mut core::ffi::c_void, lpproptagcolumnsnew : *mut SPropTagArray, lpallocatebuffer : LPALLOCATEBUFFER, lpfreebuffer : LPFREEBUFFER) -> windows_sys::core::HRESULT);
windows_targets::link!("mapi32.dll" "system" fn HrAddColumnsEx(lptbl : * mut core::ffi::c_void, lpproptagcolumnsnew : *mut SPropTagArray, lpallocatebuffer : LPALLOCATEBUFFER, lpfreebuffer : LPFREEBUFFER, lpfnfiltercolumns : isize) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("mapi32.dll" "system" fn HrAllocAdviseSink(lpfncallback : LPNOTIFCALLBACK, lpvcontext : *mut core::ffi::c_void, lppadvisesink : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("mapi32.dll" "system" fn HrDispatchNotifications(ulflags : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("mapi32.dll" "system" fn HrGetOneProp(lpmapiprop : * mut core::ffi::c_void, ulproptag : u32, lppprop : *mut *mut SPropValue) -> windows_sys::core::HRESULT);
windows_targets::link!("mapi32.dll" "system" fn HrIStorageFromStream(lpunkin : * mut core::ffi::c_void, lpinterface : *mut windows_sys::core::GUID, ulflags : u32, lppstorageout : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("mapi32.dll" "system" fn HrQueryAllRows(lptable : * mut core::ffi::c_void, lpproptags : *mut SPropTagArray, lprestriction : *mut SRestriction, lpsortorderset : *mut SSortOrderSet, crowsmax : i32, lpprows : *mut *mut SRowSet) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("mapi32.dll" "system" fn HrSetOneProp(lpmapiprop : * mut core::ffi::c_void, lpprop : *mut SPropValue) -> windows_sys::core::HRESULT);
windows_targets::link!("mapi32.dll" "system" fn HrThisThreadAdviseSink(lpadvisesink : * mut core::ffi::c_void, lppadvisesink : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("mapi32.dll" "system" fn LPropCompareProp(lpspropvaluea : *mut SPropValue, lpspropvalueb : *mut SPropValue) -> i32);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("mapi32.dll" "system" fn LpValFindProp(ulproptag : u32, cvalues : u32, lpproparray : *mut SPropValue) -> *mut SPropValue);
windows_targets::link!("mapi32.dll" "system" fn MAPIDeinitIdle());
windows_targets::link!("mapi32.dll" "system" fn MAPIGetDefaultMalloc() -> * mut core::ffi::c_void);
windows_targets::link!("mapi32.dll" "system" fn MAPIInitIdle(lpvreserved : *mut core::ffi::c_void) -> i32);
windows_targets::link!("mapi32.dll" "system" fn OpenStreamOnFile(lpallocatebuffer : LPALLOCATEBUFFER, lpfreebuffer : LPFREEBUFFER, ulflags : u32, lpszfilename : *const i8, lpszprefix : *const i8, lppstream : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("mapi32.dll" "system" fn PpropFindProp(lpproparray : *mut SPropValue, cvalues : u32, ulproptag : u32) -> *mut SPropValue);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("mapi32.dll" "system" fn PropCopyMore(lpspropvaluedest : *mut SPropValue, lpspropvaluesrc : *mut SPropValue, lpfallocmore : LPALLOCATEMORE, lpvobject : *mut core::ffi::c_void) -> i32);
windows_targets::link!("mapi32.dll" "system" fn RTFSync(lpmessage : * mut core::ffi::c_void, ulflags : u32, lpfmessageupdated : *mut super::super::Foundation:: BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("mapi32.dll" "system" fn ScCopyNotifications(cnotification : i32, lpnotifications : *mut NOTIFICATION, lpvdst : *mut core::ffi::c_void, lpcb : *mut u32) -> i32);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("mapi32.dll" "system" fn ScCopyProps(cvalues : i32, lpproparray : *mut SPropValue, lpvdst : *mut core::ffi::c_void, lpcb : *mut u32) -> i32);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("mapi32.dll" "system" fn ScCountNotifications(cnotifications : i32, lpnotifications : *mut NOTIFICATION, lpcb : *mut u32) -> i32);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("mapi32.dll" "system" fn ScCountProps(cvalues : i32, lpproparray : *mut SPropValue, lpcb : *mut u32) -> i32);
windows_targets::link!("mapi32.dll" "system" fn ScCreateConversationIndex(cbparent : u32, lpbparent : *mut u8, lpcbconvindex : *mut u32, lppbconvindex : *mut *mut u8) -> i32);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("mapi32.dll" "system" fn ScDupPropset(cvalues : i32, lpproparray : *mut SPropValue, lpallocatebuffer : LPALLOCATEBUFFER, lppproparray : *mut *mut SPropValue) -> i32);
windows_targets::link!("mapi32.dll" "system" fn ScInitMapiUtil(ulflags : u32) -> i32);
windows_targets::link!("mapi32.dll" "system" fn ScLocalPathFromUNC(lpszunc : windows_sys::core::PCSTR, lpszlocal : windows_sys::core::PCSTR, cchlocal : u32) -> i32);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("mapi32.dll" "system" fn ScRelocNotifications(cnotification : i32, lpnotifications : *mut NOTIFICATION, lpvbaseold : *mut core::ffi::c_void, lpvbasenew : *mut core::ffi::c_void, lpcb : *mut u32) -> i32);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("mapi32.dll" "system" fn ScRelocProps(cvalues : i32, lpproparray : *mut SPropValue, lpvbaseold : *mut core::ffi::c_void, lpvbasenew : *mut core::ffi::c_void, lpcb : *mut u32) -> i32);
windows_targets::link!("mapi32.dll" "system" fn ScUNCFromLocalPath(lpszlocal : windows_sys::core::PCSTR, lpszunc : windows_sys::core::PCSTR, cchunc : u32) -> i32);
windows_targets::link!("mapi32.dll" "system" fn SzFindCh(lpsz : *mut i8, ch : u16) -> *mut i8);
windows_targets::link!("mapi32.dll" "system" fn SzFindLastCh(lpsz : *mut i8, ch : u16) -> *mut i8);
windows_targets::link!("mapi32.dll" "system" fn SzFindSz(lpsz : *mut i8, lpszkey : *mut i8) -> *mut i8);
windows_targets::link!("mapi32.dll" "system" fn UFromSz(lpsz : *mut i8) -> u32);
windows_targets::link!("mapi32.dll" "system" fn UlAddRef(lpunk : *mut core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_System_Com")]
windows_targets::link!("mapi32.dll" "system" fn UlPropSize(lpspropvalue : *mut SPropValue) -> u32);
windows_targets::link!("mapi32.dll" "system" fn UlRelease(lpunk : *mut core::ffi::c_void) -> u32);
windows_targets::link!("mapi32.dll" "system" fn WrapCompressedRTFStream(lpcompressedrtfstream : * mut core::ffi::c_void, ulflags : u32, lpuncompressedrtfstream : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("mapi32.dll" "system" fn WrapStoreEntryID(ulflags : u32, lpszdllname : *const i8, cborigentry : u32, lporigentry : *const ENTRYID, lpcbwrappedentry : *mut u32, lppwrappedentry : *mut *mut ENTRYID) -> windows_sys::core::HRESULT);
pub const E_IMAPI_BURN_VERIFICATION_FAILED: windows_sys::core::HRESULT = 0xC0AA0007_u32 as _;
pub const E_IMAPI_DF2DATA_CLIENT_NAME_IS_NOT_VALID: windows_sys::core::HRESULT = 0xC0AA0408_u32 as _;
pub const E_IMAPI_DF2DATA_INVALID_MEDIA_STATE: windows_sys::core::HRESULT = 0xC0AA0402_u32 as _;
pub const E_IMAPI_DF2DATA_MEDIA_IS_NOT_SUPPORTED: windows_sys::core::HRESULT = 0xC0AA0406_u32 as _;
pub const E_IMAPI_DF2DATA_MEDIA_NOT_BLANK: windows_sys::core::HRESULT = 0xC0AA0405_u32 as _;
pub const E_IMAPI_DF2DATA_RECORDER_NOT_SUPPORTED: windows_sys::core::HRESULT = 0xC0AA0407_u32 as _;
pub const E_IMAPI_DF2DATA_STREAM_NOT_SUPPORTED: windows_sys::core::HRESULT = 0xC0AA0403_u32 as _;
pub const E_IMAPI_DF2DATA_STREAM_TOO_LARGE_FOR_CURRENT_MEDIA: windows_sys::core::HRESULT = 0xC0AA0404_u32 as _;
pub const E_IMAPI_DF2DATA_WRITE_IN_PROGRESS: windows_sys::core::HRESULT = 0xC0AA0400_u32 as _;
pub const E_IMAPI_DF2DATA_WRITE_NOT_IN_PROGRESS: windows_sys::core::HRESULT = 0xC0AA0401_u32 as _;
pub const E_IMAPI_DF2RAW_CLIENT_NAME_IS_NOT_VALID: windows_sys::core::HRESULT = 0xC0AA0604_u32 as _;
pub const E_IMAPI_DF2RAW_DATA_BLOCK_TYPE_NOT_SUPPORTED: windows_sys::core::HRESULT = 0xC0AA060E_u32 as _;
pub const E_IMAPI_DF2RAW_MEDIA_IS_NOT_BLANK: windows_sys::core::HRESULT = 0xC0AA0606_u32 as _;
pub const E_IMAPI_DF2RAW_MEDIA_IS_NOT_PREPARED: windows_sys::core::HRESULT = 0xC0AA0602_u32 as _;
pub const E_IMAPI_DF2RAW_MEDIA_IS_NOT_SUPPORTED: windows_sys::core::HRESULT = 0xC0AA0607_u32 as _;
pub const E_IMAPI_DF2RAW_MEDIA_IS_PREPARED: windows_sys::core::HRESULT = 0xC0AA0603_u32 as _;
pub const E_IMAPI_DF2RAW_NOT_ENOUGH_SPACE: windows_sys::core::HRESULT = 0xC0AA0609_u32 as _;
pub const E_IMAPI_DF2RAW_NO_RECORDER_SPECIFIED: windows_sys::core::HRESULT = 0xC0AA060A_u32 as _;
pub const E_IMAPI_DF2RAW_RECORDER_NOT_SUPPORTED: windows_sys::core::HRESULT = 0xC0AA0610_u32 as _;
pub const E_IMAPI_DF2RAW_STREAM_LEADIN_TOO_SHORT: windows_sys::core::HRESULT = 0xC0AA060F_u32 as _;
pub const E_IMAPI_DF2RAW_STREAM_NOT_SUPPORTED: windows_sys::core::HRESULT = 0xC0AA060D_u32 as _;
pub const E_IMAPI_DF2RAW_WRITE_IN_PROGRESS: windows_sys::core::HRESULT = 0xC0AA0600_u32 as _;
pub const E_IMAPI_DF2RAW_WRITE_NOT_IN_PROGRESS: windows_sys::core::HRESULT = 0xC0AA0601_u32 as _;
pub const E_IMAPI_DF2TAO_CLIENT_NAME_IS_NOT_VALID: windows_sys::core::HRESULT = 0xC0AA050F_u32 as _;
pub const E_IMAPI_DF2TAO_INVALID_ISRC: windows_sys::core::HRESULT = 0xC0AA050B_u32 as _;
pub const E_IMAPI_DF2TAO_INVALID_MCN: windows_sys::core::HRESULT = 0xC0AA050C_u32 as _;
pub const E_IMAPI_DF2TAO_MEDIA_IS_NOT_BLANK: windows_sys::core::HRESULT = 0xC0AA0506_u32 as _;
pub const E_IMAPI_DF2TAO_MEDIA_IS_NOT_PREPARED: windows_sys::core::HRESULT = 0xC0AA0502_u32 as _;
pub const E_IMAPI_DF2TAO_MEDIA_IS_NOT_SUPPORTED: windows_sys::core::HRESULT = 0xC0AA0507_u32 as _;
pub const E_IMAPI_DF2TAO_MEDIA_IS_PREPARED: windows_sys::core::HRESULT = 0xC0AA0503_u32 as _;
pub const E_IMAPI_DF2TAO_NOT_ENOUGH_SPACE: windows_sys::core::HRESULT = 0xC0AA0509_u32 as _;
pub const E_IMAPI_DF2TAO_NO_RECORDER_SPECIFIED: windows_sys::core::HRESULT = 0xC0AA050A_u32 as _;
pub const E_IMAPI_DF2TAO_PROPERTY_FOR_BLANK_MEDIA_ONLY: windows_sys::core::HRESULT = 0xC0AA0504_u32 as _;
pub const E_IMAPI_DF2TAO_RECORDER_NOT_SUPPORTED: windows_sys::core::HRESULT = 0xC0AA050E_u32 as _;
pub const E_IMAPI_DF2TAO_STREAM_NOT_SUPPORTED: windows_sys::core::HRESULT = 0xC0AA050D_u32 as _;
pub const E_IMAPI_DF2TAO_TABLE_OF_CONTENTS_EMPTY_DISC: windows_sys::core::HRESULT = 0xC0AA0505_u32 as _;
pub const E_IMAPI_DF2TAO_TRACK_LIMIT_REACHED: windows_sys::core::HRESULT = 0xC0AA0508_u32 as _;
pub const E_IMAPI_DF2TAO_WRITE_IN_PROGRESS: windows_sys::core::HRESULT = 0xC0AA0500_u32 as _;
pub const E_IMAPI_DF2TAO_WRITE_NOT_IN_PROGRESS: windows_sys::core::HRESULT = 0xC0AA0501_u32 as _;
pub const E_IMAPI_ERASE_CLIENT_NAME_IS_NOT_VALID: windows_sys::core::HRESULT = 0xC0AA090B_u32 as _;
pub const E_IMAPI_ERASE_DISC_INFORMATION_TOO_SMALL: windows_sys::core::HRESULT = 0x80AA0902_u32 as _;
pub const E_IMAPI_ERASE_DRIVE_FAILED_ERASE_COMMAND: windows_sys::core::HRESULT = 0x80AA0905_u32 as _;
pub const E_IMAPI_ERASE_DRIVE_FAILED_SPINUP_COMMAND: windows_sys::core::HRESULT = 0x80AA0908_u32 as _;
pub const E_IMAPI_ERASE_MEDIA_IS_NOT_ERASABLE: windows_sys::core::HRESULT = 0x80AA0904_u32 as _;
pub const E_IMAPI_ERASE_MEDIA_IS_NOT_SUPPORTED: windows_sys::core::HRESULT = 0xC0AA0909_u32 as _;
pub const E_IMAPI_ERASE_MODE_PAGE_2A_TOO_SMALL: windows_sys::core::HRESULT = 0x80AA0903_u32 as _;
pub const E_IMAPI_ERASE_ONLY_ONE_RECORDER_SUPPORTED: windows_sys::core::HRESULT = 0x80AA0901_u32 as _;
pub const E_IMAPI_ERASE_RECORDER_IN_USE: windows_sys::core::HRESULT = 0x80AA0900_u32 as _;
pub const E_IMAPI_ERASE_RECORDER_NOT_SUPPORTED: windows_sys::core::HRESULT = 0xC0AA090A_u32 as _;
pub const E_IMAPI_ERASE_TOOK_LONGER_THAN_ONE_HOUR: windows_sys::core::HRESULT = 0x80AA0906_u32 as _;
pub const E_IMAPI_ERASE_UNEXPECTED_DRIVE_RESPONSE_DURING_ERASE: windows_sys::core::HRESULT = 0x80AA0907_u32 as _;
pub const E_IMAPI_LOSS_OF_STREAMING: windows_sys::core::HRESULT = 0xC0AA0300_u32 as _;
pub const E_IMAPI_RAW_IMAGE_INSUFFICIENT_SPACE: windows_sys::core::HRESULT = 0x80AA0A05_u32 as _;
pub const E_IMAPI_RAW_IMAGE_IS_READ_ONLY: windows_sys::core::HRESULT = 0x80AA0A00_u32 as _;
pub const E_IMAPI_RAW_IMAGE_NO_TRACKS: windows_sys::core::HRESULT = 0x80AA0A03_u32 as _;
pub const E_IMAPI_RAW_IMAGE_SECTOR_TYPE_NOT_SUPPORTED: windows_sys::core::HRESULT = 0x80AA0A02_u32 as _;
pub const E_IMAPI_RAW_IMAGE_TOO_MANY_TRACKS: windows_sys::core::HRESULT = 0x80AA0A01_u32 as _;
pub const E_IMAPI_RAW_IMAGE_TOO_MANY_TRACK_INDEXES: windows_sys::core::HRESULT = 0x80AA0A06_u32 as _;
pub const E_IMAPI_RAW_IMAGE_TRACKS_ALREADY_ADDED: windows_sys::core::HRESULT = 0x80AA0A04_u32 as _;
pub const E_IMAPI_RAW_IMAGE_TRACK_INDEX_NOT_FOUND: windows_sys::core::HRESULT = 0x80AA0A07_u32 as _;
pub const E_IMAPI_RAW_IMAGE_TRACK_INDEX_OFFSET_ZERO_CANNOT_BE_CLEARED: windows_sys::core::HRESULT = 0x80AA0A09_u32 as _;
pub const E_IMAPI_RAW_IMAGE_TRACK_INDEX_TOO_CLOSE_TO_OTHER_INDEX: windows_sys::core::HRESULT = 0x80AA0A0A_u32 as _;
pub const E_IMAPI_RECORDER_CLIENT_NAME_IS_NOT_VALID: windows_sys::core::HRESULT = 0xC0AA0211_u32 as _;
pub const E_IMAPI_RECORDER_COMMAND_TIMEOUT: windows_sys::core::HRESULT = 0xC0AA020D_u32 as _;
pub const E_IMAPI_RECORDER_DVD_STRUCTURE_NOT_PRESENT: windows_sys::core::HRESULT = 0xC0AA020E_u32 as _;
pub const E_IMAPI_RECORDER_FEATURE_IS_NOT_CURRENT: windows_sys::core::HRESULT = 0xC0AA020B_u32 as _;
pub const E_IMAPI_RECORDER_GET_CONFIGURATION_NOT_SUPPORTED: windows_sys::core::HRESULT = 0xC0AA020C_u32 as _;
pub const E_IMAPI_RECORDER_INVALID_MODE_PARAMETERS: windows_sys::core::HRESULT = 0xC0AA0208_u32 as _;
pub const E_IMAPI_RECORDER_INVALID_RESPONSE_FROM_DEVICE: windows_sys::core::HRESULT = 0xC0AA02FF_u32 as _;
pub const E_IMAPI_RECORDER_LOCKED: windows_sys::core::HRESULT = 0xC0AA0210_u32 as _;
pub const E_IMAPI_RECORDER_MEDIA_BECOMING_READY: windows_sys::core::HRESULT = 0xC0AA0205_u32 as _;
pub const E_IMAPI_RECORDER_MEDIA_BUSY: windows_sys::core::HRESULT = 0xC0AA0207_u32 as _;
pub const E_IMAPI_RECORDER_MEDIA_FORMAT_IN_PROGRESS: windows_sys::core::HRESULT = 0xC0AA0206_u32 as _;
pub const E_IMAPI_RECORDER_MEDIA_INCOMPATIBLE: windows_sys::core::HRESULT = 0xC0AA0203_u32 as _;
pub const E_IMAPI_RECORDER_MEDIA_NOT_FORMATTED: windows_sys::core::HRESULT = 0xC0AA0212_u32 as _;
pub const E_IMAPI_RECORDER_MEDIA_NO_MEDIA: windows_sys::core::HRESULT = 0xC0AA0202_u32 as _;
pub const E_IMAPI_RECORDER_MEDIA_SPEED_MISMATCH: windows_sys::core::HRESULT = 0xC0AA020F_u32 as _;
pub const E_IMAPI_RECORDER_MEDIA_UPSIDE_DOWN: windows_sys::core::HRESULT = 0xC0AA0204_u32 as _;
pub const E_IMAPI_RECORDER_MEDIA_WRITE_PROTECTED: windows_sys::core::HRESULT = 0xC0AA0209_u32 as _;
pub const E_IMAPI_RECORDER_NO_SUCH_FEATURE: windows_sys::core::HRESULT = 0xC0AA020A_u32 as _;
pub const E_IMAPI_RECORDER_NO_SUCH_MODE_PAGE: windows_sys::core::HRESULT = 0xC0AA0201_u32 as _;
pub const E_IMAPI_RECORDER_REQUIRED: windows_sys::core::HRESULT = 0xC0AA0003_u32 as _;
pub const E_IMAPI_REQUEST_CANCELLED: windows_sys::core::HRESULT = 0xC0AA0002_u32 as _;
pub const E_IMAPI_UNEXPECTED_RESPONSE_FROM_DEVICE: windows_sys::core::HRESULT = 0xC0AA0301_u32 as _;
pub const FACILITY_IMAPI2: u32 = 170u32;
pub const IMAPI_E_BAD_MULTISESSION_PARAMETER: windows_sys::core::HRESULT = 0xC0AAB162_u32 as _;
pub const IMAPI_E_BOOT_EMULATION_IMAGE_SIZE_MISMATCH: windows_sys::core::HRESULT = 0xC0AAB14A_u32 as _;
pub const IMAPI_E_BOOT_IMAGE_DATA: windows_sys::core::HRESULT = 0xC0AAB148_u32 as _;
pub const IMAPI_E_BOOT_OBJECT_CONFLICT: windows_sys::core::HRESULT = 0xC0AAB149_u32 as _;
pub const IMAPI_E_DATA_STREAM_CREATE_FAILURE: windows_sys::core::HRESULT = 0xC0AAB12A_u32 as _;
pub const IMAPI_E_DATA_STREAM_INCONSISTENCY: windows_sys::core::HRESULT = 0xC0AAB128_u32 as _;
pub const IMAPI_E_DATA_STREAM_READ_FAILURE: windows_sys::core::HRESULT = 0xC0AAB129_u32 as _;
pub const IMAPI_E_DATA_TOO_BIG: windows_sys::core::HRESULT = 0xC0AAB132_u32 as _;
pub const IMAPI_E_DIRECTORY_READ_FAILURE: windows_sys::core::HRESULT = 0xC0AAB12B_u32 as _;
pub const IMAPI_E_DIR_NOT_EMPTY: windows_sys::core::HRESULT = 0xC0AAB10A_u32 as _;
pub const IMAPI_E_DIR_NOT_FOUND: windows_sys::core::HRESULT = 0xC0AAB11A_u32 as _;
pub const IMAPI_E_DISC_MISMATCH: windows_sys::core::HRESULT = 0xC0AAB158_u32 as _;
pub const IMAPI_E_DUP_NAME: windows_sys::core::HRESULT = 0xC0AAB112_u32 as _;
pub const IMAPI_E_EMPTY_DISC: windows_sys::core::HRESULT = 0xC0AAB150_u32 as _;
pub const IMAPI_E_FILE_NOT_FOUND: windows_sys::core::HRESULT = 0xC0AAB119_u32 as _;
pub const IMAPI_E_FILE_SYSTEM_CHANGE_NOT_ALLOWED: windows_sys::core::HRESULT = 0xC0AAB163_u32 as _;
pub const IMAPI_E_FILE_SYSTEM_FEATURE_NOT_SUPPORTED: windows_sys::core::HRESULT = 0xC0AAB154_u32 as _;
pub const IMAPI_E_FILE_SYSTEM_NOT_EMPTY: windows_sys::core::HRESULT = 0xC0AAB106_u32 as _;
pub const IMAPI_E_FILE_SYSTEM_NOT_FOUND: windows_sys::core::HRESULT = 0xC0AAB152_u32 as _;
pub const IMAPI_E_FILE_SYSTEM_READ_CONSISTENCY_ERROR: windows_sys::core::HRESULT = 0xC0AAB153_u32 as _;
pub const IMAPI_E_FSI_INTERNAL_ERROR: windows_sys::core::HRESULT = 0xC0AAB100_u32 as _;
pub const IMAPI_E_IMAGEMANAGER_IMAGE_NOT_ALIGNED: windows_sys::core::HRESULT = 0xC0AAB200_u32 as _;
pub const IMAPI_E_IMAGEMANAGER_IMAGE_TOO_BIG: windows_sys::core::HRESULT = 0xC0AAB203_u32 as _;
pub const IMAPI_E_IMAGEMANAGER_NO_IMAGE: windows_sys::core::HRESULT = 0xC0AAB202_u32 as _;
pub const IMAPI_E_IMAGEMANAGER_NO_VALID_VD_FOUND: windows_sys::core::HRESULT = 0xC0AAB201_u32 as _;
pub const IMAPI_E_IMAGE_SIZE_LIMIT: windows_sys::core::HRESULT = 0xC0AAB120_u32 as _;
pub const IMAPI_E_IMAGE_TOO_BIG: windows_sys::core::HRESULT = 0xC0AAB121_u32 as _;
pub const IMAPI_E_IMPORT_MEDIA_NOT_ALLOWED: windows_sys::core::HRESULT = 0xC0AAB159_u32 as _;
pub const IMAPI_E_IMPORT_READ_FAILURE: windows_sys::core::HRESULT = 0xC0AAB157_u32 as _;
pub const IMAPI_E_IMPORT_SEEK_FAILURE: windows_sys::core::HRESULT = 0xC0AAB156_u32 as _;
pub const IMAPI_E_IMPORT_TYPE_COLLISION_DIRECTORY_EXISTS_AS_FILE: windows_sys::core::HRESULT = 0xC0AAB15E_u32 as _;
pub const IMAPI_E_IMPORT_TYPE_COLLISION_FILE_EXISTS_AS_DIRECTORY: windows_sys::core::HRESULT = 0xC0AAB155_u32 as _;
pub const IMAPI_E_INCOMPATIBLE_MULTISESSION_TYPE: windows_sys::core::HRESULT = 0xC0AAB15B_u32 as _;
pub const IMAPI_E_INCOMPATIBLE_PREVIOUS_SESSION: windows_sys::core::HRESULT = 0xC0AAB133_u32 as _;
pub const IMAPI_E_INVALID_DATE: windows_sys::core::HRESULT = 0xC0AAB105_u32 as _;
pub const IMAPI_E_INVALID_PARAM: windows_sys::core::HRESULT = 0xC0AAB101_u32 as _;
pub const IMAPI_E_INVALID_PATH: windows_sys::core::HRESULT = 0xC0AAB110_u32 as _;
pub const IMAPI_E_INVALID_VOLUME_NAME: windows_sys::core::HRESULT = 0xC0AAB104_u32 as _;
pub const IMAPI_E_INVALID_WORKING_DIRECTORY: windows_sys::core::HRESULT = 0xC0AAB140_u32 as _;
pub const IMAPI_E_ISO9660_LEVELS: windows_sys::core::HRESULT = 0xC0AAB131_u32 as _;
pub const IMAPI_E_ITEM_NOT_FOUND: windows_sys::core::HRESULT = 0xC0AAB118_u32 as _;
pub const IMAPI_E_MULTISESSION_NOT_SET: windows_sys::core::HRESULT = 0xC0AAB15D_u32 as _;
pub const IMAPI_E_NOT_DIR: windows_sys::core::HRESULT = 0xC0AAB109_u32 as _;
pub const IMAPI_E_NOT_FILE: windows_sys::core::HRESULT = 0xC0AAB108_u32 as _;
pub const IMAPI_E_NOT_IN_FILE_SYSTEM: windows_sys::core::HRESULT = 0xC0AAB10B_u32 as _;
pub const IMAPI_E_NO_COMPATIBLE_MULTISESSION_TYPE: windows_sys::core::HRESULT = 0xC0AAB15C_u32 as _;
pub const IMAPI_E_NO_OUTPUT: windows_sys::core::HRESULT = 0xC0AAB103_u32 as _;
pub const IMAPI_E_NO_SUPPORTED_FILE_SYSTEM: windows_sys::core::HRESULT = 0xC0AAB151_u32 as _;
pub const IMAPI_E_NO_UNIQUE_NAME: windows_sys::core::HRESULT = 0xC0AAB113_u32 as _;
pub const IMAPI_E_PROPERTY_NOT_ACCESSIBLE: windows_sys::core::HRESULT = 0xC0AAB160_u32 as _;
pub const IMAPI_E_READONLY: windows_sys::core::HRESULT = 0xC0AAB102_u32 as _;
pub const IMAPI_E_RESTRICTED_NAME_VIOLATION: windows_sys::core::HRESULT = 0xC0AAB111_u32 as _;
pub const IMAPI_E_STASHFILE_MOVE: windows_sys::core::HRESULT = 0xC0AAB142_u32 as _;
pub const IMAPI_E_STASHFILE_OPEN_FAILURE: windows_sys::core::HRESULT = 0xC0AAB138_u32 as _;
pub const IMAPI_E_STASHFILE_READ_FAILURE: windows_sys::core::HRESULT = 0xC0AAB13B_u32 as _;
pub const IMAPI_E_STASHFILE_SEEK_FAILURE: windows_sys::core::HRESULT = 0xC0AAB139_u32 as _;
pub const IMAPI_E_STASHFILE_WRITE_FAILURE: windows_sys::core::HRESULT = 0xC0AAB13A_u32 as _;
pub const IMAPI_E_TOO_MANY_DIRS: windows_sys::core::HRESULT = 0xC0AAB130_u32 as _;
pub const IMAPI_E_UDF_NOT_WRITE_COMPATIBLE: windows_sys::core::HRESULT = 0xC0AAB15A_u32 as _;
pub const IMAPI_E_UDF_REVISION_CHANGE_NOT_ALLOWED: windows_sys::core::HRESULT = 0xC0AAB161_u32 as _;
pub const IMAPI_E_WORKING_DIRECTORY_SPACE: windows_sys::core::HRESULT = 0xC0AAB141_u32 as _;
pub const IMAPI_S_IMAGE_FEATURE_NOT_SUPPORTED: windows_sys::core::HRESULT = 0xAAB15F_u32 as _;
pub const MAPI_COMPOUND: u32 = 128u32;
pub const MAPI_DIM: u32 = 1u32;
pub const MAPI_ERROR_VERSION: i32 = 0i32;
pub const MAPI_E_CALL_FAILED: i32 = -2147467259i32;
pub const MAPI_E_INTERFACE_NOT_SUPPORTED: i32 = -2147467262i32;
pub const MAPI_E_INVALID_PARAMETER: i32 = -2147024809i32;
pub const MAPI_E_NOT_ENOUGH_MEMORY: i32 = -2147024882i32;
pub const MAPI_E_NO_ACCESS: i32 = -2147024891i32;
pub const MAPI_NOTRECIP: u32 = 64u32;
pub const MAPI_NOTRESERVED: u32 = 8u32;
pub const MAPI_NOW: u32 = 16u32;
pub const MAPI_ONE_OFF_NO_RICH_INFO: u32 = 1u32;
pub const MAPI_P1: u32 = 268435456u32;
pub const MAPI_SHORTTERM: u32 = 128u32;
pub const MAPI_SUBMITTED: u32 = 2147483648u32;
pub const MAPI_THISSESSION: u32 = 32u32;
pub const MAPI_USE_DEFAULT: u32 = 64u32;
pub const MNID_ID: u32 = 0u32;
pub const MNID_STRING: u32 = 1u32;
pub const MV_FLAG: u32 = 4096u32;
pub const MV_INSTANCE: u32 = 8192u32;
pub const OPENSTREAMONFILE: windows_sys::core::PCSTR = windows_sys::core::s!("OpenStreamOnFile");
pub const PRIHIGHEST: u32 = 32767u32;
pub const PRILOWEST: i32 = -32768i32;
pub const PRIUSER: u32 = 0u32;
pub const PROP_ID_INVALID: u32 = 65535u32;
pub const PROP_ID_NULL: u32 = 0u32;
pub const PROP_ID_SECURE_MAX: u32 = 26623u32;
pub const PROP_ID_SECURE_MIN: u32 = 26608u32;
pub const SERVICE_UI_ALLOWED: u32 = 16u32;
pub const SERVICE_UI_ALWAYS: u32 = 2u32;
pub const S_IMAPI_BOTHADJUSTED: windows_sys::core::HRESULT = 0xAA0006_u32 as _;
pub const S_IMAPI_COMMAND_HAS_SENSE_DATA: windows_sys::core::HRESULT = 0xAA0200_u32 as _;
pub const S_IMAPI_RAW_IMAGE_TRACK_INDEX_ALREADY_EXISTS: windows_sys::core::HRESULT = 0xAA0A08_u32 as _;
pub const S_IMAPI_ROTATIONADJUSTED: windows_sys::core::HRESULT = 0xAA0005_u32 as _;
pub const S_IMAPI_SPEEDADJUSTED: windows_sys::core::HRESULT = 0xAA0004_u32 as _;
pub const S_IMAPI_WRITE_NOT_IN_PROGRESS: windows_sys::core::HRESULT = 0xAA0302_u32 as _;
pub const TABLE_CHANGED: u32 = 1u32;
pub const TABLE_ERROR: u32 = 2u32;
pub const TABLE_RELOAD: u32 = 9u32;
pub const TABLE_RESTRICT_DONE: u32 = 7u32;
pub const TABLE_ROW_ADDED: u32 = 3u32;
pub const TABLE_ROW_DELETED: u32 = 4u32;
pub const TABLE_ROW_MODIFIED: u32 = 5u32;
pub const TABLE_SETCOL_DONE: u32 = 8u32;
pub const TABLE_SORT_DONE: u32 = 6u32;
pub const TAD_ALL_ROWS: u32 = 1u32;
pub const UI_CURRENT_PROVIDER_FIRST: u32 = 4u32;
pub const UI_SERVICE: u32 = 2u32;
pub const WABOBJECT_LDAPURL_RETURN_MAILUSER: u32 = 1u32;
pub const WABOBJECT_ME_NEW: u32 = 1u32;
pub const WABOBJECT_ME_NOCREATE: u32 = 2u32;
pub const WAB_CONTEXT_ADRLIST: u32 = 2u32;
pub const WAB_DISPLAY_ISNTDS: u32 = 4u32;
pub const WAB_DISPLAY_LDAPURL: u32 = 1u32;
pub const WAB_DLL_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("WAB32.DLL");
pub const WAB_DLL_PATH_KEY: windows_sys::core::PCWSTR = windows_sys::core::w!("Software\\Microsoft\\WAB\\DLLPath");
pub const WAB_ENABLE_PROFILES: u32 = 4194304u32;
pub const WAB_IGNORE_PROFILES: u32 = 8388608u32;
pub const WAB_LOCAL_CONTAINERS: u32 = 1048576u32;
pub const WAB_PROFILE_CONTENTS: u32 = 2097152u32;
pub const WAB_USE_OE_SENDMAIL: u32 = 1u32;
pub const WAB_VCARD_FILE: u32 = 0u32;
pub const WAB_VCARD_STREAM: u32 = 1u32;
pub const cchProfileNameMax: u32 = 64u32;
pub const cchProfilePassMax: u32 = 64u32;
pub const fMapiUnicode: u32 = 0u32;
pub const genderFemale: Gender = 1i32;
pub const genderMale: Gender = 2i32;
pub const genderUnspecified: Gender = 0i32;
pub const hrSuccess: u32 = 0u32;
pub const szHrDispatchNotifications: windows_sys::core::PCSTR = windows_sys::core::s!("HrDispatchNotifications");
pub const szMAPINotificationMsg: windows_sys::core::PCSTR = windows_sys::core::s!("MAPI Notify window message");
pub const szScCreateConversationIndex: windows_sys::core::PCSTR = windows_sys::core::s!("ScCreateConversationIndex");
pub type Gender = i32;
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct ADRENTRY {
    pub ulReserved1: u32,
    pub cValues: u32,
    pub rgPropVals: *mut SPropValue,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct ADRLIST {
    pub cEntries: u32,
    pub aEntries: [ADRENTRY; 1],
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct ADRPARM {
    pub cbABContEntryID: u32,
    pub lpABContEntryID: *mut ENTRYID,
    pub ulFlags: u32,
    pub lpReserved: *mut core::ffi::c_void,
    pub ulHelpContext: u32,
    pub lpszHelpFileName: *mut i8,
    pub lpfnABSDI: LPFNABSDI,
    pub lpfnDismiss: LPFNDISMISS,
    pub lpvDismissContext: *mut core::ffi::c_void,
    pub lpszCaption: *mut i8,
    pub lpszNewEntryTitle: *mut i8,
    pub lpszDestWellsTitle: *mut i8,
    pub cDestFields: u32,
    pub nDestFieldFocus: u32,
    pub lppszDestTitles: *mut *mut i8,
    pub lpulDestComps: *mut u32,
    pub lpContRestriction: *mut SRestriction,
    pub lpHierRestriction: *mut SRestriction,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DTBLBUTTON {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
    pub ulPRControl: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DTBLCHECKBOX {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
    pub ulPRPropertyName: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DTBLCOMBOBOX {
    pub ulbLpszCharsAllowed: u32,
    pub ulFlags: u32,
    pub ulNumCharsAllowed: u32,
    pub ulPRPropertyName: u32,
    pub ulPRTableName: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DTBLDDLBX {
    pub ulFlags: u32,
    pub ulPRDisplayProperty: u32,
    pub ulPRSetProperty: u32,
    pub ulPRTableName: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DTBLEDIT {
    pub ulbLpszCharsAllowed: u32,
    pub ulFlags: u32,
    pub ulNumCharsAllowed: u32,
    pub ulPropTag: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DTBLGROUPBOX {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DTBLLABEL {
    pub ulbLpszLabelName: u32,
    pub ulFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DTBLLBX {
    pub ulFlags: u32,
    pub ulPRSetProperty: u32,
    pub ulPRTableName: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DTBLMVDDLBX {
    pub ulFlags: u32,
    pub ulMVPropTag: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DTBLMVLISTBOX {
    pub ulFlags: u32,
    pub ulMVPropTag: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DTBLPAGE {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
    pub ulbLpszComponent: u32,
    pub ulContext: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DTBLRADIOBUTTON {
    pub ulbLpszLabel: u32,
    pub ulFlags: u32,
    pub ulcButtons: u32,
    pub ulPropTag: u32,
    pub lReturnValue: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DTCTL {
    pub ulCtlType: u32,
    pub ulCtlFlags: u32,
    pub lpbNotif: *mut u8,
    pub cbNotif: u32,
    pub lpszFilter: *mut i8,
    pub ulItemID: u32,
    pub ctl: DTCTL_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DTCTL_0 {
    pub lpv: *mut core::ffi::c_void,
    pub lplabel: *mut DTBLLABEL,
    pub lpedit: *mut DTBLEDIT,
    pub lplbx: *mut DTBLLBX,
    pub lpcombobox: *mut DTBLCOMBOBOX,
    pub lpddlbx: *mut DTBLDDLBX,
    pub lpcheckbox: *mut DTBLCHECKBOX,
    pub lpgroupbox: *mut DTBLGROUPBOX,
    pub lpbutton: *mut DTBLBUTTON,
    pub lpradiobutton: *mut DTBLRADIOBUTTON,
    pub lpmvlbx: *mut DTBLMVLISTBOX,
    pub lpmvddlbx: *mut DTBLMVDDLBX,
    pub lppage: *mut DTBLPAGE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DTPAGE {
    pub cctl: u32,
    pub lpszResourceName: *mut i8,
    pub Anonymous: DTPAGE_0,
    pub lpctl: *mut DTCTL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DTPAGE_0 {
    pub lpszComponent: *mut i8,
    pub ulItemID: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ENTRYID {
    pub abFlags: [u8; 4],
    pub ab: [u8; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ERROR_NOTIFICATION {
    pub cbEntryID: u32,
    pub lpEntryID: *mut ENTRYID,
    pub scode: i32,
    pub ulFlags: u32,
    pub lpMAPIError: *mut MAPIERROR,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXTENDED_NOTIFICATION {
    pub ulEvent: u32,
    pub cb: u32,
    pub pbEventParameters: *mut u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FLATENTRY {
    pub cb: u32,
    pub abEntry: [u8; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FLATENTRYLIST {
    pub cEntries: u32,
    pub cbEntries: u32,
    pub abEntries: [u8; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FLATMTSIDLIST {
    pub cMTSIDs: u32,
    pub cbMTSIDs: u32,
    pub abMTSIDs: [u8; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FlagList {
    pub cFlags: u32,
    pub ulFlag: [u32; 1],
}
pub type LPWABACTIONITEM = isize;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MAPIERROR {
    pub ulVersion: u32,
    pub lpszError: *mut i8,
    pub lpszComponent: *mut i8,
    pub ulLowLevelError: u32,
    pub ulContext: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MAPINAMEID {
    pub lpguid: *mut windows_sys::core::GUID,
    pub ulKind: u32,
    pub Kind: MAPINAMEID_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MAPINAMEID_0 {
    pub lID: i32,
    pub lpwstrName: windows_sys::core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MAPIUID {
    pub ab: [u8; 16],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MTSID {
    pub cb: u32,
    pub ab: [u8; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NEWMAIL_NOTIFICATION {
    pub cbEntryID: u32,
    pub lpEntryID: *mut ENTRYID,
    pub cbParentID: u32,
    pub lpParentID: *mut ENTRYID,
    pub ulFlags: u32,
    pub lpszMessageClass: *mut i8,
    pub ulMessageFlags: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct NOTIFICATION {
    pub ulEventType: u32,
    pub ulAlignPad: u32,
    pub info: NOTIFICATION_0,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub union NOTIFICATION_0 {
    pub err: ERROR_NOTIFICATION,
    pub newmail: NEWMAIL_NOTIFICATION,
    pub obj: OBJECT_NOTIFICATION,
    pub tab: TABLE_NOTIFICATION,
    pub ext: EXTENDED_NOTIFICATION,
    pub statobj: STATUS_OBJECT_NOTIFICATION,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NOTIFKEY {
    pub cb: u32,
    pub ab: [u8; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OBJECT_NOTIFICATION {
    pub cbEntryID: u32,
    pub lpEntryID: *mut ENTRYID,
    pub ulObjType: u32,
    pub cbParentID: u32,
    pub lpParentID: *mut ENTRYID,
    pub cbOldID: u32,
    pub lpOldID: *mut ENTRYID,
    pub cbOldParentID: u32,
    pub lpOldParentID: *mut ENTRYID,
    pub lpPropTagArray: *mut SPropTagArray,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct SAndRestriction {
    pub cRes: u32,
    pub lpRes: *mut SRestriction,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SAppTimeArray {
    pub cValues: u32,
    pub lpat: *mut f64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SBinary {
    pub cb: u32,
    pub lpb: *mut u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SBinaryArray {
    pub cValues: u32,
    pub lpbin: *mut SBinary,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SBitMaskRestriction {
    pub relBMR: u32,
    pub ulPropTag: u32,
    pub ulMask: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct SCommentRestriction {
    pub cValues: u32,
    pub lpRes: *mut SRestriction,
    pub lpProp: *mut SPropValue,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SComparePropsRestriction {
    pub relop: u32,
    pub ulPropTag1: u32,
    pub ulPropTag2: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct SContentRestriction {
    pub ulFuzzyLevel: u32,
    pub ulPropTag: u32,
    pub lpProp: *mut SPropValue,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct SCurrencyArray {
    pub cValues: u32,
    pub lpcur: *mut super::Com::CY,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDateTimeArray {
    pub cValues: u32,
    pub lpft: *mut super::super::Foundation::FILETIME,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDoubleArray {
    pub cValues: u32,
    pub lpdbl: *mut f64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SExistRestriction {
    pub ulReserved1: u32,
    pub ulPropTag: u32,
    pub ulReserved2: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SGuidArray {
    pub cValues: u32,
    pub lpguid: *mut windows_sys::core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SLPSTRArray {
    pub cValues: u32,
    pub lppszA: *mut windows_sys::core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SLargeIntegerArray {
    pub cValues: u32,
    pub lpli: *mut i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SLongArray {
    pub cValues: u32,
    pub lpl: *mut i32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct SNotRestriction {
    pub ulReserved: u32,
    pub lpRes: *mut SRestriction,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct SOrRestriction {
    pub cRes: u32,
    pub lpRes: *mut SRestriction,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SPropProblem {
    pub ulIndex: u32,
    pub ulPropTag: u32,
    pub scode: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SPropProblemArray {
    pub cProblem: u32,
    pub aProblem: [SPropProblem; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SPropTagArray {
    pub cValues: u32,
    pub aulPropTag: [u32; 1],
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct SPropValue {
    pub ulPropTag: u32,
    pub dwAlignPad: u32,
    pub Value: __UPV,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct SPropertyRestriction {
    pub relop: u32,
    pub ulPropTag: u32,
    pub lpProp: *mut SPropValue,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SRealArray {
    pub cValues: u32,
    pub lpflt: *mut f32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct SRestriction {
    pub rt: u32,
    pub res: SRestriction_0,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub union SRestriction_0 {
    pub resCompareProps: SComparePropsRestriction,
    pub resAnd: SAndRestriction,
    pub resOr: SOrRestriction,
    pub resNot: SNotRestriction,
    pub resContent: SContentRestriction,
    pub resProperty: SPropertyRestriction,
    pub resBitMask: SBitMaskRestriction,
    pub resSize: SSizeRestriction,
    pub resExist: SExistRestriction,
    pub resSub: SSubRestriction,
    pub resComment: SCommentRestriction,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct SRow {
    pub ulAdrEntryPad: u32,
    pub cValues: u32,
    pub lpProps: *mut SPropValue,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct SRowSet {
    pub cRows: u32,
    pub aRow: [SRow; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SShortArray {
    pub cValues: u32,
    pub lpi: *mut i16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SSizeRestriction {
    pub relop: u32,
    pub ulPropTag: u32,
    pub cb: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SSortOrder {
    pub ulPropTag: u32,
    pub ulOrder: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SSortOrderSet {
    pub cSorts: u32,
    pub cCategories: u32,
    pub cExpanded: u32,
    pub aSort: [SSortOrder; 1],
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct SSubRestriction {
    pub ulSubObject: u32,
    pub lpRes: *mut SRestriction,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct STATUS_OBJECT_NOTIFICATION {
    pub cbEntryID: u32,
    pub lpEntryID: *mut ENTRYID,
    pub cValues: u32,
    pub lpPropVals: *mut SPropValue,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SWStringArray {
    pub cValues: u32,
    pub lppszW: *mut windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct TABLE_NOTIFICATION {
    pub ulTableEvent: u32,
    pub hResult: windows_sys::core::HRESULT,
    pub propIndex: SPropValue,
    pub propPrior: SPropValue,
    pub row: SRow,
    pub ulPad: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WABEXTDISPLAY {
    pub cbSize: u32,
    pub lpWABObject: *mut core::ffi::c_void,
    pub lpAdrBook: *mut core::ffi::c_void,
    pub lpPropObj: *mut core::ffi::c_void,
    pub fReadOnly: super::super::Foundation::BOOL,
    pub fDataChanged: super::super::Foundation::BOOL,
    pub ulFlags: u32,
    pub lpv: *mut core::ffi::c_void,
    pub lpsz: *mut i8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WABIMPORTPARAM {
    pub cbSize: u32,
    pub lpAdrBook: *mut core::ffi::c_void,
    pub hWnd: super::super::Foundation::HWND,
    pub ulFlags: u32,
    pub lpszFileName: windows_sys::core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WAB_PARAM {
    pub cbSize: u32,
    pub hwnd: super::super::Foundation::HWND,
    pub szFileName: windows_sys::core::PSTR,
    pub ulFlags: u32,
    pub guidPSExt: windows_sys::core::GUID,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub union __UPV {
    pub i: i16,
    pub l: i32,
    pub ul: u32,
    pub flt: f32,
    pub dbl: f64,
    pub b: u16,
    pub cur: super::Com::CY,
    pub at: f64,
    pub ft: super::super::Foundation::FILETIME,
    pub lpszA: windows_sys::core::PSTR,
    pub bin: SBinary,
    pub lpszW: windows_sys::core::PWSTR,
    pub lpguid: *mut windows_sys::core::GUID,
    pub li: i64,
    pub MVi: SShortArray,
    pub MVl: SLongArray,
    pub MVflt: SRealArray,
    pub MVdbl: SDoubleArray,
    pub MVcur: SCurrencyArray,
    pub MVat: SAppTimeArray,
    pub MVft: SDateTimeArray,
    pub MVbin: SBinaryArray,
    pub MVszA: SLPSTRArray,
    pub MVszW: SWStringArray,
    pub MVguid: SGuidArray,
    pub MVli: SLargeIntegerArray,
    pub err: i32,
    pub x: i32,
}
pub type CALLERRELEASE = Option<unsafe extern "system" fn(ulcallerdata: u32, lptbldata: *mut core::ffi::c_void, lpvue: *mut core::ffi::c_void)>;
pub type LPALLOCATEBUFFER = Option<unsafe extern "system" fn(cbsize: u32, lppbuffer: *mut *mut core::ffi::c_void) -> i32>;
pub type LPALLOCATEMORE = Option<unsafe extern "system" fn(cbsize: u32, lpobject: *mut core::ffi::c_void, lppbuffer: *mut *mut core::ffi::c_void) -> i32>;
pub type LPCREATECONVERSATIONINDEX = Option<unsafe extern "system" fn(cbparent: u32, lpbparent: *mut u8, lpcbconvindex: *mut u32, lppbconvindex: *mut *mut u8) -> i32>;
pub type LPDISPATCHNOTIFICATIONS = Option<unsafe extern "system" fn(ulflags: u32) -> windows_sys::core::HRESULT>;
pub type LPFNABSDI = Option<unsafe extern "system" fn(uluiparam: usize, lpvmsg: *mut core::ffi::c_void) -> super::super::Foundation::BOOL>;
pub type LPFNBUTTON = Option<unsafe extern "system" fn(uluiparam: usize, lpvcontext: *mut core::ffi::c_void, cbentryid: u32, lpselection: *mut ENTRYID, ulflags: u32) -> i32>;
pub type LPFNDISMISS = Option<unsafe extern "system" fn(uluiparam: usize, lpvcontext: *mut core::ffi::c_void)>;
pub type LPFREEBUFFER = Option<unsafe extern "system" fn(lpbuffer: *mut core::ffi::c_void) -> u32>;
#[cfg(feature = "Win32_System_Com")]
pub type LPNOTIFCALLBACK = Option<unsafe extern "system" fn(lpvcontext: *mut core::ffi::c_void, cnotification: u32, lpnotifications: *mut NOTIFICATION) -> i32>;
pub type LPOPENSTREAMONFILE = Option<unsafe extern "system" fn(lpallocatebuffer: LPALLOCATEBUFFER, lpfreebuffer: LPFREEBUFFER, ulflags: u32, lpszfilename: *const i8, lpszprefix: *const i8, lppstream: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
pub type LPWABALLOCATEBUFFER = Option<unsafe extern "system" fn(lpwabobject: *mut core::ffi::c_void, cbsize: u32, lppbuffer: *mut *mut core::ffi::c_void) -> i32>;
pub type LPWABALLOCATEMORE = Option<unsafe extern "system" fn(lpwabobject: *mut core::ffi::c_void, cbsize: u32, lpobject: *mut core::ffi::c_void, lppbuffer: *mut *mut core::ffi::c_void) -> i32>;
pub type LPWABFREEBUFFER = Option<unsafe extern "system" fn(lpwabobject: *mut core::ffi::c_void, lpbuffer: *mut core::ffi::c_void) -> u32>;
pub type LPWABOPEN = Option<unsafe extern "system" fn(lppadrbook: *mut *mut core::ffi::c_void, lppwabobject: *mut *mut core::ffi::c_void, lpwp: *mut WAB_PARAM, reserved2: u32) -> windows_sys::core::HRESULT>;
pub type LPWABOPENEX = Option<unsafe extern "system" fn(lppadrbook: *mut *mut core::ffi::c_void, lppwabobject: *mut *mut core::ffi::c_void, lpwp: *mut WAB_PARAM, reserved: u32, fnallocatebuffer: LPALLOCATEBUFFER, fnallocatemore: LPALLOCATEMORE, fnfreebuffer: LPFREEBUFFER) -> windows_sys::core::HRESULT>;
pub type PFNIDLE = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void) -> super::super::Foundation::BOOL>;
