// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::ULONG;
use shared::rpcndr::byte;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LPCWSTR, LPWSTR};
ENUM!{enum BG_CERT_STORE_LOCATION {
    BG_CERT_STORE_LOCATION_CURRENT_USER = 0,
    BG_CERT_STORE_LOCATION_LOCAL_MACHINE = BG_CERT_STORE_LOCATION_CURRENT_USER + 1,
    BG_CERT_STORE_LOCATION_CURRENT_SERVICE = BG_CERT_STORE_LOCATION_LOCAL_MACHINE + 1,
    BG_CERT_STORE_LOCATION_SERVICES = BG_CERT_STORE_LOCATION_CURRENT_SERVICE + 1,
    BG_CERT_STORE_LOCATION_USERS = BG_CERT_STORE_LOCATION_SERVICES + 1,
    BG_CERT_STORE_LOCATION_CURRENT_USER_GROUP_POLICY = BG_CERT_STORE_LOCATION_USERS + 1,
    BG_CERT_STORE_LOCATION_LOCAL_MACHINE_GROUP_POLICY
        = BG_CERT_STORE_LOCATION_CURRENT_USER_GROUP_POLICY + 1,
    BG_CERT_STORE_LOCATION_LOCAL_MACHINE_ENTERPRISE
        = BG_CERT_STORE_LOCATION_LOCAL_MACHINE_GROUP_POLICY + 1,
}}
RIDL!{#[uuid(0xf1bd1079, 0x9f01, 0x4bdc, 0x80, 0x36, 0xf0, 0x9b, 0x70, 0x09, 0x50, 0x66)]
interface IBackgroundCopyJobHttpOptions(IBackgroundCopyJobHttpOptionsVtbl):
    IUnknown(IUnknownVtbl) {
    fn SetClientCertificateByID(
        StoreLocation: BG_CERT_STORE_LOCATION,
        StoreName: LPCWSTR,
        pCertHashBlob: *mut byte,
    ) -> HRESULT,
    fn SetClientCertificateByName(
        StoreLocation: BG_CERT_STORE_LOCATION,
        StoreName: LPCWSTR,
        SubjectName: LPCWSTR,
    ) -> HRESULT,
    fn RemoveClientCertificate() -> HRESULT,
    fn GetClientCertificate(
        pStoreLocation: *mut BG_CERT_STORE_LOCATION,
        pStoreName: *mut LPWSTR,
        ppCertHashBlob: *mut *mut byte,
        pSubjectName: *mut LPWSTR,
    ) -> HRESULT,
    fn SetCustomHeaders(
        RequestHeaders: LPCWSTR,
    ) -> HRESULT,
    fn GetCustomHeaders(
        pRequestHeaders: *mut LPWSTR,
    ) -> HRESULT,
    fn SetSecurityFlags(
        Flags: ULONG,
    ) -> HRESULT,
    fn GetSecurityFlags(
        pFlags: *mut ULONG,
    ) -> HRESULT,
}}
pub const BG_SSL_ENABLE_CRL_CHECK: ULONG = 0x0001;
pub const BG_SSL_IGNORE_CERT_CN_INVALID: ULONG = 0x0002;
pub const BG_SSL_IGNORE_CERT_DATE_INVALID: ULONG = 0x0004;
pub const BG_SSL_IGNORE_UNKNOWN_CA: ULONG = 0x0008;
pub const BG_SSL_IGNORE_CERT_WRONG_USAGE: ULONG = 0x0010;
pub const BG_HTTP_REDIRECT_POLICY_MASK: ULONG = 0x0700;
pub const BG_HTTP_REDIRECT_POLICY_ALLOW_SILENT: ULONG = 0x0000;
pub const BG_HTTP_REDIRECT_POLICY_ALLOW_REPORT: ULONG = 0x0100;
pub const BG_HTTP_REDIRECT_POLICY_DISALLOW: ULONG = 0x0200;
pub const BG_HTTP_REDIRECT_POLICY_ALLOW_HTTPS_TO_HTTP: ULONG = 0x0800;
