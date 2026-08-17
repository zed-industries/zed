use core_foundation_sys::base::CFTypeRef;
use core_foundation_sys::base::OSStatus;
use core_foundation_sys::bundle::CFBundleRef;
use core_foundation_sys::dictionary::CFDictionaryRef;
use core_foundation_sys::string::CFStringRef;
use std::os::raw::{c_char, c_void};

pub const errAuthorizationSuccess: OSStatus = 0;
pub const errAuthorizationInvalidSet: OSStatus = -60001;
pub const errAuthorizationInvalidRef: OSStatus = -60002;
pub const errAuthorizationInvalidTag: OSStatus = -60003;
pub const errAuthorizationInvalidPointer: OSStatus = -60004;
pub const errAuthorizationDenied: OSStatus = -60005;
pub const errAuthorizationCanceled: OSStatus = -60006;
pub const errAuthorizationInteractionNotAllowed: OSStatus = -60007;
pub const errAuthorizationInternal: OSStatus = -60008;
pub const errAuthorizationExternalizeNotAllowed: OSStatus = -60009;
pub const errAuthorizationInternalizeNotAllowed: OSStatus = -60010;
pub const errAuthorizationInvalidFlags: OSStatus = -60011;
pub const errAuthorizationToolExecuteFailure: OSStatus = -60031;
pub const errAuthorizationToolEnvironmentError: OSStatus = -60032;
pub const errAuthorizationBadAddress: OSStatus = -60033;

pub type AuthorizationFlags = u32;
pub const kAuthorizationFlagDefaults: AuthorizationFlags = 0;
pub const kAuthorizationFlagInteractionAllowed: AuthorizationFlags = 1;
pub const kAuthorizationFlagExtendRights: AuthorizationFlags = 2;
pub const kAuthorizationFlagPartialRights: AuthorizationFlags = 4;
pub const kAuthorizationFlagDestroyRights: AuthorizationFlags = 8;
pub const kAuthorizationFlagPreAuthorize: AuthorizationFlags = 16;

pub type AuthorizationRef = *mut c_void;
pub type AuthorizationString = *const c_char;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AuthorizationItem {
    pub name: AuthorizationString,
    pub valueLength: usize,
    pub value: *mut c_void,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AuthorizationItemSet {
    pub count: u32,
    pub items: *mut AuthorizationItem,
}

pub const kAuthorizationExternalFormLength: usize = 32;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AuthorizationExternalForm {
    pub bytes: [c_char; kAuthorizationExternalFormLength],
}

pub type AuthorizationRights = AuthorizationItemSet;
pub type AuthorizationEnvironment = AuthorizationItemSet;

pub type AuthorizationAsyncCallback =
    unsafe extern "C" fn(err: OSStatus, blockAuthorizedRights: *mut AuthorizationRights);

extern "C" {
    pub fn AuthorizationCreate(
        rights: *const AuthorizationRights,
        environment: *const AuthorizationEnvironment,
        flags: AuthorizationFlags,
        authorization: *mut AuthorizationRef,
    ) -> OSStatus;

    pub fn AuthorizationFree(
        authorization: AuthorizationRef,
        flags: AuthorizationFlags,
    ) -> OSStatus;

    pub fn AuthorizationCopyRights(
        authorization: AuthorizationRef,
        rights: *const AuthorizationRights,
        environment: *const AuthorizationEnvironment,
        flags: AuthorizationFlags,
        authorizedRights: *mut *mut AuthorizationRights,
    ) -> OSStatus;

    pub fn AuthorizationCopyRightsAsync(
        authorization: AuthorizationRef,
        rights: *const AuthorizationRights,
        environment: *const AuthorizationEnvironment,
        flags: AuthorizationFlags,
        callbackBlock: AuthorizationAsyncCallback,
    );

    pub fn AuthorizationCopyInfo(
        authorization: AuthorizationRef,
        tag: AuthorizationString,
        info: *mut *mut AuthorizationItemSet,
    ) -> OSStatus;

    pub fn AuthorizationMakeExternalForm(
        authorization: AuthorizationRef,
        extForm: *mut AuthorizationExternalForm,
    ) -> OSStatus;

    pub fn AuthorizationCreateFromExternalForm(
        extForm: *const AuthorizationExternalForm,
        authorization: *mut AuthorizationRef,
    ) -> OSStatus;

    pub fn AuthorizationFreeItemSet(set: *mut AuthorizationItemSet) -> OSStatus;

    pub fn AuthorizationRightGet(
        rightName: *const c_char,
        rightDefinition: *mut CFDictionaryRef,
    ) -> OSStatus;

    pub fn AuthorizationRightSet(
        authorization: AuthorizationRef,
        rightName: *const c_char,
        rightDefinition: CFTypeRef,
        descriptionKey: CFStringRef,
        bundle: CFBundleRef,
        localeTableName: CFStringRef,
    ) -> OSStatus;

    pub fn AuthorizationRightRemove(
        authorization: AuthorizationRef,
        rightName: *const c_char,
    ) -> OSStatus;

    #[cfg(target_os = "macos")]
    pub fn AuthorizationExecuteWithPrivileges(
        authorization: AuthorizationRef,
        pathToTool: *const c_char,
        options: AuthorizationFlags,
        arguments: *const *mut c_char,
        communicationsPipe: *mut *mut libc::FILE,
    ) -> OSStatus;

    #[cfg(target_os = "macos")]
    pub fn AuthorizationCopyPrivilegedReference(
        authorization: *mut AuthorizationRef,
        flags: AuthorizationFlags,
    ) -> OSStatus;
}
