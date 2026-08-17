// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::UINT64;
use shared::rpcndr::byte;
use um::bits::{IBackgroundCopyJob, IBackgroundCopyJobVtbl};
use um::winnt::{HRESULT, LPCWSTR, LPWSTR};
STRUCT!{struct BG_JOB_REPLY_PROGRESS {
    BytesTotal: UINT64,
    BytesTransferred: UINT64,
}}
ENUM!{enum BG_AUTH_TARGET {
    BG_AUTH_TARGET_SERVER = 1,
    BG_AUTH_TARGET_PROXY = BG_AUTH_TARGET_SERVER + 1,
}}
ENUM!{enum BG_AUTH_SCHEME {
    BG_AUTH_SCHEME_BASIC = 1,
    BG_AUTH_SCHEME_DIGEST = BG_AUTH_SCHEME_BASIC + 1,
    BG_AUTH_SCHEME_NTLM = BG_AUTH_SCHEME_DIGEST + 1,
    BG_AUTH_SCHEME_NEGOTIATE = BG_AUTH_SCHEME_NTLM + 1,
    BG_AUTH_SCHEME_PASSPORT = BG_AUTH_SCHEME_NEGOTIATE + 1,
}}
STRUCT!{struct BG_BASIC_CREDENTIALS {
    UserName: LPWSTR,
    Password: LPWSTR,
}}
UNION!{union BG_AUTH_CREDENTIALS_UNION {
    [usize; 2],
    Basic Basic_mut: BG_BASIC_CREDENTIALS,
}}
STRUCT!{struct BG_AUTH_CREDENTIALS {
    Target: BG_AUTH_TARGET,
    Scheme: BG_AUTH_SCHEME,
    Credentials: BG_AUTH_CREDENTIALS_UNION,
}}
pub type PBG_AUTH_CREDENTIALS = *mut BG_AUTH_CREDENTIALS;
RIDL!{#[uuid(0x54b50739, 0x686f, 0x45eb, 0x9d, 0xff, 0xd6, 0xa9, 0xa0, 0xfa, 0xa9, 0xaf)]
interface IBackgroundCopyJob2(IBackgroundCopyJob2Vtbl):
    IBackgroundCopyJob(IBackgroundCopyJobVtbl) {
    fn SetNotifyCmdLine(
        Program: LPCWSTR,
        Parameters: LPCWSTR,
    ) -> HRESULT,
    fn GetNotifyCmdLine(
        pProgram: *mut LPWSTR,
        pParameters: *mut LPWSTR,
    ) -> HRESULT,
    fn GetReplyProgress(
        pProgress: *mut BG_JOB_REPLY_PROGRESS,
    ) -> HRESULT,
    fn GetReplyData(
        ppBuffer: *mut *mut byte,
        pLength: *mut UINT64,
    ) -> HRESULT,
    fn SetReplyFileName(
        ReplyFileName: LPCWSTR,
    ) -> HRESULT,
    fn GetReplyFileName(
        pReplyFileName: *mut LPWSTR,
    ) -> HRESULT,
    fn SetCredentials(
        credentials: *mut BG_AUTH_CREDENTIALS,
    ) -> HRESULT,
    fn RemoveCredentials(
        Target: BG_AUTH_TARGET,
        Scheme: BG_AUTH_SCHEME,
    ) -> HRESULT,
}}
