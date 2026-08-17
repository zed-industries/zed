pub type IEnumWIA_DEV_CAPS = *mut ::core::ffi::c_void;
pub type IEnumWIA_DEV_INFO = *mut ::core::ffi::c_void;
pub type IEnumWIA_FORMAT_INFO = *mut ::core::ffi::c_void;
pub type IEnumWiaItem = *mut ::core::ffi::c_void;
pub type IEnumWiaItem2 = *mut ::core::ffi::c_void;
pub type IWiaAppErrorHandler = *mut ::core::ffi::c_void;
pub type IWiaDataCallback = *mut ::core::ffi::c_void;
pub type IWiaDataTransfer = *mut ::core::ffi::c_void;
pub type IWiaDevMgr = *mut ::core::ffi::c_void;
pub type IWiaDevMgr2 = *mut ::core::ffi::c_void;
pub type IWiaDrvItem = *mut ::core::ffi::c_void;
pub type IWiaErrorHandler = *mut ::core::ffi::c_void;
pub type IWiaEventCallback = *mut ::core::ffi::c_void;
pub type IWiaImageFilter = *mut ::core::ffi::c_void;
pub type IWiaItem = *mut ::core::ffi::c_void;
pub type IWiaItem2 = *mut ::core::ffi::c_void;
pub type IWiaItemExtras = *mut ::core::ffi::c_void;
pub type IWiaLog = *mut ::core::ffi::c_void;
pub type IWiaLogEx = *mut ::core::ffi::c_void;
pub type IWiaMiniDrv = *mut ::core::ffi::c_void;
pub type IWiaMiniDrvCallBack = *mut ::core::ffi::c_void;
pub type IWiaMiniDrvTransferCallback = *mut ::core::ffi::c_void;
pub type IWiaNotifyDevMgr = *mut ::core::ffi::c_void;
pub type IWiaPreview = *mut ::core::ffi::c_void;
pub type IWiaPropertyStorage = *mut ::core::ffi::c_void;
pub type IWiaSegmentationFilter = *mut ::core::ffi::c_void;
pub type IWiaTransfer = *mut ::core::ffi::c_void;
pub type IWiaTransferCallback = *mut ::core::ffi::c_void;
pub type IWiaUIExtension = *mut ::core::ffi::c_void;
pub type IWiaUIExtension2 = *mut ::core::ffi::c_void;
pub type IWiaVideo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const ADVANCED_DUP: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const ADVANCED_DUPLEX: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const ALL_PAGES: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const AUTO_ADVANCE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const AUTO_SOURCE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const BACK_FIRST: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const BACK_ONLY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const BARCODE_READER: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const BARCODE_READER_READY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const BASE_VAL_WIA_ERROR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const BASE_VAL_WIA_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const BOTTOM_JUSTIFIED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const BUS_TYPE_FIREWIRE: u32 = 203u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const BUS_TYPE_PARALLEL: u32 = 202u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const BUS_TYPE_SCSI: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const BUS_TYPE_USB: u32 = 201u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CAPTUREMODE_BURST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CAPTUREMODE_NORMAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CAPTUREMODE_TIMELAPSE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CENTERED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CFSTR_WIAITEMNAMES: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("WIAItemNames");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CFSTR_WIAITEMPTR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("WIAItemPointer");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CLSID_WiaDefaultSegFilter: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd4f4d30b_0b29_4508_8922_0c5797d42765);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_GETADFAVAILABLE: u32 = 117u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_GETADFHASPAPER: u32 = 120u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_GETADFOPEN: u32 = 118u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_GETADFREADY: u32 = 119u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_GETADFSTATUS: u32 = 121u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_GETADFUNLOADREADY: u32 = 122u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_GETCAPABILITIES: u32 = 132u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_GETSUPPORTEDFILEFORMATS: u32 = 138u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_GETSUPPORTEDMEMORYFORMATS: u32 = 139u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_GETTPAAVAILABLE: u32 = 123u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_GETTPAOPENED: u32 = 124u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_GET_INTERRUPT_EVENT: u32 = 133u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_INITIALIZE: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_LOAD_ADF: u32 = 115u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_RESETSCANNER: u32 = 131u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SENDSCSICOMMAND: u32 = 127u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETCOLORDITHER: u32 = 111u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETCONTRAST: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETDATATYPE: u32 = 106u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETDITHER: u32 = 107u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETFILTER: u32 = 114u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETFORMAT: u32 = 140u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETGSDNAME: u32 = 134u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETINTENSITY: u32 = 105u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETLAMP: u32 = 126u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETMATRIX: u32 = 112u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETMIRROR: u32 = 108u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETNEGATIVE: u32 = 109u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETSCANMODE: u32 = 135u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETSPEED: u32 = 113u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETSTIDEVICEHKEY: u32 = 136u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETTONEMAP: u32 = 110u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETXRESOLUTION: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETYRESOLUTION: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_STI_DEVICERESET: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_STI_DIAGNOSTIC: u32 = 130u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_STI_GETSTATUS: u32 = 129u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_TPAREADY: u32 = 125u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_UNINITIALIZE: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_UNLOAD_ADF: u32 = 116u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const COPY_PARENT_PROPERTY_VALUES: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const DETECT_DUP: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const DETECT_DUP_AVAIL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const DETECT_FEED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const DETECT_FEED_AVAIL: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const DETECT_FILM_TPA: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const DETECT_FLAT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const DETECT_SCAN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const DETECT_STOR: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const DEVICE_ATTENTION: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const DUP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const DUPLEX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const DUP_READY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EFFECTMODE_BW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EFFECTMODE_SEPIA: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EFFECTMODE_STANDARD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const ENDORSER: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const ENDORSER_READY: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const ESC_TWAIN_CAPABILITY: u32 = 2001u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const ESC_TWAIN_PRIVATE_SUPPORTED_CAPS: u32 = 2002u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EXPOSUREMETERING_AVERAGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EXPOSUREMETERING_CENTERSPOT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EXPOSUREMETERING_CENTERWEIGHT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EXPOSUREMETERING_MULTISPOT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EXPOSUREMODE_APERTURE_PRIORITY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EXPOSUREMODE_AUTO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EXPOSUREMODE_MANUAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EXPOSUREMODE_PORTRAIT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EXPOSUREMODE_PROGRAM_ACTION: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EXPOSUREMODE_PROGRAM_CREATIVE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EXPOSUREMODE_SHUTTER_PRIORITY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FEED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FEEDER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FEED_READY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FILM_TPA: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FILM_TPA_READY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FLASHMODE_AUTO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FLASHMODE_EXTERNALSYNC: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FLASHMODE_FILL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FLASHMODE_OFF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FLASHMODE_REDEYE_AUTO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FLASHMODE_REDEYE_FILL: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FLAT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FLATBED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FLAT_COVER_UP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FLAT_READY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FOCUSMETERING_CENTERSPOT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FOCUSMETERING_MULTISPOT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FOCUSMODE_AUTO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FOCUSMODE_MACROAUTO: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FOCUSMODE_MANUAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FRONT_FIRST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FRONT_ONLY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const GUID_DEVINTERFACE_IMAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6bdd1fc6_810f_11d0_bec7_08002be2092f);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const IMPRINTER: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const IMPRINTER_READY: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const IT_MSG_DATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const IT_MSG_DATA_HEADER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const IT_MSG_FILE_PREVIEW_DATA: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const IT_MSG_FILE_PREVIEW_DATA_HEADER: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const IT_MSG_NEW_PAGE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const IT_MSG_STATUS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const IT_MSG_TERMINATION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const IT_STATUS_MASK: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const IT_STATUS_PROCESSING_DATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const IT_STATUS_TRANSFER_FROM_DEVICE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const IT_STATUS_TRANSFER_TO_CLIENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const LAMP_ERR: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const LANDSCAPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const LANSCAPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const LEFT_JUSTIFIED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const LIGHT_SOURCE_DETECT_READY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const LIGHT_SOURCE_NEGATIVE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const LIGHT_SOURCE_POSITIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const LIGHT_SOURCE_PRESENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const LIGHT_SOURCE_PRESENT_DETECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const LIGHT_SOURCE_READY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const LIGHT_SOURCE_SELECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MAX_ANSI_CHAR: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MAX_IO_HANDLES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MAX_RESERVED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MCRO_ERROR_GENERAL_ERROR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MCRO_ERROR_OFFLINE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MCRO_ERROR_PAPER_EMPTY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MCRO_ERROR_PAPER_JAM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MCRO_ERROR_PAPER_PROBLEM: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MCRO_ERROR_USER_INTERVENTION: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MCRO_STATUS_OK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MICR_READER: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MICR_READER_READY: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MIRRORED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MULTIPLE_FEED: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const NEXT_PAGE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const PAPER_JAM: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const PATCH_CODE_READER: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const PATCH_CODE_READER_READY: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const PATH_COVER_UP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const PORTRAIT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const POWERMODE_BATTERY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const POWERMODE_LINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const PREFEED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const RIGHT_JUSTIFIED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const ROT180: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const ROT270: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const SCANMODE_FINALSCAN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const SCANMODE_PREVIEWSCAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const SCAN_FINISHED: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const SCAN_FIRST: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const SCAN_NEXT: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const SHELLEX_WIAUIEXTENSION_NAME: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("WiaDialogExtensionHandlers");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const STOR: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const STORAGE_FULL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const STORAGE_READY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const SUPPORT_BW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const SUPPORT_COLOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const SUPPORT_GRAYSCALE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const TOP_JUSTIFIED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const TRANSPARENCY_DYNAMIC_FRAME_SUPPORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const TRANSPARENCY_STATIC_FRAME_SUPPORT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const TYMED_CALLBACK: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const TYMED_MULTIPAGE_CALLBACK: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const TYMED_MULTIPAGE_FILE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WHITEBALANCE_AUTO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WHITEBALANCE_DAYLIGHT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WHITEBALANCE_FLASH: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WHITEBALANCE_FLORESCENT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WHITEBALANCE_MANUAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WHITEBALANCE_ONEPUSH_AUTO: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WHITEBALANCE_TUNGSTEN: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIAU_DEBUG_TSTR: ::windows_sys::core::PCSTR = ::windows_sys::core::s!("S");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ACTION_EVENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ADVANCED_PREVIEW: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ALARM_BEEP1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ALARM_BEEP10: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ALARM_BEEP2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ALARM_BEEP3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ALARM_BEEP4: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ALARM_BEEP5: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ALARM_BEEP6: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ALARM_BEEP7: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ALARM_BEEP8: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ALARM_BEEP9: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ALARM_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_AUTO_CROP_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_AUTO_CROP_MULTI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_AUTO_CROP_SINGLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_AUTO_DESKEW_OFF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_AUTO_DESKEW_ON: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_AUTO_SEARCH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_AZTEC: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_CODABAR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_CODE128: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_CODE128A: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_CODE128B: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_CODE128C: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_CODE39: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_CODE39_FULLASCII: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_CODE39_MOD43: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_CODE93: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_CPCBINARY: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_CUSTOMBASE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_DATAMATRIX: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_DATASTRIP: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_EAN13: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_EAN8: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_EZCODE: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_FIM: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_GS1128: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_GS1DATABAR: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_HIGH_CAPACITY_COLOR: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_HORIZONTAL_SEARCH: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_HORIZONTAL_VERTICAL_SEARCH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_INTELLIGENT_MAIL: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_INTERLEAVED_2OF5: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_ITF14: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_JAN: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_MAXICODE: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_MSI: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_NONINTERLEAVED_2OF5: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_PDF417: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_PHARMACODE: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_PLANET: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_PLESSEY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_POSTBAR: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_POSTNETA: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_POSTNETB: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_POSTNETC: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_POSTNET_DPBC: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_QRCODE: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_READER_AUTO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_READER_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_READER_FEEDER_BACK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_READER_FEEDER_DUPLEX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_READER_FEEDER_FRONT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_READER_FLATBED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_RM4SCC: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_SHOTCODE: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_SMALLAZTEC: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_SPARQCODE: u32 = 43u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_TELEPEN: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_UPCA: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_UPCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_VERTICAL_HORIZONTAL_SEARCH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_VERTICAL_SEARCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BASIC_PREVIEW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BLANK_PAGE_DETECTION_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BLANK_PAGE_DISCARD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BLANK_PAGE_JOB_SEPARATOR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_CATEGORY_AUTO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xdefe5fd8_6c97_4dde_b11e_cb509b270e11);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_CATEGORY_BARCODE_READER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x36e178a0_473f_494b_af8f_6c3f6d7486fc);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_CATEGORY_ENDORSER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x47102cc3_127f_4771_adfc_991ab8ee1e97);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_CATEGORY_FEEDER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfe131934_f84c_42ad_8da4_6129cddd7288);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_CATEGORY_FEEDER_BACK: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x61ca74d4_39db_42aa_89b1_8c19c9cd4c23);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_CATEGORY_FEEDER_FRONT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4823175c_3b28_487b_a7e6_eebc17614fd1);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_CATEGORY_FILM: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfcf65be7_3ce3_4473_af85_f5d37d21b68a);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_CATEGORY_FINISHED_FILE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xff2b77ca_cf84_432b_a735_3a130dde2a88);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_CATEGORY_FLATBED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfb607b1f_43f3_488b_855b_fb703ec342a6);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_CATEGORY_FOLDER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc692a446_6f5a_481d_85bb_92e2e86fd30a);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_CATEGORY_IMPRINTER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfc65016d_9202_43dd_91a7_64c2954cfb8b);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_CATEGORY_MICR_READER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3b86c1ec_71bc_4645_b4d5_1b19da2be978);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_CATEGORY_PATCH_CODE_READER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8faa1a6d_9c8a_42cd_98b3_ee9700cbc74f);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_CATEGORY_ROOT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf193526f_59b8_4a26_9888_e16e4f97ce10);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_CMD_BUILD_DEVICE_TREE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9cba5ce0_dbea_11d2_8416_00c04fa36145);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_CMD_CHANGE_DOCUMENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x04e725b0_acae_11d2_a093_00c04f72dc3c);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_CMD_DELETE_ALL_ITEMS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe208c170_acad_11d2_a093_00c04f72dc3c);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_CMD_DELETE_DEVICE_TREE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x73815942_dbea_11d2_8416_00c04fa36145);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_CMD_DIAGNOSTIC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x10ff52f5_de04_4cf0_a5ad_691f8dce0141);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_CMD_FORMAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc3a693aa_f788_4d34_a5b0_be7190759a24);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_CMD_PAUSE_FEEDER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x50985e4d_a5b2_4b71_9c95_6d7d7c469a43);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_CMD_START_FEEDER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5a9df6c9_5f2d_4a39_9d6c_00456d047f00);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_CMD_STOP_FEEDER: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd847b06d_3905_459c_9509_9b29cdb691e7);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_CMD_SYNCHRONIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9b26b7b2_acad_11d2_a093_00c04f72dc3c);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_CMD_TAKE_PICTURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xaf933cac_acad_11d2_a093_00c04f72dc3c);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_CMD_UNLOAD_DOCUMENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1f3b3d8e_acae_11d2_a093_00c04f72dc3c);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COLOR_DROP_BLUE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COLOR_DROP_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COLOR_DROP_GREEN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COLOR_DROP_RED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COLOR_DROP_RGB: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COMPRESSION_AUTO: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COMPRESSION_BI_RLE4: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COMPRESSION_BI_RLE8: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COMPRESSION_G3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COMPRESSION_G4: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COMPRESSION_JBIG: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COMPRESSION_JPEG: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COMPRESSION_JPEG2K: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COMPRESSION_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COMPRESSION_PNG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DATA_AUTO: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DATA_COLOR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DATA_COLOR_DITHER: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DATA_COLOR_THRESHOLD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DATA_DITHER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DATA_GRAYSCALE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DATA_RAW_BGR: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DATA_RAW_CMY: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DATA_RAW_CMYK: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DATA_RAW_RGB: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DATA_RAW_YUV: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DATA_RAW_YUVK: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DATA_THRESHOLD: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DEPTH_AUTO: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DEVICE_COMMANDS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DEVICE_CONNECTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DEVICE_DIALOG_SINGLE_IMAGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DEVICE_DIALOG_USE_COMMON_UI: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DEVICE_EVENTS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DEVICE_NOT_CONNECTED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DEVINFO_ENUM_ALL: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DEVINFO_ENUM_LOCAL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_BAUDRATE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_BAUDRATE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("BaudRate");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_DEV_DESC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_DEV_DESC_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Description");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_DEV_ID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_DEV_ID_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Unique Device ID");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_DEV_NAME: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_DEV_NAME_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Name");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_DEV_TYPE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_DEV_TYPE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Type");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_DRIVER_VERSION: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_DRIVER_VERSION_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Driver Version");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_FIRST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_HW_CONFIG: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_HW_CONFIG_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Hardware Configuration");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_PNP_ID: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_PNP_ID_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("PnP ID String");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_PORT_NAME: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_PORT_NAME_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Port");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_REMOTE_DEV_ID: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_REMOTE_DEV_ID_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Remote Device ID");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_SERVER_NAME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_SERVER_NAME_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Server");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_STI_DRIVER_VERSION: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_STI_DRIVER_VERSION_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("STI Driver Version");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_STI_GEN_CAPABILITIES: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_STI_GEN_CAPABILITIES_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("STI Generic Capabilities");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_UI_CLSID: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_UI_CLSID_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("UI Class ID");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_VEND_DESC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_VEND_DESC_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Manufacturer");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_WIA_VERSION: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_WIA_VERSION_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("WIA Version");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DONT_SHOW_PREVIEW_CONTROL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DONT_USE_SEGMENTATION_FILTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPA_CONNECT_STATUS: u32 = 1027u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPA_CONNECT_STATUS_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Connect Status");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPA_DEVICE_TIME: u32 = 1028u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPA_DEVICE_TIME_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Device Time");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPA_FIRMWARE_VERSION: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPA_FIRMWARE_VERSION_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Firmware Version");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_ARTIST: u32 = 2091u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_ARTIST_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Artist");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_BATTERY_STATUS: u32 = 2065u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_BATTERY_STATUS_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Battery Status");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_BURST_INTERVAL: u32 = 2075u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_BURST_INTERVAL_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Burst Interval");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_BURST_NUMBER: u32 = 2076u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_BURST_NUMBER_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Burst Number");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_CAPTURE_DELAY: u32 = 2082u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_CAPTURE_DELAY_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Capture Delay");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_CAPTURE_MODE: u32 = 2081u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_CAPTURE_MODE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Capture Mode");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_COMPRESSION_SETTING: u32 = 2071u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_COMPRESSION_SETTING_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Compression Setting");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_CONTRAST: u32 = 2080u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_CONTRAST_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Contrast");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_COPYRIGHT_INFO: u32 = 2092u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_COPYRIGHT_INFO_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Copyright Info");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_DIGITAL_ZOOM: u32 = 2078u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_DIGITAL_ZOOM_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Digital Zoom");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_DIMENSION: u32 = 2070u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_DIMENSION_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Dimension");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_EFFECT_MODE: u32 = 2077u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_EFFECT_MODE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Effect Mode");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_EXPOSURE_COMP: u32 = 2053u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_EXPOSURE_COMP_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Exposure Compensation");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_EXPOSURE_INDEX: u32 = 2083u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_EXPOSURE_INDEX_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Exposure Index");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_EXPOSURE_METERING_MODE: u32 = 2084u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_EXPOSURE_METERING_MODE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Exposure Metering Mode");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_EXPOSURE_MODE: u32 = 2052u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_EXPOSURE_MODE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Exposure Mode");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_EXPOSURE_TIME: u32 = 2054u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_EXPOSURE_TIME_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Exposure Time");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FLASH_MODE: u32 = 2056u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FLASH_MODE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Flash Mode");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FNUMBER: u32 = 2055u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FNUMBER_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("F Number");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FOCAL_LENGTH: u32 = 2087u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FOCAL_LENGTH_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Focus Length");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FOCUS_DISTANCE: u32 = 2086u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FOCUS_DISTANCE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Focus Distance");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FOCUS_MANUAL_DIST: u32 = 2058u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FOCUS_MANUAL_DIST_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Focus Manual Dist");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FOCUS_METERING: u32 = 2072u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FOCUS_METERING_MODE: u32 = 2085u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FOCUS_METERING_MODE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Focus Metering Mode");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FOCUS_METERING_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Focus Metering Mode");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FOCUS_MODE: u32 = 2057u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FOCUS_MODE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Focus Mode");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_PAN_POSITION: u32 = 2060u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_PAN_POSITION_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Pan Position");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_PICTURES_REMAINING: u32 = 2051u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_PICTURES_REMAINING_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Pictures Remaining");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_PICTURES_TAKEN: u32 = 2050u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_PICTURES_TAKEN_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Pictures Taken");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_PICT_HEIGHT: u32 = 2069u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_PICT_HEIGHT_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Picture Height");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_PICT_WIDTH: u32 = 2068u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_PICT_WIDTH_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Picture Width");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_POWER_MODE: u32 = 2064u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_POWER_MODE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Power Mode");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_RGB_GAIN: u32 = 2088u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_RGB_GAIN_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("RGB Gain");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_SHARPNESS: u32 = 2079u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_SHARPNESS_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Sharpness");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_THUMB_HEIGHT: u32 = 2067u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_THUMB_HEIGHT_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Thumbnail Height");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_THUMB_WIDTH: u32 = 2066u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_THUMB_WIDTH_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Thumbnail Width");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_TILT_POSITION: u32 = 2061u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_TILT_POSITION_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Tilt Position");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_TIMELAPSE_INTERVAL: u32 = 2073u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_TIMELAPSE_INTERVAL_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Timelapse Interval");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_TIMELAPSE_NUMBER: u32 = 2074u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_TIMELAPSE_NUMBER_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Timelapse Number");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_TIMER_MODE: u32 = 2062u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_TIMER_MODE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Timer Mode");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_TIMER_VALUE: u32 = 2063u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_TIMER_VALUE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Timer Value");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_UPLOAD_URL: u32 = 2090u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_UPLOAD_URL_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Upload URL");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_WHITE_BALANCE: u32 = 2089u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_WHITE_BALANCE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("White Balance");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_ZOOM_POSITION: u32 = 2059u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_ZOOM_POSITION_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Zoom Position");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPF_FIRST: u32 = 3330u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPF_MOUNT_POINT: u32 = 3330u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPF_MOUNT_POINT_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Directory mount point");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DEVICE_ID: u32 = 3114u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DEVICE_ID_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Device ID");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DITHER_PATTERN_DATA: u32 = 3085u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DITHER_PATTERN_DATA_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Dither Pattern Data");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DITHER_SELECT: u32 = 3084u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DITHER_SELECT_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Dither Select");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DOCUMENT_HANDLING_CAPABILITIES: u32 = 3086u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DOCUMENT_HANDLING_CAPABILITIES_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Document Handling Capabilities");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DOCUMENT_HANDLING_CAPACITY: u32 = 3089u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DOCUMENT_HANDLING_CAPACITY_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Document Handling Capacity");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DOCUMENT_HANDLING_SELECT: u32 = 3088u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DOCUMENT_HANDLING_SELECT_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Document Handling Select");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DOCUMENT_HANDLING_STATUS: u32 = 3087u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DOCUMENT_HANDLING_STATUS_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Document Handling Status");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_ENDORSER_CHARACTERS: u32 = 3092u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_ENDORSER_CHARACTERS_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Endorser Characters");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_ENDORSER_STRING: u32 = 3093u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_ENDORSER_STRING_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Endorser String");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_FILTER_SELECT: u32 = 3083u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_FILTER_SELECT_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Filter Select");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_FIRST: u32 = 3074u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_GLOBAL_IDENTITY: u32 = 3115u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_GLOBAL_IDENTITY_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Global Identity");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_HORIZONTAL_BED_REGISTRATION: u32 = 3079u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_HORIZONTAL_BED_REGISTRATION_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Horizontal Bed Registration");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_HORIZONTAL_BED_SIZE: u32 = 3074u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_HORIZONTAL_BED_SIZE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Horizontal Bed Size");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_HORIZONTAL_SHEET_FEED_SIZE: u32 = 3076u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_HORIZONTAL_SHEET_FEED_SIZE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Horizontal Sheet Feed Size");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_MAX_SCAN_TIME: u32 = 3095u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_MAX_SCAN_TIME_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Max Scan Time");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_MIN_HORIZONTAL_SHEET_FEED_SIZE: u32 = 3104u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_MIN_HORIZONTAL_SHEET_FEED_SIZE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Minimum Horizontal Sheet Feed Size");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_MIN_VERTICAL_SHEET_FEED_SIZE: u32 = 3105u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_MIN_VERTICAL_SHEET_FEED_SIZE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Minimum Vertical Sheet Feed Size");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_OPTICAL_XRES: u32 = 3090u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_OPTICAL_XRES_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Horizontal Optical Resolution");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_OPTICAL_YRES: u32 = 3091u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_OPTICAL_YRES_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Vertical Optical Resolution");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PAD_COLOR: u32 = 3082u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PAD_COLOR_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Pad Color");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PAGES: u32 = 3096u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PAGES_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Pages");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PAGE_HEIGHT: u32 = 3099u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PAGE_HEIGHT_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Page Height");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PAGE_SIZE: u32 = 3097u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PAGE_SIZE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Page Size");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PAGE_WIDTH: u32 = 3098u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PAGE_WIDTH_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Page Width");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PLATEN_COLOR: u32 = 3081u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PLATEN_COLOR_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Platen Color");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PREVIEW: u32 = 3100u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PREVIEW_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Preview");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_SCAN_AHEAD_PAGES: u32 = 3094u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_SCAN_AHEAD_PAGES_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Scan Ahead Pages");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_SCAN_AVAILABLE_ITEM: u32 = 3116u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_SCAN_AVAILABLE_ITEM_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Scan Available Item");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_SERVICE_ID: u32 = 3113u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_SERVICE_ID_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Service ID");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_SHEET_FEEDER_REGISTRATION: u32 = 3078u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_SHEET_FEEDER_REGISTRATION_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Sheet Feeder Registration");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_SHOW_PREVIEW_CONTROL: u32 = 3103u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_SHOW_PREVIEW_CONTROL_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Show preview control");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_TRANSPARENCY: u32 = 3101u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_TRANSPARENCY_CAPABILITIES: u32 = 3106u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_TRANSPARENCY_CAPABILITIES_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Transparency Adapter Capabilities");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_TRANSPARENCY_SELECT: u32 = 3102u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_TRANSPARENCY_SELECT_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Transparency Adapter Select");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_TRANSPARENCY_STATUS: u32 = 3107u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_TRANSPARENCY_STATUS_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Transparency Adapter Status");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_TRANSPARENCY_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Transparency Adapter");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_USER_NAME: u32 = 3112u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_USER_NAME_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("User Name");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_VERTICAL_BED_REGISTRATION: u32 = 3080u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_VERTICAL_BED_REGISTRATION_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Vertical Bed Registration");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_VERTICAL_BED_SIZE: u32 = 3075u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_VERTICAL_BED_SIZE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Vertical Bed Size");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_VERTICAL_SHEET_FEED_SIZE: u32 = 3077u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_VERTICAL_SHEET_FEED_SIZE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Vertical Sheet Feed Size");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPV_DSHOW_DEVICE_PATH: u32 = 3588u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPV_DSHOW_DEVICE_PATH_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Directshow Device Path");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPV_IMAGES_DIRECTORY: u32 = 3587u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPV_IMAGES_DIRECTORY_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Images Directory");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPV_LAST_PICTURE_TAKEN: u32 = 3586u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPV_LAST_PICTURE_TAKEN_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Last Picture Taken");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ENDORSER_TOK_DATE: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("$DATE$");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ENDORSER_TOK_DAY: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("$DAY$");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ENDORSER_TOK_MONTH: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("$MONTH$");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ENDORSER_TOK_PAGE_COUNT: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("$PAGE_COUNT$");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ENDORSER_TOK_TIME: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("$TIME$");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ENDORSER_TOK_YEAR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("$YEAR$");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_BUSY: ::windows_sys::core::HRESULT = -2145320954i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_COVER_OPEN: ::windows_sys::core::HRESULT = -2145320944i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_DESTINATION: ::windows_sys::core::HRESULT = -2145320942i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_DEVICE_COMMUNICATION: ::windows_sys::core::HRESULT = -2145320950i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_DEVICE_LOCKED: ::windows_sys::core::HRESULT = -2145320947i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_EXCEPTION_IN_DRIVER: ::windows_sys::core::HRESULT = -2145320946i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_GENERAL_ERROR: ::windows_sys::core::HRESULT = -2145320959i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_INCORRECT_HARDWARE_SETTING: ::windows_sys::core::HRESULT = -2145320948i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_INVALID_COMMAND: ::windows_sys::core::HRESULT = -2145320949i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_INVALID_DRIVER_RESPONSE: ::windows_sys::core::HRESULT = -2145320945i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_ITEM_DELETED: ::windows_sys::core::HRESULT = -2145320951i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_LAMP_OFF: ::windows_sys::core::HRESULT = -2145320943i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_MAXIMUM_PRINTER_ENDORSER_COUNTER: ::windows_sys::core::HRESULT = -2145320939i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_MULTI_FEED: ::windows_sys::core::HRESULT = -2145320940i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_NETWORK_RESERVATION_FAILED: ::windows_sys::core::HRESULT = -2145320941i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_OFFLINE: ::windows_sys::core::HRESULT = -2145320955i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_PAPER_EMPTY: ::windows_sys::core::HRESULT = -2145320957i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_PAPER_JAM: ::windows_sys::core::HRESULT = -2145320958i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_PAPER_PROBLEM: ::windows_sys::core::HRESULT = -2145320956i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_USER_INTERVENTION: ::windows_sys::core::HRESULT = -2145320952i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_WARMING_UP: ::windows_sys::core::HRESULT = -2145320953i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_CANCEL_IO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc860f7b8_9ccd_41ea_bbbf_4dd09c5b1795);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_COVER_CLOSED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6714a1e6_e285_468c_9b8c_da7dc4cbaa05);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_COVER_OPEN: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x19a12136_fa1c_4f66_900f_8f914ec74ec9);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_DEVICE_CONNECTED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa28bbade_64b6_11d2_a231_00c04fa31809);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_DEVICE_CONNECTED_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Device Connected");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_DEVICE_DISCONNECTED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x143e4e83_6497_11d2_a231_00c04fa31809);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_DEVICE_DISCONNECTED_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Device Disconnected");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_DEVICE_NOT_READY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd8962d7e_e4dc_4b4d_ba29_668a87f42e6f);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_DEVICE_READY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7523ec6c_988b_419e_9a0a_425ac31b37dc);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_FEEDER_EMPTIED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe70b4b82_6dda_46bb_8ff9_53ceb1a03e35);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_FEEDER_LOADED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xcc8d701e_9aba_481d_bf74_78f763dc342a);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_FLATBED_LID_CLOSED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf879af0f_9b29_4283_ad95_d412164d39a9);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_FLATBED_LID_OPEN: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xba0a0623_437d_4f03_a97d_7793b123113c);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_HANDLER_NO_ACTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe0372b7d_e115_4525_bc55_b629e68c745a);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_HANDLER_PROMPT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5f4baad0_4d59_4fcd_b213_783ce7a92f22);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_ITEM_CREATED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4c8f4ef5_e14f_11d2_b326_00c04f68ce61);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_ITEM_DELETED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1d22a559_e14f_11d2_b326_00c04f68ce61);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_POWER_RESUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x618f153e_f686_4350_9634_4115a304830c);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_POWER_SUSPEND: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa0922ff9_c3b4_411c_9e29_03a66993d2be);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_SCAN_EMAIL_IMAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc686dcee_54f2_419e_9a27_2fc7f2e98f9e);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_SCAN_FAX_IMAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc00eb793_8c6e_11d2_977a_0000f87a926f);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_SCAN_FILM_IMAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9b2b662c_6185_438c_b68b_e39ee25e71cb);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_SCAN_IMAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa6c5a715_8c6e_11d2_977a_0000f87a926f);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_SCAN_IMAGE2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfc4767c1_c8b3_48a2_9cfa_2e90cb3d3590);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_SCAN_IMAGE3: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x154e27be_b617_4653_acc5_0fd7bd4c65ce);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_SCAN_IMAGE4: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa65b704a_7f3c_4447_a75d_8a26dfca1fdf);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_SCAN_OCR_IMAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9d095b89_37d6_4877_afed_62a297dc6dbe);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_SCAN_PRINT_IMAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb441f425_8c6e_11d2_977a_0000f87a926f);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_STI_PROXY: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd711f81f_1f0d_422d_8641_927d1b93e5e5);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_STORAGE_CREATED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x353308b2_fe73_46c8_895e_fa4551ccc85a);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_STORAGE_DELETED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5e41e75e_9390_44c5_9a51_e47019e390cf);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_TREE_UPDATED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc9859b91_4ab2_4cd6_a1fc_582eec55e585);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_VOLUME_INSERT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9638bbfd_d1bd_11d2_b31f_00c04f68ce61);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_FEEDER_CONTROL_AUTO: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_FEEDER_CONTROL_MANUAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_FILM_BW_NEGATIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_FILM_COLOR_NEGATIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_FILM_COLOR_SLIDE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_FINAL_SCAN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_FLAG_NOM: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_FLAG_NUM_ELEMS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_FLAG_VALUES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IMAGEPROC_FILTER_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("ImageProcessingFilter");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_INTENT_BEST_PREVIEW: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_INTENT_IMAGE_TYPE_COLOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_INTENT_IMAGE_TYPE_GRAYSCALE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_INTENT_IMAGE_TYPE_MASK: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_INTENT_IMAGE_TYPE_TEXT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_INTENT_MAXIMIZE_QUALITY: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_INTENT_MINIMIZE_SIZE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_INTENT_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_INTENT_SIZE_MASK: u32 = 983040u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ACCESS_RIGHTS: u32 = 4102u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ACCESS_RIGHTS_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Access Rights");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_APP_COLOR_MAPPING: u32 = 4121u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_APP_COLOR_MAPPING_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Application Applies Color Mapping");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_BITS_PER_CHANNEL: u32 = 4110u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_BITS_PER_CHANNEL_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Bits Per Channel");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_BUFFER_SIZE: u32 = 4118u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_BUFFER_SIZE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Buffer Size");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_BYTES_PER_LINE: u32 = 4113u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_BYTES_PER_LINE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Bytes Per Line");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_CHANNELS_PER_PIXEL: u32 = 4109u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_CHANNELS_PER_PIXEL_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Channels Per Pixel");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_COLOR_PROFILE: u32 = 4117u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_COLOR_PROFILE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Color Profiles");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_COMPRESSION: u32 = 4107u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_COMPRESSION_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Compression");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_DATATYPE: u32 = 4103u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_DATATYPE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Data Type");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_DEPTH: u32 = 4104u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_DEPTH_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Bits Per Pixel");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_FILENAME_EXTENSION: u32 = 4123u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_FILENAME_EXTENSION_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Filename extension");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_FIRST: u32 = 4098u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_FORMAT: u32 = 4106u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_FORMAT_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Format");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_FULL_ITEM_NAME: u32 = 4099u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_FULL_ITEM_NAME_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Full Item Name");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_GAMMA_CURVES: u32 = 4115u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_GAMMA_CURVES_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Gamma Curves");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ICM_PROFILE_NAME: u32 = 4120u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ICM_PROFILE_NAME_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Color Profile Name");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ITEMS_STORED: u32 = 4127u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ITEMS_STORED_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Items Stored");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ITEM_CATEGORY: u32 = 4125u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ITEM_CATEGORY_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Item Category");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ITEM_FLAGS: u32 = 4101u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ITEM_FLAGS_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Item Flags");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ITEM_NAME: u32 = 4098u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ITEM_NAME_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Item Name");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ITEM_SIZE: u32 = 4116u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ITEM_SIZE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Item Size");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ITEM_TIME: u32 = 4100u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ITEM_TIME_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Item Time Stamp");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_MIN_BUFFER_SIZE: u32 = 4118u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_MIN_BUFFER_SIZE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Buffer Size");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_NUMBER_OF_LINES: u32 = 4114u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_NUMBER_OF_LINES_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Number of Lines");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_PIXELS_PER_LINE: u32 = 4112u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_PIXELS_PER_LINE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Pixels Per Line");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_PLANAR: u32 = 4111u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_PLANAR_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Planar");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_PREFERRED_FORMAT: u32 = 4105u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_PREFERRED_FORMAT_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Preferred Format");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_PROP_STREAM_COMPAT_ID: u32 = 4122u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_PROP_STREAM_COMPAT_ID_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Stream Compatibility ID");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_RAW_BITS_PER_CHANNEL: u32 = 4128u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_RAW_BITS_PER_CHANNEL_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Raw Bits Per Channel");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_REGION_TYPE: u32 = 4119u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_REGION_TYPE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Region Type");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_SUPPRESS_PROPERTY_PAGE: u32 = 4124u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_SUPPRESS_PROPERTY_PAGE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Suppress a property page");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_TYMED: u32 = 4108u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_TYMED_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Media Type");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_UPLOAD_ITEM_SIZE: u32 = 4126u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_UPLOAD_ITEM_SIZE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Upload Item Size");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_AUDIO_AVAILABLE: u32 = 5125u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_AUDIO_AVAILABLE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Audio Available");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_AUDIO_DATA: u32 = 5127u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_AUDIO_DATA_FORMAT: u32 = 5126u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_AUDIO_DATA_FORMAT_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Audio Format");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_AUDIO_DATA_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Audio Data");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_FIRST: u32 = 5122u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_NUM_PICT_PER_ROW: u32 = 5128u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_NUM_PICT_PER_ROW_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Pictures per Row");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_SEQUENCE: u32 = 5129u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_SEQUENCE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Sequence Number");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_THUMBNAIL: u32 = 5122u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_THUMBNAIL_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Thumbnail Data");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_THUMB_HEIGHT: u32 = 5124u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_THUMB_HEIGHT_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Thumbnail Height");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_THUMB_WIDTH: u32 = 5123u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_THUMB_WIDTH_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Thumbnail Width");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_TIMEDELAY: u32 = 5130u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_TIMEDELAY_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Time Delay");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_ALARM: u32 = 4185u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_ALARM_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Alarm");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_AUTO_CROP: u32 = 4170u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_AUTO_CROP_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Auto-Crop");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_AUTO_DESKEW: u32 = 3107u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_AUTO_DESKEW_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Automatic Deskew");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_BARCODE_READER: u32 = 4150u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_BARCODE_READER_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Barcode Reader");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_BARCODE_SEARCH_DIRECTION: u32 = 4152u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_BARCODE_SEARCH_DIRECTION_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Barcode Search Direction");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_BARCODE_SEARCH_TIMEOUT: u32 = 4154u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_BARCODE_SEARCH_TIMEOUT_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Barcode Search Timeout");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_BLANK_PAGES: u32 = 4167u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_BLANK_PAGES_SENSITIVITY: u32 = 4192u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_BLANK_PAGES_SENSITIVITY_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Blank Pages Sensitivity");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_BLANK_PAGES_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Blank Pages");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_BRIGHTNESS: u32 = 6154u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_BRIGHTNESS_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Brightness");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_COLOR_DROP: u32 = 4176u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_COLOR_DROP_BLUE: u32 = 4179u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_COLOR_DROP_BLUE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Color Drop Blue");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_COLOR_DROP_GREEN: u32 = 4178u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_COLOR_DROP_GREEN_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Color Drop Green");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_COLOR_DROP_MULTI: u32 = 4191u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_COLOR_DROP_MULTI_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Color Drop Multiple");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_COLOR_DROP_RED: u32 = 4177u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_COLOR_DROP_RED_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Color Drop Red");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_COLOR_DROP_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Color Drop");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_CONTRAST: u32 = 6155u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_CONTRAST_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Contrast");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_CUR_INTENT: u32 = 6146u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_CUR_INTENT_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Current Intent");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_DESKEW_X: u32 = 6162u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_DESKEW_X_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("DeskewX");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_DESKEW_Y: u32 = 6163u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_DESKEW_Y_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("DeskewY");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_DOCUMENT_HANDLING_SELECT: u32 = 3088u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_DOCUMENT_HANDLING_SELECT_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Document Handling Select");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_ENABLED_BARCODE_TYPES: u32 = 4156u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_ENABLED_BARCODE_TYPES_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Enabled Barcode Types");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_ENABLED_PATCH_CODE_TYPES: u32 = 4163u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_ENABLED_PATCH_CODE_TYPES_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Enabled Path Code Types");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_FEEDER_CONTROL: u32 = 4182u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_FEEDER_CONTROL_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Feeder Control");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_FILM_NODE_NAME: u32 = 4129u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_FILM_NODE_NAME_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Film Node Name");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_FILM_SCAN_MODE: u32 = 3104u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_FILM_SCAN_MODE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Film Scan Mode");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_FIRST: u32 = 6146u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_INVERT: u32 = 6160u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_INVERT_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Invert");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_JOB_SEPARATORS: u32 = 4165u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_JOB_SEPARATORS_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Job Separators");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_LAMP: u32 = 3105u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_LAMP_AUTO_OFF: u32 = 3106u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_LAMP_AUTO_OFF_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Lamp Auto Off");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_LAMP_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Lamp");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_LONG_DOCUMENT: u32 = 4166u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_LONG_DOCUMENT_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Long Document");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MAXIMUM_BARCODES_PER_PAGE: u32 = 4151u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MAXIMUM_BARCODES_PER_PAGE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Maximum Barcodes Per Page");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MAXIMUM_BARCODE_SEARCH_RETRIES: u32 = 4153u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MAXIMUM_BARCODE_SEARCH_RETRIES_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Barcode Search Retries");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MAX_HORIZONTAL_SIZE: u32 = 6165u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MAX_HORIZONTAL_SIZE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Maximum Horizontal Scan Size");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MAX_VERTICAL_SIZE: u32 = 6166u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MAX_VERTICAL_SIZE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Maximum Vertical Scan Size");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MICR_READER: u32 = 4164u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MICR_READER_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("MICR Reader");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MIN_HORIZONTAL_SIZE: u32 = 6167u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MIN_HORIZONTAL_SIZE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Minimum Horizontal Scan Size");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MIN_VERTICAL_SIZE: u32 = 6168u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MIN_VERTICAL_SIZE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Minimum Vertical Scan Size");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MIRROR: u32 = 6158u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MIRROR_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Mirror");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MULTI_FEED: u32 = 4168u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MULTI_FEED_DETECT_METHOD: u32 = 4193u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MULTI_FEED_DETECT_METHOD_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Multi-Feed Detection Method");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MULTI_FEED_SENSITIVITY: u32 = 4169u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MULTI_FEED_SENSITIVITY_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Multi-Feed Sensitivity");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MULTI_FEED_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Multi-Feed");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OPTICAL_XRES: u32 = 3090u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OPTICAL_XRES_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Horizontal Optical Resolution");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OPTICAL_YRES: u32 = 3091u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OPTICAL_YRES_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Vertical Optical Resolution");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_ORIENTATION: u32 = 6156u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_ORIENTATION_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Orientation");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OVER_SCAN: u32 = 4171u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OVER_SCAN_BOTTOM: u32 = 4175u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OVER_SCAN_BOTTOM_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Overscan Bottom");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OVER_SCAN_LEFT: u32 = 4172u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OVER_SCAN_LEFT_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Overscan Left");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OVER_SCAN_RIGHT: u32 = 4173u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OVER_SCAN_RIGHT_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Overscan Right");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OVER_SCAN_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Overscan");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OVER_SCAN_TOP: u32 = 4174u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OVER_SCAN_TOP_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Overscan Top");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PAGES: u32 = 3096u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PAGES_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Pages");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PAGE_HEIGHT: u32 = 3099u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PAGE_HEIGHT_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Page Height");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PAGE_SIZE: u32 = 3097u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PAGE_SIZE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Page Size");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PAGE_WIDTH: u32 = 3098u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PAGE_WIDTH_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Page Width");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PATCH_CODE_READER: u32 = 4157u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PATCH_CODE_READER_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Patch Code Reader");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PHOTOMETRIC_INTERP: u32 = 6153u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PHOTOMETRIC_INTERP_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Photometric Interpretation");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PREVIEW: u32 = 3100u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PREVIEW_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Preview");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PREVIEW_TYPE: u32 = 3111u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PREVIEW_TYPE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Preview Type");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER: u32 = 4130u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_CHARACTER_ROTATION: u32 = 4187u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_CHARACTER_ROTATION_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser Character Rotation");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_COUNTER: u32 = 4132u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_COUNTER_DIGITS: u32 = 4190u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_COUNTER_DIGITS_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser Counter Digits");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_COUNTER_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser Counter");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_FONT_TYPE: u32 = 4184u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_FONT_TYPE_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser Font Type");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS: u32 = 4142u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_DOWNLOAD: u32 = 4149u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_DOWNLOAD_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser Graphics Download");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MAX_HEIGHT: u32 = 4147u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MAX_HEIGHT_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser Graphics Maximum Height");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MAX_WIDTH: u32 = 4145u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MAX_WIDTH_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser Graphics Maximum Width");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MIN_HEIGHT: u32 = 4146u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MIN_HEIGHT_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser Graphics Minimum Height");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MIN_WIDTH: u32 = 4144u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MIN_WIDTH_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser Graphics Minimum Width");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_POSITION: u32 = 4143u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_POSITION_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser Graphics Position");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser Graphics");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_UPLOAD: u32 = 4148u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_UPLOAD_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser Graphics Upload");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_INK: u32 = 4186u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_INK_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser Ink");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_MAX_CHARACTERS: u32 = 4188u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_MAX_CHARACTERS_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser Maximum Characters");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_MAX_GRAPHICS: u32 = 4189u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_MAX_GRAPHICS_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser Maximum Graphics");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_NUM_LINES: u32 = 4136u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_NUM_LINES_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser Lines");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_ORDER: u32 = 4131u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_ORDER_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser Order");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_PADDING: u32 = 4183u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_PADDING_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser Padding");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_STEP: u32 = 4133u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_STEP_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser Step");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_STRING: u32 = 4137u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_STRING_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser String");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_TEXT_DOWNLOAD: u32 = 4141u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_TEXT_DOWNLOAD_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser Text Download");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_TEXT_UPLOAD: u32 = 4140u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_TEXT_UPLOAD_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser Text Upload");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_VALID_CHARACTERS: u32 = 4138u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_VALID_CHARACTERS_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser Valid Characters");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_VALID_FORMAT_SPECIFIERS: u32 = 4139u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_VALID_FORMAT_SPECIFIERS_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser Valid Format Specifiers");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_XOFFSET: u32 = 4134u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_XOFFSET_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser Horizontal Offset");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_YOFFSET: u32 = 4135u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_YOFFSET_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Printer/Endorser Vertical Offset");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_ROTATION: u32 = 6157u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_ROTATION_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Rotation");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SCAN_AHEAD: u32 = 4180u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SCAN_AHEAD_CAPACITY: u32 = 4181u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SCAN_AHEAD_CAPACITY_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Scan Ahead Capacity");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SCAN_AHEAD_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Scan Ahead");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SEGMENTATION: u32 = 6164u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SEGMENTATION_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Segmentation");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SHEET_FEEDER_REGISTRATION: u32 = 3078u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SHEET_FEEDER_REGISTRATION_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Sheet Feeder Registration");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SHOW_PREVIEW_CONTROL: u32 = 3103u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SHOW_PREVIEW_CONTROL_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Show preview control");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SUPPORTED_BARCODE_TYPES: u32 = 4155u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SUPPORTED_BARCODE_TYPES_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Supported Barcode Types");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SUPPORTED_PATCH_CODE_TYPES: u32 = 4162u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SUPPORTED_PATCH_CODE_TYPES_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Supported Patch Code Types");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SUPPORTS_CHILD_ITEM_CREATION: u32 = 3108u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SUPPORTS_CHILD_ITEM_CREATION_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Supports Child Item Creation");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_THRESHOLD: u32 = 6159u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_THRESHOLD_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Threshold");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_TRANSFER_CAPABILITIES: u32 = 6169u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_TRANSFER_CAPABILITIES_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Transfer Capabilities");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_WARM_UP_TIME: u32 = 6161u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_WARM_UP_TIME_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Lamp Warm up Time");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_XEXTENT: u32 = 6151u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_XEXTENT_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Horizontal Extent");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_XPOS: u32 = 6149u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_XPOS_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Horizontal Start Position");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_XRES: u32 = 6147u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_XRES_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Horizontal Resolution");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_XSCALING: u32 = 3109u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_XSCALING_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Horizontal Scaling");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_YEXTENT: u32 = 6152u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_YEXTENT_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Vertical Extent");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_YPOS: u32 = 6150u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_YPOS_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Vertical Start Position");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_YRES: u32 = 6148u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_YRES_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Vertical Resolution");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_YSCALING: u32 = 3110u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_YSCALING_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Vertical Scaling");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IS_DEFAULT_HANDLER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ITEM_CAN_BE_DELETED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ITEM_READ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ITEM_WRITE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_LAMP_OFF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_LAMP_ON: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_LINE_ORDER_BOTTOM_TO_TOP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_LINE_ORDER_TOP_TO_BOTTOM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_LIST_COUNT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_LIST_NOM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_LIST_NUM_ELEMS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_LIST_VALUES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_LONG_DOCUMENT_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_LONG_DOCUMENT_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_LONG_DOCUMENT_SPLIT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MAJOR_EVENT_DEVICE_CONNECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MAJOR_EVENT_DEVICE_DISCONNECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MAJOR_EVENT_PICTURE_DELETED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MAJOR_EVENT_PICTURE_TAKEN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MAX_CTX_SIZE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MICR_READER_AUTO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MICR_READER_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MICR_READER_FEEDER_BACK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MICR_READER_FEEDER_DUPLEX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MICR_READER_FEEDER_FRONT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MICR_READER_FLATBED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MULTI_FEED_DETECT_CONTINUE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MULTI_FEED_DETECT_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MULTI_FEED_DETECT_METHOD_LENGTH: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MULTI_FEED_DETECT_METHOD_OVERLAP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MULTI_FEED_DETECT_STOP_ERROR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MULTI_FEED_DETECT_STOP_SUCCESS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_NOTIFICATION_EVENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_NUM_DIP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_NUM_IPC: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ORDER_BGR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ORDER_RGB: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_OVER_SCAN_ALL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_OVER_SCAN_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_OVER_SCAN_LEFT_RIGHT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_OVER_SCAN_TOP_BOTTOM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PACKED_PIXEL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_A4: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_AUTO: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_BUSINESSCARD: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_CUSTOM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_CUSTOM_BASE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_DIN_2B: u32 = 52u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_DIN_4B: u32 = 53u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_A0: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_A1: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_A10: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_A2: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_A3: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_A4: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_A5: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_A6: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_A7: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_A8: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_A9: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_B0: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_B1: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_B10: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_B2: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_B3: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_B4: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_B5: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_B6: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_B7: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_B8: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_B9: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_C0: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_C1: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_C10: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_C2: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_C3: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_C4: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_C5: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_C6: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_C7: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_C8: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_C9: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_JIS_2A: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_JIS_4A: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_JIS_B0: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_JIS_B1: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_JIS_B10: u32 = 49u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_JIS_B2: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_JIS_B3: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_JIS_B4: u32 = 43u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_JIS_B5: u32 = 44u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_JIS_B6: u32 = 45u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_JIS_B7: u32 = 46u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_JIS_B8: u32 = 47u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_JIS_B9: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_LETTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_USLEDGER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_USLEGAL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_USLETTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_USSTATEMENT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_10: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_11: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_12: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_13: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_14: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_4: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_6: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_7: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_8: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_9: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_CUSTOM_BASE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_READER_AUTO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_READER_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_READER_FEEDER_BACK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_READER_FEEDER_DUPLEX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_READER_FEEDER_FRONT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_READER_FLATBED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_T: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PHOTO_WHITE_0: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PHOTO_WHITE_1: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PLANAR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PREVIEW_SCAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_AFTER_SCAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_AUTO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_BEFORE_SCAN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_DIGITAL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_FEEDER_BACK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_FEEDER_DUPLEX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_FEEDER_FRONT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_FLATBED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_GRAPHICS_BACKGROUND: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_GRAPHICS_BOTTOM: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_GRAPHICS_BOTTOM_LEFT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_GRAPHICS_BOTTOM_RIGHT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_GRAPHICS_DEVICE_DEFAULT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_GRAPHICS_LEFT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_GRAPHICS_RIGHT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_GRAPHICS_TOP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_GRAPHICS_TOP_LEFT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_GRAPHICS_TOP_RIGHT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_AM_PM: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_DATE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_DAY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_BOLD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_EXTRA_BOLD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_ITALIC: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_ITALIC_BOLD: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_ITALIC_EXTRA_BOLD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_LARGE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_LARGE_BOLD: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_LARGE_EXTRA_BOLD: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_LARGE_ITALIC: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_LARGE_ITALIC_BOLD: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_LARGE_ITALIC_EXTRA_BOLD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_SMALL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_SMALL_BOLD: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_SMALL_EXTRA_BOLD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_SMALL_ITALIC: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_SMALL_ITALIC_BOLD: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_SMALL_ITALIC_EXTRA_BOLD: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_HOUR_12H: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_HOUR_24H: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_IMAGE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_MILLISECOND: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_MINUTE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_MONTH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_MONTH_NAME: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_MONTH_SHORT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_PADDING_BLANK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_PADDING_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_PADDING_ZERO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_PAGE_COUNT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_SECOND: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_TIME_12H: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_TIME_24H: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_WEEK_DAY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_WEEK_DAY_SHORT: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_YEAR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRIVATE_DEVPROP: u32 = 38914u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRIVATE_ITEMPROP: u32 = 71682u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PROPPAGE_CAMERA_ITEM_GENERAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PROPPAGE_DEVICE_GENERAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PROPPAGE_SCANNER_ITEM_GENERAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PROP_CACHEABLE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PROP_FLAG: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PROP_LIST: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PROP_NONE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PROP_RANGE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PROP_READ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PROP_SYNC_REQUIRED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PROP_WRITE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_RANGE_MAX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_RANGE_MIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_RANGE_NOM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_RANGE_NUM_ELEMS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_RANGE_STEP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_REGISTER_EVENT_CALLBACK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_RESERVED_FOR_NEW_PROPS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_SCAN_AHEAD_ALL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_SCAN_AHEAD_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_SCAN_AHEAD_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_SEGMENTATION_FILTER_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("SegmentationFilter");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_SELECT_DEVICE_NODEFAULT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_SEPARATOR_DETECT_NOSCAN_CONTINUE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_SEPARATOR_DETECT_NOSCAN_STOP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_SEPARATOR_DETECT_SCAN_CONTINUE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_SEPARATOR_DETECT_SCAN_STOP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_SEPARATOR_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_SET_DEFAULT_HANDLER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_SHOW_PREVIEW_CONTROL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_STATUS_CALIBRATING: ::windows_sys::core::HRESULT = 2162691i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_STATUS_CLEAR: ::windows_sys::core::HRESULT = 2162696i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_STATUS_END_OF_MEDIA: ::windows_sys::core::HRESULT = 2162689i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_STATUS_NETWORK_DEVICE_RESERVED: ::windows_sys::core::HRESULT = 2162695i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_STATUS_NOT_HANDLED: ::windows_sys::core::HRESULT = 2162698i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_STATUS_RESERVING_NETWORK_DEVICE: ::windows_sys::core::HRESULT = 2162694i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_STATUS_SKIP_ITEM: ::windows_sys::core::HRESULT = 2162697i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_STATUS_WARMING_UP: ::windows_sys::core::HRESULT = 2162690i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_S_CHANGE_DEVICE: ::windows_sys::core::HRESULT = 2162699i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_S_NO_DEVICE_AVAILABLE: ::windows_sys::core::HRESULT = -2145320939i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_TRANSFER_ACQUIRE_CHILDREN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_TRANSFER_CHILDREN_SINGLE_SCAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_TRANSFER_MSG_DEVICE_STATUS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_TRANSFER_MSG_END_OF_STREAM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_TRANSFER_MSG_END_OF_TRANSFER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_TRANSFER_MSG_NEW_PAGE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_TRANSFER_MSG_STATUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_UNREGISTER_EVENT_CALLBACK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_USE_SEGMENTATION_FILTER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_FRIENDLY_NAME: u32 = 38920u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_FRIENDLY_NAME_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Friendly name");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_MANUFACTURER: u32 = 38914u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_MANUFACTURER_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Device manufacturer");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_MANUFACTURER_URL: u32 = 38915u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_MANUFACTURER_URL_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Manufacurer URL");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_MODEL_NAME: u32 = 38916u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_MODEL_NAME_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Model name");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_MODEL_NUMBER: u32 = 38917u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_MODEL_NUMBER_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Model number");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_MODEL_URL: u32 = 38918u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_MODEL_URL_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Model URL");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_PRESENTATION_URL: u32 = 38919u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_PRESENTATION_URL_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Presentation URL");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_SCAN_AVAILABLE_ITEM: u32 = 38922u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_SCAN_AVAILABLE_ITEM_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Scan Available Item");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_SERIAL_NUMBER: u32 = 38921u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_SERIAL_NUMBER_STR: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Serial number");
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaAudFmt_AIFF: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x66e2bf4f_b6fc_443f_94c8_2f33c8a65aaf);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaAudFmt_MP3: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0fbc71fb_43bf_49f2_9190_e6fecff37e54);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaAudFmt_WAV: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf818e146_07af_40ff_ae55_be8f2c065dbe);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaAudFmt_WMA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd61d6413_8bc2_438f_93ad_21bd484db6a1);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaDevMgr: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa1f4e726_8cf1_11d1_bf92_0060081ed811);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaDevMgr2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb6c292bc_7c88_41ee_8b54_8ec92617e599);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_ASF: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8d948ee9_d0aa_4a12_9d9a_9cc5de36199b);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_AVI: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x32f8ca14_087c_4908_b7c4_6757fe7e90ab);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_BMP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb96b3cab_0728_11d3_9d7b_0000f81ef32e);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_CIFF: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9821a8ab_3a7e_4215_94e0_d27a460c03b2);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_CSV: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x355bda24_5a9f_4494_80dc_be752cecbc8c);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_DPOF: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x369eeeab_a0e8_45ca_86a6_a83ce5697e28);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_EMF: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb96b3cac_0728_11d3_9d7b_0000f81ef32e);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_EXEC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x485da097_141e_4aa5_bb3b_a5618d95d02b);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_EXIF: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb96b3cb2_0728_11d3_9d7b_0000f81ef32e);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_FLASHPIX: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb96b3cb4_0728_11d3_9d7b_0000f81ef32e);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_GIF: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb96b3cb0_0728_11d3_9d7b_0000f81ef32e);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_HTML: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xc99a4e62_99de_4a94_acca_71956ac2977d);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_ICO: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb96b3cb5_0728_11d3_9d7b_0000f81ef32e);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_JBIG: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x41e8dd92_2f0a_43d4_8636_f1614ba11e46);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_JBIG2: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbb8e7e67_283c_4235_9e59_0b9bf94ca687);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_JPEG: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb96b3cae_0728_11d3_9d7b_0000f81ef32e);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_JPEG2K: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x344ee2b2_39db_4dde_8173_c4b75f8f1e49);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_JPEG2KX: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x43e14614_c80a_4850_baf3_4b152dc8da27);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_MEMORYBMP: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb96b3caa_0728_11d3_9d7b_0000f81ef32e);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_MPG: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xecd757e4_d2ec_4f57_955d_bcf8a97c4e52);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_OXPS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2c7b1240_c14d_4109_9755_04b89025153a);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_PDFA: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9980bd5b_3463_43c7_bdca_3caa146f229f);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_PHOTOCD: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb96b3cb3_0728_11d3_9d7b_0000f81ef32e);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_PICT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa6bc85d8_6b3e_40ee_a95c_25d482e41adc);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_PNG: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb96b3caf_0728_11d3_9d7b_0000f81ef32e);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_RAW: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6f120719_f1a8_4e07_9ade_9b64c63a3dcc);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_RAWBAR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xda63f833_d26e_451e_90d2_ea55a1365d62);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_RAWMIC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x22c4f058_0d88_409c_ac1c_eec12b0ea680);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_RAWPAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7760507c_5064_400c_9a17_575624d8824b);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_RAWRGB: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xbca48b55_f272_4371_b0f1_4a150d057bb4);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_RTF: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x573dd6a3_4834_432d_a9b5_e198dd9e890d);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_SCRIPT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfe7d6c53_2dac_446a_b0bd_d73e21e924c9);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_TIFF: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb96b3cb1_0728_11d3_9d7b_0000f81ef32e);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_TXT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xfafd4d82_723f_421f_9318_30501ac44b59);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_UNDEFINED: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb96b3ca9_0728_11d3_9d7b_0000f81ef32e);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_UNICODE16: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x1b7639b6_6357_47d1_9a07_12452dc073e9);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_WMF: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb96b3cad_0728_11d3_9d7b_0000f81ef32e);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_XML: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xb9171457_dac8_4884_b393_15b471d5f07e);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_XMLBAR: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x6235701c_3a98_484c_b2a8_fdffd87e6b16);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_XMLMIC: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2d164c61_b9ae_4b23_8973_c7067e1fbd31);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_XMLPAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf8986f55_f052_460d_9523_3a7dfedbb33c);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaImgFmt_XPS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x700b4a0f_2011_411c_b430_d1e0b2e10b28);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeAnalyze: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeAudio: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeBurst: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeDeleted: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeDevice: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeDisconnected: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeDocument: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeFile: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeFolder: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeFree: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeGenerated: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeHPanorama: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeHasAttachments: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeImage: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeMask: u32 = 2148532223u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeProgrammableDataSource: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeRemoved: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeRoot: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeStorage: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeTransfer: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeTwainCapabilityPassThrough: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeVPanorama: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeVideo: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaLog: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa1e75357_881a_419e_83e2_bb16db197c68);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaVideo: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3908c3cd_4478_4536_af2f_10c25d4ef89a);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const g_dwDebugFlags: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub type WIAVIDEO_STATE = i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIAVIDEO_NO_VIDEO: WIAVIDEO_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIAVIDEO_CREATING_VIDEO: WIAVIDEO_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIAVIDEO_VIDEO_CREATED: WIAVIDEO_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIAVIDEO_VIDEO_PLAYING: WIAVIDEO_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIAVIDEO_VIDEO_PAUSED: WIAVIDEO_STATE = 5i32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIAVIDEO_DESTROYING_VIDEO: WIAVIDEO_STATE = 6i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVICEDIALOGDATA {
    pub cbSize: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub pIWiaItemRoot: IWiaItem,
    pub dwFlags: u32,
    pub lIntent: i32,
    pub lItemCount: i32,
    pub ppWiaItems: *mut IWiaItem,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVICEDIALOGDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVICEDIALOGDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVICEDIALOGDATA2 {
    pub cbSize: u32,
    pub pIWiaItemRoot: IWiaItem2,
    pub dwFlags: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub bstrFolderName: ::windows_sys::core::BSTR,
    pub bstrFilename: ::windows_sys::core::BSTR,
    pub lNumFiles: i32,
    pub pbstrFilePaths: *mut ::windows_sys::core::BSTR,
    pub pWiaItem: IWiaItem2,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVICEDIALOGDATA2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVICEDIALOGDATA2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MINIDRV_TRANSFER_CONTEXT {
    pub lSize: i32,
    pub lWidthInPixels: i32,
    pub lLines: i32,
    pub lDepth: i32,
    pub lXRes: i32,
    pub lYRes: i32,
    pub lCompression: i32,
    pub guidFormatID: ::windows_sys::core::GUID,
    pub tymed: i32,
    pub hFile: isize,
    pub cbOffset: i32,
    pub lBufferSize: i32,
    pub lActiveBuffer: i32,
    pub lNumBuffers: i32,
    pub pBaseBuffer: *mut u8,
    pub pTransferBuffer: *mut u8,
    pub bTransferDataCB: super::super::Foundation::BOOL,
    pub bClassDrvAllocBuf: super::super::Foundation::BOOL,
    pub lClientAddress: isize,
    pub pIWiaMiniDrvCallBack: IWiaMiniDrvCallBack,
    pub lImageSize: i32,
    pub lHeaderSize: i32,
    pub lItemSize: i32,
    pub cbWidthInBytes: i32,
    pub lPage: i32,
    pub lCurIfdOffset: i32,
    pub lPrevIfdOffset: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MINIDRV_TRANSFER_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MINIDRV_TRANSFER_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct RANGEVALUE {
    pub lMin: i32,
    pub lMax: i32,
    pub lStep: i32,
}
impl ::core::marker::Copy for RANGEVALUE {}
impl ::core::clone::Clone for RANGEVALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SCANINFO {
    pub ADF: i32,
    pub TPA: i32,
    pub Endorser: i32,
    pub OpticalXResolution: i32,
    pub OpticalYResolution: i32,
    pub BedWidth: i32,
    pub BedHeight: i32,
    pub IntensityRange: RANGEVALUE,
    pub ContrastRange: RANGEVALUE,
    pub SupportedCompressionType: i32,
    pub SupportedDataTypes: i32,
    pub WidthPixels: i32,
    pub WidthBytes: i32,
    pub Lines: i32,
    pub DataType: i32,
    pub PixelBits: i32,
    pub Intensity: i32,
    pub Contrast: i32,
    pub Xresolution: i32,
    pub Yresolution: i32,
    pub Window: SCANWINDOW,
    pub DitherPattern: i32,
    pub Negative: i32,
    pub Mirror: i32,
    pub AutoBack: i32,
    pub ColorDitherPattern: i32,
    pub ToneMap: i32,
    pub Compression: i32,
    pub RawDataFormat: i32,
    pub RawPixelOrder: i32,
    pub bNeedDataAlignment: i32,
    pub DelayBetweenRead: i32,
    pub MaxBufferSize: i32,
    pub DeviceIOHandles: [super::super::Foundation::HANDLE; 16],
    pub lReserved: [i32; 4],
    pub pMicroDriverContext: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SCANINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SCANINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct SCANWINDOW {
    pub xPos: i32,
    pub yPos: i32,
    pub xExtent: i32,
    pub yExtent: i32,
}
impl ::core::marker::Copy for SCANWINDOW {}
impl ::core::clone::Clone for SCANWINDOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct TWAIN_CAPABILITY {
    pub lSize: i32,
    pub lMSG: i32,
    pub lCapID: i32,
    pub lConType: i32,
    pub lRC: i32,
    pub lCC: i32,
    pub lDataSize: i32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for TWAIN_CAPABILITY {}
impl ::core::clone::Clone for TWAIN_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VAL {
    pub lVal: i32,
    pub dblVal: f64,
    pub pGuid: *mut ::windows_sys::core::GUID,
    pub pScanInfo: *mut SCANINFO,
    pub handle: super::super::Foundation::HGLOBAL,
    pub ppButtonNames: *mut *mut u16,
    pub pHandle: *mut super::super::Foundation::HANDLE,
    pub lReserved: i32,
    pub szVal: [u8; 255],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VAL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIAS_CHANGED_VALUE_INFO {
    pub bChanged: super::super::Foundation::BOOL,
    pub vt: i32,
    pub Old: WIAS_CHANGED_VALUE_INFO_1,
    pub Current: WIAS_CHANGED_VALUE_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIAS_CHANGED_VALUE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIAS_CHANGED_VALUE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WIAS_CHANGED_VALUE_INFO_0 {
    pub lVal: i32,
    pub fltVal: f32,
    pub bstrVal: ::windows_sys::core::BSTR,
    pub guidVal: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIAS_CHANGED_VALUE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIAS_CHANGED_VALUE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WIAS_CHANGED_VALUE_INFO_1 {
    pub lVal: i32,
    pub fltVal: f32,
    pub bstrVal: ::windows_sys::core::BSTR,
    pub guidVal: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIAS_CHANGED_VALUE_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIAS_CHANGED_VALUE_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIAS_DOWN_SAMPLE_INFO {
    pub ulOriginalWidth: u32,
    pub ulOriginalHeight: u32,
    pub ulBitsPerPixel: u32,
    pub ulXRes: u32,
    pub ulYRes: u32,
    pub ulDownSampledWidth: u32,
    pub ulDownSampledHeight: u32,
    pub ulActualSize: u32,
    pub ulDestBufSize: u32,
    pub ulSrcBufSize: u32,
    pub pSrcBuffer: *mut u8,
    pub pDestBuffer: *mut u8,
}
impl ::core::marker::Copy for WIAS_DOWN_SAMPLE_INFO {}
impl ::core::clone::Clone for WIAS_DOWN_SAMPLE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIAS_ENDORSER_INFO {
    pub ulPageCount: u32,
    pub ulNumEndorserValues: u32,
    pub pEndorserValues: *mut WIAS_ENDORSER_VALUE,
}
impl ::core::marker::Copy for WIAS_ENDORSER_INFO {}
impl ::core::clone::Clone for WIAS_ENDORSER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIAS_ENDORSER_VALUE {
    pub wszTokenName: ::windows_sys::core::PWSTR,
    pub wszValue: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for WIAS_ENDORSER_VALUE {}
impl ::core::clone::Clone for WIAS_ENDORSER_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIA_BARCODES {
    pub Tag: u32,
    pub Version: u32,
    pub Size: u32,
    pub Count: u32,
    pub Barcodes: [WIA_BARCODE_INFO; 1],
}
impl ::core::marker::Copy for WIA_BARCODES {}
impl ::core::clone::Clone for WIA_BARCODES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIA_BARCODE_INFO {
    pub Size: u32,
    pub Type: u32,
    pub Page: u32,
    pub Confidence: u32,
    pub XOffset: u32,
    pub YOffset: u32,
    pub Rotation: u32,
    pub Length: u32,
    pub Text: [u16; 1],
}
impl ::core::marker::Copy for WIA_BARCODE_INFO {}
impl ::core::clone::Clone for WIA_BARCODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIA_DATA_CALLBACK_HEADER {
    pub lSize: i32,
    pub guidFormatID: ::windows_sys::core::GUID,
    pub lBufferSize: i32,
    pub lPageCount: i32,
}
impl ::core::marker::Copy for WIA_DATA_CALLBACK_HEADER {}
impl ::core::clone::Clone for WIA_DATA_CALLBACK_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIA_DATA_TRANSFER_INFO {
    pub ulSize: u32,
    pub ulSection: u32,
    pub ulBufferSize: u32,
    pub bDoubleBuffer: super::super::Foundation::BOOL,
    pub ulReserved1: u32,
    pub ulReserved2: u32,
    pub ulReserved3: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIA_DATA_TRANSFER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIA_DATA_TRANSFER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIA_DEV_CAP {
    pub guid: ::windows_sys::core::GUID,
    pub ulFlags: u32,
    pub bstrName: ::windows_sys::core::BSTR,
    pub bstrDescription: ::windows_sys::core::BSTR,
    pub bstrIcon: ::windows_sys::core::BSTR,
    pub bstrCommandline: ::windows_sys::core::BSTR,
}
impl ::core::marker::Copy for WIA_DEV_CAP {}
impl ::core::clone::Clone for WIA_DEV_CAP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIA_DEV_CAP_DRV {
    pub guid: *mut ::windows_sys::core::GUID,
    pub ulFlags: u32,
    pub wszName: ::windows_sys::core::PWSTR,
    pub wszDescription: ::windows_sys::core::PWSTR,
    pub wszIcon: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for WIA_DEV_CAP_DRV {}
impl ::core::clone::Clone for WIA_DEV_CAP_DRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIA_DITHER_PATTERN_DATA {
    pub lSize: i32,
    pub bstrPatternName: ::windows_sys::core::BSTR,
    pub lPatternWidth: i32,
    pub lPatternLength: i32,
    pub cbPattern: i32,
    pub pbPattern: *mut u8,
}
impl ::core::marker::Copy for WIA_DITHER_PATTERN_DATA {}
impl ::core::clone::Clone for WIA_DITHER_PATTERN_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIA_EXTENDED_TRANSFER_INFO {
    pub ulSize: u32,
    pub ulMinBufferSize: u32,
    pub ulOptimalBufferSize: u32,
    pub ulMaxBufferSize: u32,
    pub ulNumBuffers: u32,
}
impl ::core::marker::Copy for WIA_EXTENDED_TRANSFER_INFO {}
impl ::core::clone::Clone for WIA_EXTENDED_TRANSFER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIA_FORMAT_INFO {
    pub guidFormatID: ::windows_sys::core::GUID,
    pub lTymed: i32,
}
impl ::core::marker::Copy for WIA_FORMAT_INFO {}
impl ::core::clone::Clone for WIA_FORMAT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIA_MICR {
    pub Tag: u32,
    pub Version: u32,
    pub Size: u32,
    pub Placeholder: u16,
    pub Reserved: u16,
    pub Count: u32,
    pub Micr: [WIA_MICR_INFO; 1],
}
impl ::core::marker::Copy for WIA_MICR {}
impl ::core::clone::Clone for WIA_MICR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIA_MICR_INFO {
    pub Size: u32,
    pub Page: u32,
    pub Length: u32,
    pub Text: [u16; 1],
}
impl ::core::marker::Copy for WIA_MICR_INFO {}
impl ::core::clone::Clone for WIA_MICR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIA_PATCH_CODES {
    pub Tag: u32,
    pub Version: u32,
    pub Size: u32,
    pub Count: u32,
    pub PatchCodes: [WIA_PATCH_CODE_INFO; 1],
}
impl ::core::marker::Copy for WIA_PATCH_CODES {}
impl ::core::clone::Clone for WIA_PATCH_CODES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIA_PATCH_CODE_INFO {
    pub Type: u32,
}
impl ::core::marker::Copy for WIA_PATCH_CODE_INFO {}
impl ::core::clone::Clone for WIA_PATCH_CODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIA_PROPERTY_CONTEXT {
    pub cProps: u32,
    pub pProps: *mut u32,
    pub pChanged: *mut super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIA_PROPERTY_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIA_PROPERTY_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct WIA_PROPERTY_INFO {
    pub lAccessFlags: u32,
    pub vt: super::super::System::Com::VARENUM,
    pub ValidVal: WIA_PROPERTY_INFO_0,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for WIA_PROPERTY_INFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for WIA_PROPERTY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union WIA_PROPERTY_INFO_0 {
    pub Range: WIA_PROPERTY_INFO_0_7,
    pub RangeFloat: WIA_PROPERTY_INFO_0_6,
    pub List: WIA_PROPERTY_INFO_0_4,
    pub ListFloat: WIA_PROPERTY_INFO_0_2,
    pub ListGuid: WIA_PROPERTY_INFO_0_3,
    pub ListBStr: WIA_PROPERTY_INFO_0_1,
    pub Flag: WIA_PROPERTY_INFO_0_0,
    pub None: WIA_PROPERTY_INFO_0_5,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for WIA_PROPERTY_INFO_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct WIA_PROPERTY_INFO_0_0 {
    pub Nom: i32,
    pub ValidBits: i32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for WIA_PROPERTY_INFO_0_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct WIA_PROPERTY_INFO_0_1 {
    pub cNumList: i32,
    pub Nom: ::windows_sys::core::BSTR,
    pub pList: *mut ::windows_sys::core::BSTR,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for WIA_PROPERTY_INFO_0_1 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct WIA_PROPERTY_INFO_0_2 {
    pub cNumList: i32,
    pub Nom: f64,
    pub pList: *mut u8,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for WIA_PROPERTY_INFO_0_2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct WIA_PROPERTY_INFO_0_3 {
    pub cNumList: i32,
    pub Nom: ::windows_sys::core::GUID,
    pub pList: *mut ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for WIA_PROPERTY_INFO_0_3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct WIA_PROPERTY_INFO_0_4 {
    pub cNumList: i32,
    pub Nom: i32,
    pub pList: *mut u8,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for WIA_PROPERTY_INFO_0_4 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct WIA_PROPERTY_INFO_0_5 {
    pub Dummy: i32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for WIA_PROPERTY_INFO_0_5 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0_5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct WIA_PROPERTY_INFO_0_6 {
    pub Min: f64,
    pub Nom: f64,
    pub Max: f64,
    pub Inc: f64,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for WIA_PROPERTY_INFO_0_6 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0_6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct WIA_PROPERTY_INFO_0_7 {
    pub Min: i32,
    pub Nom: i32,
    pub Max: i32,
    pub Inc: i32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for WIA_PROPERTY_INFO_0_7 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0_7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIA_PROPID_TO_NAME {
    pub propid: u32,
    pub pszName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for WIA_PROPID_TO_NAME {}
impl ::core::clone::Clone for WIA_PROPID_TO_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIA_RAW_HEADER {
    pub Tag: u32,
    pub Version: u32,
    pub HeaderSize: u32,
    pub XRes: u32,
    pub YRes: u32,
    pub XExtent: u32,
    pub YExtent: u32,
    pub BytesPerLine: u32,
    pub BitsPerPixel: u32,
    pub ChannelsPerPixel: u32,
    pub DataType: u32,
    pub BitsPerChannel: [u8; 8],
    pub Compression: u32,
    pub PhotometricInterp: u32,
    pub LineOrder: u32,
    pub RawDataOffset: u32,
    pub RawDataSize: u32,
    pub PaletteOffset: u32,
    pub PaletteSize: u32,
}
impl ::core::marker::Copy for WIA_RAW_HEADER {}
impl ::core::clone::Clone for WIA_RAW_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WiaTransferParams {
    pub lMessage: i32,
    pub lPercentComplete: i32,
    pub ulTransferredBytes: u64,
    pub hrErrorStatus: ::windows_sys::core::HRESULT,
}
impl ::core::marker::Copy for WiaTransferParams {}
impl ::core::clone::Clone for WiaTransferParams {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DeviceDialogFunction = ::core::option::Option<unsafe extern "system" fn(param0: *mut DEVICEDIALOGDATA) -> ::windows_sys::core::HRESULT>;
