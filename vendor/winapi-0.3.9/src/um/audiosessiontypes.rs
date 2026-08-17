// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::DWORD;
ENUM!{enum AUDCLNT_SHAREMODE {
    AUDCLNT_SHAREMODE_SHARED,
    AUDCLNT_SHAREMODE_EXCLUSIVE,
}}
ENUM!{enum AUDIO_STREAM_CATEGORY {
    AudioCategory_Other = 0,
    AudioCategory_ForegroundOnlyMedia = 1,
    AudioCategory_BackgroundCapableMedia = 2,
    AudioCategory_Communications = 3,
    AudioCategory_Alerts = 4,
    AudioCategory_SoundEffects = 5,
    AudioCategory_GameEffects = 6,
    AudioCategory_GameMedia = 7,
    AudioCategory_GameChat = 8,
    AudioCategory_Speech = 9,
    AudioCategory_Movie = 10,
    AudioCategory_Media = 11,
}}
pub const AUDCLNT_STREAMFLAGS_CROSSPROCESS: DWORD = 0x00010000;
pub const AUDCLNT_STREAMFLAGS_LOOPBACK: DWORD = 0x00020000;
pub const AUDCLNT_STREAMFLAGS_EVENTCALLBACK: DWORD = 0x00040000;
pub const AUDCLNT_STREAMFLAGS_NOPERSIST: DWORD = 0x00080000;
pub const AUDCLNT_STREAMFLAGS_RATEADJUST: DWORD = 0x00100000;
pub const AUDCLNT_SESSIONFLAGS_EXPIREWHENUNOWNED: DWORD = 0x10000000;
pub const AUDCLNT_SESSIONFLAGS_DISPLAY_HIDE: DWORD = 0x20000000;
pub const AUDCLNT_SESSIONFLAGS_DISPLAY_HIDEWHENEXPIRED: DWORD = 0x40000000;
ENUM!{enum AudioSessionState {
    AudioSessionStateInactive = 0,
    AudioSessionStateActive = 1,
    AudioSessionStateExpired = 2,
}}
