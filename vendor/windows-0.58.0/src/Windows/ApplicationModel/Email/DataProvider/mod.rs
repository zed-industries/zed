windows_core::imp::define_interface!(IEmailDataProviderConnection, IEmailDataProviderConnection_Vtbl, 0x3b9c9dc7_37b2_4bf0_ae30_7b644a1c96e1);
impl windows_core::RuntimeType for IEmailDataProviderConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailDataProviderConnection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MailboxSyncRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveMailboxSyncRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub DownloadMessageRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDownloadMessageRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub DownloadAttachmentRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDownloadAttachmentRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CreateFolderRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCreateFolderRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub DeleteFolderRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDeleteFolderRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub EmptyFolderRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveEmptyFolderRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub MoveFolderRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveMoveFolderRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub UpdateMeetingResponseRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveUpdateMeetingResponseRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ForwardMeetingRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveForwardMeetingRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ProposeNewTimeForMeetingRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveProposeNewTimeForMeetingRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SetAutoReplySettingsRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSetAutoReplySettingsRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub GetAutoReplySettingsRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveGetAutoReplySettingsRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ResolveRecipientsRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveResolveRecipientsRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ValidateCertificatesRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveValidateCertificatesRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ServerSearchReadBatchRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveServerSearchReadBatchRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailDataProviderTriggerDetails, IEmailDataProviderTriggerDetails_Vtbl, 0x8f3e4e50_341e_45f3_bba0_84a005e1319a);
impl windows_core::RuntimeType for IEmailDataProviderTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailDataProviderTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Connection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxCreateFolderRequest, IEmailMailboxCreateFolderRequest_Vtbl, 0x184d3775_c921_4c39_a309_e16c9f22b04b);
impl windows_core::RuntimeType for IEmailMailboxCreateFolderRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxCreateFolderRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ParentFolderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::EmailMailboxCreateFolderStatus, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxCreateFolderRequestEventArgs, IEmailMailboxCreateFolderRequestEventArgs_Vtbl, 0x03e4c02c_241c_4ea9_a68f_ff20bc5afc85);
impl windows_core::RuntimeType for IEmailMailboxCreateFolderRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxCreateFolderRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxDeleteFolderRequest, IEmailMailboxDeleteFolderRequest_Vtbl, 0x9469e88a_a931_4779_923d_09a3ea292e29);
impl windows_core::RuntimeType for IEmailMailboxDeleteFolderRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxDeleteFolderRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub EmailFolderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::EmailMailboxDeleteFolderStatus, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxDeleteFolderRequestEventArgs, IEmailMailboxDeleteFolderRequestEventArgs_Vtbl, 0xb4d32d06_2332_4678_8378_28b579336846);
impl windows_core::RuntimeType for IEmailMailboxDeleteFolderRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxDeleteFolderRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxDownloadAttachmentRequest, IEmailMailboxDownloadAttachmentRequest_Vtbl, 0x0b1dbbb4_750c_48e1_bce4_8d589684ffbc);
impl windows_core::RuntimeType for IEmailMailboxDownloadAttachmentRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxDownloadAttachmentRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub EmailMessageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub EmailAttachmentId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxDownloadAttachmentRequestEventArgs, IEmailMailboxDownloadAttachmentRequestEventArgs_Vtbl, 0xccddc46d_ffa8_4877_9f9d_fed7bcaf4104);
impl windows_core::RuntimeType for IEmailMailboxDownloadAttachmentRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxDownloadAttachmentRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxDownloadMessageRequest, IEmailMailboxDownloadMessageRequest_Vtbl, 0x497b4187_5b4d_4b23_816c_f3842beb753e);
impl windows_core::RuntimeType for IEmailMailboxDownloadMessageRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxDownloadMessageRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub EmailMessageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxDownloadMessageRequestEventArgs, IEmailMailboxDownloadMessageRequestEventArgs_Vtbl, 0x470409ad_d0a0_4a5b_bb2a_37621039c53e);
impl windows_core::RuntimeType for IEmailMailboxDownloadMessageRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxDownloadMessageRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxEmptyFolderRequest, IEmailMailboxEmptyFolderRequest_Vtbl, 0xfe4b03ab_f86d_46d9_b4ce_bc8a6d9e9268);
impl windows_core::RuntimeType for IEmailMailboxEmptyFolderRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxEmptyFolderRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub EmailFolderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::EmailMailboxEmptyFolderStatus, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxEmptyFolderRequestEventArgs, IEmailMailboxEmptyFolderRequestEventArgs_Vtbl, 0x7183f484_985a_4ac0_b33f_ee0e2627a3c0);
impl windows_core::RuntimeType for IEmailMailboxEmptyFolderRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxEmptyFolderRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxForwardMeetingRequest, IEmailMailboxForwardMeetingRequest_Vtbl, 0x616d6af1_70d4_4832_b869_b80542ae9be8);
impl windows_core::RuntimeType for IEmailMailboxForwardMeetingRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxForwardMeetingRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub EmailMessageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Recipients: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Recipients: usize,
    pub Subject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ForwardHeaderType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::EmailMessageBodyKind) -> windows_core::HRESULT,
    pub ForwardHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Comment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxForwardMeetingRequestEventArgs, IEmailMailboxForwardMeetingRequestEventArgs_Vtbl, 0x2bd8f33a_2974_4759_a5a5_58f44d3c0275);
impl windows_core::RuntimeType for IEmailMailboxForwardMeetingRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxForwardMeetingRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxGetAutoReplySettingsRequest, IEmailMailboxGetAutoReplySettingsRequest_Vtbl, 0x9b380789_1e88_4e01_84cc_1386ad9a2c2f);
impl windows_core::RuntimeType for IEmailMailboxGetAutoReplySettingsRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxGetAutoReplySettingsRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RequestedFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::EmailMailboxAutoReplyMessageResponseKind) -> windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxGetAutoReplySettingsRequestEventArgs, IEmailMailboxGetAutoReplySettingsRequestEventArgs_Vtbl, 0xd79f55c2_fd45_4004_8a91_9bacf38b7022);
impl windows_core::RuntimeType for IEmailMailboxGetAutoReplySettingsRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxGetAutoReplySettingsRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxMoveFolderRequest, IEmailMailboxMoveFolderRequest_Vtbl, 0x10ba2856_4a95_4068_91cc_67cc7acf454f);
impl windows_core::RuntimeType for IEmailMailboxMoveFolderRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxMoveFolderRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub EmailFolderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub NewParentFolderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub NewFolderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxMoveFolderRequestEventArgs, IEmailMailboxMoveFolderRequestEventArgs_Vtbl, 0x38623020_14ba_4c88_8698_7239e3c8aaa7);
impl windows_core::RuntimeType for IEmailMailboxMoveFolderRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxMoveFolderRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxProposeNewTimeForMeetingRequest, IEmailMailboxProposeNewTimeForMeetingRequest_Vtbl, 0x5aeff152_9799_4f9f_a399_ff07f3eef04e);
impl windows_core::RuntimeType for IEmailMailboxProposeNewTimeForMeetingRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxProposeNewTimeForMeetingRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub EmailMessageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub NewStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub NewDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Subject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Comment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxProposeNewTimeForMeetingRequestEventArgs, IEmailMailboxProposeNewTimeForMeetingRequestEventArgs_Vtbl, 0xfb480b98_33ad_4a67_8251_0f9c249b6a20);
impl windows_core::RuntimeType for IEmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxProposeNewTimeForMeetingRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxResolveRecipientsRequest, IEmailMailboxResolveRecipientsRequest_Vtbl, 0xefa4cf70_7b39_4c9b_811e_41eaf43a332d);
impl windows_core::RuntimeType for IEmailMailboxResolveRecipientsRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxResolveRecipientsRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Recipients: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Recipients: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReportCompletedAsync: usize,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxResolveRecipientsRequestEventArgs, IEmailMailboxResolveRecipientsRequestEventArgs_Vtbl, 0x260f9e02_b2cf_40f8_8c28_e3ed43b1e89a);
impl windows_core::RuntimeType for IEmailMailboxResolveRecipientsRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxResolveRecipientsRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxServerSearchReadBatchRequest, IEmailMailboxServerSearchReadBatchRequest_Vtbl, 0x090eebdf_5a96_41d3_8ad8_34912f9aa60e);
impl windows_core::RuntimeType for IEmailMailboxServerSearchReadBatchRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxServerSearchReadBatchRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SessionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub EmailMailboxId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub EmailFolderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Options: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SuggestedBatchSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SaveMessageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::EmailBatchStatus, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxServerSearchReadBatchRequestEventArgs, IEmailMailboxServerSearchReadBatchRequestEventArgs_Vtbl, 0x14101b4e_ed9e_45d1_ad7a_cc9b7f643ae2);
impl windows_core::RuntimeType for IEmailMailboxServerSearchReadBatchRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxServerSearchReadBatchRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxSetAutoReplySettingsRequest, IEmailMailboxSetAutoReplySettingsRequest_Vtbl, 0x75a422d0_a88e_4e54_8dc7_c243186b774e);
impl windows_core::RuntimeType for IEmailMailboxSetAutoReplySettingsRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxSetAutoReplySettingsRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AutoReplySettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxSetAutoReplySettingsRequestEventArgs, IEmailMailboxSetAutoReplySettingsRequestEventArgs_Vtbl, 0x09da11ad_d7ca_4087_ac86_53fa67f76246);
impl windows_core::RuntimeType for IEmailMailboxSetAutoReplySettingsRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxSetAutoReplySettingsRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxSyncManagerSyncRequest, IEmailMailboxSyncManagerSyncRequest_Vtbl, 0x4e10e8e4_7e67_405a_b673_dc60c91090fc);
impl windows_core::RuntimeType for IEmailMailboxSyncManagerSyncRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxSyncManagerSyncRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxSyncManagerSyncRequestEventArgs, IEmailMailboxSyncManagerSyncRequestEventArgs_Vtbl, 0x439a031a_8fcc_4ae5_b9b5_d434e0a65aa8);
impl windows_core::RuntimeType for IEmailMailboxSyncManagerSyncRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxSyncManagerSyncRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxUpdateMeetingResponseRequest, IEmailMailboxUpdateMeetingResponseRequest_Vtbl, 0x5b99ac93_b2cf_4888_ba4f_306b6b66df30);
impl windows_core::RuntimeType for IEmailMailboxUpdateMeetingResponseRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxUpdateMeetingResponseRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub EmailMessageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Response: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::EmailMeetingResponseType) -> windows_core::HRESULT,
    pub Subject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Comment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SendUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxUpdateMeetingResponseRequestEventArgs, IEmailMailboxUpdateMeetingResponseRequestEventArgs_Vtbl, 0x6898d761_56c9_4f17_be31_66fda94ba159);
impl windows_core::RuntimeType for IEmailMailboxUpdateMeetingResponseRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxUpdateMeetingResponseRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxValidateCertificatesRequest, IEmailMailboxValidateCertificatesRequest_Vtbl, 0xa94d3931_e11a_4f97_b81a_187a70a8f41a);
impl windows_core::RuntimeType for IEmailMailboxValidateCertificatesRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxValidateCertificatesRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub Certificates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    Certificates: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReportCompletedAsync: usize,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailMailboxValidateCertificatesRequestEventArgs, IEmailMailboxValidateCertificatesRequestEventArgs_Vtbl, 0x2583bf17_02ff_49fe_a73c_03f37566c691);
impl windows_core::RuntimeType for IEmailMailboxValidateCertificatesRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailMailboxValidateCertificatesRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailDataProviderConnection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailDataProviderConnection, windows_core::IUnknown, windows_core::IInspectable);
impl EmailDataProviderConnection {
    pub fn MailboxSyncRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxSyncManagerSyncRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MailboxSyncRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMailboxSyncRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMailboxSyncRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DownloadMessageRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxDownloadMessageRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DownloadMessageRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDownloadMessageRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDownloadMessageRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DownloadAttachmentRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxDownloadAttachmentRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DownloadAttachmentRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDownloadAttachmentRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDownloadAttachmentRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CreateFolderRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxCreateFolderRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFolderRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCreateFolderRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCreateFolderRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DeleteFolderRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxDeleteFolderRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteFolderRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDeleteFolderRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDeleteFolderRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn EmptyFolderRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxEmptyFolderRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmptyFolderRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveEmptyFolderRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveEmptyFolderRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn MoveFolderRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxMoveFolderRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveFolderRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMoveFolderRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMoveFolderRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn UpdateMeetingResponseRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxUpdateMeetingResponseRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateMeetingResponseRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUpdateMeetingResponseRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveUpdateMeetingResponseRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ForwardMeetingRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxForwardMeetingRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForwardMeetingRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveForwardMeetingRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveForwardMeetingRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ProposeNewTimeForMeetingRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxProposeNewTimeForMeetingRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProposeNewTimeForMeetingRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveProposeNewTimeForMeetingRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveProposeNewTimeForMeetingRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetAutoReplySettingsRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxSetAutoReplySettingsRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetAutoReplySettingsRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSetAutoReplySettingsRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSetAutoReplySettingsRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetAutoReplySettingsRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxGetAutoReplySettingsRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAutoReplySettingsRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveGetAutoReplySettingsRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveGetAutoReplySettingsRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ResolveRecipientsRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxResolveRecipientsRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolveRecipientsRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveResolveRecipientsRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveResolveRecipientsRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ValidateCertificatesRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxValidateCertificatesRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValidateCertificatesRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveValidateCertificatesRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveValidateCertificatesRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ServerSearchReadBatchRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxServerSearchReadBatchRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerSearchReadBatchRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveServerSearchReadBatchRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveServerSearchReadBatchRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for EmailDataProviderConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailDataProviderConnection>();
}
unsafe impl windows_core::Interface for EmailDataProviderConnection {
    type Vtable = IEmailDataProviderConnection_Vtbl;
    const IID: windows_core::GUID = <IEmailDataProviderConnection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailDataProviderConnection {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection";
}
unsafe impl Send for EmailDataProviderConnection {}
unsafe impl Sync for EmailDataProviderConnection {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailDataProviderTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailDataProviderTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl EmailDataProviderTriggerDetails {
    pub fn Connection(&self) -> windows_core::Result<EmailDataProviderConnection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Connection)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailDataProviderTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailDataProviderTriggerDetails>();
}
unsafe impl windows_core::Interface for EmailDataProviderTriggerDetails {
    type Vtable = IEmailDataProviderTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IEmailDataProviderTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailDataProviderTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailDataProviderTriggerDetails";
}
unsafe impl Send for EmailDataProviderTriggerDetails {}
unsafe impl Sync for EmailDataProviderTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxCreateFolderRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxCreateFolderRequest, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxCreateFolderRequest {
    pub fn EmailMailboxId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailMailboxId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParentFolderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParentFolderId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompletedAsync<P0>(&self, folder: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::EmailFolder>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), folder.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self, status: super::EmailMailboxCreateFolderStatus) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), status, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxCreateFolderRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxCreateFolderRequest>();
}
unsafe impl windows_core::Interface for EmailMailboxCreateFolderRequest {
    type Vtable = IEmailMailboxCreateFolderRequest_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxCreateFolderRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxCreateFolderRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequest";
}
unsafe impl Send for EmailMailboxCreateFolderRequest {}
unsafe impl Sync for EmailMailboxCreateFolderRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxCreateFolderRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxCreateFolderRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxCreateFolderRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<EmailMailboxCreateFolderRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxCreateFolderRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxCreateFolderRequestEventArgs>();
}
unsafe impl windows_core::Interface for EmailMailboxCreateFolderRequestEventArgs {
    type Vtable = IEmailMailboxCreateFolderRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxCreateFolderRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxCreateFolderRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequestEventArgs";
}
unsafe impl Send for EmailMailboxCreateFolderRequestEventArgs {}
unsafe impl Sync for EmailMailboxCreateFolderRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxDeleteFolderRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxDeleteFolderRequest, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxDeleteFolderRequest {
    pub fn EmailMailboxId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailMailboxId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EmailFolderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailFolderId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompletedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self, status: super::EmailMailboxDeleteFolderStatus) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), status, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxDeleteFolderRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxDeleteFolderRequest>();
}
unsafe impl windows_core::Interface for EmailMailboxDeleteFolderRequest {
    type Vtable = IEmailMailboxDeleteFolderRequest_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxDeleteFolderRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxDeleteFolderRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequest";
}
unsafe impl Send for EmailMailboxDeleteFolderRequest {}
unsafe impl Sync for EmailMailboxDeleteFolderRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxDeleteFolderRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxDeleteFolderRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxDeleteFolderRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<EmailMailboxDeleteFolderRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxDeleteFolderRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxDeleteFolderRequestEventArgs>();
}
unsafe impl windows_core::Interface for EmailMailboxDeleteFolderRequestEventArgs {
    type Vtable = IEmailMailboxDeleteFolderRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxDeleteFolderRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxDeleteFolderRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequestEventArgs";
}
unsafe impl Send for EmailMailboxDeleteFolderRequestEventArgs {}
unsafe impl Sync for EmailMailboxDeleteFolderRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxDownloadAttachmentRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxDownloadAttachmentRequest, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxDownloadAttachmentRequest {
    pub fn EmailMailboxId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailMailboxId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EmailMessageId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailMessageId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EmailAttachmentId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailAttachmentId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompletedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxDownloadAttachmentRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxDownloadAttachmentRequest>();
}
unsafe impl windows_core::Interface for EmailMailboxDownloadAttachmentRequest {
    type Vtable = IEmailMailboxDownloadAttachmentRequest_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxDownloadAttachmentRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxDownloadAttachmentRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequest";
}
unsafe impl Send for EmailMailboxDownloadAttachmentRequest {}
unsafe impl Sync for EmailMailboxDownloadAttachmentRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxDownloadAttachmentRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxDownloadAttachmentRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxDownloadAttachmentRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<EmailMailboxDownloadAttachmentRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxDownloadAttachmentRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxDownloadAttachmentRequestEventArgs>();
}
unsafe impl windows_core::Interface for EmailMailboxDownloadAttachmentRequestEventArgs {
    type Vtable = IEmailMailboxDownloadAttachmentRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxDownloadAttachmentRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxDownloadAttachmentRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequestEventArgs";
}
unsafe impl Send for EmailMailboxDownloadAttachmentRequestEventArgs {}
unsafe impl Sync for EmailMailboxDownloadAttachmentRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxDownloadMessageRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxDownloadMessageRequest, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxDownloadMessageRequest {
    pub fn EmailMailboxId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailMailboxId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EmailMessageId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailMessageId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompletedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxDownloadMessageRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxDownloadMessageRequest>();
}
unsafe impl windows_core::Interface for EmailMailboxDownloadMessageRequest {
    type Vtable = IEmailMailboxDownloadMessageRequest_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxDownloadMessageRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxDownloadMessageRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequest";
}
unsafe impl Send for EmailMailboxDownloadMessageRequest {}
unsafe impl Sync for EmailMailboxDownloadMessageRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxDownloadMessageRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxDownloadMessageRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxDownloadMessageRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<EmailMailboxDownloadMessageRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxDownloadMessageRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxDownloadMessageRequestEventArgs>();
}
unsafe impl windows_core::Interface for EmailMailboxDownloadMessageRequestEventArgs {
    type Vtable = IEmailMailboxDownloadMessageRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxDownloadMessageRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxDownloadMessageRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequestEventArgs";
}
unsafe impl Send for EmailMailboxDownloadMessageRequestEventArgs {}
unsafe impl Sync for EmailMailboxDownloadMessageRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxEmptyFolderRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxEmptyFolderRequest, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxEmptyFolderRequest {
    pub fn EmailMailboxId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailMailboxId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EmailFolderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailFolderId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompletedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self, status: super::EmailMailboxEmptyFolderStatus) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), status, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxEmptyFolderRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxEmptyFolderRequest>();
}
unsafe impl windows_core::Interface for EmailMailboxEmptyFolderRequest {
    type Vtable = IEmailMailboxEmptyFolderRequest_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxEmptyFolderRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxEmptyFolderRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequest";
}
unsafe impl Send for EmailMailboxEmptyFolderRequest {}
unsafe impl Sync for EmailMailboxEmptyFolderRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxEmptyFolderRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxEmptyFolderRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxEmptyFolderRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<EmailMailboxEmptyFolderRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxEmptyFolderRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxEmptyFolderRequestEventArgs>();
}
unsafe impl windows_core::Interface for EmailMailboxEmptyFolderRequestEventArgs {
    type Vtable = IEmailMailboxEmptyFolderRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxEmptyFolderRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxEmptyFolderRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequestEventArgs";
}
unsafe impl Send for EmailMailboxEmptyFolderRequestEventArgs {}
unsafe impl Sync for EmailMailboxEmptyFolderRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxForwardMeetingRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxForwardMeetingRequest, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxForwardMeetingRequest {
    pub fn EmailMailboxId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailMailboxId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EmailMessageId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailMessageId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Recipients(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<super::EmailRecipient>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Recipients)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Subject(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Subject)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ForwardHeaderType(&self) -> windows_core::Result<super::EmailMessageBodyKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForwardHeaderType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ForwardHeader(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForwardHeader)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompletedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxForwardMeetingRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxForwardMeetingRequest>();
}
unsafe impl windows_core::Interface for EmailMailboxForwardMeetingRequest {
    type Vtable = IEmailMailboxForwardMeetingRequest_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxForwardMeetingRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxForwardMeetingRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequest";
}
unsafe impl Send for EmailMailboxForwardMeetingRequest {}
unsafe impl Sync for EmailMailboxForwardMeetingRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxForwardMeetingRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxForwardMeetingRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxForwardMeetingRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<EmailMailboxForwardMeetingRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxForwardMeetingRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxForwardMeetingRequestEventArgs>();
}
unsafe impl windows_core::Interface for EmailMailboxForwardMeetingRequestEventArgs {
    type Vtable = IEmailMailboxForwardMeetingRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxForwardMeetingRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxForwardMeetingRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequestEventArgs";
}
unsafe impl Send for EmailMailboxForwardMeetingRequestEventArgs {}
unsafe impl Sync for EmailMailboxForwardMeetingRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxGetAutoReplySettingsRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxGetAutoReplySettingsRequest, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxGetAutoReplySettingsRequest {
    pub fn EmailMailboxId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailMailboxId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestedFormat(&self) -> windows_core::Result<super::EmailMailboxAutoReplyMessageResponseKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestedFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportCompletedAsync<P0>(&self, autoreplysettings: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::EmailMailboxAutoReplySettings>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), autoreplysettings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxGetAutoReplySettingsRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxGetAutoReplySettingsRequest>();
}
unsafe impl windows_core::Interface for EmailMailboxGetAutoReplySettingsRequest {
    type Vtable = IEmailMailboxGetAutoReplySettingsRequest_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxGetAutoReplySettingsRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxGetAutoReplySettingsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequest";
}
unsafe impl Send for EmailMailboxGetAutoReplySettingsRequest {}
unsafe impl Sync for EmailMailboxGetAutoReplySettingsRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxGetAutoReplySettingsRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxGetAutoReplySettingsRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxGetAutoReplySettingsRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<EmailMailboxGetAutoReplySettingsRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxGetAutoReplySettingsRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxGetAutoReplySettingsRequestEventArgs>();
}
unsafe impl windows_core::Interface for EmailMailboxGetAutoReplySettingsRequestEventArgs {
    type Vtable = IEmailMailboxGetAutoReplySettingsRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxGetAutoReplySettingsRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxGetAutoReplySettingsRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequestEventArgs";
}
unsafe impl Send for EmailMailboxGetAutoReplySettingsRequestEventArgs {}
unsafe impl Sync for EmailMailboxGetAutoReplySettingsRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxMoveFolderRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxMoveFolderRequest, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxMoveFolderRequest {
    pub fn EmailMailboxId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailMailboxId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EmailFolderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailFolderId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NewParentFolderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NewParentFolderId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NewFolderName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NewFolderName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompletedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxMoveFolderRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxMoveFolderRequest>();
}
unsafe impl windows_core::Interface for EmailMailboxMoveFolderRequest {
    type Vtable = IEmailMailboxMoveFolderRequest_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxMoveFolderRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxMoveFolderRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequest";
}
unsafe impl Send for EmailMailboxMoveFolderRequest {}
unsafe impl Sync for EmailMailboxMoveFolderRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxMoveFolderRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxMoveFolderRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxMoveFolderRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<EmailMailboxMoveFolderRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxMoveFolderRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxMoveFolderRequestEventArgs>();
}
unsafe impl windows_core::Interface for EmailMailboxMoveFolderRequestEventArgs {
    type Vtable = IEmailMailboxMoveFolderRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxMoveFolderRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxMoveFolderRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequestEventArgs";
}
unsafe impl Send for EmailMailboxMoveFolderRequestEventArgs {}
unsafe impl Sync for EmailMailboxMoveFolderRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxProposeNewTimeForMeetingRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxProposeNewTimeForMeetingRequest, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxProposeNewTimeForMeetingRequest {
    pub fn EmailMailboxId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailMailboxId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EmailMessageId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailMessageId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NewStartTime(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NewStartTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NewDuration(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NewDuration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Subject(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Subject)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompletedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxProposeNewTimeForMeetingRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxProposeNewTimeForMeetingRequest>();
}
unsafe impl windows_core::Interface for EmailMailboxProposeNewTimeForMeetingRequest {
    type Vtable = IEmailMailboxProposeNewTimeForMeetingRequest_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxProposeNewTimeForMeetingRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxProposeNewTimeForMeetingRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequest";
}
unsafe impl Send for EmailMailboxProposeNewTimeForMeetingRequest {}
unsafe impl Sync for EmailMailboxProposeNewTimeForMeetingRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxProposeNewTimeForMeetingRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxProposeNewTimeForMeetingRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<EmailMailboxProposeNewTimeForMeetingRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxProposeNewTimeForMeetingRequestEventArgs>();
}
unsafe impl windows_core::Interface for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    type Vtable = IEmailMailboxProposeNewTimeForMeetingRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxProposeNewTimeForMeetingRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequestEventArgs";
}
unsafe impl Send for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {}
unsafe impl Sync for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxResolveRecipientsRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxResolveRecipientsRequest, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxResolveRecipientsRequest {
    pub fn EmailMailboxId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailMailboxId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Recipients(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Recipients)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReportCompletedAsync<P0>(&self, resolutionresults: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<super::EmailRecipientResolutionResult>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), resolutionresults.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxResolveRecipientsRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxResolveRecipientsRequest>();
}
unsafe impl windows_core::Interface for EmailMailboxResolveRecipientsRequest {
    type Vtable = IEmailMailboxResolveRecipientsRequest_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxResolveRecipientsRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxResolveRecipientsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequest";
}
unsafe impl Send for EmailMailboxResolveRecipientsRequest {}
unsafe impl Sync for EmailMailboxResolveRecipientsRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxResolveRecipientsRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxResolveRecipientsRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxResolveRecipientsRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<EmailMailboxResolveRecipientsRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxResolveRecipientsRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxResolveRecipientsRequestEventArgs>();
}
unsafe impl windows_core::Interface for EmailMailboxResolveRecipientsRequestEventArgs {
    type Vtable = IEmailMailboxResolveRecipientsRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxResolveRecipientsRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxResolveRecipientsRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequestEventArgs";
}
unsafe impl Send for EmailMailboxResolveRecipientsRequestEventArgs {}
unsafe impl Sync for EmailMailboxResolveRecipientsRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxServerSearchReadBatchRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxServerSearchReadBatchRequest, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxServerSearchReadBatchRequest {
    pub fn SessionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EmailMailboxId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailMailboxId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EmailFolderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailFolderId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Options(&self) -> windows_core::Result<super::EmailQueryOptions> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Options)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SuggestedBatchSize(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuggestedBatchSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SaveMessageAsync<P0>(&self, message: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::EmailMessage>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveMessageAsync)(windows_core::Interface::as_raw(this), message.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompletedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self, batchstatus: super::EmailBatchStatus) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), batchstatus, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxServerSearchReadBatchRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxServerSearchReadBatchRequest>();
}
unsafe impl windows_core::Interface for EmailMailboxServerSearchReadBatchRequest {
    type Vtable = IEmailMailboxServerSearchReadBatchRequest_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxServerSearchReadBatchRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxServerSearchReadBatchRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequest";
}
unsafe impl Send for EmailMailboxServerSearchReadBatchRequest {}
unsafe impl Sync for EmailMailboxServerSearchReadBatchRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxServerSearchReadBatchRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxServerSearchReadBatchRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxServerSearchReadBatchRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<EmailMailboxServerSearchReadBatchRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxServerSearchReadBatchRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxServerSearchReadBatchRequestEventArgs>();
}
unsafe impl windows_core::Interface for EmailMailboxServerSearchReadBatchRequestEventArgs {
    type Vtable = IEmailMailboxServerSearchReadBatchRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxServerSearchReadBatchRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxServerSearchReadBatchRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequestEventArgs";
}
unsafe impl Send for EmailMailboxServerSearchReadBatchRequestEventArgs {}
unsafe impl Sync for EmailMailboxServerSearchReadBatchRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxSetAutoReplySettingsRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxSetAutoReplySettingsRequest, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxSetAutoReplySettingsRequest {
    pub fn EmailMailboxId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailMailboxId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AutoReplySettings(&self) -> windows_core::Result<super::EmailMailboxAutoReplySettings> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoReplySettings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompletedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxSetAutoReplySettingsRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxSetAutoReplySettingsRequest>();
}
unsafe impl windows_core::Interface for EmailMailboxSetAutoReplySettingsRequest {
    type Vtable = IEmailMailboxSetAutoReplySettingsRequest_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxSetAutoReplySettingsRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxSetAutoReplySettingsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequest";
}
unsafe impl Send for EmailMailboxSetAutoReplySettingsRequest {}
unsafe impl Sync for EmailMailboxSetAutoReplySettingsRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxSetAutoReplySettingsRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxSetAutoReplySettingsRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxSetAutoReplySettingsRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<EmailMailboxSetAutoReplySettingsRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxSetAutoReplySettingsRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxSetAutoReplySettingsRequestEventArgs>();
}
unsafe impl windows_core::Interface for EmailMailboxSetAutoReplySettingsRequestEventArgs {
    type Vtable = IEmailMailboxSetAutoReplySettingsRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxSetAutoReplySettingsRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxSetAutoReplySettingsRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequestEventArgs";
}
unsafe impl Send for EmailMailboxSetAutoReplySettingsRequestEventArgs {}
unsafe impl Sync for EmailMailboxSetAutoReplySettingsRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxSyncManagerSyncRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxSyncManagerSyncRequest, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxSyncManagerSyncRequest {
    pub fn EmailMailboxId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailMailboxId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompletedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxSyncManagerSyncRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxSyncManagerSyncRequest>();
}
unsafe impl windows_core::Interface for EmailMailboxSyncManagerSyncRequest {
    type Vtable = IEmailMailboxSyncManagerSyncRequest_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxSyncManagerSyncRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxSyncManagerSyncRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequest";
}
unsafe impl Send for EmailMailboxSyncManagerSyncRequest {}
unsafe impl Sync for EmailMailboxSyncManagerSyncRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxSyncManagerSyncRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxSyncManagerSyncRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxSyncManagerSyncRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<EmailMailboxSyncManagerSyncRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxSyncManagerSyncRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxSyncManagerSyncRequestEventArgs>();
}
unsafe impl windows_core::Interface for EmailMailboxSyncManagerSyncRequestEventArgs {
    type Vtable = IEmailMailboxSyncManagerSyncRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxSyncManagerSyncRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxSyncManagerSyncRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequestEventArgs";
}
unsafe impl Send for EmailMailboxSyncManagerSyncRequestEventArgs {}
unsafe impl Sync for EmailMailboxSyncManagerSyncRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxUpdateMeetingResponseRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxUpdateMeetingResponseRequest, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxUpdateMeetingResponseRequest {
    pub fn EmailMailboxId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailMailboxId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EmailMessageId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailMessageId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Response(&self) -> windows_core::Result<super::EmailMeetingResponseType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Response)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Subject(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Subject)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SendUpdate(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendUpdate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReportCompletedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxUpdateMeetingResponseRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxUpdateMeetingResponseRequest>();
}
unsafe impl windows_core::Interface for EmailMailboxUpdateMeetingResponseRequest {
    type Vtable = IEmailMailboxUpdateMeetingResponseRequest_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxUpdateMeetingResponseRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxUpdateMeetingResponseRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequest";
}
unsafe impl Send for EmailMailboxUpdateMeetingResponseRequest {}
unsafe impl Sync for EmailMailboxUpdateMeetingResponseRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxUpdateMeetingResponseRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxUpdateMeetingResponseRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxUpdateMeetingResponseRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<EmailMailboxUpdateMeetingResponseRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxUpdateMeetingResponseRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxUpdateMeetingResponseRequestEventArgs>();
}
unsafe impl windows_core::Interface for EmailMailboxUpdateMeetingResponseRequestEventArgs {
    type Vtable = IEmailMailboxUpdateMeetingResponseRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxUpdateMeetingResponseRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxUpdateMeetingResponseRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequestEventArgs";
}
unsafe impl Send for EmailMailboxUpdateMeetingResponseRequestEventArgs {}
unsafe impl Sync for EmailMailboxUpdateMeetingResponseRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxValidateCertificatesRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxValidateCertificatesRequest, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxValidateCertificatesRequest {
    pub fn EmailMailboxId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailMailboxId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn Certificates(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Certificates)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReportCompletedAsync<P0>(&self, validationstatuses: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<super::EmailCertificateValidationStatus>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), validationstatuses.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportFailedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxValidateCertificatesRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxValidateCertificatesRequest>();
}
unsafe impl windows_core::Interface for EmailMailboxValidateCertificatesRequest {
    type Vtable = IEmailMailboxValidateCertificatesRequest_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxValidateCertificatesRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxValidateCertificatesRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequest";
}
unsafe impl Send for EmailMailboxValidateCertificatesRequest {}
unsafe impl Sync for EmailMailboxValidateCertificatesRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailMailboxValidateCertificatesRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailMailboxValidateCertificatesRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl EmailMailboxValidateCertificatesRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<EmailMailboxValidateCertificatesRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailMailboxValidateCertificatesRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailMailboxValidateCertificatesRequestEventArgs>();
}
unsafe impl windows_core::Interface for EmailMailboxValidateCertificatesRequestEventArgs {
    type Vtable = IEmailMailboxValidateCertificatesRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IEmailMailboxValidateCertificatesRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailMailboxValidateCertificatesRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequestEventArgs";
}
unsafe impl Send for EmailMailboxValidateCertificatesRequestEventArgs {}
unsafe impl Sync for EmailMailboxValidateCertificatesRequestEventArgs {}
