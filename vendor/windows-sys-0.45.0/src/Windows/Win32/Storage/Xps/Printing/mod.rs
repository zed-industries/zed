#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
::windows_sys::core::link ! ( "xpsprint.dll""system" #[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"] fn StartXpsPrintJob ( printername : :: windows_sys::core::PCWSTR , jobname : :: windows_sys::core::PCWSTR , outputfilename : :: windows_sys::core::PCWSTR , progressevent : super::super::super::Foundation:: HANDLE , completionevent : super::super::super::Foundation:: HANDLE , printablepageson : *const u8 , printablepagesoncount : u32 , xpsprintjob : *mut IXpsPrintJob , documentstream : *mut IXpsPrintJobStream , printticketstream : *mut IXpsPrintJobStream ) -> :: windows_sys::core::HRESULT );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "xpsprint.dll""system" #[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`, `\"Win32_Foundation\"`*"] fn StartXpsPrintJob1 ( printername : :: windows_sys::core::PCWSTR , jobname : :: windows_sys::core::PCWSTR , outputfilename : :: windows_sys::core::PCWSTR , progressevent : super::super::super::Foundation:: HANDLE , completionevent : super::super::super::Foundation:: HANDLE , xpsprintjob : *mut IXpsPrintJob , printcontentreceiver : *mut super:: IXpsOMPackageTarget ) -> :: windows_sys::core::HRESULT );
pub type IPrintDocumentPackageStatusEvent = *mut ::core::ffi::c_void;
pub type IPrintDocumentPackageTarget = *mut ::core::ffi::c_void;
pub type IPrintDocumentPackageTargetFactory = *mut ::core::ffi::c_void;
pub type IXpsPrintJob = *mut ::core::ffi::c_void;
pub type IXpsPrintJobStream = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const ID_DOCUMENTPACKAGETARGET_MSXPS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9cae40a8_ded1_41c9_a9fd_d735ef33aeda);
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const ID_DOCUMENTPACKAGETARGET_OPENXPS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0056bb72_8c9c_4612_bd0f_93012a87099d);
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const ID_DOCUMENTPACKAGETARGET_OPENXPS_WITH_3D: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x63dbd720_8b14_4577_b074_7bb11b596d28);
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const PrintDocumentPackageTarget: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4842669e_9947_46ea_8ba2_d8cce432c2ca);
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const PrintDocumentPackageTargetFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x348ef17d_6c81_4982_92b4_ee188a43867a);
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub type PrintDocumentPackageCompletion = i32;
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const PrintDocumentPackageCompletion_InProgress: PrintDocumentPackageCompletion = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const PrintDocumentPackageCompletion_Completed: PrintDocumentPackageCompletion = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const PrintDocumentPackageCompletion_Canceled: PrintDocumentPackageCompletion = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const PrintDocumentPackageCompletion_Failed: PrintDocumentPackageCompletion = 3i32;
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub type XPS_JOB_COMPLETION = i32;
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const XPS_JOB_IN_PROGRESS: XPS_JOB_COMPLETION = 0i32;
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const XPS_JOB_COMPLETED: XPS_JOB_COMPLETION = 1i32;
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const XPS_JOB_CANCELLED: XPS_JOB_COMPLETION = 2i32;
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub const XPS_JOB_FAILED: XPS_JOB_COMPLETION = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub struct PrintDocumentPackageStatus {
    pub JobId: u32,
    pub CurrentDocument: i32,
    pub CurrentPage: i32,
    pub CurrentPageTotal: i32,
    pub Completion: PrintDocumentPackageCompletion,
    pub PackageStatus: ::windows_sys::core::HRESULT,
}
impl ::core::marker::Copy for PrintDocumentPackageStatus {}
impl ::core::clone::Clone for PrintDocumentPackageStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps_Printing\"`*"]
pub struct XPS_JOB_STATUS {
    pub jobId: u32,
    pub currentDocument: i32,
    pub currentPage: i32,
    pub currentPageTotal: i32,
    pub completion: XPS_JOB_COMPLETION,
    pub jobStatus: ::windows_sys::core::HRESULT,
}
impl ::core::marker::Copy for XPS_JOB_STATUS {}
impl ::core::clone::Clone for XPS_JOB_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
