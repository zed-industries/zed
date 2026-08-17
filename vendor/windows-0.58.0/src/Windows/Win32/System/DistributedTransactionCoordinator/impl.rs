pub trait IDtcLuConfigure_Impl: Sized {
    fn Add(&self, puclupair: *const u8, cblupair: u32) -> windows_core::Result<()>;
    fn Delete(&self, puclupair: *const u8, cblupair: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDtcLuConfigure {}
impl IDtcLuConfigure_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDtcLuConfigure_Vtbl
    where
        Identity: IDtcLuConfigure_Impl,
    {
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puclupair: *const u8, cblupair: u32) -> windows_core::HRESULT
        where
            Identity: IDtcLuConfigure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuConfigure_Impl::Add(this, core::mem::transmute_copy(&puclupair), core::mem::transmute_copy(&cblupair)).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puclupair: *const u8, cblupair: u32) -> windows_core::HRESULT
        where
            Identity: IDtcLuConfigure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuConfigure_Impl::Delete(this, core::mem::transmute_copy(&puclupair), core::mem::transmute_copy(&cblupair)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Add: Add::<Identity, OFFSET>, Delete: Delete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuConfigure as windows_core::Interface>::IID
    }
}
pub trait IDtcLuRecovery_Impl: Sized {}
impl windows_core::RuntimeName for IDtcLuRecovery {}
impl IDtcLuRecovery_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDtcLuRecovery_Vtbl
    where
        Identity: IDtcLuRecovery_Impl,
    {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRecovery as windows_core::Interface>::IID
    }
}
pub trait IDtcLuRecoveryFactory_Impl: Sized {
    fn Create(&self, puclupair: *const u8, cblupair: u32) -> windows_core::Result<IDtcLuRecovery>;
}
impl windows_core::RuntimeName for IDtcLuRecoveryFactory {}
impl IDtcLuRecoveryFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDtcLuRecoveryFactory_Vtbl
    where
        Identity: IDtcLuRecoveryFactory_Impl,
    {
        unsafe extern "system" fn Create<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puclupair: *const u8, cblupair: u32, pprecovery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuRecoveryFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDtcLuRecoveryFactory_Impl::Create(this, core::mem::transmute_copy(&puclupair), core::mem::transmute_copy(&cblupair)) {
                Ok(ok__) => {
                    pprecovery.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRecoveryFactory as windows_core::Interface>::IID
    }
}
pub trait IDtcLuRecoveryInitiatedByDtc_Impl: Sized {
    fn GetWork(&self, pwork: *mut DTCINITIATEDRECOVERYWORK, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDtcLuRecoveryInitiatedByDtc {}
impl IDtcLuRecoveryInitiatedByDtc_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDtcLuRecoveryInitiatedByDtc_Vtbl
    where
        Identity: IDtcLuRecoveryInitiatedByDtc_Impl,
    {
        unsafe extern "system" fn GetWork<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwork: *mut DTCINITIATEDRECOVERYWORK, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuRecoveryInitiatedByDtc_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRecoveryInitiatedByDtc_Impl::GetWork(this, core::mem::transmute_copy(&pwork), core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetWork: GetWork::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByDtc as windows_core::Interface>::IID
    }
}
pub trait IDtcLuRecoveryInitiatedByDtcStatusWork_Impl: Sized {
    fn HandleCheckLuStatus(&self, lrecoveryseqnum: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDtcLuRecoveryInitiatedByDtcStatusWork {}
impl IDtcLuRecoveryInitiatedByDtcStatusWork_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDtcLuRecoveryInitiatedByDtcStatusWork_Vtbl
    where
        Identity: IDtcLuRecoveryInitiatedByDtcStatusWork_Impl,
    {
        unsafe extern "system" fn HandleCheckLuStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lrecoveryseqnum: i32) -> windows_core::HRESULT
        where
            Identity: IDtcLuRecoveryInitiatedByDtcStatusWork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRecoveryInitiatedByDtcStatusWork_Impl::HandleCheckLuStatus(this, core::mem::transmute_copy(&lrecoveryseqnum)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HandleCheckLuStatus: HandleCheckLuStatus::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByDtcStatusWork as windows_core::Interface>::IID
    }
}
pub trait IDtcLuRecoveryInitiatedByDtcTransWork_Impl: Sized {
    fn GetLogNameSizes(&self, pcbourlogname: *mut u32, pcbremotelogname: *mut u32) -> windows_core::Result<()>;
    fn GetOurXln(&self, pxln: *mut DTCLUXLN, pourlogname: *mut u8, premotelogname: *mut u8, pdwprotocol: *mut u32) -> windows_core::Result<()>;
    fn HandleConfirmationFromOurXln(&self, confirmation: DTCLUXLNCONFIRMATION) -> windows_core::Result<()>;
    fn HandleTheirXlnResponse(&self, xln: DTCLUXLN, premotelogname: *mut u8, cbremotelogname: u32, dwprotocol: u32, pconfirmation: *mut DTCLUXLNCONFIRMATION) -> windows_core::Result<()>;
    fn HandleErrorFromOurXln(&self, error: DTCLUXLNERROR) -> windows_core::Result<()>;
    fn CheckForCompareStates(&self, fcomparestates: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetOurTransIdSize(&self, pcbourtransid: *mut u32) -> windows_core::Result<()>;
    fn GetOurCompareStates(&self, pourtransid: *mut u8, pcomparestate: *mut DTCLUCOMPARESTATE) -> windows_core::Result<()>;
    fn HandleTheirCompareStatesResponse(&self, comparestate: DTCLUCOMPARESTATE, pconfirmation: *mut DTCLUCOMPARESTATESCONFIRMATION) -> windows_core::Result<()>;
    fn HandleErrorFromOurCompareStates(&self, error: DTCLUCOMPARESTATESERROR) -> windows_core::Result<()>;
    fn ConversationLost(&self) -> windows_core::Result<()>;
    fn GetRecoverySeqNum(&self, plrecoveryseqnum: *mut i32) -> windows_core::Result<()>;
    fn ObsoleteRecoverySeqNum(&self, lnewrecoveryseqnum: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDtcLuRecoveryInitiatedByDtcTransWork {}
impl IDtcLuRecoveryInitiatedByDtcTransWork_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDtcLuRecoveryInitiatedByDtcTransWork_Vtbl
    where
        Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl,
    {
        unsafe extern "system" fn GetLogNameSizes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbourlogname: *mut u32, pcbremotelogname: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRecoveryInitiatedByDtcTransWork_Impl::GetLogNameSizes(this, core::mem::transmute_copy(&pcbourlogname), core::mem::transmute_copy(&pcbremotelogname)).into()
        }
        unsafe extern "system" fn GetOurXln<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxln: *mut DTCLUXLN, pourlogname: *mut u8, premotelogname: *mut u8, pdwprotocol: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRecoveryInitiatedByDtcTransWork_Impl::GetOurXln(this, core::mem::transmute_copy(&pxln), core::mem::transmute_copy(&pourlogname), core::mem::transmute_copy(&premotelogname), core::mem::transmute_copy(&pdwprotocol)).into()
        }
        unsafe extern "system" fn HandleConfirmationFromOurXln<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, confirmation: DTCLUXLNCONFIRMATION) -> windows_core::HRESULT
        where
            Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRecoveryInitiatedByDtcTransWork_Impl::HandleConfirmationFromOurXln(this, core::mem::transmute_copy(&confirmation)).into()
        }
        unsafe extern "system" fn HandleTheirXlnResponse<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, xln: DTCLUXLN, premotelogname: *mut u8, cbremotelogname: u32, dwprotocol: u32, pconfirmation: *mut DTCLUXLNCONFIRMATION) -> windows_core::HRESULT
        where
            Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRecoveryInitiatedByDtcTransWork_Impl::HandleTheirXlnResponse(this, core::mem::transmute_copy(&xln), core::mem::transmute_copy(&premotelogname), core::mem::transmute_copy(&cbremotelogname), core::mem::transmute_copy(&dwprotocol), core::mem::transmute_copy(&pconfirmation)).into()
        }
        unsafe extern "system" fn HandleErrorFromOurXln<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, error: DTCLUXLNERROR) -> windows_core::HRESULT
        where
            Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRecoveryInitiatedByDtcTransWork_Impl::HandleErrorFromOurXln(this, core::mem::transmute_copy(&error)).into()
        }
        unsafe extern "system" fn CheckForCompareStates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fcomparestates: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRecoveryInitiatedByDtcTransWork_Impl::CheckForCompareStates(this, core::mem::transmute_copy(&fcomparestates)).into()
        }
        unsafe extern "system" fn GetOurTransIdSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbourtransid: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRecoveryInitiatedByDtcTransWork_Impl::GetOurTransIdSize(this, core::mem::transmute_copy(&pcbourtransid)).into()
        }
        unsafe extern "system" fn GetOurCompareStates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pourtransid: *mut u8, pcomparestate: *mut DTCLUCOMPARESTATE) -> windows_core::HRESULT
        where
            Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRecoveryInitiatedByDtcTransWork_Impl::GetOurCompareStates(this, core::mem::transmute_copy(&pourtransid), core::mem::transmute_copy(&pcomparestate)).into()
        }
        unsafe extern "system" fn HandleTheirCompareStatesResponse<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparestate: DTCLUCOMPARESTATE, pconfirmation: *mut DTCLUCOMPARESTATESCONFIRMATION) -> windows_core::HRESULT
        where
            Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRecoveryInitiatedByDtcTransWork_Impl::HandleTheirCompareStatesResponse(this, core::mem::transmute_copy(&comparestate), core::mem::transmute_copy(&pconfirmation)).into()
        }
        unsafe extern "system" fn HandleErrorFromOurCompareStates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, error: DTCLUCOMPARESTATESERROR) -> windows_core::HRESULT
        where
            Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRecoveryInitiatedByDtcTransWork_Impl::HandleErrorFromOurCompareStates(this, core::mem::transmute_copy(&error)).into()
        }
        unsafe extern "system" fn ConversationLost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRecoveryInitiatedByDtcTransWork_Impl::ConversationLost(this).into()
        }
        unsafe extern "system" fn GetRecoverySeqNum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plrecoveryseqnum: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRecoveryInitiatedByDtcTransWork_Impl::GetRecoverySeqNum(this, core::mem::transmute_copy(&plrecoveryseqnum)).into()
        }
        unsafe extern "system" fn ObsoleteRecoverySeqNum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnewrecoveryseqnum: i32) -> windows_core::HRESULT
        where
            Identity: IDtcLuRecoveryInitiatedByDtcTransWork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRecoveryInitiatedByDtcTransWork_Impl::ObsoleteRecoverySeqNum(this, core::mem::transmute_copy(&lnewrecoveryseqnum)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLogNameSizes: GetLogNameSizes::<Identity, OFFSET>,
            GetOurXln: GetOurXln::<Identity, OFFSET>,
            HandleConfirmationFromOurXln: HandleConfirmationFromOurXln::<Identity, OFFSET>,
            HandleTheirXlnResponse: HandleTheirXlnResponse::<Identity, OFFSET>,
            HandleErrorFromOurXln: HandleErrorFromOurXln::<Identity, OFFSET>,
            CheckForCompareStates: CheckForCompareStates::<Identity, OFFSET>,
            GetOurTransIdSize: GetOurTransIdSize::<Identity, OFFSET>,
            GetOurCompareStates: GetOurCompareStates::<Identity, OFFSET>,
            HandleTheirCompareStatesResponse: HandleTheirCompareStatesResponse::<Identity, OFFSET>,
            HandleErrorFromOurCompareStates: HandleErrorFromOurCompareStates::<Identity, OFFSET>,
            ConversationLost: ConversationLost::<Identity, OFFSET>,
            GetRecoverySeqNum: GetRecoverySeqNum::<Identity, OFFSET>,
            ObsoleteRecoverySeqNum: ObsoleteRecoverySeqNum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByDtcTransWork as windows_core::Interface>::IID
    }
}
pub trait IDtcLuRecoveryInitiatedByLu_Impl: Sized {
    fn GetObjectToHandleWorkFromLu(&self) -> windows_core::Result<IDtcLuRecoveryInitiatedByLuWork>;
}
impl windows_core::RuntimeName for IDtcLuRecoveryInitiatedByLu {}
impl IDtcLuRecoveryInitiatedByLu_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDtcLuRecoveryInitiatedByLu_Vtbl
    where
        Identity: IDtcLuRecoveryInitiatedByLu_Impl,
    {
        unsafe extern "system" fn GetObjectToHandleWorkFromLu<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwork: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuRecoveryInitiatedByLu_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDtcLuRecoveryInitiatedByLu_Impl::GetObjectToHandleWorkFromLu(this) {
                Ok(ok__) => {
                    ppwork.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetObjectToHandleWorkFromLu: GetObjectToHandleWorkFromLu::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByLu as windows_core::Interface>::IID
    }
}
pub trait IDtcLuRecoveryInitiatedByLuWork_Impl: Sized {
    fn HandleTheirXln(&self, lrecoveryseqnum: i32, xln: DTCLUXLN, premotelogname: *mut u8, cbremotelogname: u32, pourlogname: *mut u8, cbourlogname: u32, dwprotocol: u32, presponse: *mut DTCLUXLNRESPONSE) -> windows_core::Result<()>;
    fn GetOurLogNameSize(&self, pcbourlogname: *mut u32) -> windows_core::Result<()>;
    fn GetOurXln(&self, pxln: *mut DTCLUXLN, pourlogname: *mut u8, pdwprotocol: *mut u32) -> windows_core::Result<()>;
    fn HandleConfirmationOfOurXln(&self, confirmation: DTCLUXLNCONFIRMATION) -> windows_core::Result<()>;
    fn HandleTheirCompareStates(&self, premotetransid: *mut u8, cbremotetransid: u32, comparestate: DTCLUCOMPARESTATE, presponse: *mut DTCLUCOMPARESTATESRESPONSE, pcomparestate: *mut DTCLUCOMPARESTATE) -> windows_core::Result<()>;
    fn HandleConfirmationOfOurCompareStates(&self, confirmation: DTCLUCOMPARESTATESCONFIRMATION) -> windows_core::Result<()>;
    fn HandleErrorFromOurCompareStates(&self, error: DTCLUCOMPARESTATESERROR) -> windows_core::Result<()>;
    fn ConversationLost(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDtcLuRecoveryInitiatedByLuWork {}
impl IDtcLuRecoveryInitiatedByLuWork_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDtcLuRecoveryInitiatedByLuWork_Vtbl
    where
        Identity: IDtcLuRecoveryInitiatedByLuWork_Impl,
    {
        unsafe extern "system" fn HandleTheirXln<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lrecoveryseqnum: i32, xln: DTCLUXLN, premotelogname: *mut u8, cbremotelogname: u32, pourlogname: *mut u8, cbourlogname: u32, dwprotocol: u32, presponse: *mut DTCLUXLNRESPONSE) -> windows_core::HRESULT
        where
            Identity: IDtcLuRecoveryInitiatedByLuWork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRecoveryInitiatedByLuWork_Impl::HandleTheirXln(this, core::mem::transmute_copy(&lrecoveryseqnum), core::mem::transmute_copy(&xln), core::mem::transmute_copy(&premotelogname), core::mem::transmute_copy(&cbremotelogname), core::mem::transmute_copy(&pourlogname), core::mem::transmute_copy(&cbourlogname), core::mem::transmute_copy(&dwprotocol), core::mem::transmute_copy(&presponse)).into()
        }
        unsafe extern "system" fn GetOurLogNameSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbourlogname: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDtcLuRecoveryInitiatedByLuWork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRecoveryInitiatedByLuWork_Impl::GetOurLogNameSize(this, core::mem::transmute_copy(&pcbourlogname)).into()
        }
        unsafe extern "system" fn GetOurXln<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxln: *mut DTCLUXLN, pourlogname: *mut u8, pdwprotocol: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDtcLuRecoveryInitiatedByLuWork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRecoveryInitiatedByLuWork_Impl::GetOurXln(this, core::mem::transmute_copy(&pxln), core::mem::transmute_copy(&pourlogname), core::mem::transmute_copy(&pdwprotocol)).into()
        }
        unsafe extern "system" fn HandleConfirmationOfOurXln<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, confirmation: DTCLUXLNCONFIRMATION) -> windows_core::HRESULT
        where
            Identity: IDtcLuRecoveryInitiatedByLuWork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRecoveryInitiatedByLuWork_Impl::HandleConfirmationOfOurXln(this, core::mem::transmute_copy(&confirmation)).into()
        }
        unsafe extern "system" fn HandleTheirCompareStates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, premotetransid: *mut u8, cbremotetransid: u32, comparestate: DTCLUCOMPARESTATE, presponse: *mut DTCLUCOMPARESTATESRESPONSE, pcomparestate: *mut DTCLUCOMPARESTATE) -> windows_core::HRESULT
        where
            Identity: IDtcLuRecoveryInitiatedByLuWork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRecoveryInitiatedByLuWork_Impl::HandleTheirCompareStates(this, core::mem::transmute_copy(&premotetransid), core::mem::transmute_copy(&cbremotetransid), core::mem::transmute_copy(&comparestate), core::mem::transmute_copy(&presponse), core::mem::transmute_copy(&pcomparestate)).into()
        }
        unsafe extern "system" fn HandleConfirmationOfOurCompareStates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, confirmation: DTCLUCOMPARESTATESCONFIRMATION) -> windows_core::HRESULT
        where
            Identity: IDtcLuRecoveryInitiatedByLuWork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRecoveryInitiatedByLuWork_Impl::HandleConfirmationOfOurCompareStates(this, core::mem::transmute_copy(&confirmation)).into()
        }
        unsafe extern "system" fn HandleErrorFromOurCompareStates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, error: DTCLUCOMPARESTATESERROR) -> windows_core::HRESULT
        where
            Identity: IDtcLuRecoveryInitiatedByLuWork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRecoveryInitiatedByLuWork_Impl::HandleErrorFromOurCompareStates(this, core::mem::transmute_copy(&error)).into()
        }
        unsafe extern "system" fn ConversationLost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuRecoveryInitiatedByLuWork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRecoveryInitiatedByLuWork_Impl::ConversationLost(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            HandleTheirXln: HandleTheirXln::<Identity, OFFSET>,
            GetOurLogNameSize: GetOurLogNameSize::<Identity, OFFSET>,
            GetOurXln: GetOurXln::<Identity, OFFSET>,
            HandleConfirmationOfOurXln: HandleConfirmationOfOurXln::<Identity, OFFSET>,
            HandleTheirCompareStates: HandleTheirCompareStates::<Identity, OFFSET>,
            HandleConfirmationOfOurCompareStates: HandleConfirmationOfOurCompareStates::<Identity, OFFSET>,
            HandleErrorFromOurCompareStates: HandleErrorFromOurCompareStates::<Identity, OFFSET>,
            ConversationLost: ConversationLost::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRecoveryInitiatedByLuWork as windows_core::Interface>::IID
    }
}
pub trait IDtcLuRmEnlistment_Impl: Sized {
    fn Unplug(&self, fconversationlost: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn BackedOut(&self) -> windows_core::Result<()>;
    fn BackOut(&self) -> windows_core::Result<()>;
    fn Committed(&self) -> windows_core::Result<()>;
    fn Forget(&self) -> windows_core::Result<()>;
    fn RequestCommit(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDtcLuRmEnlistment {}
impl IDtcLuRmEnlistment_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDtcLuRmEnlistment_Vtbl
    where
        Identity: IDtcLuRmEnlistment_Impl,
    {
        unsafe extern "system" fn Unplug<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fconversationlost: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDtcLuRmEnlistment_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRmEnlistment_Impl::Unplug(this, core::mem::transmute_copy(&fconversationlost)).into()
        }
        unsafe extern "system" fn BackedOut<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuRmEnlistment_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRmEnlistment_Impl::BackedOut(this).into()
        }
        unsafe extern "system" fn BackOut<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuRmEnlistment_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRmEnlistment_Impl::BackOut(this).into()
        }
        unsafe extern "system" fn Committed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuRmEnlistment_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRmEnlistment_Impl::Committed(this).into()
        }
        unsafe extern "system" fn Forget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuRmEnlistment_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRmEnlistment_Impl::Forget(this).into()
        }
        unsafe extern "system" fn RequestCommit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuRmEnlistment_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRmEnlistment_Impl::RequestCommit(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Unplug: Unplug::<Identity, OFFSET>,
            BackedOut: BackedOut::<Identity, OFFSET>,
            BackOut: BackOut::<Identity, OFFSET>,
            Committed: Committed::<Identity, OFFSET>,
            Forget: Forget::<Identity, OFFSET>,
            RequestCommit: RequestCommit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRmEnlistment as windows_core::Interface>::IID
    }
}
pub trait IDtcLuRmEnlistmentFactory_Impl: Sized {
    fn Create(&self, puclupair: *mut u8, cblupair: u32, pitransaction: Option<&ITransaction>, ptransid: *mut u8, cbtransid: u32, prmenlistmentsink: Option<&IDtcLuRmEnlistmentSink>, pprmenlistment: *mut Option<IDtcLuRmEnlistment>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDtcLuRmEnlistmentFactory {}
impl IDtcLuRmEnlistmentFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDtcLuRmEnlistmentFactory_Vtbl
    where
        Identity: IDtcLuRmEnlistmentFactory_Impl,
    {
        unsafe extern "system" fn Create<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puclupair: *mut u8, cblupair: u32, pitransaction: *mut core::ffi::c_void, ptransid: *mut u8, cbtransid: u32, prmenlistmentsink: *mut core::ffi::c_void, pprmenlistment: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuRmEnlistmentFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRmEnlistmentFactory_Impl::Create(this, core::mem::transmute_copy(&puclupair), core::mem::transmute_copy(&cblupair), windows_core::from_raw_borrowed(&pitransaction), core::mem::transmute_copy(&ptransid), core::mem::transmute_copy(&cbtransid), windows_core::from_raw_borrowed(&prmenlistmentsink), core::mem::transmute_copy(&pprmenlistment)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRmEnlistmentFactory as windows_core::Interface>::IID
    }
}
pub trait IDtcLuRmEnlistmentSink_Impl: Sized {
    fn AckUnplug(&self) -> windows_core::Result<()>;
    fn TmDown(&self) -> windows_core::Result<()>;
    fn SessionLost(&self) -> windows_core::Result<()>;
    fn BackedOut(&self) -> windows_core::Result<()>;
    fn BackOut(&self) -> windows_core::Result<()>;
    fn Committed(&self) -> windows_core::Result<()>;
    fn Forget(&self) -> windows_core::Result<()>;
    fn Prepare(&self) -> windows_core::Result<()>;
    fn RequestCommit(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDtcLuRmEnlistmentSink {}
impl IDtcLuRmEnlistmentSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDtcLuRmEnlistmentSink_Vtbl
    where
        Identity: IDtcLuRmEnlistmentSink_Impl,
    {
        unsafe extern "system" fn AckUnplug<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuRmEnlistmentSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRmEnlistmentSink_Impl::AckUnplug(this).into()
        }
        unsafe extern "system" fn TmDown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuRmEnlistmentSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRmEnlistmentSink_Impl::TmDown(this).into()
        }
        unsafe extern "system" fn SessionLost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuRmEnlistmentSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRmEnlistmentSink_Impl::SessionLost(this).into()
        }
        unsafe extern "system" fn BackedOut<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuRmEnlistmentSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRmEnlistmentSink_Impl::BackedOut(this).into()
        }
        unsafe extern "system" fn BackOut<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuRmEnlistmentSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRmEnlistmentSink_Impl::BackOut(this).into()
        }
        unsafe extern "system" fn Committed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuRmEnlistmentSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRmEnlistmentSink_Impl::Committed(this).into()
        }
        unsafe extern "system" fn Forget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuRmEnlistmentSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRmEnlistmentSink_Impl::Forget(this).into()
        }
        unsafe extern "system" fn Prepare<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuRmEnlistmentSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRmEnlistmentSink_Impl::Prepare(this).into()
        }
        unsafe extern "system" fn RequestCommit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuRmEnlistmentSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuRmEnlistmentSink_Impl::RequestCommit(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AckUnplug: AckUnplug::<Identity, OFFSET>,
            TmDown: TmDown::<Identity, OFFSET>,
            SessionLost: SessionLost::<Identity, OFFSET>,
            BackedOut: BackedOut::<Identity, OFFSET>,
            BackOut: BackOut::<Identity, OFFSET>,
            Committed: Committed::<Identity, OFFSET>,
            Forget: Forget::<Identity, OFFSET>,
            Prepare: Prepare::<Identity, OFFSET>,
            RequestCommit: RequestCommit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuRmEnlistmentSink as windows_core::Interface>::IID
    }
}
pub trait IDtcLuSubordinateDtc_Impl: Sized {
    fn Unplug(&self, fconversationlost: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn BackedOut(&self) -> windows_core::Result<()>;
    fn BackOut(&self) -> windows_core::Result<()>;
    fn Committed(&self) -> windows_core::Result<()>;
    fn Forget(&self) -> windows_core::Result<()>;
    fn Prepare(&self) -> windows_core::Result<()>;
    fn RequestCommit(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDtcLuSubordinateDtc {}
impl IDtcLuSubordinateDtc_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDtcLuSubordinateDtc_Vtbl
    where
        Identity: IDtcLuSubordinateDtc_Impl,
    {
        unsafe extern "system" fn Unplug<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fconversationlost: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDtcLuSubordinateDtc_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuSubordinateDtc_Impl::Unplug(this, core::mem::transmute_copy(&fconversationlost)).into()
        }
        unsafe extern "system" fn BackedOut<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuSubordinateDtc_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuSubordinateDtc_Impl::BackedOut(this).into()
        }
        unsafe extern "system" fn BackOut<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuSubordinateDtc_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuSubordinateDtc_Impl::BackOut(this).into()
        }
        unsafe extern "system" fn Committed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuSubordinateDtc_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuSubordinateDtc_Impl::Committed(this).into()
        }
        unsafe extern "system" fn Forget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuSubordinateDtc_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuSubordinateDtc_Impl::Forget(this).into()
        }
        unsafe extern "system" fn Prepare<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuSubordinateDtc_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuSubordinateDtc_Impl::Prepare(this).into()
        }
        unsafe extern "system" fn RequestCommit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuSubordinateDtc_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuSubordinateDtc_Impl::RequestCommit(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Unplug: Unplug::<Identity, OFFSET>,
            BackedOut: BackedOut::<Identity, OFFSET>,
            BackOut: BackOut::<Identity, OFFSET>,
            Committed: Committed::<Identity, OFFSET>,
            Forget: Forget::<Identity, OFFSET>,
            Prepare: Prepare::<Identity, OFFSET>,
            RequestCommit: RequestCommit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuSubordinateDtc as windows_core::Interface>::IID
    }
}
pub trait IDtcLuSubordinateDtcFactory_Impl: Sized {
    fn Create(&self, puclupair: *mut u8, cblupair: u32, punktransactionouter: Option<&windows_core::IUnknown>, isolevel: i32, isoflags: u32, poptions: Option<&ITransactionOptions>, pptransaction: *mut Option<ITransaction>, ptransid: *mut u8, cbtransid: u32, psubordinatedtcsink: Option<&IDtcLuSubordinateDtcSink>, ppsubordinatedtc: *mut Option<IDtcLuSubordinateDtc>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDtcLuSubordinateDtcFactory {}
impl IDtcLuSubordinateDtcFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDtcLuSubordinateDtcFactory_Vtbl
    where
        Identity: IDtcLuSubordinateDtcFactory_Impl,
    {
        unsafe extern "system" fn Create<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puclupair: *mut u8, cblupair: u32, punktransactionouter: *mut core::ffi::c_void, isolevel: i32, isoflags: u32, poptions: *mut core::ffi::c_void, pptransaction: *mut *mut core::ffi::c_void, ptransid: *mut u8, cbtransid: u32, psubordinatedtcsink: *mut core::ffi::c_void, ppsubordinatedtc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuSubordinateDtcFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuSubordinateDtcFactory_Impl::Create(this, core::mem::transmute_copy(&puclupair), core::mem::transmute_copy(&cblupair), windows_core::from_raw_borrowed(&punktransactionouter), core::mem::transmute_copy(&isolevel), core::mem::transmute_copy(&isoflags), windows_core::from_raw_borrowed(&poptions), core::mem::transmute_copy(&pptransaction), core::mem::transmute_copy(&ptransid), core::mem::transmute_copy(&cbtransid), windows_core::from_raw_borrowed(&psubordinatedtcsink), core::mem::transmute_copy(&ppsubordinatedtc)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuSubordinateDtcFactory as windows_core::Interface>::IID
    }
}
pub trait IDtcLuSubordinateDtcSink_Impl: Sized {
    fn AckUnplug(&self) -> windows_core::Result<()>;
    fn TmDown(&self) -> windows_core::Result<()>;
    fn SessionLost(&self) -> windows_core::Result<()>;
    fn BackedOut(&self) -> windows_core::Result<()>;
    fn BackOut(&self) -> windows_core::Result<()>;
    fn Committed(&self) -> windows_core::Result<()>;
    fn Forget(&self) -> windows_core::Result<()>;
    fn RequestCommit(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDtcLuSubordinateDtcSink {}
impl IDtcLuSubordinateDtcSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDtcLuSubordinateDtcSink_Vtbl
    where
        Identity: IDtcLuSubordinateDtcSink_Impl,
    {
        unsafe extern "system" fn AckUnplug<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuSubordinateDtcSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuSubordinateDtcSink_Impl::AckUnplug(this).into()
        }
        unsafe extern "system" fn TmDown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuSubordinateDtcSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuSubordinateDtcSink_Impl::TmDown(this).into()
        }
        unsafe extern "system" fn SessionLost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuSubordinateDtcSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuSubordinateDtcSink_Impl::SessionLost(this).into()
        }
        unsafe extern "system" fn BackedOut<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuSubordinateDtcSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuSubordinateDtcSink_Impl::BackedOut(this).into()
        }
        unsafe extern "system" fn BackOut<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuSubordinateDtcSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuSubordinateDtcSink_Impl::BackOut(this).into()
        }
        unsafe extern "system" fn Committed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuSubordinateDtcSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuSubordinateDtcSink_Impl::Committed(this).into()
        }
        unsafe extern "system" fn Forget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuSubordinateDtcSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuSubordinateDtcSink_Impl::Forget(this).into()
        }
        unsafe extern "system" fn RequestCommit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcLuSubordinateDtcSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcLuSubordinateDtcSink_Impl::RequestCommit(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AckUnplug: AckUnplug::<Identity, OFFSET>,
            TmDown: TmDown::<Identity, OFFSET>,
            SessionLost: SessionLost::<Identity, OFFSET>,
            BackedOut: BackedOut::<Identity, OFFSET>,
            BackOut: BackOut::<Identity, OFFSET>,
            Committed: Committed::<Identity, OFFSET>,
            Forget: Forget::<Identity, OFFSET>,
            RequestCommit: RequestCommit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcLuSubordinateDtcSink as windows_core::Interface>::IID
    }
}
pub trait IDtcNetworkAccessConfig_Impl: Sized {
    fn GetAnyNetworkAccess(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetAnyNetworkAccess(&self, banynetworkaccess: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetNetworkAdministrationAccess(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetNetworkAdministrationAccess(&self, bnetworkadministrationaccess: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetNetworkTransactionAccess(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetNetworkTransactionAccess(&self, bnetworktransactionaccess: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetNetworkClientAccess(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetNetworkClientAccess(&self, bnetworkclientaccess: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetNetworkTIPAccess(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetNetworkTIPAccess(&self, bnetworktipaccess: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetXAAccess(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetXAAccess(&self, bxaaccess: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn RestartDtcService(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDtcNetworkAccessConfig {}
impl IDtcNetworkAccessConfig_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDtcNetworkAccessConfig_Vtbl
    where
        Identity: IDtcNetworkAccessConfig_Impl,
    {
        unsafe extern "system" fn GetAnyNetworkAccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbanynetworkaccess: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDtcNetworkAccessConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDtcNetworkAccessConfig_Impl::GetAnyNetworkAccess(this) {
                Ok(ok__) => {
                    pbanynetworkaccess.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnyNetworkAccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, banynetworkaccess: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDtcNetworkAccessConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcNetworkAccessConfig_Impl::SetAnyNetworkAccess(this, core::mem::transmute_copy(&banynetworkaccess)).into()
        }
        unsafe extern "system" fn GetNetworkAdministrationAccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbnetworkadministrationaccess: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDtcNetworkAccessConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDtcNetworkAccessConfig_Impl::GetNetworkAdministrationAccess(this) {
                Ok(ok__) => {
                    pbnetworkadministrationaccess.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkAdministrationAccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bnetworkadministrationaccess: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDtcNetworkAccessConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcNetworkAccessConfig_Impl::SetNetworkAdministrationAccess(this, core::mem::transmute_copy(&bnetworkadministrationaccess)).into()
        }
        unsafe extern "system" fn GetNetworkTransactionAccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbnetworktransactionaccess: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDtcNetworkAccessConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDtcNetworkAccessConfig_Impl::GetNetworkTransactionAccess(this) {
                Ok(ok__) => {
                    pbnetworktransactionaccess.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkTransactionAccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bnetworktransactionaccess: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDtcNetworkAccessConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcNetworkAccessConfig_Impl::SetNetworkTransactionAccess(this, core::mem::transmute_copy(&bnetworktransactionaccess)).into()
        }
        unsafe extern "system" fn GetNetworkClientAccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbnetworkclientaccess: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDtcNetworkAccessConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDtcNetworkAccessConfig_Impl::GetNetworkClientAccess(this) {
                Ok(ok__) => {
                    pbnetworkclientaccess.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkClientAccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bnetworkclientaccess: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDtcNetworkAccessConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcNetworkAccessConfig_Impl::SetNetworkClientAccess(this, core::mem::transmute_copy(&bnetworkclientaccess)).into()
        }
        unsafe extern "system" fn GetNetworkTIPAccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbnetworktipaccess: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDtcNetworkAccessConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDtcNetworkAccessConfig_Impl::GetNetworkTIPAccess(this) {
                Ok(ok__) => {
                    pbnetworktipaccess.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkTIPAccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bnetworktipaccess: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDtcNetworkAccessConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcNetworkAccessConfig_Impl::SetNetworkTIPAccess(this, core::mem::transmute_copy(&bnetworktipaccess)).into()
        }
        unsafe extern "system" fn GetXAAccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbxaaccess: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDtcNetworkAccessConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDtcNetworkAccessConfig_Impl::GetXAAccess(this) {
                Ok(ok__) => {
                    pbxaaccess.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXAAccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bxaaccess: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDtcNetworkAccessConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcNetworkAccessConfig_Impl::SetXAAccess(this, core::mem::transmute_copy(&bxaaccess)).into()
        }
        unsafe extern "system" fn RestartDtcService<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcNetworkAccessConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcNetworkAccessConfig_Impl::RestartDtcService(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAnyNetworkAccess: GetAnyNetworkAccess::<Identity, OFFSET>,
            SetAnyNetworkAccess: SetAnyNetworkAccess::<Identity, OFFSET>,
            GetNetworkAdministrationAccess: GetNetworkAdministrationAccess::<Identity, OFFSET>,
            SetNetworkAdministrationAccess: SetNetworkAdministrationAccess::<Identity, OFFSET>,
            GetNetworkTransactionAccess: GetNetworkTransactionAccess::<Identity, OFFSET>,
            SetNetworkTransactionAccess: SetNetworkTransactionAccess::<Identity, OFFSET>,
            GetNetworkClientAccess: GetNetworkClientAccess::<Identity, OFFSET>,
            SetNetworkClientAccess: SetNetworkClientAccess::<Identity, OFFSET>,
            GetNetworkTIPAccess: GetNetworkTIPAccess::<Identity, OFFSET>,
            SetNetworkTIPAccess: SetNetworkTIPAccess::<Identity, OFFSET>,
            GetXAAccess: GetXAAccess::<Identity, OFFSET>,
            SetXAAccess: SetXAAccess::<Identity, OFFSET>,
            RestartDtcService: RestartDtcService::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcNetworkAccessConfig as windows_core::Interface>::IID
    }
}
pub trait IDtcNetworkAccessConfig2_Impl: Sized + IDtcNetworkAccessConfig_Impl {
    fn GetNetworkInboundAccess(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetNetworkOutboundAccess(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetNetworkInboundAccess(&self, binbound: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetNetworkOutboundAccess(&self, boutbound: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetAuthenticationLevel(&self) -> windows_core::Result<AUTHENTICATION_LEVEL>;
    fn SetAuthenticationLevel(&self, authlevel: AUTHENTICATION_LEVEL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDtcNetworkAccessConfig2 {}
impl IDtcNetworkAccessConfig2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDtcNetworkAccessConfig2_Vtbl
    where
        Identity: IDtcNetworkAccessConfig2_Impl,
    {
        unsafe extern "system" fn GetNetworkInboundAccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbinbound: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDtcNetworkAccessConfig2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDtcNetworkAccessConfig2_Impl::GetNetworkInboundAccess(this) {
                Ok(ok__) => {
                    pbinbound.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkOutboundAccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pboutbound: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDtcNetworkAccessConfig2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDtcNetworkAccessConfig2_Impl::GetNetworkOutboundAccess(this) {
                Ok(ok__) => {
                    pboutbound.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkInboundAccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, binbound: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDtcNetworkAccessConfig2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcNetworkAccessConfig2_Impl::SetNetworkInboundAccess(this, core::mem::transmute_copy(&binbound)).into()
        }
        unsafe extern "system" fn SetNetworkOutboundAccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, boutbound: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDtcNetworkAccessConfig2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcNetworkAccessConfig2_Impl::SetNetworkOutboundAccess(this, core::mem::transmute_copy(&boutbound)).into()
        }
        unsafe extern "system" fn GetAuthenticationLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pauthlevel: *mut AUTHENTICATION_LEVEL) -> windows_core::HRESULT
        where
            Identity: IDtcNetworkAccessConfig2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDtcNetworkAccessConfig2_Impl::GetAuthenticationLevel(this) {
                Ok(ok__) => {
                    pauthlevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, authlevel: AUTHENTICATION_LEVEL) -> windows_core::HRESULT
        where
            Identity: IDtcNetworkAccessConfig2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcNetworkAccessConfig2_Impl::SetAuthenticationLevel(this, core::mem::transmute_copy(&authlevel)).into()
        }
        Self {
            base__: IDtcNetworkAccessConfig_Vtbl::new::<Identity, OFFSET>(),
            GetNetworkInboundAccess: GetNetworkInboundAccess::<Identity, OFFSET>,
            GetNetworkOutboundAccess: GetNetworkOutboundAccess::<Identity, OFFSET>,
            SetNetworkInboundAccess: SetNetworkInboundAccess::<Identity, OFFSET>,
            SetNetworkOutboundAccess: SetNetworkOutboundAccess::<Identity, OFFSET>,
            GetAuthenticationLevel: GetAuthenticationLevel::<Identity, OFFSET>,
            SetAuthenticationLevel: SetAuthenticationLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcNetworkAccessConfig2 as windows_core::Interface>::IID || iid == &<IDtcNetworkAccessConfig as windows_core::Interface>::IID
    }
}
pub trait IDtcNetworkAccessConfig3_Impl: Sized + IDtcNetworkAccessConfig2_Impl {
    fn GetLUAccess(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetLUAccess(&self, bluaccess: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDtcNetworkAccessConfig3 {}
impl IDtcNetworkAccessConfig3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDtcNetworkAccessConfig3_Vtbl
    where
        Identity: IDtcNetworkAccessConfig3_Impl,
    {
        unsafe extern "system" fn GetLUAccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbluaccess: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDtcNetworkAccessConfig3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDtcNetworkAccessConfig3_Impl::GetLUAccess(this) {
                Ok(ok__) => {
                    pbluaccess.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLUAccess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bluaccess: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDtcNetworkAccessConfig3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcNetworkAccessConfig3_Impl::SetLUAccess(this, core::mem::transmute_copy(&bluaccess)).into()
        }
        Self {
            base__: IDtcNetworkAccessConfig2_Vtbl::new::<Identity, OFFSET>(),
            GetLUAccess: GetLUAccess::<Identity, OFFSET>,
            SetLUAccess: SetLUAccess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcNetworkAccessConfig3 as windows_core::Interface>::IID || iid == &<IDtcNetworkAccessConfig as windows_core::Interface>::IID || iid == &<IDtcNetworkAccessConfig2 as windows_core::Interface>::IID
    }
}
pub trait IDtcToXaHelper_Impl: Sized {
    fn Close(&self, i_fdorecovery: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn TranslateTridToXid(&self, pitransaction: Option<&ITransaction>, pguidbqual: *const windows_core::GUID, pxid: *mut XID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDtcToXaHelper {}
impl IDtcToXaHelper_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDtcToXaHelper_Vtbl
    where
        Identity: IDtcToXaHelper_Impl,
    {
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, i_fdorecovery: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDtcToXaHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcToXaHelper_Impl::Close(this, core::mem::transmute_copy(&i_fdorecovery)).into()
        }
        unsafe extern "system" fn TranslateTridToXid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitransaction: *mut core::ffi::c_void, pguidbqual: *const windows_core::GUID, pxid: *mut XID) -> windows_core::HRESULT
        where
            Identity: IDtcToXaHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcToXaHelper_Impl::TranslateTridToXid(this, windows_core::from_raw_borrowed(&pitransaction), core::mem::transmute_copy(&pguidbqual), core::mem::transmute_copy(&pxid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Close: Close::<Identity, OFFSET>,
            TranslateTridToXid: TranslateTridToXid::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcToXaHelper as windows_core::Interface>::IID
    }
}
pub trait IDtcToXaHelperFactory_Impl: Sized {
    fn Create(&self, pszdsn: &windows_core::PCSTR, pszclientdllname: &windows_core::PCSTR, pguidrm: *mut windows_core::GUID, ppxahelper: *mut Option<IDtcToXaHelper>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDtcToXaHelperFactory {}
impl IDtcToXaHelperFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDtcToXaHelperFactory_Vtbl
    where
        Identity: IDtcToXaHelperFactory_Impl,
    {
        unsafe extern "system" fn Create<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdsn: windows_core::PCSTR, pszclientdllname: windows_core::PCSTR, pguidrm: *mut windows_core::GUID, ppxahelper: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcToXaHelperFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcToXaHelperFactory_Impl::Create(this, core::mem::transmute(&pszdsn), core::mem::transmute(&pszclientdllname), core::mem::transmute_copy(&pguidrm), core::mem::transmute_copy(&ppxahelper)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcToXaHelperFactory as windows_core::Interface>::IID
    }
}
pub trait IDtcToXaHelperSinglePipe_Impl: Sized {
    fn XARMCreate(&self, pszdsn: &windows_core::PCSTR, pszclientdll: &windows_core::PCSTR, pdwrmcookie: *mut u32) -> windows_core::Result<()>;
    fn ConvertTridToXID(&self, pdwitrans: *mut u32, dwrmcookie: u32, pxid: *mut XID) -> windows_core::Result<()>;
    fn EnlistWithRM(&self, dwrmcookie: u32, i_pitransaction: Option<&ITransaction>, i_pitransres: Option<&ITransactionResourceAsync>) -> windows_core::Result<ITransactionEnlistmentAsync>;
    fn ReleaseRMCookie(&self, i_dwrmcookie: u32, i_fnormal: super::super::Foundation::BOOL);
}
impl windows_core::RuntimeName for IDtcToXaHelperSinglePipe {}
impl IDtcToXaHelperSinglePipe_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDtcToXaHelperSinglePipe_Vtbl
    where
        Identity: IDtcToXaHelperSinglePipe_Impl,
    {
        unsafe extern "system" fn XARMCreate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdsn: windows_core::PCSTR, pszclientdll: windows_core::PCSTR, pdwrmcookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDtcToXaHelperSinglePipe_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcToXaHelperSinglePipe_Impl::XARMCreate(this, core::mem::transmute(&pszdsn), core::mem::transmute(&pszclientdll), core::mem::transmute_copy(&pdwrmcookie)).into()
        }
        unsafe extern "system" fn ConvertTridToXID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwitrans: *mut u32, dwrmcookie: u32, pxid: *mut XID) -> windows_core::HRESULT
        where
            Identity: IDtcToXaHelperSinglePipe_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcToXaHelperSinglePipe_Impl::ConvertTridToXID(this, core::mem::transmute_copy(&pdwitrans), core::mem::transmute_copy(&dwrmcookie), core::mem::transmute_copy(&pxid)).into()
        }
        unsafe extern "system" fn EnlistWithRM<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwrmcookie: u32, i_pitransaction: *mut core::ffi::c_void, i_pitransres: *mut core::ffi::c_void, o_ppitransenslitment: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDtcToXaHelperSinglePipe_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDtcToXaHelperSinglePipe_Impl::EnlistWithRM(this, core::mem::transmute_copy(&dwrmcookie), windows_core::from_raw_borrowed(&i_pitransaction), windows_core::from_raw_borrowed(&i_pitransres)) {
                Ok(ok__) => {
                    o_ppitransenslitment.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseRMCookie<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, i_dwrmcookie: u32, i_fnormal: super::super::Foundation::BOOL)
        where
            Identity: IDtcToXaHelperSinglePipe_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcToXaHelperSinglePipe_Impl::ReleaseRMCookie(this, core::mem::transmute_copy(&i_dwrmcookie), core::mem::transmute_copy(&i_fnormal))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            XARMCreate: XARMCreate::<Identity, OFFSET>,
            ConvertTridToXID: ConvertTridToXID::<Identity, OFFSET>,
            EnlistWithRM: EnlistWithRM::<Identity, OFFSET>,
            ReleaseRMCookie: ReleaseRMCookie::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcToXaHelperSinglePipe as windows_core::Interface>::IID
    }
}
pub trait IDtcToXaMapper_Impl: Sized {
    fn RequestNewResourceManager(&self, pszdsn: &windows_core::PCSTR, pszclientdllname: &windows_core::PCSTR, pdwrmcookie: *mut u32) -> windows_core::Result<()>;
    fn TranslateTridToXid(&self, pdwitransaction: *const u32, dwrmcookie: u32, pxid: *mut XID) -> windows_core::Result<()>;
    fn EnlistResourceManager(&self, dwrmcookie: u32, pdwitransaction: *const u32) -> windows_core::Result<()>;
    fn ReleaseResourceManager(&self, dwrmcookie: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDtcToXaMapper {}
impl IDtcToXaMapper_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDtcToXaMapper_Vtbl
    where
        Identity: IDtcToXaMapper_Impl,
    {
        unsafe extern "system" fn RequestNewResourceManager<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdsn: windows_core::PCSTR, pszclientdllname: windows_core::PCSTR, pdwrmcookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDtcToXaMapper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcToXaMapper_Impl::RequestNewResourceManager(this, core::mem::transmute(&pszdsn), core::mem::transmute(&pszclientdllname), core::mem::transmute_copy(&pdwrmcookie)).into()
        }
        unsafe extern "system" fn TranslateTridToXid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwitransaction: *const u32, dwrmcookie: u32, pxid: *mut XID) -> windows_core::HRESULT
        where
            Identity: IDtcToXaMapper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcToXaMapper_Impl::TranslateTridToXid(this, core::mem::transmute_copy(&pdwitransaction), core::mem::transmute_copy(&dwrmcookie), core::mem::transmute_copy(&pxid)).into()
        }
        unsafe extern "system" fn EnlistResourceManager<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwrmcookie: u32, pdwitransaction: *const u32) -> windows_core::HRESULT
        where
            Identity: IDtcToXaMapper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcToXaMapper_Impl::EnlistResourceManager(this, core::mem::transmute_copy(&dwrmcookie), core::mem::transmute_copy(&pdwitransaction)).into()
        }
        unsafe extern "system" fn ReleaseResourceManager<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwrmcookie: u32) -> windows_core::HRESULT
        where
            Identity: IDtcToXaMapper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDtcToXaMapper_Impl::ReleaseResourceManager(this, core::mem::transmute_copy(&dwrmcookie)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RequestNewResourceManager: RequestNewResourceManager::<Identity, OFFSET>,
            TranslateTridToXid: TranslateTridToXid::<Identity, OFFSET>,
            EnlistResourceManager: EnlistResourceManager::<Identity, OFFSET>,
            ReleaseResourceManager: ReleaseResourceManager::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDtcToXaMapper as windows_core::Interface>::IID
    }
}
pub trait IGetDispenser_Impl: Sized {
    fn GetDispenser(&self, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IGetDispenser {}
impl IGetDispenser_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGetDispenser_Vtbl
    where
        Identity: IGetDispenser_Impl,
    {
        unsafe extern "system" fn GetDispenser<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGetDispenser_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGetDispenser_Impl::GetDispenser(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppvobject)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDispenser: GetDispenser::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetDispenser as windows_core::Interface>::IID
    }
}
pub trait IKernelTransaction_Impl: Sized {
    fn GetHandle(&self) -> windows_core::Result<super::super::Foundation::HANDLE>;
}
impl windows_core::RuntimeName for IKernelTransaction {}
impl IKernelTransaction_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IKernelTransaction_Vtbl
    where
        Identity: IKernelTransaction_Impl,
    {
        unsafe extern "system" fn GetHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IKernelTransaction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKernelTransaction_Impl::GetHandle(this) {
                Ok(ok__) => {
                    phandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetHandle: GetHandle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKernelTransaction as windows_core::Interface>::IID
    }
}
pub trait ILastResourceManager_Impl: Sized {
    fn TransactionCommitted(&self, pprepinfo: *const u8, cbprepinfo: u32) -> windows_core::Result<()>;
    fn RecoveryDone(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ILastResourceManager {}
impl ILastResourceManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ILastResourceManager_Vtbl
    where
        Identity: ILastResourceManager_Impl,
    {
        unsafe extern "system" fn TransactionCommitted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32) -> windows_core::HRESULT
        where
            Identity: ILastResourceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILastResourceManager_Impl::TransactionCommitted(this, core::mem::transmute_copy(&pprepinfo), core::mem::transmute_copy(&cbprepinfo)).into()
        }
        unsafe extern "system" fn RecoveryDone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILastResourceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILastResourceManager_Impl::RecoveryDone(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TransactionCommitted: TransactionCommitted::<Identity, OFFSET>,
            RecoveryDone: RecoveryDone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILastResourceManager as windows_core::Interface>::IID
    }
}
pub trait IPrepareInfo_Impl: Sized {
    fn GetPrepareInfoSize(&self, pcbprepinfo: *mut u32) -> windows_core::Result<()>;
    fn GetPrepareInfo(&self, pprepinfo: *mut u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrepareInfo {}
impl IPrepareInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrepareInfo_Vtbl
    where
        Identity: IPrepareInfo_Impl,
    {
        unsafe extern "system" fn GetPrepareInfoSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbprepinfo: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrepareInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrepareInfo_Impl::GetPrepareInfoSize(this, core::mem::transmute_copy(&pcbprepinfo)).into()
        }
        unsafe extern "system" fn GetPrepareInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprepinfo: *mut u8) -> windows_core::HRESULT
        where
            Identity: IPrepareInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrepareInfo_Impl::GetPrepareInfo(this, core::mem::transmute_copy(&pprepinfo)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPrepareInfoSize: GetPrepareInfoSize::<Identity, OFFSET>,
            GetPrepareInfo: GetPrepareInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrepareInfo as windows_core::Interface>::IID
    }
}
pub trait IPrepareInfo2_Impl: Sized {
    fn GetPrepareInfoSize(&self) -> windows_core::Result<u32>;
    fn GetPrepareInfo(&self, cbprepareinfo: u32, pprepinfo: *mut u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPrepareInfo2 {}
impl IPrepareInfo2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPrepareInfo2_Vtbl
    where
        Identity: IPrepareInfo2_Impl,
    {
        unsafe extern "system" fn GetPrepareInfoSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbprepinfo: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPrepareInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPrepareInfo2_Impl::GetPrepareInfoSize(this) {
                Ok(ok__) => {
                    pcbprepinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrepareInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbprepareinfo: u32, pprepinfo: *mut u8) -> windows_core::HRESULT
        where
            Identity: IPrepareInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPrepareInfo2_Impl::GetPrepareInfo(this, core::mem::transmute_copy(&cbprepareinfo), core::mem::transmute_copy(&pprepinfo)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPrepareInfoSize: GetPrepareInfoSize::<Identity, OFFSET>,
            GetPrepareInfo: GetPrepareInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrepareInfo2 as windows_core::Interface>::IID
    }
}
pub trait IRMHelper_Impl: Sized {
    fn RMCount(&self, dwctotalnumberofrms: u32) -> windows_core::Result<()>;
    fn RMInfo(&self, pxa_switch: *mut xa_switch_t, fcdeclcallingconv: super::super::Foundation::BOOL, pszopenstring: &windows_core::PCSTR, pszclosestring: &windows_core::PCSTR, guidrmrecovery: &windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRMHelper {}
impl IRMHelper_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRMHelper_Vtbl
    where
        Identity: IRMHelper_Impl,
    {
        unsafe extern "system" fn RMCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwctotalnumberofrms: u32) -> windows_core::HRESULT
        where
            Identity: IRMHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRMHelper_Impl::RMCount(this, core::mem::transmute_copy(&dwctotalnumberofrms)).into()
        }
        unsafe extern "system" fn RMInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxa_switch: *mut xa_switch_t, fcdeclcallingconv: super::super::Foundation::BOOL, pszopenstring: windows_core::PCSTR, pszclosestring: windows_core::PCSTR, guidrmrecovery: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IRMHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRMHelper_Impl::RMInfo(this, core::mem::transmute_copy(&pxa_switch), core::mem::transmute_copy(&fcdeclcallingconv), core::mem::transmute(&pszopenstring), core::mem::transmute(&pszclosestring), core::mem::transmute(&guidrmrecovery)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RMCount: RMCount::<Identity, OFFSET>, RMInfo: RMInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRMHelper as windows_core::Interface>::IID
    }
}
pub trait IResourceManager_Impl: Sized {
    fn Enlist(&self, ptransaction: Option<&ITransaction>, pres: Option<&ITransactionResourceAsync>, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut Option<ITransactionEnlistmentAsync>) -> windows_core::Result<()>;
    fn Reenlist(&self, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32) -> windows_core::Result<XACTSTAT>;
    fn ReenlistmentComplete(&self) -> windows_core::Result<()>;
    fn GetDistributedTransactionManager(&self, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IResourceManager {}
impl IResourceManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IResourceManager_Vtbl
    where
        Identity: IResourceManager_Impl,
    {
        unsafe extern "system" fn Enlist<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut core::ffi::c_void, pres: *mut core::ffi::c_void, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IResourceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IResourceManager_Impl::Enlist(this, windows_core::from_raw_borrowed(&ptransaction), windows_core::from_raw_borrowed(&pres), core::mem::transmute_copy(&puow), core::mem::transmute_copy(&pisolevel), core::mem::transmute_copy(&ppenlist)).into()
        }
        unsafe extern "system" fn Reenlist<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> windows_core::HRESULT
        where
            Identity: IResourceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IResourceManager_Impl::Reenlist(this, core::mem::transmute_copy(&pprepinfo), core::mem::transmute_copy(&cbprepinfo), core::mem::transmute_copy(&ltimeout)) {
                Ok(ok__) => {
                    pxactstat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReenlistmentComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IResourceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IResourceManager_Impl::ReenlistmentComplete(this).into()
        }
        unsafe extern "system" fn GetDistributedTransactionManager<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IResourceManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IResourceManager_Impl::GetDistributedTransactionManager(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppvobject)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Enlist: Enlist::<Identity, OFFSET>,
            Reenlist: Reenlist::<Identity, OFFSET>,
            ReenlistmentComplete: ReenlistmentComplete::<Identity, OFFSET>,
            GetDistributedTransactionManager: GetDistributedTransactionManager::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IResourceManager as windows_core::Interface>::IID
    }
}
pub trait IResourceManager2_Impl: Sized + IResourceManager_Impl {
    fn Enlist2(&self, ptransaction: Option<&ITransaction>, presasync: Option<&ITransactionResourceAsync>, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut XID, ppenlist: *mut Option<ITransactionEnlistmentAsync>) -> windows_core::Result<()>;
    fn Reenlist2(&self, pxid: *const XID, dwtimeout: u32) -> windows_core::Result<XACTSTAT>;
}
impl windows_core::RuntimeName for IResourceManager2 {}
impl IResourceManager2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IResourceManager2_Vtbl
    where
        Identity: IResourceManager2_Impl,
    {
        unsafe extern "system" fn Enlist2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut core::ffi::c_void, presasync: *mut core::ffi::c_void, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut XID, ppenlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IResourceManager2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IResourceManager2_Impl::Enlist2(this, windows_core::from_raw_borrowed(&ptransaction), windows_core::from_raw_borrowed(&presasync), core::mem::transmute_copy(&puow), core::mem::transmute_copy(&pisolevel), core::mem::transmute_copy(&pxid), core::mem::transmute_copy(&ppenlist)).into()
        }
        unsafe extern "system" fn Reenlist2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxid: *const XID, dwtimeout: u32, pxactstat: *mut XACTSTAT) -> windows_core::HRESULT
        where
            Identity: IResourceManager2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IResourceManager2_Impl::Reenlist2(this, core::mem::transmute_copy(&pxid), core::mem::transmute_copy(&dwtimeout)) {
                Ok(ok__) => {
                    pxactstat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IResourceManager_Vtbl::new::<Identity, OFFSET>(), Enlist2: Enlist2::<Identity, OFFSET>, Reenlist2: Reenlist2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IResourceManager2 as windows_core::Interface>::IID || iid == &<IResourceManager as windows_core::Interface>::IID
    }
}
pub trait IResourceManagerFactory_Impl: Sized {
    fn Create(&self, pguidrm: *const windows_core::GUID, pszrmname: &windows_core::PCSTR, piresmgrsink: Option<&IResourceManagerSink>) -> windows_core::Result<IResourceManager>;
}
impl windows_core::RuntimeName for IResourceManagerFactory {}
impl IResourceManagerFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IResourceManagerFactory_Vtbl
    where
        Identity: IResourceManagerFactory_Impl,
    {
        unsafe extern "system" fn Create<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidrm: *const windows_core::GUID, pszrmname: windows_core::PCSTR, piresmgrsink: *mut core::ffi::c_void, ppresmgr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IResourceManagerFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IResourceManagerFactory_Impl::Create(this, core::mem::transmute_copy(&pguidrm), core::mem::transmute(&pszrmname), windows_core::from_raw_borrowed(&piresmgrsink)) {
                Ok(ok__) => {
                    ppresmgr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IResourceManagerFactory as windows_core::Interface>::IID
    }
}
pub trait IResourceManagerFactory2_Impl: Sized + IResourceManagerFactory_Impl {
    fn CreateEx(&self, pguidrm: *const windows_core::GUID, pszrmname: &windows_core::PCSTR, piresmgrsink: Option<&IResourceManagerSink>, riidrequested: *const windows_core::GUID, ppvresmgr: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IResourceManagerFactory2 {}
impl IResourceManagerFactory2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IResourceManagerFactory2_Vtbl
    where
        Identity: IResourceManagerFactory2_Impl,
    {
        unsafe extern "system" fn CreateEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidrm: *const windows_core::GUID, pszrmname: windows_core::PCSTR, piresmgrsink: *mut core::ffi::c_void, riidrequested: *const windows_core::GUID, ppvresmgr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IResourceManagerFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IResourceManagerFactory2_Impl::CreateEx(this, core::mem::transmute_copy(&pguidrm), core::mem::transmute(&pszrmname), windows_core::from_raw_borrowed(&piresmgrsink), core::mem::transmute_copy(&riidrequested), core::mem::transmute_copy(&ppvresmgr)).into()
        }
        Self { base__: IResourceManagerFactory_Vtbl::new::<Identity, OFFSET>(), CreateEx: CreateEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IResourceManagerFactory2 as windows_core::Interface>::IID || iid == &<IResourceManagerFactory as windows_core::Interface>::IID
    }
}
pub trait IResourceManagerRejoinable_Impl: Sized + IResourceManager2_Impl {
    fn Rejoin(&self, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32) -> windows_core::Result<XACTSTAT>;
}
impl windows_core::RuntimeName for IResourceManagerRejoinable {}
impl IResourceManagerRejoinable_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IResourceManagerRejoinable_Vtbl
    where
        Identity: IResourceManagerRejoinable_Impl,
    {
        unsafe extern "system" fn Rejoin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprepinfo: *const u8, cbprepinfo: u32, ltimeout: u32, pxactstat: *mut XACTSTAT) -> windows_core::HRESULT
        where
            Identity: IResourceManagerRejoinable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IResourceManagerRejoinable_Impl::Rejoin(this, core::mem::transmute_copy(&pprepinfo), core::mem::transmute_copy(&cbprepinfo), core::mem::transmute_copy(&ltimeout)) {
                Ok(ok__) => {
                    pxactstat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IResourceManager2_Vtbl::new::<Identity, OFFSET>(), Rejoin: Rejoin::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IResourceManagerRejoinable as windows_core::Interface>::IID || iid == &<IResourceManager as windows_core::Interface>::IID || iid == &<IResourceManager2 as windows_core::Interface>::IID
    }
}
pub trait IResourceManagerSink_Impl: Sized {
    fn TMDown(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IResourceManagerSink {}
impl IResourceManagerSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IResourceManagerSink_Vtbl
    where
        Identity: IResourceManagerSink_Impl,
    {
        unsafe extern "system" fn TMDown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IResourceManagerSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IResourceManagerSink_Impl::TMDown(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), TMDown: TMDown::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IResourceManagerSink as windows_core::Interface>::IID
    }
}
pub trait ITipHelper_Impl: Sized {
    fn Pull(&self, i_psztxurl: *const u8) -> windows_core::Result<ITransaction>;
    fn PullAsync(&self, i_psztxurl: *const u8, i_ptippullsink: Option<&ITipPullSink>) -> windows_core::Result<ITransaction>;
    fn GetLocalTmUrl(&self) -> windows_core::Result<*mut u8>;
}
impl windows_core::RuntimeName for ITipHelper {}
impl ITipHelper_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITipHelper_Vtbl
    where
        Identity: ITipHelper_Impl,
    {
        unsafe extern "system" fn Pull<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, i_psztxurl: *const u8, o_ppitransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITipHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITipHelper_Impl::Pull(this, core::mem::transmute_copy(&i_psztxurl)) {
                Ok(ok__) => {
                    o_ppitransaction.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PullAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, i_psztxurl: *const u8, i_ptippullsink: *mut core::ffi::c_void, o_ppitransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITipHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITipHelper_Impl::PullAsync(this, core::mem::transmute_copy(&i_psztxurl), windows_core::from_raw_borrowed(&i_ptippullsink)) {
                Ok(ok__) => {
                    o_ppitransaction.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalTmUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, o_ppszlocaltmurl: *mut *mut u8) -> windows_core::HRESULT
        where
            Identity: ITipHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITipHelper_Impl::GetLocalTmUrl(this) {
                Ok(ok__) => {
                    o_ppszlocaltmurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Pull: Pull::<Identity, OFFSET>,
            PullAsync: PullAsync::<Identity, OFFSET>,
            GetLocalTmUrl: GetLocalTmUrl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITipHelper as windows_core::Interface>::IID
    }
}
pub trait ITipPullSink_Impl: Sized {
    fn PullComplete(&self, i_hrpull: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITipPullSink {}
impl ITipPullSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITipPullSink_Vtbl
    where
        Identity: ITipPullSink_Impl,
    {
        unsafe extern "system" fn PullComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, i_hrpull: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ITipPullSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITipPullSink_Impl::PullComplete(this, core::mem::transmute_copy(&i_hrpull)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), PullComplete: PullComplete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITipPullSink as windows_core::Interface>::IID
    }
}
pub trait ITipTransaction_Impl: Sized {
    fn Push(&self, i_pszremotetmurl: *const u8) -> windows_core::Result<windows_core::PSTR>;
    fn GetTransactionUrl(&self) -> windows_core::Result<windows_core::PSTR>;
}
impl windows_core::RuntimeName for ITipTransaction {}
impl ITipTransaction_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITipTransaction_Vtbl
    where
        Identity: ITipTransaction_Impl,
    {
        unsafe extern "system" fn Push<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, i_pszremotetmurl: *const u8, o_ppszremotetxurl: *mut windows_core::PSTR) -> windows_core::HRESULT
        where
            Identity: ITipTransaction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITipTransaction_Impl::Push(this, core::mem::transmute_copy(&i_pszremotetmurl)) {
                Ok(ok__) => {
                    o_ppszremotetxurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransactionUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, o_ppszlocaltxurl: *mut windows_core::PSTR) -> windows_core::HRESULT
        where
            Identity: ITipTransaction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITipTransaction_Impl::GetTransactionUrl(this) {
                Ok(ok__) => {
                    o_ppszlocaltxurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Push: Push::<Identity, OFFSET>,
            GetTransactionUrl: GetTransactionUrl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITipTransaction as windows_core::Interface>::IID
    }
}
pub trait ITmNodeName_Impl: Sized {
    fn GetNodeNameSize(&self) -> windows_core::Result<u32>;
    fn GetNodeName(&self, cbnodenamebuffersize: u32, pnodenamebuffer: &windows_core::PWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITmNodeName {}
impl ITmNodeName_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITmNodeName_Vtbl
    where
        Identity: ITmNodeName_Impl,
    {
        unsafe extern "system" fn GetNodeNameSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbnodenamesize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITmNodeName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITmNodeName_Impl::GetNodeNameSize(this) {
                Ok(ok__) => {
                    pcbnodenamesize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNodeName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbnodenamebuffersize: u32, pnodenamebuffer: windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ITmNodeName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITmNodeName_Impl::GetNodeName(this, core::mem::transmute_copy(&cbnodenamebuffersize), core::mem::transmute(&pnodenamebuffer)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNodeNameSize: GetNodeNameSize::<Identity, OFFSET>,
            GetNodeName: GetNodeName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITmNodeName as windows_core::Interface>::IID
    }
}
pub trait ITransaction_Impl: Sized {
    fn Commit(&self, fretaining: super::super::Foundation::BOOL, grftc: u32, grfrm: u32) -> windows_core::Result<()>;
    fn Abort(&self, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, fasync: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetTransactionInfo(&self, pinfo: *mut XACTTRANSINFO) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITransaction {}
impl ITransaction_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransaction_Vtbl
    where
        Identity: ITransaction_Impl,
    {
        unsafe extern "system" fn Commit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fretaining: super::super::Foundation::BOOL, grftc: u32, grfrm: u32) -> windows_core::HRESULT
        where
            Identity: ITransaction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransaction_Impl::Commit(this, core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&grftc), core::mem::transmute_copy(&grfrm)).into()
        }
        unsafe extern "system" fn Abort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, fasync: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITransaction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransaction_Impl::Abort(this, core::mem::transmute_copy(&pboidreason), core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&fasync)).into()
        }
        unsafe extern "system" fn GetTransactionInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut XACTTRANSINFO) -> windows_core::HRESULT
        where
            Identity: ITransaction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransaction_Impl::GetTransactionInfo(this, core::mem::transmute_copy(&pinfo)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Commit: Commit::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
            GetTransactionInfo: GetTransactionInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransaction as windows_core::Interface>::IID
    }
}
pub trait ITransaction2_Impl: Sized + ITransactionCloner_Impl {
    fn GetTransactionInfo2(&self, pinfo: *mut XACTTRANSINFO) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITransaction2 {}
impl ITransaction2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransaction2_Vtbl
    where
        Identity: ITransaction2_Impl,
    {
        unsafe extern "system" fn GetTransactionInfo2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut XACTTRANSINFO) -> windows_core::HRESULT
        where
            Identity: ITransaction2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransaction2_Impl::GetTransactionInfo2(this, core::mem::transmute_copy(&pinfo)).into()
        }
        Self { base__: ITransactionCloner_Vtbl::new::<Identity, OFFSET>(), GetTransactionInfo2: GetTransactionInfo2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransaction2 as windows_core::Interface>::IID || iid == &<ITransaction as windows_core::Interface>::IID || iid == &<ITransactionCloner as windows_core::Interface>::IID
    }
}
pub trait ITransactionCloner_Impl: Sized + ITransaction_Impl {
    fn CloneWithCommitDisabled(&self) -> windows_core::Result<ITransaction>;
}
impl windows_core::RuntimeName for ITransactionCloner {}
impl ITransactionCloner_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionCloner_Vtbl
    where
        Identity: ITransactionCloner_Impl,
    {
        unsafe extern "system" fn CloneWithCommitDisabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionCloner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITransactionCloner_Impl::CloneWithCommitDisabled(this) {
                Ok(ok__) => {
                    ppitransaction.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ITransaction_Vtbl::new::<Identity, OFFSET>(), CloneWithCommitDisabled: CloneWithCommitDisabled::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionCloner as windows_core::Interface>::IID || iid == &<ITransaction as windows_core::Interface>::IID
    }
}
pub trait ITransactionDispenser_Impl: Sized {
    fn GetOptionsObject(&self) -> windows_core::Result<ITransactionOptions>;
    fn BeginTransaction(&self, punkouter: Option<&windows_core::IUnknown>, isolevel: i32, isoflags: u32, poptions: Option<&ITransactionOptions>) -> windows_core::Result<ITransaction>;
}
impl windows_core::RuntimeName for ITransactionDispenser {}
impl ITransactionDispenser_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionDispenser_Vtbl
    where
        Identity: ITransactionDispenser_Impl,
    {
        unsafe extern "system" fn GetOptionsObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionDispenser_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITransactionDispenser_Impl::GetOptionsObject(this) {
                Ok(ok__) => {
                    ppoptions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, isolevel: i32, isoflags: u32, poptions: *mut core::ffi::c_void, pptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionDispenser_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITransactionDispenser_Impl::BeginTransaction(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&isolevel), core::mem::transmute_copy(&isoflags), windows_core::from_raw_borrowed(&poptions)) {
                Ok(ok__) => {
                    pptransaction.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOptionsObject: GetOptionsObject::<Identity, OFFSET>,
            BeginTransaction: BeginTransaction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionDispenser as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITransactionEnlistmentAsync_Impl: Sized {
    fn PrepareRequestDone(&self, hr: windows_core::HRESULT, pmk: Option<&super::Com::IMoniker>, pboidreason: *const BOID) -> windows_core::Result<()>;
    fn CommitRequestDone(&self, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn AbortRequestDone(&self, hr: windows_core::HRESULT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITransactionEnlistmentAsync {}
#[cfg(feature = "Win32_System_Com")]
impl ITransactionEnlistmentAsync_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionEnlistmentAsync_Vtbl
    where
        Identity: ITransactionEnlistmentAsync_Impl,
    {
        unsafe extern "system" fn PrepareRequestDone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT, pmk: *mut core::ffi::c_void, pboidreason: *const BOID) -> windows_core::HRESULT
        where
            Identity: ITransactionEnlistmentAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionEnlistmentAsync_Impl::PrepareRequestDone(this, core::mem::transmute_copy(&hr), windows_core::from_raw_borrowed(&pmk), core::mem::transmute_copy(&pboidreason)).into()
        }
        unsafe extern "system" fn CommitRequestDone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ITransactionEnlistmentAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionEnlistmentAsync_Impl::CommitRequestDone(this, core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn AbortRequestDone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ITransactionEnlistmentAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionEnlistmentAsync_Impl::AbortRequestDone(this, core::mem::transmute_copy(&hr)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PrepareRequestDone: PrepareRequestDone::<Identity, OFFSET>,
            CommitRequestDone: CommitRequestDone::<Identity, OFFSET>,
            AbortRequestDone: AbortRequestDone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionEnlistmentAsync as windows_core::Interface>::IID
    }
}
pub trait ITransactionExport_Impl: Sized {
    fn Export(&self, punktransaction: Option<&windows_core::IUnknown>) -> windows_core::Result<u32>;
    fn GetTransactionCookie(&self, punktransaction: Option<&windows_core::IUnknown>, cbtransactioncookie: u32, rgbtransactioncookie: *mut u8, pcbused: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITransactionExport {}
impl ITransactionExport_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionExport_Vtbl
    where
        Identity: ITransactionExport_Impl,
    {
        unsafe extern "system" fn Export<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punktransaction: *mut core::ffi::c_void, pcbtransactioncookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITransactionExport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITransactionExport_Impl::Export(this, windows_core::from_raw_borrowed(&punktransaction)) {
                Ok(ok__) => {
                    pcbtransactioncookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransactionCookie<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punktransaction: *mut core::ffi::c_void, cbtransactioncookie: u32, rgbtransactioncookie: *mut u8, pcbused: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITransactionExport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionExport_Impl::GetTransactionCookie(this, windows_core::from_raw_borrowed(&punktransaction), core::mem::transmute_copy(&cbtransactioncookie), core::mem::transmute_copy(&rgbtransactioncookie), core::mem::transmute_copy(&pcbused)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Export: Export::<Identity, OFFSET>,
            GetTransactionCookie: GetTransactionCookie::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionExport as windows_core::Interface>::IID
    }
}
pub trait ITransactionExportFactory_Impl: Sized {
    fn GetRemoteClassId(&self) -> windows_core::Result<windows_core::GUID>;
    fn Create(&self, cbwhereabouts: u32, rgbwhereabouts: *const u8) -> windows_core::Result<ITransactionExport>;
}
impl windows_core::RuntimeName for ITransactionExportFactory {}
impl ITransactionExportFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionExportFactory_Vtbl
    where
        Identity: ITransactionExportFactory_Impl,
    {
        unsafe extern "system" fn GetRemoteClassId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITransactionExportFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITransactionExportFactory_Impl::GetRemoteClassId(this) {
                Ok(ok__) => {
                    pclsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbwhereabouts: u32, rgbwhereabouts: *const u8, ppexport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionExportFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITransactionExportFactory_Impl::Create(this, core::mem::transmute_copy(&cbwhereabouts), core::mem::transmute_copy(&rgbwhereabouts)) {
                Ok(ok__) => {
                    ppexport.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRemoteClassId: GetRemoteClassId::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionExportFactory as windows_core::Interface>::IID
    }
}
pub trait ITransactionImport_Impl: Sized {
    fn Import(&self, cbtransactioncookie: u32, rgbtransactioncookie: *const u8, piid: *const windows_core::GUID, ppvtransaction: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITransactionImport {}
impl ITransactionImport_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionImport_Vtbl
    where
        Identity: ITransactionImport_Impl,
    {
        unsafe extern "system" fn Import<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbtransactioncookie: u32, rgbtransactioncookie: *const u8, piid: *const windows_core::GUID, ppvtransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionImport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionImport_Impl::Import(this, core::mem::transmute_copy(&cbtransactioncookie), core::mem::transmute_copy(&rgbtransactioncookie), core::mem::transmute_copy(&piid), core::mem::transmute_copy(&ppvtransaction)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Import: Import::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionImport as windows_core::Interface>::IID
    }
}
pub trait ITransactionImportWhereabouts_Impl: Sized {
    fn GetWhereaboutsSize(&self) -> windows_core::Result<u32>;
    fn GetWhereabouts(&self, cbwhereabouts: u32, rgbwhereabouts: *mut u8, pcbused: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITransactionImportWhereabouts {}
impl ITransactionImportWhereabouts_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionImportWhereabouts_Vtbl
    where
        Identity: ITransactionImportWhereabouts_Impl,
    {
        unsafe extern "system" fn GetWhereaboutsSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbwhereabouts: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITransactionImportWhereabouts_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITransactionImportWhereabouts_Impl::GetWhereaboutsSize(this) {
                Ok(ok__) => {
                    pcbwhereabouts.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWhereabouts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbwhereabouts: u32, rgbwhereabouts: *mut u8, pcbused: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITransactionImportWhereabouts_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionImportWhereabouts_Impl::GetWhereabouts(this, core::mem::transmute_copy(&cbwhereabouts), core::mem::transmute_copy(&rgbwhereabouts), core::mem::transmute_copy(&pcbused)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWhereaboutsSize: GetWhereaboutsSize::<Identity, OFFSET>,
            GetWhereabouts: GetWhereabouts::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionImportWhereabouts as windows_core::Interface>::IID
    }
}
pub trait ITransactionLastEnlistmentAsync_Impl: Sized {
    fn TransactionOutcome(&self, xactstat: XACTSTAT, pboidreason: *const BOID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITransactionLastEnlistmentAsync {}
impl ITransactionLastEnlistmentAsync_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionLastEnlistmentAsync_Vtbl
    where
        Identity: ITransactionLastEnlistmentAsync_Impl,
    {
        unsafe extern "system" fn TransactionOutcome<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, xactstat: XACTSTAT, pboidreason: *const BOID) -> windows_core::HRESULT
        where
            Identity: ITransactionLastEnlistmentAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionLastEnlistmentAsync_Impl::TransactionOutcome(this, core::mem::transmute_copy(&xactstat), core::mem::transmute_copy(&pboidreason)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), TransactionOutcome: TransactionOutcome::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionLastEnlistmentAsync as windows_core::Interface>::IID
    }
}
pub trait ITransactionLastResourceAsync_Impl: Sized {
    fn DelegateCommit(&self, grfrm: u32) -> windows_core::Result<()>;
    fn ForgetRequest(&self, pnewuow: *const BOID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITransactionLastResourceAsync {}
impl ITransactionLastResourceAsync_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionLastResourceAsync_Vtbl
    where
        Identity: ITransactionLastResourceAsync_Impl,
    {
        unsafe extern "system" fn DelegateCommit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfrm: u32) -> windows_core::HRESULT
        where
            Identity: ITransactionLastResourceAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionLastResourceAsync_Impl::DelegateCommit(this, core::mem::transmute_copy(&grfrm)).into()
        }
        unsafe extern "system" fn ForgetRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewuow: *const BOID) -> windows_core::HRESULT
        where
            Identity: ITransactionLastResourceAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionLastResourceAsync_Impl::ForgetRequest(this, core::mem::transmute_copy(&pnewuow)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DelegateCommit: DelegateCommit::<Identity, OFFSET>,
            ForgetRequest: ForgetRequest::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionLastResourceAsync as windows_core::Interface>::IID
    }
}
pub trait ITransactionOptions_Impl: Sized {
    fn SetOptions(&self, poptions: *const XACTOPT) -> windows_core::Result<()>;
    fn GetOptions(&self, poptions: *mut XACTOPT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITransactionOptions {}
impl ITransactionOptions_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionOptions_Vtbl
    where
        Identity: ITransactionOptions_Impl,
    {
        unsafe extern "system" fn SetOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, poptions: *const XACTOPT) -> windows_core::HRESULT
        where
            Identity: ITransactionOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionOptions_Impl::SetOptions(this, core::mem::transmute_copy(&poptions)).into()
        }
        unsafe extern "system" fn GetOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, poptions: *mut XACTOPT) -> windows_core::HRESULT
        where
            Identity: ITransactionOptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionOptions_Impl::GetOptions(this, core::mem::transmute_copy(&poptions)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetOptions: SetOptions::<Identity, OFFSET>,
            GetOptions: GetOptions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionOptions as windows_core::Interface>::IID
    }
}
pub trait ITransactionOutcomeEvents_Impl: Sized {
    fn Committed(&self, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn Aborted(&self, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn HeuristicDecision(&self, dwdecision: u32, pboidreason: *const BOID, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn Indoubt(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITransactionOutcomeEvents {}
impl ITransactionOutcomeEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionOutcomeEvents_Vtbl
    where
        Identity: ITransactionOutcomeEvents_Impl,
    {
        unsafe extern "system" fn Committed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ITransactionOutcomeEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionOutcomeEvents_Impl::Committed(this, core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&pnewuow), core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn Aborted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID, hr: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ITransactionOutcomeEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionOutcomeEvents_Impl::Aborted(this, core::mem::transmute_copy(&pboidreason), core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&pnewuow), core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn HeuristicDecision<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdecision: u32, pboidreason: *const BOID, hr: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ITransactionOutcomeEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionOutcomeEvents_Impl::HeuristicDecision(this, core::mem::transmute_copy(&dwdecision), core::mem::transmute_copy(&pboidreason), core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn Indoubt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionOutcomeEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionOutcomeEvents_Impl::Indoubt(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Committed: Committed::<Identity, OFFSET>,
            Aborted: Aborted::<Identity, OFFSET>,
            HeuristicDecision: HeuristicDecision::<Identity, OFFSET>,
            Indoubt: Indoubt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionOutcomeEvents as windows_core::Interface>::IID
    }
}
pub trait ITransactionPhase0EnlistmentAsync_Impl: Sized {
    fn Enable(&self) -> windows_core::Result<()>;
    fn WaitForEnlistment(&self) -> windows_core::Result<()>;
    fn Phase0Done(&self) -> windows_core::Result<()>;
    fn Unenlist(&self) -> windows_core::Result<()>;
    fn GetTransaction(&self) -> windows_core::Result<ITransaction>;
}
impl windows_core::RuntimeName for ITransactionPhase0EnlistmentAsync {}
impl ITransactionPhase0EnlistmentAsync_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionPhase0EnlistmentAsync_Vtbl
    where
        Identity: ITransactionPhase0EnlistmentAsync_Impl,
    {
        unsafe extern "system" fn Enable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionPhase0EnlistmentAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionPhase0EnlistmentAsync_Impl::Enable(this).into()
        }
        unsafe extern "system" fn WaitForEnlistment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionPhase0EnlistmentAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionPhase0EnlistmentAsync_Impl::WaitForEnlistment(this).into()
        }
        unsafe extern "system" fn Phase0Done<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionPhase0EnlistmentAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionPhase0EnlistmentAsync_Impl::Phase0Done(this).into()
        }
        unsafe extern "system" fn Unenlist<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionPhase0EnlistmentAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionPhase0EnlistmentAsync_Impl::Unenlist(this).into()
        }
        unsafe extern "system" fn GetTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionPhase0EnlistmentAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITransactionPhase0EnlistmentAsync_Impl::GetTransaction(this) {
                Ok(ok__) => {
                    ppitransaction.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Enable: Enable::<Identity, OFFSET>,
            WaitForEnlistment: WaitForEnlistment::<Identity, OFFSET>,
            Phase0Done: Phase0Done::<Identity, OFFSET>,
            Unenlist: Unenlist::<Identity, OFFSET>,
            GetTransaction: GetTransaction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionPhase0EnlistmentAsync as windows_core::Interface>::IID
    }
}
pub trait ITransactionPhase0Factory_Impl: Sized {
    fn Create(&self, pphase0notify: Option<&ITransactionPhase0NotifyAsync>) -> windows_core::Result<ITransactionPhase0EnlistmentAsync>;
}
impl windows_core::RuntimeName for ITransactionPhase0Factory {}
impl ITransactionPhase0Factory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionPhase0Factory_Vtbl
    where
        Identity: ITransactionPhase0Factory_Impl,
    {
        unsafe extern "system" fn Create<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphase0notify: *mut core::ffi::c_void, ppphase0enlistment: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionPhase0Factory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITransactionPhase0Factory_Impl::Create(this, windows_core::from_raw_borrowed(&pphase0notify)) {
                Ok(ok__) => {
                    ppphase0enlistment.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionPhase0Factory as windows_core::Interface>::IID
    }
}
pub trait ITransactionPhase0NotifyAsync_Impl: Sized {
    fn Phase0Request(&self, fabortinghint: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn EnlistCompleted(&self, status: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITransactionPhase0NotifyAsync {}
impl ITransactionPhase0NotifyAsync_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionPhase0NotifyAsync_Vtbl
    where
        Identity: ITransactionPhase0NotifyAsync_Impl,
    {
        unsafe extern "system" fn Phase0Request<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fabortinghint: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITransactionPhase0NotifyAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionPhase0NotifyAsync_Impl::Phase0Request(this, core::mem::transmute_copy(&fabortinghint)).into()
        }
        unsafe extern "system" fn EnlistCompleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ITransactionPhase0NotifyAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionPhase0NotifyAsync_Impl::EnlistCompleted(this, core::mem::transmute_copy(&status)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Phase0Request: Phase0Request::<Identity, OFFSET>,
            EnlistCompleted: EnlistCompleted::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionPhase0NotifyAsync as windows_core::Interface>::IID
    }
}
pub trait ITransactionReceiver_Impl: Sized {
    fn UnmarshalPropagationToken(&self, cbtoken: u32, rgbtoken: *const u8) -> windows_core::Result<ITransaction>;
    fn GetReturnTokenSize(&self) -> windows_core::Result<u32>;
    fn MarshalReturnToken(&self, cbreturntoken: u32, rgbreturntoken: *mut u8, pcbused: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITransactionReceiver {}
impl ITransactionReceiver_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionReceiver_Vtbl
    where
        Identity: ITransactionReceiver_Impl,
    {
        unsafe extern "system" fn UnmarshalPropagationToken<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbtoken: u32, rgbtoken: *const u8, pptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionReceiver_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITransactionReceiver_Impl::UnmarshalPropagationToken(this, core::mem::transmute_copy(&cbtoken), core::mem::transmute_copy(&rgbtoken)) {
                Ok(ok__) => {
                    pptransaction.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReturnTokenSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbreturntoken: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITransactionReceiver_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITransactionReceiver_Impl::GetReturnTokenSize(this) {
                Ok(ok__) => {
                    pcbreturntoken.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarshalReturnToken<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbreturntoken: u32, rgbreturntoken: *mut u8, pcbused: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITransactionReceiver_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionReceiver_Impl::MarshalReturnToken(this, core::mem::transmute_copy(&cbreturntoken), core::mem::transmute_copy(&rgbreturntoken), core::mem::transmute_copy(&pcbused)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionReceiver_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionReceiver_Impl::Reset(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            UnmarshalPropagationToken: UnmarshalPropagationToken::<Identity, OFFSET>,
            GetReturnTokenSize: GetReturnTokenSize::<Identity, OFFSET>,
            MarshalReturnToken: MarshalReturnToken::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionReceiver as windows_core::Interface>::IID
    }
}
pub trait ITransactionReceiverFactory_Impl: Sized {
    fn Create(&self) -> windows_core::Result<ITransactionReceiver>;
}
impl windows_core::RuntimeName for ITransactionReceiverFactory {}
impl ITransactionReceiverFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionReceiverFactory_Vtbl
    where
        Identity: ITransactionReceiverFactory_Impl,
    {
        unsafe extern "system" fn Create<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppreceiver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionReceiverFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITransactionReceiverFactory_Impl::Create(this) {
                Ok(ok__) => {
                    ppreceiver.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionReceiverFactory as windows_core::Interface>::IID
    }
}
pub trait ITransactionResource_Impl: Sized {
    fn PrepareRequest(&self, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn CommitRequest(&self, grfrm: u32, pnewuow: *const BOID) -> windows_core::Result<()>;
    fn AbortRequest(&self, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> windows_core::Result<()>;
    fn TMDown(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITransactionResource {}
impl ITransactionResource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionResource_Vtbl
    where
        Identity: ITransactionResource_Impl,
    {
        unsafe extern "system" fn PrepareRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITransactionResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionResource_Impl::PrepareRequest(this, core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&grfrm), core::mem::transmute_copy(&fwantmoniker), core::mem::transmute_copy(&fsinglephase)).into()
        }
        unsafe extern "system" fn CommitRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfrm: u32, pnewuow: *const BOID) -> windows_core::HRESULT
        where
            Identity: ITransactionResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionResource_Impl::CommitRequest(this, core::mem::transmute_copy(&grfrm), core::mem::transmute_copy(&pnewuow)).into()
        }
        unsafe extern "system" fn AbortRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> windows_core::HRESULT
        where
            Identity: ITransactionResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionResource_Impl::AbortRequest(this, core::mem::transmute_copy(&pboidreason), core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&pnewuow)).into()
        }
        unsafe extern "system" fn TMDown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionResource_Impl::TMDown(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PrepareRequest: PrepareRequest::<Identity, OFFSET>,
            CommitRequest: CommitRequest::<Identity, OFFSET>,
            AbortRequest: AbortRequest::<Identity, OFFSET>,
            TMDown: TMDown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionResource as windows_core::Interface>::IID
    }
}
pub trait ITransactionResourceAsync_Impl: Sized {
    fn PrepareRequest(&self, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn CommitRequest(&self, grfrm: u32, pnewuow: *const BOID) -> windows_core::Result<()>;
    fn AbortRequest(&self, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> windows_core::Result<()>;
    fn TMDown(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITransactionResourceAsync {}
impl ITransactionResourceAsync_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionResourceAsync_Vtbl
    where
        Identity: ITransactionResourceAsync_Impl,
    {
        unsafe extern "system" fn PrepareRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fretaining: super::super::Foundation::BOOL, grfrm: u32, fwantmoniker: super::super::Foundation::BOOL, fsinglephase: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITransactionResourceAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionResourceAsync_Impl::PrepareRequest(this, core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&grfrm), core::mem::transmute_copy(&fwantmoniker), core::mem::transmute_copy(&fsinglephase)).into()
        }
        unsafe extern "system" fn CommitRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfrm: u32, pnewuow: *const BOID) -> windows_core::HRESULT
        where
            Identity: ITransactionResourceAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionResourceAsync_Impl::CommitRequest(this, core::mem::transmute_copy(&grfrm), core::mem::transmute_copy(&pnewuow)).into()
        }
        unsafe extern "system" fn AbortRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pboidreason: *const BOID, fretaining: super::super::Foundation::BOOL, pnewuow: *const BOID) -> windows_core::HRESULT
        where
            Identity: ITransactionResourceAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionResourceAsync_Impl::AbortRequest(this, core::mem::transmute_copy(&pboidreason), core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&pnewuow)).into()
        }
        unsafe extern "system" fn TMDown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionResourceAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionResourceAsync_Impl::TMDown(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PrepareRequest: PrepareRequest::<Identity, OFFSET>,
            CommitRequest: CommitRequest::<Identity, OFFSET>,
            AbortRequest: AbortRequest::<Identity, OFFSET>,
            TMDown: TMDown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionResourceAsync as windows_core::Interface>::IID
    }
}
pub trait ITransactionTransmitter_Impl: Sized {
    fn Set(&self, ptransaction: Option<&ITransaction>) -> windows_core::Result<()>;
    fn GetPropagationTokenSize(&self) -> windows_core::Result<u32>;
    fn MarshalPropagationToken(&self, cbtoken: u32, rgbtoken: *mut u8, pcbused: *mut u32) -> windows_core::Result<()>;
    fn UnmarshalReturnToken(&self, cbreturntoken: u32, rgbreturntoken: *const u8) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITransactionTransmitter {}
impl ITransactionTransmitter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionTransmitter_Vtbl
    where
        Identity: ITransactionTransmitter_Impl,
    {
        unsafe extern "system" fn Set<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionTransmitter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionTransmitter_Impl::Set(this, windows_core::from_raw_borrowed(&ptransaction)).into()
        }
        unsafe extern "system" fn GetPropagationTokenSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbtoken: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITransactionTransmitter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITransactionTransmitter_Impl::GetPropagationTokenSize(this) {
                Ok(ok__) => {
                    pcbtoken.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarshalPropagationToken<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbtoken: u32, rgbtoken: *mut u8, pcbused: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITransactionTransmitter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionTransmitter_Impl::MarshalPropagationToken(this, core::mem::transmute_copy(&cbtoken), core::mem::transmute_copy(&rgbtoken), core::mem::transmute_copy(&pcbused)).into()
        }
        unsafe extern "system" fn UnmarshalReturnToken<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbreturntoken: u32, rgbreturntoken: *const u8) -> windows_core::HRESULT
        where
            Identity: ITransactionTransmitter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionTransmitter_Impl::UnmarshalReturnToken(this, core::mem::transmute_copy(&cbreturntoken), core::mem::transmute_copy(&rgbreturntoken)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionTransmitter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionTransmitter_Impl::Reset(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Set: Set::<Identity, OFFSET>,
            GetPropagationTokenSize: GetPropagationTokenSize::<Identity, OFFSET>,
            MarshalPropagationToken: MarshalPropagationToken::<Identity, OFFSET>,
            UnmarshalReturnToken: UnmarshalReturnToken::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionTransmitter as windows_core::Interface>::IID
    }
}
pub trait ITransactionTransmitterFactory_Impl: Sized {
    fn Create(&self) -> windows_core::Result<ITransactionTransmitter>;
}
impl windows_core::RuntimeName for ITransactionTransmitterFactory {}
impl ITransactionTransmitterFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionTransmitterFactory_Vtbl
    where
        Identity: ITransactionTransmitterFactory_Impl,
    {
        unsafe extern "system" fn Create<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptransmitter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionTransmitterFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITransactionTransmitterFactory_Impl::Create(this) {
                Ok(ok__) => {
                    pptransmitter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionTransmitterFactory as windows_core::Interface>::IID
    }
}
pub trait ITransactionVoterBallotAsync2_Impl: Sized {
    fn VoteRequestDone(&self, hr: windows_core::HRESULT, pboidreason: *const BOID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITransactionVoterBallotAsync2 {}
impl ITransactionVoterBallotAsync2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionVoterBallotAsync2_Vtbl
    where
        Identity: ITransactionVoterBallotAsync2_Impl,
    {
        unsafe extern "system" fn VoteRequestDone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT, pboidreason: *const BOID) -> windows_core::HRESULT
        where
            Identity: ITransactionVoterBallotAsync2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionVoterBallotAsync2_Impl::VoteRequestDone(this, core::mem::transmute_copy(&hr), core::mem::transmute_copy(&pboidreason)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), VoteRequestDone: VoteRequestDone::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionVoterBallotAsync2 as windows_core::Interface>::IID
    }
}
pub trait ITransactionVoterFactory2_Impl: Sized {
    fn Create(&self, ptransaction: Option<&ITransaction>, pvoternotify: Option<&ITransactionVoterNotifyAsync2>) -> windows_core::Result<ITransactionVoterBallotAsync2>;
}
impl windows_core::RuntimeName for ITransactionVoterFactory2 {}
impl ITransactionVoterFactory2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionVoterFactory2_Vtbl
    where
        Identity: ITransactionVoterFactory2_Impl,
    {
        unsafe extern "system" fn Create<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut core::ffi::c_void, pvoternotify: *mut core::ffi::c_void, ppvoterballot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionVoterFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITransactionVoterFactory2_Impl::Create(this, windows_core::from_raw_borrowed(&ptransaction), windows_core::from_raw_borrowed(&pvoternotify)) {
                Ok(ok__) => {
                    ppvoterballot.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionVoterFactory2 as windows_core::Interface>::IID
    }
}
pub trait ITransactionVoterNotifyAsync2_Impl: Sized + ITransactionOutcomeEvents_Impl {
    fn VoteRequest(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITransactionVoterNotifyAsync2 {}
impl ITransactionVoterNotifyAsync2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionVoterNotifyAsync2_Vtbl
    where
        Identity: ITransactionVoterNotifyAsync2_Impl,
    {
        unsafe extern "system" fn VoteRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionVoterNotifyAsync2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionVoterNotifyAsync2_Impl::VoteRequest(this).into()
        }
        Self { base__: ITransactionOutcomeEvents_Vtbl::new::<Identity, OFFSET>(), VoteRequest: VoteRequest::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionVoterNotifyAsync2 as windows_core::Interface>::IID || iid == &<ITransactionOutcomeEvents as windows_core::Interface>::IID
    }
}
pub trait IXAConfig_Impl: Sized {
    fn Initialize(&self, clsidhelperdll: &windows_core::GUID) -> windows_core::Result<()>;
    fn Terminate(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXAConfig {}
impl IXAConfig_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXAConfig_Vtbl
    where
        Identity: IXAConfig_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsidhelperdll: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IXAConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXAConfig_Impl::Initialize(this, core::mem::transmute(&clsidhelperdll)).into()
        }
        unsafe extern "system" fn Terminate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXAConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXAConfig_Impl::Terminate(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Terminate: Terminate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXAConfig as windows_core::Interface>::IID
    }
}
pub trait IXAObtainRMInfo_Impl: Sized {
    fn ObtainRMInfo(&self, pirmhelper: Option<&IRMHelper>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXAObtainRMInfo {}
impl IXAObtainRMInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXAObtainRMInfo_Vtbl
    where
        Identity: IXAObtainRMInfo_Impl,
    {
        unsafe extern "system" fn ObtainRMInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pirmhelper: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXAObtainRMInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXAObtainRMInfo_Impl::ObtainRMInfo(this, windows_core::from_raw_borrowed(&pirmhelper)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ObtainRMInfo: ObtainRMInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXAObtainRMInfo as windows_core::Interface>::IID
    }
}
pub trait IXATransLookup_Impl: Sized {
    fn Lookup(&self) -> windows_core::Result<ITransaction>;
}
impl windows_core::RuntimeName for IXATransLookup {}
impl IXATransLookup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXATransLookup_Vtbl
    where
        Identity: IXATransLookup_Impl,
    {
        unsafe extern "system" fn Lookup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXATransLookup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXATransLookup_Impl::Lookup(this) {
                Ok(ok__) => {
                    pptransaction.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Lookup: Lookup::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXATransLookup as windows_core::Interface>::IID
    }
}
pub trait IXATransLookup2_Impl: Sized {
    fn Lookup(&self, pxid: *const XID) -> windows_core::Result<ITransaction>;
}
impl windows_core::RuntimeName for IXATransLookup2 {}
impl IXATransLookup2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXATransLookup2_Vtbl
    where
        Identity: IXATransLookup2_Impl,
    {
        unsafe extern "system" fn Lookup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxid: *const XID, pptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXATransLookup2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXATransLookup2_Impl::Lookup(this, core::mem::transmute_copy(&pxid)) {
                Ok(ok__) => {
                    pptransaction.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Lookup: Lookup::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXATransLookup2 as windows_core::Interface>::IID
    }
}
