windows_core::imp::define_interface!(IWaaSAssessor, IWaaSAssessor_Vtbl, 0x2347bbef_1a3b_45a4_902d_3e09c269b45e);
windows_core::imp::interface_hierarchy!(IWaaSAssessor, windows_core::IUnknown);
impl IWaaSAssessor {
    pub unsafe fn GetOSUpdateAssessment(&self) -> windows_core::Result<OSUpdateAssessment> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOSUpdateAssessment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWaaSAssessor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOSUpdateAssessment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OSUpdateAssessment) -> windows_core::HRESULT,
}
pub trait IWaaSAssessor_Impl: windows_core::IUnknownImpl {
    fn GetOSUpdateAssessment(&self) -> windows_core::Result<OSUpdateAssessment>;
}
impl IWaaSAssessor_Vtbl {
    pub const fn new<Identity: IWaaSAssessor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOSUpdateAssessment<Identity: IWaaSAssessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result: *mut OSUpdateAssessment) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWaaSAssessor_Impl::GetOSUpdateAssessment(this) {
                    Ok(ok__) => {
                        result.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetOSUpdateAssessment: GetOSUpdateAssessment::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWaaSAssessor as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWaaSAssessor {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OSUpdateAssessment {
    pub isEndOfSupport: windows_core::BOOL,
    pub assessmentForCurrent: UpdateAssessment,
    pub assessmentForUpToDate: UpdateAssessment,
    pub securityStatus: UpdateAssessmentStatus,
    pub assessmentTime: super::super::Foundation::FILETIME,
    pub releaseInfoTime: super::super::Foundation::FILETIME,
    pub currentOSBuild: windows_core::PWSTR,
    pub currentOSReleaseTime: super::super::Foundation::FILETIME,
    pub upToDateOSBuild: windows_core::PWSTR,
    pub upToDateOSReleaseTime: super::super::Foundation::FILETIME,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UpdateAssessment {
    pub status: UpdateAssessmentStatus,
    pub impact: UpdateImpactLevel,
    pub daysOutOfDate: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UpdateAssessmentStatus(pub i32);
pub const UpdateAssessmentStatus_Latest: UpdateAssessmentStatus = UpdateAssessmentStatus(0i32);
pub const UpdateAssessmentStatus_NotLatestDeferredFeature: UpdateAssessmentStatus = UpdateAssessmentStatus(5i32);
pub const UpdateAssessmentStatus_NotLatestDeferredQuality: UpdateAssessmentStatus = UpdateAssessmentStatus(6i32);
pub const UpdateAssessmentStatus_NotLatestEndOfSupport: UpdateAssessmentStatus = UpdateAssessmentStatus(3i32);
pub const UpdateAssessmentStatus_NotLatestHardRestriction: UpdateAssessmentStatus = UpdateAssessmentStatus(2i32);
pub const UpdateAssessmentStatus_NotLatestManaged: UpdateAssessmentStatus = UpdateAssessmentStatus(9i32);
pub const UpdateAssessmentStatus_NotLatestPausedFeature: UpdateAssessmentStatus = UpdateAssessmentStatus(7i32);
pub const UpdateAssessmentStatus_NotLatestPausedQuality: UpdateAssessmentStatus = UpdateAssessmentStatus(8i32);
pub const UpdateAssessmentStatus_NotLatestServicingTrain: UpdateAssessmentStatus = UpdateAssessmentStatus(4i32);
pub const UpdateAssessmentStatus_NotLatestSoftRestriction: UpdateAssessmentStatus = UpdateAssessmentStatus(1i32);
pub const UpdateAssessmentStatus_NotLatestTargetedVersion: UpdateAssessmentStatus = UpdateAssessmentStatus(11i32);
pub const UpdateAssessmentStatus_NotLatestUnknown: UpdateAssessmentStatus = UpdateAssessmentStatus(10i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UpdateImpactLevel(pub i32);
pub const UpdateImpactLevel_High: UpdateImpactLevel = UpdateImpactLevel(3i32);
pub const UpdateImpactLevel_Low: UpdateImpactLevel = UpdateImpactLevel(1i32);
pub const UpdateImpactLevel_Medium: UpdateImpactLevel = UpdateImpactLevel(2i32);
pub const UpdateImpactLevel_None: UpdateImpactLevel = UpdateImpactLevel(0i32);
pub const WaaSAssessor: windows_core::GUID = windows_core::GUID::from_u128(0x098ef871_fa9f_46af_8958_c083515d7c9c);
