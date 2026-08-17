// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::{UINT32, UINT64};
use shared::guiddef::REFIID;
use um::objidl::IApartmentShutdown;
use um::winnt::{HRESULT, VOID};
use winrt::activation::IActivationFactory;
use winrt::hstring::HSTRING;
use winrt::inspectable::IInspectable;
ENUM!{enum RO_INIT_TYPE {
    RO_INIT_SINGLETHREADED = 0,
    RO_INIT_MULTITHREADED = 1,
}}
pub enum RO_REGISTRATION_COOKIE__ {}
pub type RO_REGISTRATION_COOKIE = *mut RO_REGISTRATION_COOKIE__;
FN!{stdcall PFNGETACTIVATIONFACTORY(
    HSTRING,
    *mut *mut IActivationFactory,
) -> HRESULT}
extern "system" {
    pub fn RoInitialize(
        initType: RO_INIT_TYPE,
    ) -> HRESULT;
    pub fn RoUninitialize();
    pub fn RoActivateInstance(
        activatableClassId: HSTRING,
        instance: *mut *mut IInspectable,
    ) -> HRESULT;
    pub fn RoRegisterActivationFactories(
        activatableClassIds: *const HSTRING,
        activationFactoryCallbacks: *const PFNGETACTIVATIONFACTORY,
        count: UINT32,
        cookie: *mut RO_REGISTRATION_COOKIE,
    ) -> HRESULT;
    pub fn RoRevokeActivationFactories(
        cookie: RO_REGISTRATION_COOKIE,
    );
    pub fn RoGetActivationFactory(
        activatableClassId: HSTRING,
        iid: REFIID,
        factory: *mut *mut VOID,
    ) -> HRESULT;
}
DECLARE_HANDLE!{APARTMENT_SHUTDOWN_REGISTRATION_COOKIE, APARTMENT_SHUTDOWN_REGISTRATION_COOKIE__}
extern "system" {
    pub fn RoRegisterForApartmentShutdown(
        callbackObject: *const IApartmentShutdown,
        apartmentIdentifier: *mut UINT64,
        regCookie: *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE,
    ) -> HRESULT;
    pub fn RoUnregisterForApartmentShutdown(
        regCookie: APARTMENT_SHUTDOWN_REGISTRATION_COOKIE,
    ) -> HRESULT;
    pub fn RoGetApartmentIdentifier(
        apartmentIdentifier: *mut UINT64,
    ) -> HRESULT;
}
