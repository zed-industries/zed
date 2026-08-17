// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! ApiSet Contract for api-ms-win-mm-playsound-l1-1-0
use shared::minwindef::{BOOL, DWORD, HMODULE, UINT};
use um::winnt::{LPCSTR, LPCWSTR};
extern "system" {
    pub fn sndPlaySoundA(
        pszSound: LPCSTR,
        fuSound: UINT,
    ) -> BOOL;
    pub fn sndPlaySoundW(
        pszSound: LPCWSTR,
        fuSound: UINT,
    ) -> BOOL;
}
pub const SND_SYNC: DWORD = 0x0000;
pub const SND_ASYNC: DWORD = 0x0001;
pub const SND_NODEFAULT: DWORD = 0x0002;
pub const SND_MEMORY: DWORD = 0x0004;
pub const SND_LOOP: DWORD = 0x0008;
pub const SND_NOSTOP: DWORD = 0x0010;
pub const SND_NOWAIT: DWORD = 0x00002000;
pub const SND_ALIAS: DWORD = 0x00010000;
pub const SND_ALIAS_ID: DWORD = 0x00110000;
pub const SND_FILENAME: DWORD = 0x00020000;
pub const SND_RESOURCE: DWORD = 0x00040004;
pub const SND_PURGE: DWORD = 0x0040;
pub const SND_APPLICATION: DWORD = 0x0080;
pub const SND_SENTRY: DWORD = 0x00080000;
pub const SND_RING: DWORD = 0x00100000;
pub const SND_SYSTEM: DWORD = 0x00200000;
extern "system" {
    pub fn PlaySoundA(
        pszSound: LPCSTR,
        hmod: HMODULE,
        fdwSound: DWORD,
    ) -> BOOL;
    pub fn PlaySoundW(
        pszSound: LPCWSTR,
        hmod: HMODULE,
        fdwSound: DWORD,
    ) -> BOOL;
}
