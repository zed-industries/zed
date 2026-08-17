windows_core::imp::define_interface!(IAppointmentCalendarCancelMeetingRequest, IAppointmentCalendarCancelMeetingRequest_Vtbl, 0x49460f8d_6434_40d7_ad46_6297419314d1);
impl windows_core::RuntimeType for IAppointmentCalendarCancelMeetingRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentCalendarCancelMeetingRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AppointmentCalendarLocalId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AppointmentLocalId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AppointmentOriginalStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Subject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Comment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub NotifyInvitees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentCalendarCancelMeetingRequestEventArgs, IAppointmentCalendarCancelMeetingRequestEventArgs_Vtbl, 0x1a79be16_7f30_4e35_beef_9d2c7b6dcae1);
impl windows_core::RuntimeType for IAppointmentCalendarCancelMeetingRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentCalendarCancelMeetingRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentCalendarCreateOrUpdateAppointmentRequest, IAppointmentCalendarCreateOrUpdateAppointmentRequest_Vtbl, 0x2e62f2b2_ca96_48ac_9124_406b19fefa70);
impl windows_core::RuntimeType for IAppointmentCalendarCreateOrUpdateAppointmentRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentCalendarCreateOrUpdateAppointmentRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AppointmentCalendarLocalId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Appointment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyInvitees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ChangedProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ChangedProperties: usize,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs, IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_Vtbl, 0xcf8ded28_002e_4bf7_8e9d_5e20d49aa3ba);
impl windows_core::RuntimeType for IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentCalendarForwardMeetingRequest, IAppointmentCalendarForwardMeetingRequest_Vtbl, 0x82e5ee56_26b6_4253_8a8f_6cf5f2ff7884);
impl windows_core::RuntimeType for IAppointmentCalendarForwardMeetingRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentCalendarForwardMeetingRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AppointmentCalendarLocalId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AppointmentLocalId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AppointmentOriginalStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Invitees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Invitees: usize,
    pub Subject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ForwardHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Comment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentCalendarForwardMeetingRequestEventArgs, IAppointmentCalendarForwardMeetingRequestEventArgs_Vtbl, 0x3109151a_23a2_42fd_9c82_c9a60d59f8a8);
impl windows_core::RuntimeType for IAppointmentCalendarForwardMeetingRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentCalendarForwardMeetingRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentCalendarProposeNewTimeForMeetingRequest, IAppointmentCalendarProposeNewTimeForMeetingRequest_Vtbl, 0xce1c63f5_edf6_43c3_82b7_be6b368c6900);
impl windows_core::RuntimeType for IAppointmentCalendarProposeNewTimeForMeetingRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentCalendarProposeNewTimeForMeetingRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AppointmentCalendarLocalId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AppointmentLocalId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AppointmentOriginalStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NewStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub NewDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Subject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Comment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs, IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_Vtbl, 0xd2d777d8_fed1_4280_a3ba_2e1f47609aa2);
impl windows_core::RuntimeType for IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentCalendarSyncManagerSyncRequest, IAppointmentCalendarSyncManagerSyncRequest_Vtbl, 0x12ab382b_7163_4a56_9a4e_7223a84adf46);
impl windows_core::RuntimeType for IAppointmentCalendarSyncManagerSyncRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentCalendarSyncManagerSyncRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AppointmentCalendarLocalId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentCalendarSyncManagerSyncRequestEventArgs, IAppointmentCalendarSyncManagerSyncRequestEventArgs_Vtbl, 0xca17c6f7_0284_4edd_87ba_4d8f69dcf5c0);
impl windows_core::RuntimeType for IAppointmentCalendarSyncManagerSyncRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentCalendarSyncManagerSyncRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentCalendarUpdateMeetingResponseRequest, IAppointmentCalendarUpdateMeetingResponseRequest_Vtbl, 0xa36d608c_c29d_4b94_b086_7e9ff7bd84a0);
impl windows_core::RuntimeType for IAppointmentCalendarUpdateMeetingResponseRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentCalendarUpdateMeetingResponseRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AppointmentCalendarLocalId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AppointmentLocalId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AppointmentOriginalStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Response: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::AppointmentParticipantResponse) -> windows_core::HRESULT,
    pub Subject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Comment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SendUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentCalendarUpdateMeetingResponseRequestEventArgs, IAppointmentCalendarUpdateMeetingResponseRequestEventArgs_Vtbl, 0x88759883_97bf_479d_aed5_0be8ce567d1e);
impl windows_core::RuntimeType for IAppointmentCalendarUpdateMeetingResponseRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentCalendarUpdateMeetingResponseRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentDataProviderConnection, IAppointmentDataProviderConnection_Vtbl, 0xf3dd9d83_3254_465f_abdb_928046552cf4);
impl windows_core::RuntimeType for IAppointmentDataProviderConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentDataProviderConnection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SyncRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSyncRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CreateOrUpdateAppointmentRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCreateOrUpdateAppointmentRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CancelMeetingRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCancelMeetingRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ForwardMeetingRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveForwardMeetingRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ProposeNewTimeForMeetingRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveProposeNewTimeForMeetingRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub UpdateMeetingResponseRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveUpdateMeetingResponseRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentDataProviderTriggerDetails, IAppointmentDataProviderTriggerDetails_Vtbl, 0xb3283c01_7e12_4e5e_b1ef_74fb68ac6f2a);
impl windows_core::RuntimeType for IAppointmentDataProviderTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentDataProviderTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Connection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppointmentCalendarCancelMeetingRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentCalendarCancelMeetingRequest, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentCalendarCancelMeetingRequest {
    pub fn AppointmentCalendarLocalId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppointmentCalendarLocalId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppointmentLocalId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppointmentLocalId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppointmentOriginalStartTime(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppointmentOriginalStartTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    pub fn NotifyInvitees(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NotifyInvitees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
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
impl windows_core::RuntimeType for AppointmentCalendarCancelMeetingRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentCalendarCancelMeetingRequest>();
}
unsafe impl windows_core::Interface for AppointmentCalendarCancelMeetingRequest {
    type Vtable = IAppointmentCalendarCancelMeetingRequest_Vtbl;
    const IID: windows_core::GUID = <IAppointmentCalendarCancelMeetingRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentCalendarCancelMeetingRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCancelMeetingRequest";
}
unsafe impl Send for AppointmentCalendarCancelMeetingRequest {}
unsafe impl Sync for AppointmentCalendarCancelMeetingRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppointmentCalendarCancelMeetingRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentCalendarCancelMeetingRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentCalendarCancelMeetingRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<AppointmentCalendarCancelMeetingRequest> {
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
impl windows_core::RuntimeType for AppointmentCalendarCancelMeetingRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentCalendarCancelMeetingRequestEventArgs>();
}
unsafe impl windows_core::Interface for AppointmentCalendarCancelMeetingRequestEventArgs {
    type Vtable = IAppointmentCalendarCancelMeetingRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAppointmentCalendarCancelMeetingRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentCalendarCancelMeetingRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCancelMeetingRequestEventArgs";
}
unsafe impl Send for AppointmentCalendarCancelMeetingRequestEventArgs {}
unsafe impl Sync for AppointmentCalendarCancelMeetingRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppointmentCalendarCreateOrUpdateAppointmentRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentCalendarCreateOrUpdateAppointmentRequest, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentCalendarCreateOrUpdateAppointmentRequest {
    pub fn AppointmentCalendarLocalId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppointmentCalendarLocalId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Appointment(&self) -> windows_core::Result<super::Appointment> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Appointment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NotifyInvitees(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NotifyInvitees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ChangedProperties(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChangedProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompletedAsync<P0>(&self, createdorupdatedappointment: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportCompletedAsync)(windows_core::Interface::as_raw(this), createdorupdatedappointment.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
impl windows_core::RuntimeType for AppointmentCalendarCreateOrUpdateAppointmentRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentCalendarCreateOrUpdateAppointmentRequest>();
}
unsafe impl windows_core::Interface for AppointmentCalendarCreateOrUpdateAppointmentRequest {
    type Vtable = IAppointmentCalendarCreateOrUpdateAppointmentRequest_Vtbl;
    const IID: windows_core::GUID = <IAppointmentCalendarCreateOrUpdateAppointmentRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentCalendarCreateOrUpdateAppointmentRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCreateOrUpdateAppointmentRequest";
}
unsafe impl Send for AppointmentCalendarCreateOrUpdateAppointmentRequest {}
unsafe impl Sync for AppointmentCalendarCreateOrUpdateAppointmentRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<AppointmentCalendarCreateOrUpdateAppointmentRequest> {
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
impl windows_core::RuntimeType for AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs>();
}
unsafe impl windows_core::Interface for AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {
    type Vtable = IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs";
}
unsafe impl Send for AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {}
unsafe impl Sync for AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppointmentCalendarForwardMeetingRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentCalendarForwardMeetingRequest, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentCalendarForwardMeetingRequest {
    pub fn AppointmentCalendarLocalId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppointmentCalendarLocalId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppointmentLocalId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppointmentLocalId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppointmentOriginalStartTime(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppointmentOriginalStartTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Invitees(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<super::AppointmentInvitee>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Invitees)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Subject(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Subject)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
impl windows_core::RuntimeType for AppointmentCalendarForwardMeetingRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentCalendarForwardMeetingRequest>();
}
unsafe impl windows_core::Interface for AppointmentCalendarForwardMeetingRequest {
    type Vtable = IAppointmentCalendarForwardMeetingRequest_Vtbl;
    const IID: windows_core::GUID = <IAppointmentCalendarForwardMeetingRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentCalendarForwardMeetingRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarForwardMeetingRequest";
}
unsafe impl Send for AppointmentCalendarForwardMeetingRequest {}
unsafe impl Sync for AppointmentCalendarForwardMeetingRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppointmentCalendarForwardMeetingRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentCalendarForwardMeetingRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentCalendarForwardMeetingRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<AppointmentCalendarForwardMeetingRequest> {
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
impl windows_core::RuntimeType for AppointmentCalendarForwardMeetingRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentCalendarForwardMeetingRequestEventArgs>();
}
unsafe impl windows_core::Interface for AppointmentCalendarForwardMeetingRequestEventArgs {
    type Vtable = IAppointmentCalendarForwardMeetingRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAppointmentCalendarForwardMeetingRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentCalendarForwardMeetingRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarForwardMeetingRequestEventArgs";
}
unsafe impl Send for AppointmentCalendarForwardMeetingRequestEventArgs {}
unsafe impl Sync for AppointmentCalendarForwardMeetingRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppointmentCalendarProposeNewTimeForMeetingRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentCalendarProposeNewTimeForMeetingRequest, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentCalendarProposeNewTimeForMeetingRequest {
    pub fn AppointmentCalendarLocalId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppointmentCalendarLocalId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppointmentLocalId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppointmentLocalId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppointmentOriginalStartTime(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppointmentOriginalStartTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
impl windows_core::RuntimeType for AppointmentCalendarProposeNewTimeForMeetingRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentCalendarProposeNewTimeForMeetingRequest>();
}
unsafe impl windows_core::Interface for AppointmentCalendarProposeNewTimeForMeetingRequest {
    type Vtable = IAppointmentCalendarProposeNewTimeForMeetingRequest_Vtbl;
    const IID: windows_core::GUID = <IAppointmentCalendarProposeNewTimeForMeetingRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentCalendarProposeNewTimeForMeetingRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarProposeNewTimeForMeetingRequest";
}
unsafe impl Send for AppointmentCalendarProposeNewTimeForMeetingRequest {}
unsafe impl Sync for AppointmentCalendarProposeNewTimeForMeetingRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<AppointmentCalendarProposeNewTimeForMeetingRequest> {
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
impl windows_core::RuntimeType for AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs>();
}
unsafe impl windows_core::Interface for AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {
    type Vtable = IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs";
}
unsafe impl Send for AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {}
unsafe impl Sync for AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppointmentCalendarSyncManagerSyncRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentCalendarSyncManagerSyncRequest, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentCalendarSyncManagerSyncRequest {
    pub fn AppointmentCalendarLocalId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppointmentCalendarLocalId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
impl windows_core::RuntimeType for AppointmentCalendarSyncManagerSyncRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentCalendarSyncManagerSyncRequest>();
}
unsafe impl windows_core::Interface for AppointmentCalendarSyncManagerSyncRequest {
    type Vtable = IAppointmentCalendarSyncManagerSyncRequest_Vtbl;
    const IID: windows_core::GUID = <IAppointmentCalendarSyncManagerSyncRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentCalendarSyncManagerSyncRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarSyncManagerSyncRequest";
}
unsafe impl Send for AppointmentCalendarSyncManagerSyncRequest {}
unsafe impl Sync for AppointmentCalendarSyncManagerSyncRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppointmentCalendarSyncManagerSyncRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentCalendarSyncManagerSyncRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentCalendarSyncManagerSyncRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<AppointmentCalendarSyncManagerSyncRequest> {
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
impl windows_core::RuntimeType for AppointmentCalendarSyncManagerSyncRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentCalendarSyncManagerSyncRequestEventArgs>();
}
unsafe impl windows_core::Interface for AppointmentCalendarSyncManagerSyncRequestEventArgs {
    type Vtable = IAppointmentCalendarSyncManagerSyncRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAppointmentCalendarSyncManagerSyncRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentCalendarSyncManagerSyncRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarSyncManagerSyncRequestEventArgs";
}
unsafe impl Send for AppointmentCalendarSyncManagerSyncRequestEventArgs {}
unsafe impl Sync for AppointmentCalendarSyncManagerSyncRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppointmentCalendarUpdateMeetingResponseRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentCalendarUpdateMeetingResponseRequest, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentCalendarUpdateMeetingResponseRequest {
    pub fn AppointmentCalendarLocalId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppointmentCalendarLocalId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppointmentLocalId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppointmentLocalId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppointmentOriginalStartTime(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppointmentOriginalStartTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Response(&self) -> windows_core::Result<super::AppointmentParticipantResponse> {
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
impl windows_core::RuntimeType for AppointmentCalendarUpdateMeetingResponseRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentCalendarUpdateMeetingResponseRequest>();
}
unsafe impl windows_core::Interface for AppointmentCalendarUpdateMeetingResponseRequest {
    type Vtable = IAppointmentCalendarUpdateMeetingResponseRequest_Vtbl;
    const IID: windows_core::GUID = <IAppointmentCalendarUpdateMeetingResponseRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentCalendarUpdateMeetingResponseRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarUpdateMeetingResponseRequest";
}
unsafe impl Send for AppointmentCalendarUpdateMeetingResponseRequest {}
unsafe impl Sync for AppointmentCalendarUpdateMeetingResponseRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppointmentCalendarUpdateMeetingResponseRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentCalendarUpdateMeetingResponseRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentCalendarUpdateMeetingResponseRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<AppointmentCalendarUpdateMeetingResponseRequest> {
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
impl windows_core::RuntimeType for AppointmentCalendarUpdateMeetingResponseRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentCalendarUpdateMeetingResponseRequestEventArgs>();
}
unsafe impl windows_core::Interface for AppointmentCalendarUpdateMeetingResponseRequestEventArgs {
    type Vtable = IAppointmentCalendarUpdateMeetingResponseRequestEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAppointmentCalendarUpdateMeetingResponseRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentCalendarUpdateMeetingResponseRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarUpdateMeetingResponseRequestEventArgs";
}
unsafe impl Send for AppointmentCalendarUpdateMeetingResponseRequestEventArgs {}
unsafe impl Sync for AppointmentCalendarUpdateMeetingResponseRequestEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppointmentDataProviderConnection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentDataProviderConnection, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentDataProviderConnection {
    pub fn SyncRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<AppointmentDataProviderConnection, AppointmentCalendarSyncManagerSyncRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SyncRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSyncRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSyncRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CreateOrUpdateAppointmentRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<AppointmentDataProviderConnection, AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateOrUpdateAppointmentRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCreateOrUpdateAppointmentRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCreateOrUpdateAppointmentRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CancelMeetingRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<AppointmentDataProviderConnection, AppointmentCalendarCancelMeetingRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelMeetingRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCancelMeetingRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCancelMeetingRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ForwardMeetingRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<AppointmentDataProviderConnection, AppointmentCalendarForwardMeetingRequestEventArgs>>,
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
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<AppointmentDataProviderConnection, AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs>>,
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
    pub fn UpdateMeetingResponseRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<AppointmentDataProviderConnection, AppointmentCalendarUpdateMeetingResponseRequestEventArgs>>,
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
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for AppointmentDataProviderConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentDataProviderConnection>();
}
unsafe impl windows_core::Interface for AppointmentDataProviderConnection {
    type Vtable = IAppointmentDataProviderConnection_Vtbl;
    const IID: windows_core::GUID = <IAppointmentDataProviderConnection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentDataProviderConnection {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderConnection";
}
unsafe impl Send for AppointmentDataProviderConnection {}
unsafe impl Sync for AppointmentDataProviderConnection {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppointmentDataProviderTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentDataProviderTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl AppointmentDataProviderTriggerDetails {
    pub fn Connection(&self) -> windows_core::Result<AppointmentDataProviderConnection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Connection)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppointmentDataProviderTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentDataProviderTriggerDetails>();
}
unsafe impl windows_core::Interface for AppointmentDataProviderTriggerDetails {
    type Vtable = IAppointmentDataProviderTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IAppointmentDataProviderTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentDataProviderTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderTriggerDetails";
}
unsafe impl Send for AppointmentDataProviderTriggerDetails {}
unsafe impl Sync for AppointmentDataProviderTriggerDetails {}
