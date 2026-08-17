#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
pub mod AppointmentsProvider;
#[cfg(feature = "ApplicationModel_Appointments_DataProvider")]
pub mod DataProvider;
windows_core::imp::define_interface!(IAppointment, IAppointment_Vtbl, 0xdd002f2f_2bdd_4076_90a3_22c275312965);
impl windows_core::RuntimeType for IAppointment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointment_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Location: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetLocation: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Subject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Details: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDetails: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Reminder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetReminder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Organizer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOrganizer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Invitees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Invitees: usize,
    pub Recurrence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRecurrence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BusyStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppointmentBusyStatus) -> windows_core::HRESULT,
    pub SetBusyStatus: unsafe extern "system" fn(*mut core::ffi::c_void, AppointmentBusyStatus) -> windows_core::HRESULT,
    pub AllDay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllDay: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Sensitivity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppointmentSensitivity) -> windows_core::HRESULT,
    pub SetSensitivity: unsafe extern "system" fn(*mut core::ffi::c_void, AppointmentSensitivity) -> windows_core::HRESULT,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointment2, IAppointment2_Vtbl, 0x5e85983c_540f_3452_9b5c_0dd7ad4c65a2);
impl windows_core::RuntimeType for IAppointment2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointment2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LocalId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub CalendarId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RoamingId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetRoamingId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub OriginalStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsResponseRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsResponseRequested: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AllowNewTimeProposal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowNewTimeProposal: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub OnlineMeetingLink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetOnlineMeetingLink: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReplyTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetReplyTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserResponse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppointmentParticipantResponse) -> windows_core::HRESULT,
    pub SetUserResponse: unsafe extern "system" fn(*mut core::ffi::c_void, AppointmentParticipantResponse) -> windows_core::HRESULT,
    pub HasInvitees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsCanceledMeeting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsCanceledMeeting: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsOrganizedByUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsOrganizedByUser: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointment3, IAppointment3_Vtbl, 0xbfcc45a9_8961_4991_934b_c48768e5a96c);
impl windows_core::RuntimeType for IAppointment3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointment3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ChangeNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub RemoteChangeNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub SetRemoteChangeNumber: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub DetailsKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppointmentDetailsKind) -> windows_core::HRESULT,
    pub SetDetailsKind: unsafe extern "system" fn(*mut core::ffi::c_void, AppointmentDetailsKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentCalendar, IAppointmentCalendar_Vtbl, 0x5273819d_8339_3d4f_a02f_64084452bb5d);
impl windows_core::RuntimeType for IAppointmentCalendar {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentCalendar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub DisplayColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    DisplayColor: usize,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub LocalId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsHidden: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub OtherAppReadAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppointmentCalendarOtherAppReadAccess) -> windows_core::HRESULT,
    pub SetOtherAppReadAccess: unsafe extern "system" fn(*mut core::ffi::c_void, AppointmentCalendarOtherAppReadAccess) -> windows_core::HRESULT,
    pub OtherAppWriteAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppointmentCalendarOtherAppWriteAccess) -> windows_core::HRESULT,
    pub SetOtherAppWriteAccess: unsafe extern "system" fn(*mut core::ffi::c_void, AppointmentCalendarOtherAppWriteAccess) -> windows_core::HRESULT,
    pub SourceDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SummaryCardView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppointmentSummaryCardView) -> windows_core::HRESULT,
    pub SetSummaryCardView: unsafe extern "system" fn(*mut core::ffi::c_void, AppointmentSummaryCardView) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppointmentsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppointmentsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppointmentsAsyncWithOptions: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, super::super::Foundation::TimeSpan, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppointmentsAsyncWithOptions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindExceptionsFromMasterAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindExceptionsFromMasterAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllInstancesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::super::Foundation::DateTime, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllInstancesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllInstancesAsyncWithOptions: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::super::Foundation::DateTime, super::super::Foundation::TimeSpan, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllInstancesAsyncWithOptions: usize,
    pub GetAppointmentAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAppointmentInstanceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindUnexpandedAppointmentsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindUnexpandedAppointmentsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindUnexpandedAppointmentsAsyncWithOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindUnexpandedAppointmentsAsyncWithOptions: usize,
    pub DeleteAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SaveAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteAppointmentAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteAppointmentInstanceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SaveAppointmentAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentCalendar2, IAppointmentCalendar2_Vtbl, 0x18e7e422_2467_4e1c_a459_d8a29303d092);
impl windows_core::RuntimeType for IAppointmentCalendar2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentCalendar2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SyncManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub SetDisplayColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetDisplayColor: usize,
    pub SetIsHidden: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub UserDataAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub CanCreateOrUpdateAppointments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCanCreateOrUpdateAppointments: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CanCancelMeetings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCanCancelMeetings: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CanForwardMeetings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCanForwardMeetings: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CanProposeNewTimeForMeetings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCanProposeNewTimeForMeetings: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CanUpdateMeetingResponses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCanUpdateMeetingResponses: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CanNotifyInvitees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCanNotifyInvitees: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub MustNofityInvitees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetMustNofityInvitees: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub TryCreateOrUpdateAppointmentAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryCancelMeetingAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TryForwardMeetingAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryForwardMeetingAsync: usize,
    pub TryProposeNewTimeForMeetingAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::DateTime, super::super::Foundation::TimeSpan, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryUpdateMeetingResponseAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, AppointmentParticipantResponse, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentCalendar3, IAppointmentCalendar3_Vtbl, 0xeb23d22b_a685_42ae_8495_b3119adb4167);
impl windows_core::RuntimeType for IAppointmentCalendar3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentCalendar3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RegisterSyncManagerAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentCalendarSyncManager, IAppointmentCalendarSyncManager_Vtbl, 0x2b21b3a0_4aff_4392_bc5f_5645ffcffb17);
impl windows_core::RuntimeType for IAppointmentCalendarSyncManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentCalendarSyncManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppointmentCalendarSyncStatus) -> windows_core::HRESULT,
    pub LastSuccessfulSyncTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub LastAttemptedSyncTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub SyncAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SyncStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSyncStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentCalendarSyncManager2, IAppointmentCalendarSyncManager2_Vtbl, 0x647528ad_0d29_4c7c_aaa7_bf996805537c);
impl windows_core::RuntimeType for IAppointmentCalendarSyncManager2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentCalendarSyncManager2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, AppointmentCalendarSyncStatus) -> windows_core::HRESULT,
    pub SetLastSuccessfulSyncTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub SetLastAttemptedSyncTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentConflictResult, IAppointmentConflictResult_Vtbl, 0xd5cdf0be_2f2f_3b7d_af0a_a7e20f3a46e3);
impl windows_core::RuntimeType for IAppointmentConflictResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentConflictResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppointmentConflictType) -> windows_core::HRESULT,
    pub Date: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentException, IAppointmentException_Vtbl, 0xa2076767_16f6_4bce_9f5a_8600b8019fcb);
impl windows_core::RuntimeType for IAppointmentException {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentException_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Appointment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ExceptionProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExceptionProperties: usize,
    pub IsDeleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentInvitee, IAppointmentInvitee_Vtbl, 0x13bf0796_9842_495b_b0e7_ef8f79c0701d);
impl windows_core::RuntimeType for IAppointmentInvitee {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentInvitee_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Role: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppointmentParticipantRole) -> windows_core::HRESULT,
    pub SetRole: unsafe extern "system" fn(*mut core::ffi::c_void, AppointmentParticipantRole) -> windows_core::HRESULT,
    pub Response: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppointmentParticipantResponse) -> windows_core::HRESULT,
    pub SetResponse: unsafe extern "system" fn(*mut core::ffi::c_void, AppointmentParticipantResponse) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentManagerForUser, IAppointmentManagerForUser_Vtbl, 0x70261423_73cc_4660_b318_b01365302a03);
impl windows_core::RuntimeType for IAppointmentManagerForUser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentManagerForUser_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ShowAddAppointmentAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::Rect, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Popups")]
    pub ShowAddAppointmentWithPlacementAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::Rect, super::super::UI::Popups::Placement, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    ShowAddAppointmentWithPlacementAsync: usize,
    pub ShowReplaceAppointmentAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, super::super::Foundation::Rect, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Popups")]
    pub ShowReplaceAppointmentWithPlacementAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, super::super::Foundation::Rect, super::super::UI::Popups::Placement, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    ShowReplaceAppointmentWithPlacementAsync: usize,
    #[cfg(feature = "UI_Popups")]
    pub ShowReplaceAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, super::super::Foundation::Rect, super::super::UI::Popups::Placement, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    ShowReplaceAppointmentWithPlacementAndDateAsync: usize,
    pub ShowRemoveAppointmentAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::super::Foundation::Rect, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Popups")]
    pub ShowRemoveAppointmentWithPlacementAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::super::Foundation::Rect, super::super::UI::Popups::Placement, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    ShowRemoveAppointmentWithPlacementAsync: usize,
    #[cfg(feature = "UI_Popups")]
    pub ShowRemoveAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::super::Foundation::Rect, super::super::UI::Popups::Placement, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    ShowRemoveAppointmentWithPlacementAndDateAsync: usize,
    pub ShowTimeFrameAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowAppointmentDetailsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowAppointmentDetailsWithDateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowEditNewAppointmentAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestStoreAsync: unsafe extern "system" fn(*mut core::ffi::c_void, AppointmentStoreAccessType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
windows_core::imp::define_interface!(IAppointmentManagerStatics, IAppointmentManagerStatics_Vtbl, 0x3a30fa01_5c40_499d_b33f_a43050f74fc4);
impl windows_core::RuntimeType for IAppointmentManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ShowAddAppointmentAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::Rect, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Popups")]
    pub ShowAddAppointmentWithPlacementAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::Rect, super::super::UI::Popups::Placement, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    ShowAddAppointmentWithPlacementAsync: usize,
    pub ShowReplaceAppointmentAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, super::super::Foundation::Rect, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Popups")]
    pub ShowReplaceAppointmentWithPlacementAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, super::super::Foundation::Rect, super::super::UI::Popups::Placement, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    ShowReplaceAppointmentWithPlacementAsync: usize,
    #[cfg(feature = "UI_Popups")]
    pub ShowReplaceAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, super::super::Foundation::Rect, super::super::UI::Popups::Placement, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    ShowReplaceAppointmentWithPlacementAndDateAsync: usize,
    pub ShowRemoveAppointmentAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::super::Foundation::Rect, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Popups")]
    pub ShowRemoveAppointmentWithPlacementAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::super::Foundation::Rect, super::super::UI::Popups::Placement, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    ShowRemoveAppointmentWithPlacementAsync: usize,
    #[cfg(feature = "UI_Popups")]
    pub ShowRemoveAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::super::Foundation::Rect, super::super::UI::Popups::Placement, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    ShowRemoveAppointmentWithPlacementAndDateAsync: usize,
    pub ShowTimeFrameAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentManagerStatics2, IAppointmentManagerStatics2_Vtbl, 0x0a81f60d_d04f_4034_af72_a36573b45ff0);
impl windows_core::RuntimeType for IAppointmentManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ShowAppointmentDetailsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowAppointmentDetailsWithDateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowEditNewAppointmentAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestStoreAsync: unsafe extern "system" fn(*mut core::ffi::c_void, AppointmentStoreAccessType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentManagerStatics3, IAppointmentManagerStatics3_Vtbl, 0x2f9ae09c_b34c_4dc7_a35d_cafd88ae3ec6);
impl windows_core::RuntimeType for IAppointmentManagerStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentManagerStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
windows_core::imp::define_interface!(IAppointmentParticipant, IAppointmentParticipant_Vtbl, 0x615e2902_9718_467b_83fb_b293a19121de);
impl core::ops::Deref for IAppointmentParticipant {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppointmentParticipant, windows_core::IUnknown, windows_core::IInspectable);
impl IAppointmentParticipant {
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDisplayName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Address(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Address)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAddress(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAddress)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for IAppointmentParticipant {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentParticipant_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Address: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetAddress: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentPropertiesStatics, IAppointmentPropertiesStatics_Vtbl, 0x25141fe9_68ae_3aae_855f_bc4441caa234);
impl windows_core::RuntimeType for IAppointmentPropertiesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentPropertiesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Subject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Location: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Reminder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub BusyStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Sensitivity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub OriginalStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsResponseRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AllowNewTimeProposal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AllDay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Details: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub OnlineMeetingLink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReplyTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Organizer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub UserResponse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub HasInvitees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsCanceledMeeting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsOrganizedByUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Recurrence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Invitees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DefaultProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DefaultProperties: usize,
}
windows_core::imp::define_interface!(IAppointmentPropertiesStatics2, IAppointmentPropertiesStatics2_Vtbl, 0xdffc434b_b017_45dd_8af5_d163d10801bb);
impl windows_core::RuntimeType for IAppointmentPropertiesStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentPropertiesStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ChangeNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RemoteChangeNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DetailsKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentRecurrence, IAppointmentRecurrence_Vtbl, 0xd87b3e83_15a6_487b_b959_0c361e60e954);
impl windows_core::RuntimeType for IAppointmentRecurrence {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentRecurrence_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Unit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppointmentRecurrenceUnit) -> windows_core::HRESULT,
    pub SetUnit: unsafe extern "system" fn(*mut core::ffi::c_void, AppointmentRecurrenceUnit) -> windows_core::HRESULT,
    pub Occurrences: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOccurrences: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Until: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUntil: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Interval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DaysOfWeek: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppointmentDaysOfWeek) -> windows_core::HRESULT,
    pub SetDaysOfWeek: unsafe extern "system" fn(*mut core::ffi::c_void, AppointmentDaysOfWeek) -> windows_core::HRESULT,
    pub WeekOfMonth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppointmentWeekOfMonth) -> windows_core::HRESULT,
    pub SetWeekOfMonth: unsafe extern "system" fn(*mut core::ffi::c_void, AppointmentWeekOfMonth) -> windows_core::HRESULT,
    pub Month: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMonth: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Day: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetDay: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentRecurrence2, IAppointmentRecurrence2_Vtbl, 0x3df3a2e0_05a7_4f50_9f86_b03f9436254d);
impl windows_core::RuntimeType for IAppointmentRecurrence2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentRecurrence2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RecurrenceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RecurrenceType) -> windows_core::HRESULT,
    pub TimeZone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetTimeZone: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentRecurrence3, IAppointmentRecurrence3_Vtbl, 0x89ff96d9_da4d_4a17_8dd2_1cebc2b5ff9d);
impl windows_core::RuntimeType for IAppointmentRecurrence3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentRecurrence3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CalendarIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentStore, IAppointmentStore_Vtbl, 0xa461918c_7a47_4d96_96c9_15cd8a05a735);
impl windows_core::RuntimeType for IAppointmentStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentStore_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ChangeTracker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateAppointmentCalendarAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAppointmentCalendarAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAppointmentAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAppointmentInstanceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppointmentCalendarsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppointmentCalendarsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppointmentCalendarsAsyncWithOptions: unsafe extern "system" fn(*mut core::ffi::c_void, FindAppointmentCalendarsOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppointmentCalendarsAsyncWithOptions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppointmentsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppointmentsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppointmentsAsyncWithOptions: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, super::super::Foundation::TimeSpan, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppointmentsAsyncWithOptions: usize,
    pub FindConflictAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindConflictAsyncWithInstanceStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoveAppointmentAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowAddAppointmentAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::Rect, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowReplaceAppointmentAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, super::super::Foundation::Rect, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Popups")]
    pub ShowReplaceAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, super::super::Foundation::Rect, super::super::UI::Popups::Placement, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    ShowReplaceAppointmentWithPlacementAndDateAsync: usize,
    pub ShowRemoveAppointmentAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::super::Foundation::Rect, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Popups")]
    pub ShowRemoveAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::super::Foundation::Rect, super::super::UI::Popups::Placement, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    ShowRemoveAppointmentWithPlacementAndDateAsync: usize,
    pub ShowAppointmentDetailsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowAppointmentDetailsWithDateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowEditNewAppointmentAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindLocalIdsFromRoamingIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindLocalIdsFromRoamingIdAsync: usize,
}
windows_core::imp::define_interface!(IAppointmentStore2, IAppointmentStore2_Vtbl, 0x25c48c20_1c41_424f_8084_67c1cfe0a854);
impl windows_core::RuntimeType for IAppointmentStore2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentStore2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StoreChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStoreChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CreateAppointmentCalendarInAccountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentStore3, IAppointmentStore3_Vtbl, 0x4251940b_b078_470a_9a40_c2e01761f72f);
impl windows_core::RuntimeType for IAppointmentStore3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentStore3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetChangeTracker: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentStoreChange, IAppointmentStoreChange_Vtbl, 0xa5a6e035_0a33_3654_8463_b543e90c3b79);
impl windows_core::RuntimeType for IAppointmentStoreChange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentStoreChange_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Appointment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ChangeType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppointmentStoreChangeType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentStoreChange2, IAppointmentStoreChange2_Vtbl, 0xb37d0dce_5211_4402_a608_a96fe70b8ee2);
impl windows_core::RuntimeType for IAppointmentStoreChange2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentStoreChange2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AppointmentCalendar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentStoreChangeReader, IAppointmentStoreChangeReader_Vtbl, 0x8b2409f1_65f3_42a0_961d_4c209bf30370);
impl windows_core::RuntimeType for IAppointmentStoreChangeReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentStoreChangeReader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
    pub AcceptChanges: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AcceptChangesThrough: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentStoreChangeTracker, IAppointmentStoreChangeTracker_Vtbl, 0x1b25f4b1_8ece_4f17_93c8_e6412458fd5c);
impl windows_core::RuntimeType for IAppointmentStoreChangeTracker {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentStoreChangeTracker_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetChangeReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentStoreChangeTracker2, IAppointmentStoreChangeTracker2_Vtbl, 0xb66aaf45_9542_4cf7_8550_eb370e0c08d3);
impl windows_core::RuntimeType for IAppointmentStoreChangeTracker2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentStoreChangeTracker2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsTracking: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentStoreChangedDeferral, IAppointmentStoreChangedDeferral_Vtbl, 0x4cb82026_fedb_4bc3_9662_95a9befdf4df);
impl windows_core::RuntimeType for IAppointmentStoreChangedDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentStoreChangedDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentStoreChangedEventArgs, IAppointmentStoreChangedEventArgs_Vtbl, 0x2285f8b9_0791_417e_bfea_cc6d41636c8c);
impl windows_core::RuntimeType for IAppointmentStoreChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentStoreChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentStoreNotificationTriggerDetails, IAppointmentStoreNotificationTriggerDetails_Vtbl, 0x9b33cb11_c301_421e_afef_047ecfa76adb);
impl windows_core::RuntimeType for IAppointmentStoreNotificationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentStoreNotificationTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IFindAppointmentsOptions, IFindAppointmentsOptions_Vtbl, 0x55f7dc55_9942_3086_82b5_2cb29f64d5f5);
impl windows_core::RuntimeType for IFindAppointmentsOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFindAppointmentsOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CalendarIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CalendarIds: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FetchProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FetchProperties: usize,
    pub IncludeHidden: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIncludeHidden: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub MaxCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaxCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Appointment(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Appointment, windows_core::IUnknown, windows_core::IInspectable);
impl Appointment {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Appointment, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn StartTime(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStartTime(&self, value: super::super::Foundation::DateTime) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStartTime)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Duration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDuration(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDuration)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Location(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Location)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLocation(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLocation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
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
    pub fn Details(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Details)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDetails(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDetails)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Reminder(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reminder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetReminder<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReminder)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Organizer(&self) -> windows_core::Result<AppointmentOrganizer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Organizer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOrganizer<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AppointmentOrganizer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOrganizer)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Invitees(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<AppointmentInvitee>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Invitees)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Recurrence(&self) -> windows_core::Result<AppointmentRecurrence> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Recurrence)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRecurrence<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AppointmentRecurrence>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRecurrence)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn BusyStatus(&self) -> windows_core::Result<AppointmentBusyStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BusyStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBusyStatus(&self, value: AppointmentBusyStatus) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBusyStatus)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllDay(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllDay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllDay(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllDay)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Sensitivity(&self) -> windows_core::Result<AppointmentSensitivity> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Sensitivity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSensitivity(&self, value: AppointmentSensitivity) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSensitivity)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn LocalId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CalendarId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CalendarId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RoamingId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoamingId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRoamingId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRoamingId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn OriginalStartTime(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = &windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalStartTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsResponseRequested(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsResponseRequested)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsResponseRequested(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsResponseRequested)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowNewTimeProposal(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowNewTimeProposal)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowNewTimeProposal(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowNewTimeProposal)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OnlineMeetingLink(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OnlineMeetingLink)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOnlineMeetingLink(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOnlineMeetingLink)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ReplyTime(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = &windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReplyTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetReplyTime<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = &windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReplyTime)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn UserResponse(&self) -> windows_core::Result<AppointmentParticipantResponse> {
        let this = &windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserResponse)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUserResponse(&self, value: AppointmentParticipantResponse) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUserResponse)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HasInvitees(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasInvitees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCanceledMeeting(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCanceledMeeting)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsCanceledMeeting(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsCanceledMeeting)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsOrganizedByUser(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOrganizedByUser)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsOrganizedByUser(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppointment2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsOrganizedByUser)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ChangeNumber(&self) -> windows_core::Result<u64> {
        let this = &windows_core::Interface::cast::<IAppointment3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChangeNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RemoteChangeNumber(&self) -> windows_core::Result<u64> {
        let this = &windows_core::Interface::cast::<IAppointment3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteChangeNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRemoteChangeNumber(&self, value: u64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppointment3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRemoteChangeNumber)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DetailsKind(&self) -> windows_core::Result<AppointmentDetailsKind> {
        let this = &windows_core::Interface::cast::<IAppointment3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DetailsKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDetailsKind(&self, value: AppointmentDetailsKind) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppointment3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDetailsKind)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for Appointment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointment>();
}
unsafe impl windows_core::Interface for Appointment {
    type Vtable = IAppointment_Vtbl;
    const IID: windows_core::GUID = <IAppointment as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Appointment {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.Appointment";
}
unsafe impl Send for Appointment {}
unsafe impl Sync for Appointment {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppointmentCalendar(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentCalendar, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentCalendar {
    #[cfg(feature = "UI")]
    pub fn DisplayColor(&self) -> windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDisplayName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn LocalId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsHidden(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHidden)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OtherAppReadAccess(&self) -> windows_core::Result<AppointmentCalendarOtherAppReadAccess> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OtherAppReadAccess)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOtherAppReadAccess(&self, value: AppointmentCalendarOtherAppReadAccess) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOtherAppReadAccess)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OtherAppWriteAccess(&self) -> windows_core::Result<AppointmentCalendarOtherAppWriteAccess> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OtherAppWriteAccess)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOtherAppWriteAccess(&self, value: AppointmentCalendarOtherAppWriteAccess) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOtherAppWriteAccess)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SourceDisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceDisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SummaryCardView(&self) -> windows_core::Result<AppointmentSummaryCardView> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SummaryCardView)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSummaryCardView(&self, value: AppointmentSummaryCardView) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSummaryCardView)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAppointmentsAsync(&self, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAppointmentsAsync)(windows_core::Interface::as_raw(this), rangestart, rangelength, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAppointmentsAsyncWithOptions<P0>(&self, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, options: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>
    where
        P0: windows_core::Param<FindAppointmentsOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAppointmentsAsyncWithOptions)(windows_core::Interface::as_raw(this), rangestart, rangelength, options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindExceptionsFromMasterAsync(&self, masterlocalid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentException>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindExceptionsFromMasterAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(masterlocalid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllInstancesAsync(&self, masterlocalid: &windows_core::HSTRING, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllInstancesAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(masterlocalid), rangestart, rangelength, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllInstancesAsyncWithOptions<P0>(&self, masterlocalid: &windows_core::HSTRING, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, poptions: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>
    where
        P0: windows_core::Param<FindAppointmentsOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllInstancesAsyncWithOptions)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(masterlocalid), rangestart, rangelength, poptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAppointmentAsync(&self, localid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<Appointment>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAppointmentAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(localid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAppointmentInstanceAsync(&self, localid: &windows_core::HSTRING, instancestarttime: super::super::Foundation::DateTime) -> windows_core::Result<super::super::Foundation::IAsyncOperation<Appointment>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAppointmentInstanceAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(localid), instancestarttime, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindUnexpandedAppointmentsAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindUnexpandedAppointmentsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindUnexpandedAppointmentsAsyncWithOptions<P0>(&self, options: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>
    where
        P0: windows_core::Param<FindAppointmentsOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindUnexpandedAppointmentsAsyncWithOptions)(windows_core::Interface::as_raw(this), options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SaveAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteAppointmentAsync(&self, localid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteAppointmentAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(localid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteAppointmentInstanceAsync(&self, localid: &windows_core::HSTRING, instancestarttime: super::super::Foundation::DateTime) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteAppointmentInstanceAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(localid), instancestarttime, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SaveAppointmentAsync<P0>(&self, pappointment: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveAppointmentAsync)(windows_core::Interface::as_raw(this), pappointment.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SyncManager(&self) -> windows_core::Result<AppointmentCalendarSyncManager> {
        let this = &windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SyncManager)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoteId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRemoteId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRemoteId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn SetDisplayColor(&self, value: super::super::UI::Color) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetIsHidden(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsHidden)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UserDataAccountId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserDataAccountId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CanCreateOrUpdateAppointments(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanCreateOrUpdateAppointments)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanCreateOrUpdateAppointments(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanCreateOrUpdateAppointments)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanCancelMeetings(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanCancelMeetings)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanCancelMeetings(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanCancelMeetings)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanForwardMeetings(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanForwardMeetings)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanForwardMeetings(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanForwardMeetings)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanProposeNewTimeForMeetings(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanProposeNewTimeForMeetings)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanProposeNewTimeForMeetings(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanProposeNewTimeForMeetings)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanUpdateMeetingResponses(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanUpdateMeetingResponses)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanUpdateMeetingResponses(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanUpdateMeetingResponses)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanNotifyInvitees(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanNotifyInvitees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanNotifyInvitees(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanNotifyInvitees)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MustNofityInvitees(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MustNofityInvitees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMustNofityInvitees(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMustNofityInvitees)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TryCreateOrUpdateAppointmentAsync<P0>(&self, appointment: P0, notifyinvitees: bool) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<Appointment>,
    {
        let this = &windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCreateOrUpdateAppointmentAsync)(windows_core::Interface::as_raw(this), appointment.param().abi(), notifyinvitees, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryCancelMeetingAsync<P0>(&self, meeting: P0, subject: &windows_core::HSTRING, comment: &windows_core::HSTRING, notifyinvitees: bool) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<Appointment>,
    {
        let this = &windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCancelMeetingAsync)(windows_core::Interface::as_raw(this), meeting.param().abi(), core::mem::transmute_copy(subject), core::mem::transmute_copy(comment), notifyinvitees, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryForwardMeetingAsync<P0, P1>(&self, meeting: P0, invitees: P1, subject: &windows_core::HSTRING, forwardheader: &windows_core::HSTRING, comment: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<Appointment>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<AppointmentInvitee>>,
    {
        let this = &windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryForwardMeetingAsync)(windows_core::Interface::as_raw(this), meeting.param().abi(), invitees.param().abi(), core::mem::transmute_copy(subject), core::mem::transmute_copy(forwardheader), core::mem::transmute_copy(comment), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryProposeNewTimeForMeetingAsync<P0>(&self, meeting: P0, newstarttime: super::super::Foundation::DateTime, newduration: super::super::Foundation::TimeSpan, subject: &windows_core::HSTRING, comment: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<Appointment>,
    {
        let this = &windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryProposeNewTimeForMeetingAsync)(windows_core::Interface::as_raw(this), meeting.param().abi(), newstarttime, newduration, core::mem::transmute_copy(subject), core::mem::transmute_copy(comment), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryUpdateMeetingResponseAsync<P0>(&self, meeting: P0, response: AppointmentParticipantResponse, subject: &windows_core::HSTRING, comment: &windows_core::HSTRING, sendupdate: bool) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<Appointment>,
    {
        let this = &windows_core::Interface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryUpdateMeetingResponseAsync)(windows_core::Interface::as_raw(this), meeting.param().abi(), response, core::mem::transmute_copy(subject), core::mem::transmute_copy(comment), sendupdate, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RegisterSyncManagerAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IAppointmentCalendar3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegisterSyncManagerAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppointmentCalendar {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentCalendar>();
}
unsafe impl windows_core::Interface for AppointmentCalendar {
    type Vtable = IAppointmentCalendar_Vtbl;
    const IID: windows_core::GUID = <IAppointmentCalendar as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentCalendar {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentCalendar";
}
unsafe impl Send for AppointmentCalendar {}
unsafe impl Sync for AppointmentCalendar {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppointmentCalendarSyncManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentCalendarSyncManager, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentCalendarSyncManager {
    pub fn Status(&self) -> windows_core::Result<AppointmentCalendarSyncStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LastSuccessfulSyncTime(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastSuccessfulSyncTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LastAttemptedSyncTime(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastAttemptedSyncTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SyncAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SyncAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SyncStatusChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppointmentCalendarSyncManager, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SyncStatusChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSyncStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSyncStatusChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetStatus(&self, value: AppointmentCalendarSyncStatus) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppointmentCalendarSyncManager2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStatus)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetLastSuccessfulSyncTime(&self, value: super::super::Foundation::DateTime) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppointmentCalendarSyncManager2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLastSuccessfulSyncTime)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetLastAttemptedSyncTime(&self, value: super::super::Foundation::DateTime) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppointmentCalendarSyncManager2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLastAttemptedSyncTime)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for AppointmentCalendarSyncManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentCalendarSyncManager>();
}
unsafe impl windows_core::Interface for AppointmentCalendarSyncManager {
    type Vtable = IAppointmentCalendarSyncManager_Vtbl;
    const IID: windows_core::GUID = <IAppointmentCalendarSyncManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentCalendarSyncManager {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentCalendarSyncManager";
}
unsafe impl Send for AppointmentCalendarSyncManager {}
unsafe impl Sync for AppointmentCalendarSyncManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppointmentConflictResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentConflictResult, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentConflictResult {
    pub fn Type(&self) -> windows_core::Result<AppointmentConflictType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Date(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Date)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppointmentConflictResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentConflictResult>();
}
unsafe impl windows_core::Interface for AppointmentConflictResult {
    type Vtable = IAppointmentConflictResult_Vtbl;
    const IID: windows_core::GUID = <IAppointmentConflictResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentConflictResult {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentConflictResult";
}
unsafe impl Send for AppointmentConflictResult {}
unsafe impl Sync for AppointmentConflictResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppointmentException(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentException, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentException {
    pub fn Appointment(&self) -> windows_core::Result<Appointment> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Appointment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExceptionProperties(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExceptionProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsDeleted(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDeleted)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppointmentException {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentException>();
}
unsafe impl windows_core::Interface for AppointmentException {
    type Vtable = IAppointmentException_Vtbl;
    const IID: windows_core::GUID = <IAppointmentException as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentException {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentException";
}
unsafe impl Send for AppointmentException {}
unsafe impl Sync for AppointmentException {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppointmentInvitee(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentInvitee, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(AppointmentInvitee, IAppointmentParticipant);
impl AppointmentInvitee {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppointmentInvitee, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Role(&self) -> windows_core::Result<AppointmentParticipantRole> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Role)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRole(&self, value: AppointmentParticipantRole) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRole)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Response(&self) -> windows_core::Result<AppointmentParticipantResponse> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Response)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetResponse(&self, value: AppointmentParticipantResponse) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetResponse)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppointmentParticipant>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDisplayName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppointmentParticipant>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Address(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppointmentParticipant>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Address)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAddress(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppointmentParticipant>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAddress)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for AppointmentInvitee {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentInvitee>();
}
unsafe impl windows_core::Interface for AppointmentInvitee {
    type Vtable = IAppointmentInvitee_Vtbl;
    const IID: windows_core::GUID = <IAppointmentInvitee as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentInvitee {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentInvitee";
}
unsafe impl Send for AppointmentInvitee {}
unsafe impl Sync for AppointmentInvitee {}
pub struct AppointmentManager;
impl AppointmentManager {
    pub fn ShowAddAppointmentAsync<P0>(appointment: P0, selection: super::super::Foundation::Rect) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>>
    where
        P0: windows_core::Param<Appointment>,
    {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowAddAppointmentAsync)(windows_core::Interface::as_raw(this), appointment.param().abi(), selection, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "UI_Popups")]
    pub fn ShowAddAppointmentWithPlacementAsync<P0>(appointment: P0, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>>
    where
        P0: windows_core::Param<Appointment>,
    {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowAddAppointmentWithPlacementAsync)(windows_core::Interface::as_raw(this), appointment.param().abi(), selection, preferredplacement, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ShowReplaceAppointmentAsync<P0>(appointmentid: &windows_core::HSTRING, appointment: P0, selection: super::super::Foundation::Rect) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>>
    where
        P0: windows_core::Param<Appointment>,
    {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowReplaceAppointmentAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appointmentid), appointment.param().abi(), selection, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "UI_Popups")]
    pub fn ShowReplaceAppointmentWithPlacementAsync<P0>(appointmentid: &windows_core::HSTRING, appointment: P0, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>>
    where
        P0: windows_core::Param<Appointment>,
    {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowReplaceAppointmentWithPlacementAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appointmentid), appointment.param().abi(), selection, preferredplacement, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "UI_Popups")]
    pub fn ShowReplaceAppointmentWithPlacementAndDateAsync<P0>(appointmentid: &windows_core::HSTRING, appointment: P0, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>>
    where
        P0: windows_core::Param<Appointment>,
    {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowReplaceAppointmentWithPlacementAndDateAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appointmentid), appointment.param().abi(), selection, preferredplacement, instancestartdate, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ShowRemoveAppointmentAsync(appointmentid: &windows_core::HSTRING, selection: super::super::Foundation::Rect) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowRemoveAppointmentAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appointmentid), selection, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "UI_Popups")]
    pub fn ShowRemoveAppointmentWithPlacementAsync(appointmentid: &windows_core::HSTRING, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowRemoveAppointmentWithPlacementAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appointmentid), selection, preferredplacement, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "UI_Popups")]
    pub fn ShowRemoveAppointmentWithPlacementAndDateAsync(appointmentid: &windows_core::HSTRING, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowRemoveAppointmentWithPlacementAndDateAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appointmentid), selection, preferredplacement, instancestartdate, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ShowTimeFrameAsync(timetoshow: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowTimeFrameAsync)(windows_core::Interface::as_raw(this), timetoshow, duration, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ShowAppointmentDetailsAsync(appointmentid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::IAppointmentManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowAppointmentDetailsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appointmentid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ShowAppointmentDetailsWithDateAsync(appointmentid: &windows_core::HSTRING, instancestartdate: super::super::Foundation::DateTime) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::IAppointmentManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowAppointmentDetailsWithDateAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appointmentid), instancestartdate, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ShowEditNewAppointmentAsync<P0>(appointment: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>>
    where
        P0: windows_core::Param<Appointment>,
    {
        Self::IAppointmentManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowEditNewAppointmentAsync)(windows_core::Interface::as_raw(this), appointment.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RequestStoreAsync(options: AppointmentStoreAccessType) -> windows_core::Result<super::super::Foundation::IAsyncOperation<AppointmentStore>> {
        Self::IAppointmentManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestStoreAsync)(windows_core::Interface::as_raw(this), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<P0>(user: P0) -> windows_core::Result<AppointmentManagerForUser>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        Self::IAppointmentManagerStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IAppointmentManagerStatics<R, F: FnOnce(&IAppointmentManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppointmentManager, IAppointmentManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAppointmentManagerStatics2<R, F: FnOnce(&IAppointmentManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppointmentManager, IAppointmentManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAppointmentManagerStatics3<R, F: FnOnce(&IAppointmentManagerStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppointmentManager, IAppointmentManagerStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for AppointmentManager {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppointmentManagerForUser(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentManagerForUser, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentManagerForUser {
    pub fn ShowAddAppointmentAsync<P0>(&self, appointment: P0, selection: super::super::Foundation::Rect) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>>
    where
        P0: windows_core::Param<Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowAddAppointmentAsync)(windows_core::Interface::as_raw(this), appointment.param().abi(), selection, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn ShowAddAppointmentWithPlacementAsync<P0>(&self, appointment: P0, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>>
    where
        P0: windows_core::Param<Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowAddAppointmentWithPlacementAsync)(windows_core::Interface::as_raw(this), appointment.param().abi(), selection, preferredplacement, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ShowReplaceAppointmentAsync<P0>(&self, appointmentid: &windows_core::HSTRING, appointment: P0, selection: super::super::Foundation::Rect) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>>
    where
        P0: windows_core::Param<Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowReplaceAppointmentAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appointmentid), appointment.param().abi(), selection, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn ShowReplaceAppointmentWithPlacementAsync<P0>(&self, appointmentid: &windows_core::HSTRING, appointment: P0, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>>
    where
        P0: windows_core::Param<Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowReplaceAppointmentWithPlacementAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appointmentid), appointment.param().abi(), selection, preferredplacement, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn ShowReplaceAppointmentWithPlacementAndDateAsync<P0>(&self, appointmentid: &windows_core::HSTRING, appointment: P0, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>>
    where
        P0: windows_core::Param<Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowReplaceAppointmentWithPlacementAndDateAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appointmentid), appointment.param().abi(), selection, preferredplacement, instancestartdate, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ShowRemoveAppointmentAsync(&self, appointmentid: &windows_core::HSTRING, selection: super::super::Foundation::Rect) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowRemoveAppointmentAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appointmentid), selection, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn ShowRemoveAppointmentWithPlacementAsync(&self, appointmentid: &windows_core::HSTRING, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowRemoveAppointmentWithPlacementAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appointmentid), selection, preferredplacement, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn ShowRemoveAppointmentWithPlacementAndDateAsync(&self, appointmentid: &windows_core::HSTRING, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowRemoveAppointmentWithPlacementAndDateAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appointmentid), selection, preferredplacement, instancestartdate, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ShowTimeFrameAsync(&self, timetoshow: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowTimeFrameAsync)(windows_core::Interface::as_raw(this), timetoshow, duration, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ShowAppointmentDetailsAsync(&self, appointmentid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowAppointmentDetailsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appointmentid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ShowAppointmentDetailsWithDateAsync(&self, appointmentid: &windows_core::HSTRING, instancestartdate: super::super::Foundation::DateTime) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowAppointmentDetailsWithDateAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appointmentid), instancestartdate, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ShowEditNewAppointmentAsync<P0>(&self, appointment: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>>
    where
        P0: windows_core::Param<Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowEditNewAppointmentAsync)(windows_core::Interface::as_raw(this), appointment.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestStoreAsync(&self, options: AppointmentStoreAccessType) -> windows_core::Result<super::super::Foundation::IAsyncOperation<AppointmentStore>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestStoreAsync)(windows_core::Interface::as_raw(this), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppointmentManagerForUser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentManagerForUser>();
}
unsafe impl windows_core::Interface for AppointmentManagerForUser {
    type Vtable = IAppointmentManagerForUser_Vtbl;
    const IID: windows_core::GUID = <IAppointmentManagerForUser as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentManagerForUser {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentManagerForUser";
}
unsafe impl Send for AppointmentManagerForUser {}
unsafe impl Sync for AppointmentManagerForUser {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppointmentOrganizer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentOrganizer, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(AppointmentOrganizer, IAppointmentParticipant);
impl AppointmentOrganizer {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppointmentOrganizer, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDisplayName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Address(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Address)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAddress(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAddress)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for AppointmentOrganizer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentParticipant>();
}
unsafe impl windows_core::Interface for AppointmentOrganizer {
    type Vtable = IAppointmentParticipant_Vtbl;
    const IID: windows_core::GUID = <IAppointmentParticipant as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentOrganizer {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentOrganizer";
}
unsafe impl Send for AppointmentOrganizer {}
unsafe impl Sync for AppointmentOrganizer {}
pub struct AppointmentProperties;
impl AppointmentProperties {
    pub fn Subject() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Subject)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Location() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Location)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn StartTime() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Duration() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Reminder() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reminder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn BusyStatus() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BusyStatus)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Sensitivity() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Sensitivity)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn OriginalStartTime() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalStartTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsResponseRequested() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsResponseRequested)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn AllowNewTimeProposal() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowNewTimeProposal)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn AllDay() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllDay)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Details() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Details)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn OnlineMeetingLink() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OnlineMeetingLink)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ReplyTime() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReplyTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Organizer() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Organizer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn UserResponse() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserResponse)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn HasInvitees() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasInvitees)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsCanceledMeeting() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCanceledMeeting)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsOrganizedByUser() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOrganizedByUser)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Recurrence() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Recurrence)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Uri() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Invitees() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Invitees)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DefaultProperties() -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ChangeNumber() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChangeNumber)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RemoteChangeNumber() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteChangeNumber)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn DetailsKind() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DetailsKind)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IAppointmentPropertiesStatics<R, F: FnOnce(&IAppointmentPropertiesStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppointmentProperties, IAppointmentPropertiesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAppointmentPropertiesStatics2<R, F: FnOnce(&IAppointmentPropertiesStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppointmentProperties, IAppointmentPropertiesStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for AppointmentProperties {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentProperties";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppointmentRecurrence(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentRecurrence, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentRecurrence {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppointmentRecurrence, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Unit(&self) -> windows_core::Result<AppointmentRecurrenceUnit> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Unit)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUnit(&self, value: AppointmentRecurrenceUnit) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUnit)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Occurrences(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Occurrences)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOccurrences<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<u32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOccurrences)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Until(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Until)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetUntil<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUntil)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Interval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Interval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInterval(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DaysOfWeek(&self) -> windows_core::Result<AppointmentDaysOfWeek> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DaysOfWeek)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDaysOfWeek(&self, value: AppointmentDaysOfWeek) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDaysOfWeek)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WeekOfMonth(&self) -> windows_core::Result<AppointmentWeekOfMonth> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WeekOfMonth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetWeekOfMonth(&self, value: AppointmentWeekOfMonth) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetWeekOfMonth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Month(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Month)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMonth(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMonth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Day(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Day)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDay(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDay)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RecurrenceType(&self) -> windows_core::Result<RecurrenceType> {
        let this = &windows_core::Interface::cast::<IAppointmentRecurrence2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecurrenceType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TimeZone(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppointmentRecurrence2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeZone)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTimeZone(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppointmentRecurrence2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTimeZone)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn CalendarIdentifier(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppointmentRecurrence3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CalendarIdentifier)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppointmentRecurrence {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentRecurrence>();
}
unsafe impl windows_core::Interface for AppointmentRecurrence {
    type Vtable = IAppointmentRecurrence_Vtbl;
    const IID: windows_core::GUID = <IAppointmentRecurrence as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentRecurrence {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentRecurrence";
}
unsafe impl Send for AppointmentRecurrence {}
unsafe impl Sync for AppointmentRecurrence {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppointmentStore(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentStore, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentStore {
    pub fn ChangeTracker(&self) -> windows_core::Result<AppointmentStoreChangeTracker> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChangeTracker)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateAppointmentCalendarAsync(&self, name: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<AppointmentCalendar>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAppointmentCalendarAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAppointmentCalendarAsync(&self, calendarid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<AppointmentCalendar>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAppointmentCalendarAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(calendarid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAppointmentAsync(&self, localid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<Appointment>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAppointmentAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(localid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAppointmentInstanceAsync(&self, localid: &windows_core::HSTRING, instancestarttime: super::super::Foundation::DateTime) -> windows_core::Result<super::super::Foundation::IAsyncOperation<Appointment>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAppointmentInstanceAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(localid), instancestarttime, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAppointmentCalendarsAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentCalendar>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAppointmentCalendarsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAppointmentCalendarsAsyncWithOptions(&self, options: FindAppointmentCalendarsOptions) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentCalendar>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAppointmentCalendarsAsyncWithOptions)(windows_core::Interface::as_raw(this), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAppointmentsAsync(&self, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAppointmentsAsync)(windows_core::Interface::as_raw(this), rangestart, rangelength, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAppointmentsAsyncWithOptions<P0>(&self, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, options: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>
    where
        P0: windows_core::Param<FindAppointmentsOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAppointmentsAsyncWithOptions)(windows_core::Interface::as_raw(this), rangestart, rangelength, options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FindConflictAsync<P0>(&self, appointment: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<AppointmentConflictResult>>
    where
        P0: windows_core::Param<Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindConflictAsync)(windows_core::Interface::as_raw(this), appointment.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FindConflictAsyncWithInstanceStart<P0>(&self, appointment: P0, instancestarttime: super::super::Foundation::DateTime) -> windows_core::Result<super::super::Foundation::IAsyncOperation<AppointmentConflictResult>>
    where
        P0: windows_core::Param<Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindConflictAsyncWithInstanceStart)(windows_core::Interface::as_raw(this), appointment.param().abi(), instancestarttime, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MoveAppointmentAsync<P0, P1>(&self, appointment: P0, destinationcalendar: P1) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<Appointment>,
        P1: windows_core::Param<AppointmentCalendar>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveAppointmentAsync)(windows_core::Interface::as_raw(this), appointment.param().abi(), destinationcalendar.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ShowAddAppointmentAsync<P0>(&self, appointment: P0, selection: super::super::Foundation::Rect) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>>
    where
        P0: windows_core::Param<Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowAddAppointmentAsync)(windows_core::Interface::as_raw(this), appointment.param().abi(), selection, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ShowReplaceAppointmentAsync<P0>(&self, localid: &windows_core::HSTRING, appointment: P0, selection: super::super::Foundation::Rect) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>>
    where
        P0: windows_core::Param<Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowReplaceAppointmentAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(localid), appointment.param().abi(), selection, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn ShowReplaceAppointmentWithPlacementAndDateAsync<P0>(&self, localid: &windows_core::HSTRING, appointment: P0, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>>
    where
        P0: windows_core::Param<Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowReplaceAppointmentWithPlacementAndDateAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(localid), appointment.param().abi(), selection, preferredplacement, instancestartdate, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ShowRemoveAppointmentAsync(&self, localid: &windows_core::HSTRING, selection: super::super::Foundation::Rect) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowRemoveAppointmentAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(localid), selection, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn ShowRemoveAppointmentWithPlacementAndDateAsync(&self, localid: &windows_core::HSTRING, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowRemoveAppointmentWithPlacementAndDateAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(localid), selection, preferredplacement, instancestartdate, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ShowAppointmentDetailsAsync(&self, localid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowAppointmentDetailsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(localid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ShowAppointmentDetailsWithDateAsync(&self, localid: &windows_core::HSTRING, instancestartdate: super::super::Foundation::DateTime) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowAppointmentDetailsWithDateAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(localid), instancestartdate, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ShowEditNewAppointmentAsync<P0>(&self, appointment: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>>
    where
        P0: windows_core::Param<Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowEditNewAppointmentAsync)(windows_core::Interface::as_raw(this), appointment.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindLocalIdsFromRoamingIdAsync(&self, roamingid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindLocalIdsFromRoamingIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(roamingid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StoreChanged<P0>(&self, phandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AppointmentStore, AppointmentStoreChangedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IAppointmentStore2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StoreChanged)(windows_core::Interface::as_raw(this), phandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStoreChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppointmentStore2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveStoreChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CreateAppointmentCalendarInAccountAsync(&self, name: &windows_core::HSTRING, userdataaccountid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<AppointmentCalendar>> {
        let this = &windows_core::Interface::cast::<IAppointmentStore2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAppointmentCalendarInAccountAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(userdataaccountid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetChangeTracker(&self, identity: &windows_core::HSTRING) -> windows_core::Result<AppointmentStoreChangeTracker> {
        let this = &windows_core::Interface::cast::<IAppointmentStore3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChangeTracker)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(identity), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppointmentStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentStore>();
}
unsafe impl windows_core::Interface for AppointmentStore {
    type Vtable = IAppointmentStore_Vtbl;
    const IID: windows_core::GUID = <IAppointmentStore as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentStore {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStore";
}
unsafe impl Send for AppointmentStore {}
unsafe impl Sync for AppointmentStore {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppointmentStoreChange(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentStoreChange, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentStoreChange {
    pub fn Appointment(&self) -> windows_core::Result<Appointment> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Appointment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChangeType(&self) -> windows_core::Result<AppointmentStoreChangeType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChangeType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AppointmentCalendar(&self) -> windows_core::Result<AppointmentCalendar> {
        let this = &windows_core::Interface::cast::<IAppointmentStoreChange2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppointmentCalendar)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppointmentStoreChange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentStoreChange>();
}
unsafe impl windows_core::Interface for AppointmentStoreChange {
    type Vtable = IAppointmentStoreChange_Vtbl;
    const IID: windows_core::GUID = <IAppointmentStoreChange as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentStoreChange {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreChange";
}
unsafe impl Send for AppointmentStoreChange {}
unsafe impl Sync for AppointmentStoreChange {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppointmentStoreChangeReader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentStoreChangeReader, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentStoreChangeReader {
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentStoreChange>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadBatchAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AcceptChanges(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AcceptChanges)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn AcceptChangesThrough<P0>(&self, lastchangetoaccept: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AppointmentStoreChange>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AcceptChangesThrough)(windows_core::Interface::as_raw(this), lastchangetoaccept.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for AppointmentStoreChangeReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentStoreChangeReader>();
}
unsafe impl windows_core::Interface for AppointmentStoreChangeReader {
    type Vtable = IAppointmentStoreChangeReader_Vtbl;
    const IID: windows_core::GUID = <IAppointmentStoreChangeReader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentStoreChangeReader {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreChangeReader";
}
unsafe impl Send for AppointmentStoreChangeReader {}
unsafe impl Sync for AppointmentStoreChangeReader {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppointmentStoreChangeTracker(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentStoreChangeTracker, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentStoreChangeTracker {
    pub fn GetChangeReader(&self) -> windows_core::Result<AppointmentStoreChangeReader> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChangeReader)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Enable(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Enable)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Reset)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsTracking(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppointmentStoreChangeTracker2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTracking)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppointmentStoreChangeTracker {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentStoreChangeTracker>();
}
unsafe impl windows_core::Interface for AppointmentStoreChangeTracker {
    type Vtable = IAppointmentStoreChangeTracker_Vtbl;
    const IID: windows_core::GUID = <IAppointmentStoreChangeTracker as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentStoreChangeTracker {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreChangeTracker";
}
unsafe impl Send for AppointmentStoreChangeTracker {}
unsafe impl Sync for AppointmentStoreChangeTracker {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppointmentStoreChangedDeferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentStoreChangedDeferral, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentStoreChangedDeferral {
    pub fn Complete(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for AppointmentStoreChangedDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentStoreChangedDeferral>();
}
unsafe impl windows_core::Interface for AppointmentStoreChangedDeferral {
    type Vtable = IAppointmentStoreChangedDeferral_Vtbl;
    const IID: windows_core::GUID = <IAppointmentStoreChangedDeferral as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentStoreChangedDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreChangedDeferral";
}
unsafe impl Send for AppointmentStoreChangedDeferral {}
unsafe impl Sync for AppointmentStoreChangedDeferral {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppointmentStoreChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentStoreChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentStoreChangedEventArgs {
    pub fn GetDeferral(&self) -> windows_core::Result<AppointmentStoreChangedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppointmentStoreChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentStoreChangedEventArgs>();
}
unsafe impl windows_core::Interface for AppointmentStoreChangedEventArgs {
    type Vtable = IAppointmentStoreChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAppointmentStoreChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentStoreChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreChangedEventArgs";
}
unsafe impl Send for AppointmentStoreChangedEventArgs {}
unsafe impl Sync for AppointmentStoreChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppointmentStoreNotificationTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentStoreNotificationTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentStoreNotificationTriggerDetails {}
impl windows_core::RuntimeType for AppointmentStoreNotificationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentStoreNotificationTriggerDetails>();
}
unsafe impl windows_core::Interface for AppointmentStoreNotificationTriggerDetails {
    type Vtable = IAppointmentStoreNotificationTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IAppointmentStoreNotificationTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentStoreNotificationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreNotificationTriggerDetails";
}
unsafe impl Send for AppointmentStoreNotificationTriggerDetails {}
unsafe impl Sync for AppointmentStoreNotificationTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FindAppointmentsOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FindAppointmentsOptions, windows_core::IUnknown, windows_core::IInspectable);
impl FindAppointmentsOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FindAppointmentsOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CalendarIds(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CalendarIds)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FetchProperties(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FetchProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IncludeHidden(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncludeHidden)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIncludeHidden(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIncludeHidden)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxCount(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaxCount)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for FindAppointmentsOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFindAppointmentsOptions>();
}
unsafe impl windows_core::Interface for FindAppointmentsOptions {
    type Vtable = IFindAppointmentsOptions_Vtbl;
    const IID: windows_core::GUID = <IFindAppointmentsOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FindAppointmentsOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.FindAppointmentsOptions";
}
unsafe impl Send for FindAppointmentsOptions {}
unsafe impl Sync for FindAppointmentsOptions {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppointmentBusyStatus(pub i32);
impl AppointmentBusyStatus {
    pub const Busy: Self = Self(0i32);
    pub const Tentative: Self = Self(1i32);
    pub const Free: Self = Self(2i32);
    pub const OutOfOffice: Self = Self(3i32);
    pub const WorkingElsewhere: Self = Self(4i32);
}
impl windows_core::TypeKind for AppointmentBusyStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppointmentBusyStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppointmentBusyStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppointmentBusyStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentBusyStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppointmentCalendarOtherAppReadAccess(pub i32);
impl AppointmentCalendarOtherAppReadAccess {
    pub const SystemOnly: Self = Self(0i32);
    pub const Limited: Self = Self(1i32);
    pub const Full: Self = Self(2i32);
    pub const None: Self = Self(3i32);
}
impl windows_core::TypeKind for AppointmentCalendarOtherAppReadAccess {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppointmentCalendarOtherAppReadAccess {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppointmentCalendarOtherAppReadAccess").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppointmentCalendarOtherAppReadAccess {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentCalendarOtherAppReadAccess;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppointmentCalendarOtherAppWriteAccess(pub i32);
impl AppointmentCalendarOtherAppWriteAccess {
    pub const None: Self = Self(0i32);
    pub const SystemOnly: Self = Self(1i32);
    pub const Limited: Self = Self(2i32);
}
impl windows_core::TypeKind for AppointmentCalendarOtherAppWriteAccess {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppointmentCalendarOtherAppWriteAccess {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppointmentCalendarOtherAppWriteAccess").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppointmentCalendarOtherAppWriteAccess {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentCalendarOtherAppWriteAccess;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppointmentCalendarSyncStatus(pub i32);
impl AppointmentCalendarSyncStatus {
    pub const Idle: Self = Self(0i32);
    pub const Syncing: Self = Self(1i32);
    pub const UpToDate: Self = Self(2i32);
    pub const AuthenticationError: Self = Self(3i32);
    pub const PolicyError: Self = Self(4i32);
    pub const UnknownError: Self = Self(5i32);
    pub const ManualAccountRemovalRequired: Self = Self(6i32);
}
impl windows_core::TypeKind for AppointmentCalendarSyncStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppointmentCalendarSyncStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppointmentCalendarSyncStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppointmentCalendarSyncStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentCalendarSyncStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppointmentConflictType(pub i32);
impl AppointmentConflictType {
    pub const None: Self = Self(0i32);
    pub const Adjacent: Self = Self(1i32);
    pub const Overlap: Self = Self(2i32);
}
impl windows_core::TypeKind for AppointmentConflictType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppointmentConflictType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppointmentConflictType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppointmentConflictType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentConflictType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppointmentDaysOfWeek(pub u32);
impl AppointmentDaysOfWeek {
    pub const None: Self = Self(0u32);
    pub const Sunday: Self = Self(1u32);
    pub const Monday: Self = Self(2u32);
    pub const Tuesday: Self = Self(4u32);
    pub const Wednesday: Self = Self(8u32);
    pub const Thursday: Self = Self(16u32);
    pub const Friday: Self = Self(32u32);
    pub const Saturday: Self = Self(64u32);
}
impl windows_core::TypeKind for AppointmentDaysOfWeek {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppointmentDaysOfWeek {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppointmentDaysOfWeek").field(&self.0).finish()
    }
}
impl AppointmentDaysOfWeek {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for AppointmentDaysOfWeek {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for AppointmentDaysOfWeek {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for AppointmentDaysOfWeek {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for AppointmentDaysOfWeek {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for AppointmentDaysOfWeek {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for AppointmentDaysOfWeek {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentDaysOfWeek;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppointmentDetailsKind(pub i32);
impl AppointmentDetailsKind {
    pub const PlainText: Self = Self(0i32);
    pub const Html: Self = Self(1i32);
}
impl windows_core::TypeKind for AppointmentDetailsKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppointmentDetailsKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppointmentDetailsKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppointmentDetailsKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentDetailsKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppointmentParticipantResponse(pub i32);
impl AppointmentParticipantResponse {
    pub const None: Self = Self(0i32);
    pub const Tentative: Self = Self(1i32);
    pub const Accepted: Self = Self(2i32);
    pub const Declined: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
}
impl windows_core::TypeKind for AppointmentParticipantResponse {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppointmentParticipantResponse {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppointmentParticipantResponse").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppointmentParticipantResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentParticipantResponse;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppointmentParticipantRole(pub i32);
impl AppointmentParticipantRole {
    pub const RequiredAttendee: Self = Self(0i32);
    pub const OptionalAttendee: Self = Self(1i32);
    pub const Resource: Self = Self(2i32);
}
impl windows_core::TypeKind for AppointmentParticipantRole {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppointmentParticipantRole {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppointmentParticipantRole").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppointmentParticipantRole {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentParticipantRole;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppointmentRecurrenceUnit(pub i32);
impl AppointmentRecurrenceUnit {
    pub const Daily: Self = Self(0i32);
    pub const Weekly: Self = Self(1i32);
    pub const Monthly: Self = Self(2i32);
    pub const MonthlyOnDay: Self = Self(3i32);
    pub const Yearly: Self = Self(4i32);
    pub const YearlyOnDay: Self = Self(5i32);
}
impl windows_core::TypeKind for AppointmentRecurrenceUnit {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppointmentRecurrenceUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppointmentRecurrenceUnit").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppointmentRecurrenceUnit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentRecurrenceUnit;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppointmentSensitivity(pub i32);
impl AppointmentSensitivity {
    pub const Public: Self = Self(0i32);
    pub const Private: Self = Self(1i32);
}
impl windows_core::TypeKind for AppointmentSensitivity {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppointmentSensitivity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppointmentSensitivity").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppointmentSensitivity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentSensitivity;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppointmentStoreAccessType(pub i32);
impl AppointmentStoreAccessType {
    pub const AppCalendarsReadWrite: Self = Self(0i32);
    pub const AllCalendarsReadOnly: Self = Self(1i32);
    pub const AllCalendarsReadWrite: Self = Self(2i32);
}
impl windows_core::TypeKind for AppointmentStoreAccessType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppointmentStoreAccessType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppointmentStoreAccessType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppointmentStoreAccessType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentStoreAccessType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppointmentStoreChangeType(pub i32);
impl AppointmentStoreChangeType {
    pub const AppointmentCreated: Self = Self(0i32);
    pub const AppointmentModified: Self = Self(1i32);
    pub const AppointmentDeleted: Self = Self(2i32);
    pub const ChangeTrackingLost: Self = Self(3i32);
    pub const CalendarCreated: Self = Self(4i32);
    pub const CalendarModified: Self = Self(5i32);
    pub const CalendarDeleted: Self = Self(6i32);
}
impl windows_core::TypeKind for AppointmentStoreChangeType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppointmentStoreChangeType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppointmentStoreChangeType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppointmentStoreChangeType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentStoreChangeType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppointmentSummaryCardView(pub i32);
impl AppointmentSummaryCardView {
    pub const System: Self = Self(0i32);
    pub const App: Self = Self(1i32);
}
impl windows_core::TypeKind for AppointmentSummaryCardView {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppointmentSummaryCardView {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppointmentSummaryCardView").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppointmentSummaryCardView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentSummaryCardView;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppointmentWeekOfMonth(pub i32);
impl AppointmentWeekOfMonth {
    pub const First: Self = Self(0i32);
    pub const Second: Self = Self(1i32);
    pub const Third: Self = Self(2i32);
    pub const Fourth: Self = Self(3i32);
    pub const Last: Self = Self(4i32);
}
impl windows_core::TypeKind for AppointmentWeekOfMonth {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppointmentWeekOfMonth {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppointmentWeekOfMonth").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppointmentWeekOfMonth {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentWeekOfMonth;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FindAppointmentCalendarsOptions(pub u32);
impl FindAppointmentCalendarsOptions {
    pub const None: Self = Self(0u32);
    pub const IncludeHidden: Self = Self(1u32);
}
impl windows_core::TypeKind for FindAppointmentCalendarsOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FindAppointmentCalendarsOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FindAppointmentCalendarsOptions").field(&self.0).finish()
    }
}
impl FindAppointmentCalendarsOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for FindAppointmentCalendarsOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for FindAppointmentCalendarsOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for FindAppointmentCalendarsOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for FindAppointmentCalendarsOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for FindAppointmentCalendarsOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for FindAppointmentCalendarsOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.FindAppointmentCalendarsOptions;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RecurrenceType(pub i32);
impl RecurrenceType {
    pub const Master: Self = Self(0i32);
    pub const Instance: Self = Self(1i32);
    pub const ExceptionInstance: Self = Self(2i32);
}
impl windows_core::TypeKind for RecurrenceType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RecurrenceType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RecurrenceType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for RecurrenceType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.RecurrenceType;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
