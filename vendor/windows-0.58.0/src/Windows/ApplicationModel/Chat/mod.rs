windows_core::imp::define_interface!(IChatCapabilities, IChatCapabilities_Vtbl, 0x3aff77bc_39c9_4dd1_ad2d_3964dd9d403f);
impl windows_core::RuntimeType for IChatCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsOnline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsChatCapable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsFileTransferCapable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsGeoLocationPushCapable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsIntegratedMessagingCapable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatCapabilitiesManagerStatics, IChatCapabilitiesManagerStatics_Vtbl, 0xb57a2f30_7041_458e_b0cf_7c0d9fea333a);
impl windows_core::RuntimeType for IChatCapabilitiesManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatCapabilitiesManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCachedCapabilitiesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCapabilitiesFromNetworkAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatCapabilitiesManagerStatics2, IChatCapabilitiesManagerStatics2_Vtbl, 0xe30d4274_d5c1_4ac9_9ffc_40e69184fec8);
impl windows_core::RuntimeType for IChatCapabilitiesManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatCapabilitiesManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCachedCapabilitiesForTransportAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCapabilitiesFromNetworkForTransportAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatConversation, IChatConversation_Vtbl, 0xa58c080d_1a6f_46dc_8f3d_f5028660b6ee);
impl windows_core::RuntimeType for IChatConversation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatConversation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HasUnreadMessages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Subject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsConversationMuted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsConversationMuted: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub MostRecentMessageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Participants: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Participants: usize,
    pub ThreadingInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMessageReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MarkAllMessagesAsReadAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MarkMessagesAsReadAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SaveAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyLocalParticipantComposing: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, bool) -> windows_core::HRESULT,
    pub NotifyRemoteParticipantComposing: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, bool) -> windows_core::HRESULT,
    pub RemoteParticipantComposingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRemoteParticipantComposingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatConversation2, IChatConversation2_Vtbl, 0x0a030cd1_983a_47aa_9a90_ee48ee997b59);
impl windows_core::RuntimeType for IChatConversation2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatConversation2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CanModifyParticipants: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCanModifyParticipants: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatConversationReader, IChatConversationReader_Vtbl, 0x055136d2_de32_4a47_a93a_b3dc0833852b);
impl windows_core::RuntimeType for IChatConversationReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatConversationReader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchWithCountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchWithCountAsync: usize,
}
windows_core::imp::define_interface!(IChatConversationThreadingInfo, IChatConversationThreadingInfo_Vtbl, 0x331c21dc_7a07_4422_a32c_24be7c6dab24);
impl windows_core::RuntimeType for IChatConversationThreadingInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatConversationThreadingInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContactId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetContactId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Custom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetCustom: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ConversationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetConversationId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Participants: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Participants: usize,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ChatConversationThreadingKind) -> windows_core::HRESULT,
    pub SetKind: unsafe extern "system" fn(*mut core::ffi::c_void, ChatConversationThreadingKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatItem, IChatItem_Vtbl, 0x8751d000_ceb1_4243_b803_15d45a1dd428);
impl core::ops::Deref for IChatItem {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IChatItem, windows_core::IUnknown, windows_core::IInspectable);
impl IChatItem {
    pub fn ItemKind(&self) -> windows_core::Result<ChatItemKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ItemKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for IChatItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ItemKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ChatItemKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatMessage, IChatMessage_Vtbl, 0x4b39052a_1142_5089_76da_f2db3d17cd05);
impl windows_core::RuntimeType for IChatMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Attachments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Attachments: usize,
    pub Body: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetBody: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub From: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsForwardingDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsIncoming: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsRead: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub LocalTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub NetworkTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Recipients: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Recipients: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RecipientSendStatuses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RecipientSendStatuses: usize,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ChatMessageStatus) -> windows_core::HRESULT,
    pub Subject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TransportFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TransportId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetTransportId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatMessage2, IChatMessage2_Vtbl, 0x86668332_543f_49f5_ac71_6c2afc6565fd);
impl windows_core::RuntimeType for IChatMessage2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessage2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EstimatedDownloadSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub SetEstimatedDownloadSize: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub SetFrom: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsAutoReply: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsAutoReply: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SetIsForwardingDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsReplyDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsIncoming: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SetIsRead: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsSeen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsSeen: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsSimMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetLocalTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub MessageKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ChatMessageKind) -> windows_core::HRESULT,
    pub SetMessageKind: unsafe extern "system" fn(*mut core::ffi::c_void, ChatMessageKind) -> windows_core::HRESULT,
    pub MessageOperatorKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ChatMessageOperatorKind) -> windows_core::HRESULT,
    pub SetMessageOperatorKind: unsafe extern "system" fn(*mut core::ffi::c_void, ChatMessageOperatorKind) -> windows_core::HRESULT,
    pub SetNetworkTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub IsReceivedDuringQuietHours: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsReceivedDuringQuietHours: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, ChatMessageStatus) -> windows_core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ShouldSuppressNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetShouldSuppressNotification: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ThreadingInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetThreadingInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RecipientsDeliveryInfos: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RecipientsDeliveryInfos: usize,
}
windows_core::imp::define_interface!(IChatMessage3, IChatMessage3_Vtbl, 0x74eb2fb0_3ba7_459f_8e0b_e8af0febd9ad);
impl windows_core::RuntimeType for IChatMessage3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessage3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RemoteId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatMessage4, IChatMessage4_Vtbl, 0x2d144b0f_d2bf_460c_aa68_6d3f8483c9bf);
impl windows_core::RuntimeType for IChatMessage4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessage4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SyncId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetSyncId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatMessageAttachment, IChatMessageAttachment_Vtbl, 0xc7c4fd74_bf63_58eb_508c_8b863ff16b67);
impl windows_core::RuntimeType for IChatMessageAttachment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessageAttachment_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub DataStreamReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DataStreamReference: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetDataStreamReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetDataStreamReference: usize,
    pub GroupId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetGroupId: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub MimeType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetMimeType: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatMessageAttachment2, IChatMessageAttachment2_Vtbl, 0x5ed99270_7dd1_4a87_a8ce_acdd87d80dc8);
impl windows_core::RuntimeType for IChatMessageAttachment2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessageAttachment2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetThumbnail: usize,
    pub TransferProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetTransferProgress: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub OriginalFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetOriginalFileName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatMessageAttachmentFactory, IChatMessageAttachmentFactory_Vtbl, 0x205852a2_a356_5b71_6ca9_66c985b7d0d5);
impl windows_core::RuntimeType for IChatMessageAttachmentFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessageAttachmentFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateChatMessageAttachment: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateChatMessageAttachment: usize,
}
windows_core::imp::define_interface!(IChatMessageBlockingStatic, IChatMessageBlockingStatic_Vtbl, 0xf6b9a380_cdea_11e4_8830_0800200c9a66);
impl windows_core::RuntimeType for IChatMessageBlockingStatic {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessageBlockingStatic_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MarkMessageAsBlockedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatMessageChange, IChatMessageChange_Vtbl, 0x1c18c355_421e_54b8_6d38_6b3a6c82fccc);
impl windows_core::RuntimeType for IChatMessageChange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessageChange_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ChangeType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ChatMessageChangeType) -> windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatMessageChangeReader, IChatMessageChangeReader_Vtbl, 0x14267020_28ce_5f26_7b05_9a5c7cce87ca);
impl windows_core::RuntimeType for IChatMessageChangeReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessageChangeReader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AcceptChanges: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AcceptChangesThrough: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
}
windows_core::imp::define_interface!(IChatMessageChangeTracker, IChatMessageChangeTracker_Vtbl, 0x60b7f066_70a0_5224_508c_242ef7c1d06f);
impl windows_core::RuntimeType for IChatMessageChangeTracker {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessageChangeTracker_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetChangeReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatMessageChangedDeferral, IChatMessageChangedDeferral_Vtbl, 0xfbc6b30c_788c_4dcc_ace7_6282382968cf);
impl windows_core::RuntimeType for IChatMessageChangedDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessageChangedDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatMessageChangedEventArgs, IChatMessageChangedEventArgs_Vtbl, 0xb6b73e2d_691c_4edf_8660_6eb9896892e3);
impl windows_core::RuntimeType for IChatMessageChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessageChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatMessageManager2Statics, IChatMessageManager2Statics_Vtbl, 0x1d45390f_9f4f_4e35_964e_1b9ca61ac044);
impl windows_core::RuntimeType for IChatMessageManager2Statics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessageManager2Statics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RegisterTransportAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTransportAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatMessageManagerStatic, IChatMessageManagerStatic_Vtbl, 0xf15c60f7_d5e8_5e92_556d_e03b60253104);
impl windows_core::RuntimeType for IChatMessageManagerStatic {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessageManagerStatic_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTransportsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTransportsAsync: usize,
    pub RequestStoreAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowComposeSmsMessageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowSmsSettings: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatMessageManagerStatics3, IChatMessageManagerStatics3_Vtbl, 0x208b830d_6755_48cc_9ab3_fd03c463fc92);
impl windows_core::RuntimeType for IChatMessageManagerStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessageManagerStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestSyncManagerAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatMessageNotificationTriggerDetails, IChatMessageNotificationTriggerDetails_Vtbl, 0xfd344dfb_3063_4e17_8586_c6c08262e6c0);
impl windows_core::RuntimeType for IChatMessageNotificationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessageNotificationTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ChatMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatMessageNotificationTriggerDetails2, IChatMessageNotificationTriggerDetails2_Vtbl, 0x6bb522e0_aa07_4fd1_9471_77934fb75ee6);
impl windows_core::RuntimeType for IChatMessageNotificationTriggerDetails2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessageNotificationTriggerDetails2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ShouldDisplayToast: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ShouldUpdateDetailText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ShouldUpdateBadge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ShouldUpdateActionCenter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatMessageReader, IChatMessageReader_Vtbl, 0xb6ea78ce_4489_56f9_76aa_e204682514cf);
impl windows_core::RuntimeType for IChatMessageReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessageReader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
}
windows_core::imp::define_interface!(IChatMessageReader2, IChatMessageReader2_Vtbl, 0x89643683_64bb_470d_9df4_0de8be1a05bf);
impl windows_core::RuntimeType for IChatMessageReader2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessageReader2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchWithCountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchWithCountAsync: usize,
}
windows_core::imp::define_interface!(IChatMessageStore, IChatMessageStore_Vtbl, 0x31f2fd01_ccf6_580b_4976_0a07dd5d3b47);
impl windows_core::RuntimeType for IChatMessageStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessageStore_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ChangeTracker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteMessageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DownloadMessageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMessageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMessageReader1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMessageReader2: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MarkMessageReadAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RetrySendMessageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendMessageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ValidateMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MessageChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveMessageChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatMessageStore2, IChatMessageStore2_Vtbl, 0xad4dc4ee_3ad4_491b_b311_abdf9bb22768);
impl windows_core::RuntimeType for IChatMessageStore2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessageStore2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ForwardMessageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ForwardMessageAsync: usize,
    pub GetConversationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConversationForTransportsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConversationForTransportsAsync: usize,
    pub GetConversationFromThreadingInfoAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetConversationReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConversationForTransportsReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConversationForTransportsReader: usize,
    pub GetMessageByRemoteIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetUnseenCountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUnseenCountForTransportsReaderAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUnseenCountForTransportsReaderAsync: usize,
    pub MarkAsSeenAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MarkAsSeenForTransportsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MarkAsSeenForTransportsAsync: usize,
    pub GetSearchReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SaveMessageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryCancelDownloadMessageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryCancelSendMessageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StoreChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStoreChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatMessageStore3, IChatMessageStore3_Vtbl, 0x9adbbb09_4345_4ec1_8b74_b7338243719c);
impl windows_core::RuntimeType for IChatMessageStore3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessageStore3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetMessageBySyncIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatMessageStoreChangedEventArgs, IChatMessageStoreChangedEventArgs_Vtbl, 0x65c66fac_fe8c_46d4_9119_57b8410311d5);
impl windows_core::RuntimeType for IChatMessageStoreChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessageStoreChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ChatStoreChangedEventKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatMessageTransport, IChatMessageTransport_Vtbl, 0x63a9dbf8_e6b3_5c9a_5f85_d47925b9bd18);
impl windows_core::RuntimeType for IChatMessageTransport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessageTransport_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsAppSetAsNotificationProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsActive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub TransportFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TransportId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RequestSetAsNotificationProviderAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatMessageTransport2, IChatMessageTransport2_Vtbl, 0x90a75622_d84a_4c22_a94d_544444edc8a1);
impl windows_core::RuntimeType for IChatMessageTransport2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessageTransport2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Configuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TransportKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ChatMessageTransportKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatMessageTransportConfiguration, IChatMessageTransportConfiguration_Vtbl, 0x879ff725_1a08_4aca_a075_3355126312e6);
impl windows_core::RuntimeType for IChatMessageTransportConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessageTransportConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MaxAttachmentCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MaxMessageSizeInKilobytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MaxRecipientCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub SupportedVideoFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SupportedVideoFormat: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ExtendedProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExtendedProperties: usize,
}
windows_core::imp::define_interface!(IChatMessageValidationResult, IChatMessageValidationResult_Vtbl, 0x25e93a03_28ec_5889_569b_7e486b126f18);
impl windows_core::RuntimeType for IChatMessageValidationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessageValidationResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MaxPartCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PartCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemainingCharacterCountInPart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ChatMessageValidationStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatQueryOptions, IChatQueryOptions_Vtbl, 0x2fd364a6_bf36_42f7_b7e7_923c0aabfe16);
impl windows_core::RuntimeType for IChatQueryOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatQueryOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SearchString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetSearchString: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatRecipientDeliveryInfo, IChatRecipientDeliveryInfo_Vtbl, 0xffc7b2a2_283c_4c0a_8a0e_8c33bdbf0545);
impl windows_core::RuntimeType for IChatRecipientDeliveryInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatRecipientDeliveryInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TransportAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetTransportAddress: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DeliveryTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDeliveryTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReadTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetReadTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TransportErrorCodeCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ChatTransportErrorCodeCategory) -> windows_core::HRESULT,
    pub TransportInterpretedErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ChatTransportInterpretedErrorCode) -> windows_core::HRESULT,
    pub TransportErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsErrorPermanent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ChatMessageStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatSearchReader, IChatSearchReader_Vtbl, 0x4665fe49_9020_4752_980d_39612325f589);
impl windows_core::RuntimeType for IChatSearchReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatSearchReader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchWithCountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchWithCountAsync: usize,
}
windows_core::imp::define_interface!(IChatSyncConfiguration, IChatSyncConfiguration_Vtbl, 0x09f869b2_69f4_4aff_82b6_06992ff402d2);
impl windows_core::RuntimeType for IChatSyncConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatSyncConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSyncEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsSyncEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub RestoreHistorySpan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ChatRestoreHistorySpan) -> windows_core::HRESULT,
    pub SetRestoreHistorySpan: unsafe extern "system" fn(*mut core::ffi::c_void, ChatRestoreHistorySpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatSyncManager, IChatSyncManager_Vtbl, 0x7ba52c63_2650_486f_b4b4_6bd9d3d63c84);
impl windows_core::RuntimeType for IChatSyncManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatSyncManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Configuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub AssociateAccountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    AssociateAccountAsync: usize,
    pub UnassociateAccountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub IsAccountAssociated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    IsAccountAssociated: usize,
    pub StartSync: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetConfigurationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRcsEndUserMessage, IRcsEndUserMessage_Vtbl, 0xd7cda5eb_cbd7_4f3b_8526_b506dec35c53);
impl windows_core::RuntimeType for IRcsEndUserMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRcsEndUserMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TransportId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsPinRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Actions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Actions: usize,
    pub SendResponseAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendResponseWithPinAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRcsEndUserMessageAction, IRcsEndUserMessageAction_Vtbl, 0x92378737_9b42_46d3_9d5e_3c1b2dae7cb8);
impl windows_core::RuntimeType for IRcsEndUserMessageAction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRcsEndUserMessageAction_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRcsEndUserMessageAvailableEventArgs, IRcsEndUserMessageAvailableEventArgs_Vtbl, 0x2d45ae01_3f89_41ea_9702_9e9ed411aa98);
impl windows_core::RuntimeType for IRcsEndUserMessageAvailableEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRcsEndUserMessageAvailableEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsMessageAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRcsEndUserMessageAvailableTriggerDetails, IRcsEndUserMessageAvailableTriggerDetails_Vtbl, 0x5b97742d_351f_4692_b41e_1b035dc18986);
impl windows_core::RuntimeType for IRcsEndUserMessageAvailableTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRcsEndUserMessageAvailableTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRcsEndUserMessageManager, IRcsEndUserMessageManager_Vtbl, 0x3054ae5a_4d1f_4b59_9433_126c734e86a6);
impl windows_core::RuntimeType for IRcsEndUserMessageManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRcsEndUserMessageManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MessageAvailableChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveMessageAvailableChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRcsManagerStatics, IRcsManagerStatics_Vtbl, 0x7d270ac5_0abd_4f31_9b99_a59e71a7b731);
impl windows_core::RuntimeType for IRcsManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRcsManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetEndUserMessageManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTransportsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTransportsAsync: usize,
    pub GetTransportAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LeaveConversationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRcsManagerStatics2, IRcsManagerStatics2_Vtbl, 0xcd49ad18_ad8a_42aa_8eeb_a798a8808959);
impl windows_core::RuntimeType for IRcsManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRcsManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TransportListChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveTransportListChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRcsServiceKindSupportedChangedEventArgs, IRcsServiceKindSupportedChangedEventArgs_Vtbl, 0xf47ea244_e783_4866_b3a7_4e5ccf023070);
impl windows_core::RuntimeType for IRcsServiceKindSupportedChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRcsServiceKindSupportedChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ServiceKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RcsServiceKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRcsTransport, IRcsTransport_Vtbl, 0xfea34759_f37c_4319_8546_ec84d21d30ff);
impl windows_core::RuntimeType for IRcsTransport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRcsTransport_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ExtendedProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExtendedProperties: usize,
    pub IsActive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub TransportFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TransportId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Configuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsStoreAndForwardEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, RcsServiceKind, *mut bool) -> windows_core::HRESULT,
    pub IsServiceKindSupported: unsafe extern "system" fn(*mut core::ffi::c_void, RcsServiceKind, *mut bool) -> windows_core::HRESULT,
    pub ServiceKindSupportedChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveServiceKindSupportedChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRcsTransportConfiguration, IRcsTransportConfiguration_Vtbl, 0x1fccb102_2472_4bb9_9988_c1211c83e8a9);
impl windows_core::RuntimeType for IRcsTransportConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRcsTransportConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MaxAttachmentCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MaxMessageSizeInKilobytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MaxGroupMessageSizeInKilobytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MaxRecipientCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MaxFileSizeInKilobytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub WarningFileSizeInKilobytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteParticipantComposingChangedEventArgs, IRemoteParticipantComposingChangedEventArgs_Vtbl, 0x1ec045a7_cfc9_45c9_9876_449f2bc180f5);
impl windows_core::RuntimeType for IRemoteParticipantComposingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteParticipantComposingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TransportId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ParticipantAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsComposing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChatCapabilities(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChatCapabilities, windows_core::IUnknown, windows_core::IInspectable);
impl ChatCapabilities {
    pub fn IsOnline(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOnline)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsChatCapable(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsChatCapable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsFileTransferCapable(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFileTransferCapable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsGeoLocationPushCapable(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsGeoLocationPushCapable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsIntegratedMessagingCapable(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsIntegratedMessagingCapable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ChatCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChatCapabilities>();
}
unsafe impl windows_core::Interface for ChatCapabilities {
    type Vtable = IChatCapabilities_Vtbl;
    const IID: windows_core::GUID = <IChatCapabilities as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChatCapabilities {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatCapabilities";
}
unsafe impl Send for ChatCapabilities {}
unsafe impl Sync for ChatCapabilities {}
pub struct ChatCapabilitiesManager;
impl ChatCapabilitiesManager {
    pub fn GetCachedCapabilitiesAsync(address: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>> {
        Self::IChatCapabilitiesManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCachedCapabilitiesAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(address), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetCapabilitiesFromNetworkAsync(address: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>> {
        Self::IChatCapabilitiesManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCapabilitiesFromNetworkAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(address), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetCachedCapabilitiesForTransportAsync(address: &windows_core::HSTRING, transportid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>> {
        Self::IChatCapabilitiesManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCachedCapabilitiesForTransportAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(address), core::mem::transmute_copy(transportid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetCapabilitiesFromNetworkForTransportAsync(address: &windows_core::HSTRING, transportid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>> {
        Self::IChatCapabilitiesManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCapabilitiesFromNetworkForTransportAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(address), core::mem::transmute_copy(transportid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IChatCapabilitiesManagerStatics<R, F: FnOnce(&IChatCapabilitiesManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ChatCapabilitiesManager, IChatCapabilitiesManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IChatCapabilitiesManagerStatics2<R, F: FnOnce(&IChatCapabilitiesManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ChatCapabilitiesManager, IChatCapabilitiesManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for ChatCapabilitiesManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatCapabilitiesManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChatConversation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChatConversation, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ChatConversation, IChatItem);
impl ChatConversation {
    pub fn HasUnreadMessages(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasUnreadMessages)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Subject(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Subject)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSubject(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSubject)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsConversationMuted(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsConversationMuted)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsConversationMuted(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsConversationMuted)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MostRecentMessageId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MostRecentMessageId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Participants(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Participants)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ThreadingInfo(&self) -> windows_core::Result<ChatConversationThreadingInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ThreadingInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetMessageReader(&self) -> windows_core::Result<ChatMessageReader> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMessageReader)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MarkAllMessagesAsReadAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MarkAllMessagesAsReadAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MarkMessagesAsReadAsync(&self, value: super::super::Foundation::DateTime) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MarkMessagesAsReadAsync)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SaveAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NotifyLocalParticipantComposing(&self, transportid: &windows_core::HSTRING, participantaddress: &windows_core::HSTRING, iscomposing: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NotifyLocalParticipantComposing)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(transportid), core::mem::transmute_copy(participantaddress), iscomposing).ok() }
    }
    pub fn NotifyRemoteParticipantComposing(&self, transportid: &windows_core::HSTRING, participantaddress: &windows_core::HSTRING, iscomposing: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).NotifyRemoteParticipantComposing)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(transportid), core::mem::transmute_copy(participantaddress), iscomposing).ok() }
    }
    pub fn RemoteParticipantComposingChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ChatConversation, RemoteParticipantComposingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteParticipantComposingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRemoteParticipantComposingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRemoteParticipantComposingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CanModifyParticipants(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IChatConversation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanModifyParticipants)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanModifyParticipants(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IChatConversation2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanModifyParticipants)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ItemKind(&self) -> windows_core::Result<ChatItemKind> {
        let this = &windows_core::Interface::cast::<IChatItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ItemKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ChatConversation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChatConversation>();
}
unsafe impl windows_core::Interface for ChatConversation {
    type Vtable = IChatConversation_Vtbl;
    const IID: windows_core::GUID = <IChatConversation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChatConversation {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatConversation";
}
unsafe impl Send for ChatConversation {}
unsafe impl Sync for ChatConversation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChatConversationReader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChatConversationReader, windows_core::IUnknown, windows_core::IInspectable);
impl ChatConversationReader {
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatConversation>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadBatchAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchWithCountAsync(&self, count: i32) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatConversation>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadBatchWithCountAsync)(windows_core::Interface::as_raw(this), count, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ChatConversationReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChatConversationReader>();
}
unsafe impl windows_core::Interface for ChatConversationReader {
    type Vtable = IChatConversationReader_Vtbl;
    const IID: windows_core::GUID = <IChatConversationReader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChatConversationReader {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatConversationReader";
}
unsafe impl Send for ChatConversationReader {}
unsafe impl Sync for ChatConversationReader {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChatConversationThreadingInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChatConversationThreadingInfo, windows_core::IUnknown, windows_core::IInspectable);
impl ChatConversationThreadingInfo {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ChatConversationThreadingInfo, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ContactId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContactId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContactId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Custom(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Custom)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCustom(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCustom)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ConversationId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConversationId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetConversationId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetConversationId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Participants(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Participants)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ChatConversationThreadingKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKind(&self, value: ChatConversationThreadingKind) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetKind)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for ChatConversationThreadingInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChatConversationThreadingInfo>();
}
unsafe impl windows_core::Interface for ChatConversationThreadingInfo {
    type Vtable = IChatConversationThreadingInfo_Vtbl;
    const IID: windows_core::GUID = <IChatConversationThreadingInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChatConversationThreadingInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatConversationThreadingInfo";
}
unsafe impl Send for ChatConversationThreadingInfo {}
unsafe impl Sync for ChatConversationThreadingInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChatMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChatMessage, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ChatMessage, IChatItem);
impl ChatMessage {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ChatMessage, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ItemKind(&self) -> windows_core::Result<ChatItemKind> {
        let this = &windows_core::Interface::cast::<IChatItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ItemKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Attachments(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<ChatMessageAttachment>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Attachments)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Body(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Body)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBody(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBody)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn From(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).From)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsForwardingDisabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsForwardingDisabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsIncoming(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsIncoming)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsRead(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRead)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LocalTimestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalTimestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NetworkTimestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkTimestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Recipients(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Recipients)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RecipientSendStatuses(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, ChatMessageStatus>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecipientSendStatuses)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Status(&self) -> windows_core::Result<ChatMessageStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Subject(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Subject)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TransportFriendlyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransportFriendlyName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TransportId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransportId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTransportId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTransportId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn EstimatedDownloadSize(&self) -> windows_core::Result<u64> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EstimatedDownloadSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEstimatedDownloadSize(&self, value: u64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetEstimatedDownloadSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetFrom(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFrom)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsAutoReply(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAutoReply)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsAutoReply(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsAutoReply)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetIsForwardingDisabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsForwardingDisabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsReplyDisabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReplyDisabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsIncoming(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsIncoming)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetIsRead(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsRead)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsSeen(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSeen)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsSeen(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsSeen)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsSimMessage(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSimMessage)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLocalTimestamp(&self, value: super::super::Foundation::DateTime) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLocalTimestamp)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MessageKind(&self) -> windows_core::Result<ChatMessageKind> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMessageKind(&self, value: ChatMessageKind) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMessageKind)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MessageOperatorKind(&self) -> windows_core::Result<ChatMessageOperatorKind> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageOperatorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMessageOperatorKind(&self, value: ChatMessageOperatorKind) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMessageOperatorKind)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetNetworkTimestamp(&self, value: super::super::Foundation::DateTime) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNetworkTimestamp)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsReceivedDuringQuietHours(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReceivedDuringQuietHours)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsReceivedDuringQuietHours(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsReceivedDuringQuietHours)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetRemoteId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRemoteId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn SetStatus(&self, value: ChatMessageStatus) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStatus)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetSubject(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSubject)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ShouldSuppressNotification(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShouldSuppressNotification)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetShouldSuppressNotification(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetShouldSuppressNotification)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ThreadingInfo(&self) -> windows_core::Result<ChatConversationThreadingInfo> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ThreadingInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetThreadingInfo<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ChatConversationThreadingInfo>,
    {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetThreadingInfo)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RecipientsDeliveryInfos(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<ChatRecipientDeliveryInfo>> {
        let this = &windows_core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecipientsDeliveryInfos)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoteId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IChatMessage3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SyncId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IChatMessage4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SyncId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSyncId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IChatMessage4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSyncId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for ChatMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChatMessage>();
}
unsafe impl windows_core::Interface for ChatMessage {
    type Vtable = IChatMessage_Vtbl;
    const IID: windows_core::GUID = <IChatMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChatMessage {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessage";
}
unsafe impl Send for ChatMessage {}
unsafe impl Sync for ChatMessage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChatMessageAttachment(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChatMessageAttachment, windows_core::IUnknown, windows_core::IInspectable);
impl ChatMessageAttachment {
    #[cfg(feature = "Storage_Streams")]
    pub fn DataStreamReference(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataStreamReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetDataStreamReference<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDataStreamReference)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn GroupId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GroupId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetGroupId(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetGroupId)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MimeType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MimeType)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMimeType(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMimeType)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &windows_core::Interface::cast::<IChatMessageAttachment2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Thumbnail)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetThumbnail<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = &windows_core::Interface::cast::<IChatMessageAttachment2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetThumbnail)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn TransferProgress(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IChatMessageAttachment2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransferProgress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTransferProgress(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IChatMessageAttachment2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTransferProgress)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OriginalFileName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IChatMessageAttachment2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalFileName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOriginalFileName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IChatMessageAttachment2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOriginalFileName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateChatMessageAttachment<P0>(mimetype: &windows_core::HSTRING, datastreamreference: P0) -> windows_core::Result<ChatMessageAttachment>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        Self::IChatMessageAttachmentFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateChatMessageAttachment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(mimetype), datastreamreference.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IChatMessageAttachmentFactory<R, F: FnOnce(&IChatMessageAttachmentFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ChatMessageAttachment, IChatMessageAttachmentFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ChatMessageAttachment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChatMessageAttachment>();
}
unsafe impl windows_core::Interface for ChatMessageAttachment {
    type Vtable = IChatMessageAttachment_Vtbl;
    const IID: windows_core::GUID = <IChatMessageAttachment as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChatMessageAttachment {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageAttachment";
}
unsafe impl Send for ChatMessageAttachment {}
unsafe impl Sync for ChatMessageAttachment {}
pub struct ChatMessageBlocking;
impl ChatMessageBlocking {
    pub fn MarkMessageAsBlockedAsync(localchatmessageid: &windows_core::HSTRING, blocked: bool) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::IChatMessageBlockingStatic(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MarkMessageAsBlockedAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(localchatmessageid), blocked, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IChatMessageBlockingStatic<R, F: FnOnce(&IChatMessageBlockingStatic) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ChatMessageBlocking, IChatMessageBlockingStatic> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for ChatMessageBlocking {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageBlocking";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChatMessageChange(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChatMessageChange, windows_core::IUnknown, windows_core::IInspectable);
impl ChatMessageChange {
    pub fn ChangeType(&self) -> windows_core::Result<ChatMessageChangeType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChangeType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Message(&self) -> windows_core::Result<ChatMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Message)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ChatMessageChange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChatMessageChange>();
}
unsafe impl windows_core::Interface for ChatMessageChange {
    type Vtable = IChatMessageChange_Vtbl;
    const IID: windows_core::GUID = <IChatMessageChange as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChatMessageChange {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageChange";
}
unsafe impl Send for ChatMessageChange {}
unsafe impl Sync for ChatMessageChange {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChatMessageChangeReader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChatMessageChangeReader, windows_core::IUnknown, windows_core::IInspectable);
impl ChatMessageChangeReader {
    pub fn AcceptChanges(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AcceptChanges)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn AcceptChangesThrough<P0>(&self, lastchangetoacknowledge: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ChatMessageChange>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AcceptChangesThrough)(windows_core::Interface::as_raw(this), lastchangetoacknowledge.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessageChange>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadBatchAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ChatMessageChangeReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChatMessageChangeReader>();
}
unsafe impl windows_core::Interface for ChatMessageChangeReader {
    type Vtable = IChatMessageChangeReader_Vtbl;
    const IID: windows_core::GUID = <IChatMessageChangeReader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChatMessageChangeReader {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageChangeReader";
}
unsafe impl Send for ChatMessageChangeReader {}
unsafe impl Sync for ChatMessageChangeReader {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChatMessageChangeTracker(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChatMessageChangeTracker, windows_core::IUnknown, windows_core::IInspectable);
impl ChatMessageChangeTracker {
    pub fn Enable(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Enable)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetChangeReader(&self) -> windows_core::Result<ChatMessageChangeReader> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChangeReader)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Reset(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Reset)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for ChatMessageChangeTracker {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChatMessageChangeTracker>();
}
unsafe impl windows_core::Interface for ChatMessageChangeTracker {
    type Vtable = IChatMessageChangeTracker_Vtbl;
    const IID: windows_core::GUID = <IChatMessageChangeTracker as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChatMessageChangeTracker {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageChangeTracker";
}
unsafe impl Send for ChatMessageChangeTracker {}
unsafe impl Sync for ChatMessageChangeTracker {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChatMessageChangedDeferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChatMessageChangedDeferral, windows_core::IUnknown, windows_core::IInspectable);
impl ChatMessageChangedDeferral {
    pub fn Complete(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for ChatMessageChangedDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChatMessageChangedDeferral>();
}
unsafe impl windows_core::Interface for ChatMessageChangedDeferral {
    type Vtable = IChatMessageChangedDeferral_Vtbl;
    const IID: windows_core::GUID = <IChatMessageChangedDeferral as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChatMessageChangedDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageChangedDeferral";
}
unsafe impl Send for ChatMessageChangedDeferral {}
unsafe impl Sync for ChatMessageChangedDeferral {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChatMessageChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChatMessageChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ChatMessageChangedEventArgs {
    pub fn GetDeferral(&self) -> windows_core::Result<ChatMessageChangedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ChatMessageChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChatMessageChangedEventArgs>();
}
unsafe impl windows_core::Interface for ChatMessageChangedEventArgs {
    type Vtable = IChatMessageChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IChatMessageChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChatMessageChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageChangedEventArgs";
}
unsafe impl Send for ChatMessageChangedEventArgs {}
unsafe impl Sync for ChatMessageChangedEventArgs {}
pub struct ChatMessageManager;
impl ChatMessageManager {
    pub fn RegisterTransportAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        Self::IChatMessageManager2Statics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegisterTransportAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetTransportAsync(transportid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ChatMessageTransport>> {
        Self::IChatMessageManager2Statics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTransportAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(transportid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTransportsAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessageTransport>>> {
        Self::IChatMessageManagerStatic(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTransportsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RequestStoreAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<ChatMessageStore>> {
        Self::IChatMessageManagerStatic(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestStoreAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ShowComposeSmsMessageAsync<P0>(message: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<ChatMessage>,
    {
        Self::IChatMessageManagerStatic(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowComposeSmsMessageAsync)(windows_core::Interface::as_raw(this), message.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ShowSmsSettings() -> windows_core::Result<()> {
        Self::IChatMessageManagerStatic(|this| unsafe { (windows_core::Interface::vtable(this).ShowSmsSettings)(windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn RequestSyncManagerAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<ChatSyncManager>> {
        Self::IChatMessageManagerStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestSyncManagerAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IChatMessageManager2Statics<R, F: FnOnce(&IChatMessageManager2Statics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ChatMessageManager, IChatMessageManager2Statics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IChatMessageManagerStatic<R, F: FnOnce(&IChatMessageManagerStatic) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ChatMessageManager, IChatMessageManagerStatic> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IChatMessageManagerStatics3<R, F: FnOnce(&IChatMessageManagerStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ChatMessageManager, IChatMessageManagerStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for ChatMessageManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChatMessageNotificationTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChatMessageNotificationTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl ChatMessageNotificationTriggerDetails {
    pub fn ChatMessage(&self) -> windows_core::Result<ChatMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChatMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ShouldDisplayToast(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IChatMessageNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShouldDisplayToast)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ShouldUpdateDetailText(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IChatMessageNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShouldUpdateDetailText)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ShouldUpdateBadge(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IChatMessageNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShouldUpdateBadge)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ShouldUpdateActionCenter(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IChatMessageNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShouldUpdateActionCenter)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ChatMessageNotificationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChatMessageNotificationTriggerDetails>();
}
unsafe impl windows_core::Interface for ChatMessageNotificationTriggerDetails {
    type Vtable = IChatMessageNotificationTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IChatMessageNotificationTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChatMessageNotificationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageNotificationTriggerDetails";
}
unsafe impl Send for ChatMessageNotificationTriggerDetails {}
unsafe impl Sync for ChatMessageNotificationTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChatMessageReader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChatMessageReader, windows_core::IUnknown, windows_core::IInspectable);
impl ChatMessageReader {
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessage>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadBatchAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchWithCountAsync(&self, count: i32) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessage>>> {
        let this = &windows_core::Interface::cast::<IChatMessageReader2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadBatchWithCountAsync)(windows_core::Interface::as_raw(this), count, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ChatMessageReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChatMessageReader>();
}
unsafe impl windows_core::Interface for ChatMessageReader {
    type Vtable = IChatMessageReader_Vtbl;
    const IID: windows_core::GUID = <IChatMessageReader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChatMessageReader {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageReader";
}
unsafe impl Send for ChatMessageReader {}
unsafe impl Sync for ChatMessageReader {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChatMessageStore(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChatMessageStore, windows_core::IUnknown, windows_core::IInspectable);
impl ChatMessageStore {
    pub fn ChangeTracker(&self) -> windows_core::Result<ChatMessageChangeTracker> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChangeTracker)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteMessageAsync(&self, localmessageid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteMessageAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(localmessageid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DownloadMessageAsync(&self, localchatmessageid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DownloadMessageAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(localchatmessageid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetMessageAsync(&self, localchatmessageid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ChatMessage>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMessageAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(localchatmessageid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetMessageReader1(&self) -> windows_core::Result<ChatMessageReader> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMessageReader1)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetMessageReader2(&self, recenttimelimit: super::super::Foundation::TimeSpan) -> windows_core::Result<ChatMessageReader> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMessageReader2)(windows_core::Interface::as_raw(this), recenttimelimit, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MarkMessageReadAsync(&self, localchatmessageid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MarkMessageReadAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(localchatmessageid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RetrySendMessageAsync(&self, localchatmessageid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RetrySendMessageAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(localchatmessageid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SendMessageAsync<P0>(&self, chatmessage: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<ChatMessage>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendMessageAsync)(windows_core::Interface::as_raw(this), chatmessage.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ValidateMessage<P0>(&self, chatmessage: P0) -> windows_core::Result<ChatMessageValidationResult>
    where
        P0: windows_core::Param<ChatMessage>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValidateMessage)(windows_core::Interface::as_raw(this), chatmessage.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MessageChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ChatMessageStore, ChatMessageChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMessageChanged(&self, value: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMessageChanged)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ForwardMessageAsync<P0>(&self, localchatmessageid: &windows_core::HSTRING, addresses: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ChatMessage>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = &windows_core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForwardMessageAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(localchatmessageid), addresses.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetConversationAsync(&self, conversationid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ChatConversation>> {
        let this = &windows_core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetConversationAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(conversationid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConversationForTransportsAsync<P0>(&self, conversationid: &windows_core::HSTRING, transportids: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ChatConversation>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = &windows_core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetConversationForTransportsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(conversationid), transportids.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetConversationFromThreadingInfoAsync<P0>(&self, threadinginfo: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ChatConversation>>
    where
        P0: windows_core::Param<ChatConversationThreadingInfo>,
    {
        let this = &windows_core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetConversationFromThreadingInfoAsync)(windows_core::Interface::as_raw(this), threadinginfo.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetConversationReader(&self) -> windows_core::Result<ChatConversationReader> {
        let this = &windows_core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetConversationReader)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConversationForTransportsReader<P0>(&self, transportids: P0) -> windows_core::Result<ChatConversationReader>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = &windows_core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetConversationForTransportsReader)(windows_core::Interface::as_raw(this), transportids.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetMessageByRemoteIdAsync(&self, transportid: &windows_core::HSTRING, remoteid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ChatMessage>> {
        let this = &windows_core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMessageByRemoteIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(transportid), core::mem::transmute_copy(remoteid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetUnseenCountAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<i32>> {
        let this = &windows_core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUnseenCountAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUnseenCountForTransportsReaderAsync<P0>(&self, transportids: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<i32>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = &windows_core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUnseenCountForTransportsReaderAsync)(windows_core::Interface::as_raw(this), transportids.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MarkAsSeenAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MarkAsSeenAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn MarkAsSeenForTransportsAsync<P0>(&self, transportids: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = &windows_core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MarkAsSeenForTransportsAsync)(windows_core::Interface::as_raw(this), transportids.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetSearchReader<P0>(&self, value: P0) -> windows_core::Result<ChatSearchReader>
    where
        P0: windows_core::Param<ChatQueryOptions>,
    {
        let this = &windows_core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSearchReader)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SaveMessageAsync<P0>(&self, chatmessage: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<ChatMessage>,
    {
        let this = &windows_core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveMessageAsync)(windows_core::Interface::as_raw(this), chatmessage.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryCancelDownloadMessageAsync(&self, localchatmessageid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCancelDownloadMessageAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(localchatmessageid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryCancelSendMessageAsync(&self, localchatmessageid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCancelSendMessageAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(localchatmessageid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StoreChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ChatMessageStore, ChatMessageStoreChangedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StoreChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStoreChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveStoreChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetMessageBySyncIdAsync(&self, syncid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ChatMessage>> {
        let this = &windows_core::Interface::cast::<IChatMessageStore3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMessageBySyncIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(syncid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ChatMessageStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChatMessageStore>();
}
unsafe impl windows_core::Interface for ChatMessageStore {
    type Vtable = IChatMessageStore_Vtbl;
    const IID: windows_core::GUID = <IChatMessageStore as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChatMessageStore {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageStore";
}
unsafe impl Send for ChatMessageStore {}
unsafe impl Sync for ChatMessageStore {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChatMessageStoreChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChatMessageStoreChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ChatMessageStoreChangedEventArgs {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<ChatStoreChangedEventKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ChatMessageStoreChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChatMessageStoreChangedEventArgs>();
}
unsafe impl windows_core::Interface for ChatMessageStoreChangedEventArgs {
    type Vtable = IChatMessageStoreChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IChatMessageStoreChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChatMessageStoreChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageStoreChangedEventArgs";
}
unsafe impl Send for ChatMessageStoreChangedEventArgs {}
unsafe impl Sync for ChatMessageStoreChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChatMessageTransport(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChatMessageTransport, windows_core::IUnknown, windows_core::IInspectable);
impl ChatMessageTransport {
    pub fn IsAppSetAsNotificationProvider(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAppSetAsNotificationProvider)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsActive(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsActive)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TransportFriendlyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransportFriendlyName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TransportId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransportId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestSetAsNotificationProviderAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestSetAsNotificationProviderAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Configuration(&self) -> windows_core::Result<ChatMessageTransportConfiguration> {
        let this = &windows_core::Interface::cast::<IChatMessageTransport2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Configuration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TransportKind(&self) -> windows_core::Result<ChatMessageTransportKind> {
        let this = &windows_core::Interface::cast::<IChatMessageTransport2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransportKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ChatMessageTransport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChatMessageTransport>();
}
unsafe impl windows_core::Interface for ChatMessageTransport {
    type Vtable = IChatMessageTransport_Vtbl;
    const IID: windows_core::GUID = <IChatMessageTransport as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChatMessageTransport {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageTransport";
}
unsafe impl Send for ChatMessageTransport {}
unsafe impl Sync for ChatMessageTransport {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChatMessageTransportConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChatMessageTransportConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl ChatMessageTransportConfiguration {
    pub fn MaxAttachmentCount(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxAttachmentCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxMessageSizeInKilobytes(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxMessageSizeInKilobytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxRecipientCount(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxRecipientCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SupportedVideoFormat(&self) -> windows_core::Result<super::super::Media::MediaProperties::MediaEncodingProfile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedVideoFormat)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ChatMessageTransportConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChatMessageTransportConfiguration>();
}
unsafe impl windows_core::Interface for ChatMessageTransportConfiguration {
    type Vtable = IChatMessageTransportConfiguration_Vtbl;
    const IID: windows_core::GUID = <IChatMessageTransportConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChatMessageTransportConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageTransportConfiguration";
}
unsafe impl Send for ChatMessageTransportConfiguration {}
unsafe impl Sync for ChatMessageTransportConfiguration {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChatMessageValidationResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChatMessageValidationResult, windows_core::IUnknown, windows_core::IInspectable);
impl ChatMessageValidationResult {
    pub fn MaxPartCount(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxPartCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PartCount(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PartCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemainingCharacterCountInPart(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemainingCharacterCountInPart)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Status(&self) -> windows_core::Result<ChatMessageValidationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ChatMessageValidationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChatMessageValidationResult>();
}
unsafe impl windows_core::Interface for ChatMessageValidationResult {
    type Vtable = IChatMessageValidationResult_Vtbl;
    const IID: windows_core::GUID = <IChatMessageValidationResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChatMessageValidationResult {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageValidationResult";
}
unsafe impl Send for ChatMessageValidationResult {}
unsafe impl Sync for ChatMessageValidationResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChatQueryOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChatQueryOptions, windows_core::IUnknown, windows_core::IInspectable);
impl ChatQueryOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ChatQueryOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SearchString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SearchString)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSearchString(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSearchString)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for ChatQueryOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChatQueryOptions>();
}
unsafe impl windows_core::Interface for ChatQueryOptions {
    type Vtable = IChatQueryOptions_Vtbl;
    const IID: windows_core::GUID = <IChatQueryOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChatQueryOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatQueryOptions";
}
unsafe impl Send for ChatQueryOptions {}
unsafe impl Sync for ChatQueryOptions {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChatRecipientDeliveryInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChatRecipientDeliveryInfo, windows_core::IUnknown, windows_core::IInspectable);
impl ChatRecipientDeliveryInfo {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ChatRecipientDeliveryInfo, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn TransportAddress(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransportAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTransportAddress(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTransportAddress)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn DeliveryTime(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeliveryTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDeliveryTime<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDeliveryTime)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ReadTime(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetReadTime<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReadTime)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn TransportErrorCodeCategory(&self) -> windows_core::Result<ChatTransportErrorCodeCategory> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransportErrorCodeCategory)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TransportInterpretedErrorCode(&self) -> windows_core::Result<ChatTransportInterpretedErrorCode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransportInterpretedErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TransportErrorCode(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransportErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsErrorPermanent(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsErrorPermanent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Status(&self) -> windows_core::Result<ChatMessageStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ChatRecipientDeliveryInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChatRecipientDeliveryInfo>();
}
unsafe impl windows_core::Interface for ChatRecipientDeliveryInfo {
    type Vtable = IChatRecipientDeliveryInfo_Vtbl;
    const IID: windows_core::GUID = <IChatRecipientDeliveryInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChatRecipientDeliveryInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatRecipientDeliveryInfo";
}
unsafe impl Send for ChatRecipientDeliveryInfo {}
unsafe impl Sync for ChatRecipientDeliveryInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChatSearchReader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChatSearchReader, windows_core::IUnknown, windows_core::IInspectable);
impl ChatSearchReader {
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IChatItem>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadBatchAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchWithCountAsync(&self, count: i32) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IChatItem>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadBatchWithCountAsync)(windows_core::Interface::as_raw(this), count, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ChatSearchReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChatSearchReader>();
}
unsafe impl windows_core::Interface for ChatSearchReader {
    type Vtable = IChatSearchReader_Vtbl;
    const IID: windows_core::GUID = <IChatSearchReader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChatSearchReader {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatSearchReader";
}
unsafe impl Send for ChatSearchReader {}
unsafe impl Sync for ChatSearchReader {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChatSyncConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChatSyncConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl ChatSyncConfiguration {
    pub fn IsSyncEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSyncEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsSyncEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsSyncEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RestoreHistorySpan(&self) -> windows_core::Result<ChatRestoreHistorySpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RestoreHistorySpan)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRestoreHistorySpan(&self, value: ChatRestoreHistorySpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRestoreHistorySpan)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for ChatSyncConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChatSyncConfiguration>();
}
unsafe impl windows_core::Interface for ChatSyncConfiguration {
    type Vtable = IChatSyncConfiguration_Vtbl;
    const IID: windows_core::GUID = <IChatSyncConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChatSyncConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatSyncConfiguration";
}
unsafe impl Send for ChatSyncConfiguration {}
unsafe impl Sync for ChatSyncConfiguration {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChatSyncManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChatSyncManager, windows_core::IUnknown, windows_core::IInspectable);
impl ChatSyncManager {
    pub fn Configuration(&self) -> windows_core::Result<ChatSyncConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Configuration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn AssociateAccountAsync<P0>(&self, webaccount: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Security::Credentials::WebAccount>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AssociateAccountAsync)(windows_core::Interface::as_raw(this), webaccount.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UnassociateAccountAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnassociateAccountAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn IsAccountAssociated<P0>(&self, webaccount: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::super::Security::Credentials::WebAccount>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAccountAssociated)(windows_core::Interface::as_raw(this), webaccount.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn StartSync(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StartSync)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetConfigurationAsync<P0>(&self, configuration: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<ChatSyncConfiguration>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetConfigurationAsync)(windows_core::Interface::as_raw(this), configuration.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ChatSyncManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChatSyncManager>();
}
unsafe impl windows_core::Interface for ChatSyncManager {
    type Vtable = IChatSyncManager_Vtbl;
    const IID: windows_core::GUID = <IChatSyncManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChatSyncManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatSyncManager";
}
unsafe impl Send for ChatSyncManager {}
unsafe impl Sync for ChatSyncManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RcsEndUserMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RcsEndUserMessage, windows_core::IUnknown, windows_core::IInspectable);
impl RcsEndUserMessage {
    pub fn TransportId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransportId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsPinRequired(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPinRequired)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Actions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<RcsEndUserMessageAction>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Actions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SendResponseAsync<P0>(&self, action: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<RcsEndUserMessageAction>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendResponseAsync)(windows_core::Interface::as_raw(this), action.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SendResponseWithPinAsync<P0>(&self, action: P0, pin: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<RcsEndUserMessageAction>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendResponseWithPinAsync)(windows_core::Interface::as_raw(this), action.param().abi(), core::mem::transmute_copy(pin), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RcsEndUserMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRcsEndUserMessage>();
}
unsafe impl windows_core::Interface for RcsEndUserMessage {
    type Vtable = IRcsEndUserMessage_Vtbl;
    const IID: windows_core::GUID = <IRcsEndUserMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RcsEndUserMessage {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsEndUserMessage";
}
unsafe impl Send for RcsEndUserMessage {}
unsafe impl Sync for RcsEndUserMessage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RcsEndUserMessageAction(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RcsEndUserMessageAction, windows_core::IUnknown, windows_core::IInspectable);
impl RcsEndUserMessageAction {
    pub fn Label(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Label)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RcsEndUserMessageAction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRcsEndUserMessageAction>();
}
unsafe impl windows_core::Interface for RcsEndUserMessageAction {
    type Vtable = IRcsEndUserMessageAction_Vtbl;
    const IID: windows_core::GUID = <IRcsEndUserMessageAction as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RcsEndUserMessageAction {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsEndUserMessageAction";
}
unsafe impl Send for RcsEndUserMessageAction {}
unsafe impl Sync for RcsEndUserMessageAction {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RcsEndUserMessageAvailableEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RcsEndUserMessageAvailableEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RcsEndUserMessageAvailableEventArgs {
    pub fn IsMessageAvailable(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMessageAvailable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Message(&self) -> windows_core::Result<RcsEndUserMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Message)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RcsEndUserMessageAvailableEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRcsEndUserMessageAvailableEventArgs>();
}
unsafe impl windows_core::Interface for RcsEndUserMessageAvailableEventArgs {
    type Vtable = IRcsEndUserMessageAvailableEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRcsEndUserMessageAvailableEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RcsEndUserMessageAvailableEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableEventArgs";
}
unsafe impl Send for RcsEndUserMessageAvailableEventArgs {}
unsafe impl Sync for RcsEndUserMessageAvailableEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RcsEndUserMessageAvailableTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RcsEndUserMessageAvailableTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl RcsEndUserMessageAvailableTriggerDetails {
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RcsEndUserMessageAvailableTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRcsEndUserMessageAvailableTriggerDetails>();
}
unsafe impl windows_core::Interface for RcsEndUserMessageAvailableTriggerDetails {
    type Vtable = IRcsEndUserMessageAvailableTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IRcsEndUserMessageAvailableTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RcsEndUserMessageAvailableTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableTriggerDetails";
}
unsafe impl Send for RcsEndUserMessageAvailableTriggerDetails {}
unsafe impl Sync for RcsEndUserMessageAvailableTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RcsEndUserMessageManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RcsEndUserMessageManager, windows_core::IUnknown, windows_core::IInspectable);
impl RcsEndUserMessageManager {
    pub fn MessageAvailableChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RcsEndUserMessageManager, RcsEndUserMessageAvailableEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageAvailableChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMessageAvailableChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMessageAvailableChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for RcsEndUserMessageManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRcsEndUserMessageManager>();
}
unsafe impl windows_core::Interface for RcsEndUserMessageManager {
    type Vtable = IRcsEndUserMessageManager_Vtbl;
    const IID: windows_core::GUID = <IRcsEndUserMessageManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RcsEndUserMessageManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsEndUserMessageManager";
}
unsafe impl Send for RcsEndUserMessageManager {}
unsafe impl Sync for RcsEndUserMessageManager {}
pub struct RcsManager;
impl RcsManager {
    pub fn GetEndUserMessageManager() -> windows_core::Result<RcsEndUserMessageManager> {
        Self::IRcsManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetEndUserMessageManager)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTransportsAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<RcsTransport>>> {
        Self::IRcsManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTransportsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetTransportAsync(transportid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<RcsTransport>> {
        Self::IRcsManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTransportAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(transportid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn LeaveConversationAsync<P0>(conversation: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<ChatConversation>,
    {
        Self::IRcsManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LeaveConversationAsync)(windows_core::Interface::as_raw(this), conversation.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TransportListChanged<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IRcsManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransportListChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveTransportListChanged(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IRcsManagerStatics2(|this| unsafe { (windows_core::Interface::vtable(this).RemoveTransportListChanged)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc(hidden)]
    pub fn IRcsManagerStatics<R, F: FnOnce(&IRcsManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RcsManager, IRcsManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IRcsManagerStatics2<R, F: FnOnce(&IRcsManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RcsManager, IRcsManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for RcsManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RcsServiceKindSupportedChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RcsServiceKindSupportedChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RcsServiceKindSupportedChangedEventArgs {
    pub fn ServiceKind(&self) -> windows_core::Result<RcsServiceKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for RcsServiceKindSupportedChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRcsServiceKindSupportedChangedEventArgs>();
}
unsafe impl windows_core::Interface for RcsServiceKindSupportedChangedEventArgs {
    type Vtable = IRcsServiceKindSupportedChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRcsServiceKindSupportedChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RcsServiceKindSupportedChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsServiceKindSupportedChangedEventArgs";
}
unsafe impl Send for RcsServiceKindSupportedChangedEventArgs {}
unsafe impl Sync for RcsServiceKindSupportedChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RcsTransport(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RcsTransport, windows_core::IUnknown, windows_core::IInspectable);
impl RcsTransport {
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsActive(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsActive)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TransportFriendlyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransportFriendlyName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TransportId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransportId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Configuration(&self) -> windows_core::Result<RcsTransportConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Configuration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsStoreAndForwardEnabled(&self, servicekind: RcsServiceKind) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStoreAndForwardEnabled)(windows_core::Interface::as_raw(this), servicekind, &mut result__).map(|| result__)
        }
    }
    pub fn IsServiceKindSupported(&self, servicekind: RcsServiceKind) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsServiceKindSupported)(windows_core::Interface::as_raw(this), servicekind, &mut result__).map(|| result__)
        }
    }
    pub fn ServiceKindSupportedChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<RcsTransport, RcsServiceKindSupportedChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceKindSupportedChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveServiceKindSupportedChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveServiceKindSupportedChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for RcsTransport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRcsTransport>();
}
unsafe impl windows_core::Interface for RcsTransport {
    type Vtable = IRcsTransport_Vtbl;
    const IID: windows_core::GUID = <IRcsTransport as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RcsTransport {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsTransport";
}
unsafe impl Send for RcsTransport {}
unsafe impl Sync for RcsTransport {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RcsTransportConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RcsTransportConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl RcsTransportConfiguration {
    pub fn MaxAttachmentCount(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxAttachmentCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxMessageSizeInKilobytes(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxMessageSizeInKilobytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxGroupMessageSizeInKilobytes(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxGroupMessageSizeInKilobytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxRecipientCount(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxRecipientCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxFileSizeInKilobytes(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxFileSizeInKilobytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn WarningFileSizeInKilobytes(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WarningFileSizeInKilobytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for RcsTransportConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRcsTransportConfiguration>();
}
unsafe impl windows_core::Interface for RcsTransportConfiguration {
    type Vtable = IRcsTransportConfiguration_Vtbl;
    const IID: windows_core::GUID = <IRcsTransportConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RcsTransportConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsTransportConfiguration";
}
unsafe impl Send for RcsTransportConfiguration {}
unsafe impl Sync for RcsTransportConfiguration {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RemoteParticipantComposingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteParticipantComposingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RemoteParticipantComposingChangedEventArgs {
    pub fn TransportId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransportId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParticipantAddress(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParticipantAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsComposing(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsComposing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for RemoteParticipantComposingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteParticipantComposingChangedEventArgs>();
}
unsafe impl windows_core::Interface for RemoteParticipantComposingChangedEventArgs {
    type Vtable = IRemoteParticipantComposingChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IRemoteParticipantComposingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteParticipantComposingChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RemoteParticipantComposingChangedEventArgs";
}
unsafe impl Send for RemoteParticipantComposingChangedEventArgs {}
unsafe impl Sync for RemoteParticipantComposingChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ChatConversationThreadingKind(pub i32);
impl ChatConversationThreadingKind {
    pub const Participants: Self = Self(0i32);
    pub const ContactId: Self = Self(1i32);
    pub const ConversationId: Self = Self(2i32);
    pub const Custom: Self = Self(3i32);
}
impl windows_core::TypeKind for ChatConversationThreadingKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ChatConversationThreadingKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ChatConversationThreadingKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ChatConversationThreadingKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatConversationThreadingKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ChatItemKind(pub i32);
impl ChatItemKind {
    pub const Message: Self = Self(0i32);
    pub const Conversation: Self = Self(1i32);
}
impl windows_core::TypeKind for ChatItemKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ChatItemKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ChatItemKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ChatItemKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatItemKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ChatMessageChangeType(pub i32);
impl ChatMessageChangeType {
    pub const MessageCreated: Self = Self(0i32);
    pub const MessageModified: Self = Self(1i32);
    pub const MessageDeleted: Self = Self(2i32);
    pub const ChangeTrackingLost: Self = Self(3i32);
}
impl windows_core::TypeKind for ChatMessageChangeType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ChatMessageChangeType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ChatMessageChangeType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ChatMessageChangeType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageChangeType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ChatMessageKind(pub i32);
impl ChatMessageKind {
    pub const Standard: Self = Self(0i32);
    pub const FileTransferRequest: Self = Self(1i32);
    pub const TransportCustom: Self = Self(2i32);
    pub const JoinedConversation: Self = Self(3i32);
    pub const LeftConversation: Self = Self(4i32);
    pub const OtherParticipantJoinedConversation: Self = Self(5i32);
    pub const OtherParticipantLeftConversation: Self = Self(6i32);
}
impl windows_core::TypeKind for ChatMessageKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ChatMessageKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ChatMessageKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ChatMessageKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ChatMessageOperatorKind(pub i32);
impl ChatMessageOperatorKind {
    pub const Unspecified: Self = Self(0i32);
    pub const Sms: Self = Self(1i32);
    pub const Mms: Self = Self(2i32);
    pub const Rcs: Self = Self(3i32);
}
impl windows_core::TypeKind for ChatMessageOperatorKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ChatMessageOperatorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ChatMessageOperatorKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ChatMessageOperatorKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageOperatorKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ChatMessageStatus(pub i32);
impl ChatMessageStatus {
    pub const Draft: Self = Self(0i32);
    pub const Sending: Self = Self(1i32);
    pub const Sent: Self = Self(2i32);
    pub const SendRetryNeeded: Self = Self(3i32);
    pub const SendFailed: Self = Self(4i32);
    pub const Received: Self = Self(5i32);
    pub const ReceiveDownloadNeeded: Self = Self(6i32);
    pub const ReceiveDownloadFailed: Self = Self(7i32);
    pub const ReceiveDownloading: Self = Self(8i32);
    pub const Deleted: Self = Self(9i32);
    pub const Declined: Self = Self(10i32);
    pub const Cancelled: Self = Self(11i32);
    pub const Recalled: Self = Self(12i32);
    pub const ReceiveRetryNeeded: Self = Self(13i32);
}
impl windows_core::TypeKind for ChatMessageStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ChatMessageStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ChatMessageStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ChatMessageStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ChatMessageTransportKind(pub i32);
impl ChatMessageTransportKind {
    pub const Text: Self = Self(0i32);
    pub const Untriaged: Self = Self(1i32);
    pub const Blocked: Self = Self(2i32);
    pub const Custom: Self = Self(3i32);
}
impl windows_core::TypeKind for ChatMessageTransportKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ChatMessageTransportKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ChatMessageTransportKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ChatMessageTransportKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageTransportKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ChatMessageValidationStatus(pub i32);
impl ChatMessageValidationStatus {
    pub const Valid: Self = Self(0i32);
    pub const NoRecipients: Self = Self(1i32);
    pub const InvalidData: Self = Self(2i32);
    pub const MessageTooLarge: Self = Self(3i32);
    pub const TooManyRecipients: Self = Self(4i32);
    pub const TransportInactive: Self = Self(5i32);
    pub const TransportNotFound: Self = Self(6i32);
    pub const TooManyAttachments: Self = Self(7i32);
    pub const InvalidRecipients: Self = Self(8i32);
    pub const InvalidBody: Self = Self(9i32);
    pub const InvalidOther: Self = Self(10i32);
    pub const ValidWithLargeMessage: Self = Self(11i32);
    pub const VoiceRoamingRestriction: Self = Self(12i32);
    pub const DataRoamingRestriction: Self = Self(13i32);
}
impl windows_core::TypeKind for ChatMessageValidationStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ChatMessageValidationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ChatMessageValidationStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ChatMessageValidationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageValidationStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ChatRestoreHistorySpan(pub i32);
impl ChatRestoreHistorySpan {
    pub const LastMonth: Self = Self(0i32);
    pub const LastYear: Self = Self(1i32);
    pub const AnyTime: Self = Self(2i32);
}
impl windows_core::TypeKind for ChatRestoreHistorySpan {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ChatRestoreHistorySpan {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ChatRestoreHistorySpan").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ChatRestoreHistorySpan {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatRestoreHistorySpan;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ChatStoreChangedEventKind(pub i32);
impl ChatStoreChangedEventKind {
    pub const NotificationsMissed: Self = Self(0i32);
    pub const StoreModified: Self = Self(1i32);
    pub const MessageCreated: Self = Self(2i32);
    pub const MessageModified: Self = Self(3i32);
    pub const MessageDeleted: Self = Self(4i32);
    pub const ConversationModified: Self = Self(5i32);
    pub const ConversationDeleted: Self = Self(6i32);
    pub const ConversationTransportDeleted: Self = Self(7i32);
}
impl windows_core::TypeKind for ChatStoreChangedEventKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ChatStoreChangedEventKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ChatStoreChangedEventKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ChatStoreChangedEventKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatStoreChangedEventKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ChatTransportErrorCodeCategory(pub i32);
impl ChatTransportErrorCodeCategory {
    pub const None: Self = Self(0i32);
    pub const Http: Self = Self(1i32);
    pub const Network: Self = Self(2i32);
    pub const MmsServer: Self = Self(3i32);
}
impl windows_core::TypeKind for ChatTransportErrorCodeCategory {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ChatTransportErrorCodeCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ChatTransportErrorCodeCategory").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ChatTransportErrorCodeCategory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatTransportErrorCodeCategory;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ChatTransportInterpretedErrorCode(pub i32);
impl ChatTransportInterpretedErrorCode {
    pub const None: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const InvalidRecipientAddress: Self = Self(2i32);
    pub const NetworkConnectivity: Self = Self(3i32);
    pub const ServiceDenied: Self = Self(4i32);
    pub const Timeout: Self = Self(5i32);
}
impl windows_core::TypeKind for ChatTransportInterpretedErrorCode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ChatTransportInterpretedErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ChatTransportInterpretedErrorCode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ChatTransportInterpretedErrorCode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatTransportInterpretedErrorCode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RcsServiceKind(pub i32);
impl RcsServiceKind {
    pub const Chat: Self = Self(0i32);
    pub const GroupChat: Self = Self(1i32);
    pub const FileTransfer: Self = Self(2i32);
    pub const Capability: Self = Self(3i32);
}
impl windows_core::TypeKind for RcsServiceKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RcsServiceKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RcsServiceKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for RcsServiceKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.RcsServiceKind;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
