// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Url History Interfaces
use ctypes::c_void;
use shared::guiddef::REFIID;
use shared::minwindef::{BOOL, DWORD, FILETIME, ULONG};
use shared::wtypesbase::LPCOLESTR;
use um::docobj::{IOleCommandTarget, IOleCommandTargetVtbl};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LPWSTR};
pub const STATURL_QUERYFLAG_ISCACHED: DWORD = 0x00010000;
pub const STATURL_QUERYFLAG_NOURL: DWORD = 0x00020000;
pub const STATURL_QUERYFLAG_NOTITLE: DWORD = 0x00040000;
pub const STATURL_QUERYFLAG_TOPLEVEL: DWORD = 0x00080000;
pub const STATURLFLAG_ISCACHED: DWORD = 0x00000001;
pub const STATURLFLAG_ISTOPLEVEL: DWORD = 0x00000002;
ENUM!{enum ADDURL_FLAG {
    ADDURL_FIRST = 0,
    ADDURL_ADDTOHISTORYANDCACHE = 0,
    ADDURL_ADDTOCACHE = 1,
    ADDURL_Max = 2147483647,
}}
pub type LPENUMSTATURL = *mut IEnumSTATURL;
STRUCT!{struct STATURL {
    cbSize: DWORD,
    pwcsUrl: LPWSTR,
    pwcsTitle: LPWSTR,
    ftLastVisited: FILETIME,
    ftLastUpdated: FILETIME,
    ftExpires: FILETIME,
    dwFlags: DWORD,
}}
pub type LPSTATURL = *mut STATURL;
RIDL!{#[uuid(0x3c374a42, 0xbae4, 0x11cf, 0xbf, 0x7d, 0x00, 0xaa, 0x00, 0x69, 0x46, 0xee)]
interface IEnumSTATURL(IEnumSTATURLVtbl): IUnknown(IUnknownVtbl) {
    fn Next(
        celt: ULONG,
        rgelt: LPSTATURL,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppenum: *mut *mut IEnumSTATURL,
    ) -> HRESULT,
    fn SetFilter(
        poszFilter: LPCOLESTR,
        dwFlags: DWORD,
    ) -> HRESULT,
}}
pub type LPURLHISTORYSTG = *mut IUrlHistoryStg;
RIDL!{#[uuid(0x3c374a41, 0xbae4, 0x11cf, 0xbf, 0x7d, 0x00, 0xaa, 0x00, 0x69, 0x46, 0xee)]
interface IUrlHistoryStg(IUrlHistoryStgVtbl): IUnknown(IUnknownVtbl) {
    fn AddUrl(
        pocsUrl: LPCOLESTR,
    ) -> HRESULT,
    fn DeleteUrl(
        pocsUrl: LPCOLESTR,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn QueryUrl(
        pocsUrl: LPCOLESTR,
        dwFlags: DWORD,
        lpSTATURL: LPSTATURL,
    ) -> HRESULT,
    fn BindToObject(
        pocsUrl: LPCOLESTR,
        riid: REFIID,
        ppvOut: *mut *mut c_void,
    ) -> HRESULT,
    fn EnumUrls(
        ppEnum: *mut *mut IEnumSTATURL,
    ) -> HRESULT,
}}
pub type LPURLHISTORYSTG2 = *mut IUrlHistoryStg2;
RIDL!{#[uuid(0xafa0dc11, 0xc313, 0x11d0, 0x83, 0x1a, 0x00, 0xc0, 0x4f, 0xd5, 0xae, 0x38)]
interface IUrlHistoryStg2(IUrlHistoryStg2Vtbl): IUrlHistoryStg(IUrlHistoryStgVtbl) {
    fn AddUrlAndNotify(
        pocsUrl: LPCOLESTR,
        pocsTitle: LPCOLESTR,
        dwFlags: DWORD,
        fWriteHistory: BOOL,
        poctNotify: *mut IOleCommandTarget,
        punkISFolder: *mut IUnknown,
    ) -> HRESULT,
    fn ClearHistory() -> HRESULT,
}}
pub type LPURLHISTORYNOTIFY = *mut IUrlHistoryNotify;
RIDL!{#[uuid(0xbc40bec1, 0xc493, 0x11d0, 0x83, 0x1b, 0x00, 0xc0, 0x4f, 0xd5, 0xae, 0x38)]
interface IUrlHistoryNotify(IUrlHistoryNotifyVtbl):
    IOleCommandTarget(IOleCommandTargetVtbl) {}
}
