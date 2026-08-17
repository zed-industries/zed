// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! MM procedure declarations, constant definitions and macros
use shared::basetsd::DWORD_PTR;
use shared::minwindef::{BYTE, DWORD, UINT, WORD};
use shared::mmreg::WAVEFORMATEX;
use um::winnt::{LPSTR, WCHAR};
//109 (Win 7 SDK)
pub type MMVERSION = UINT;
pub type MMRESULT = UINT;
STRUCT!{#[repr(packed)] struct MMTIME_smpte {
    hour: BYTE,
    min: BYTE,
    sec: BYTE,
    frame: BYTE,
    fps: BYTE,
    dummy: BYTE,
    pad: [BYTE; 2],
}}
STRUCT!{#[repr(packed)] struct MMTIME_midi {
    songptrpos: DWORD,
}}
UNION!{#[repr(packed)] union MMTIME_u {
    [u32; 2],
    ms ms_mut: DWORD,
    sample sample_mut: DWORD,
    cb cb_mut: DWORD,
    ticks ticks_mut: DWORD,
    smpte smpte_mut: MMTIME_smpte,
    midi midi_mut: MMTIME_midi,
}}
STRUCT!{#[repr(packed)] struct MMTIME {
    wType: UINT,
    u: MMTIME_u,
}}
pub type PMMTIME = *mut MMTIME;
pub type NPMMTIME = *mut MMTIME;
pub type LPMMTIME = *mut MMTIME;
pub const TIME_MS: UINT = 0x0001;
pub const TIME_SAMPLES: UINT = 0x0002;
pub const TIME_BYTES: UINT = 0x0004;
pub const TIME_SMPTE: UINT = 0x0008;
pub const TIME_MIDI: UINT = 0x0010;
pub const TIME_TICKS: UINT = 0x0020;
pub const MM_JOY1MOVE: UINT = 0x3A0;
pub const MM_JOY2MOVE: UINT = 0x3A1;
pub const MM_JOY1ZMOVE: UINT = 0x3A2;
pub const MM_JOY2ZMOVE: UINT = 0x3A3;
pub const MM_JOY1BUTTONDOWN: UINT = 0x3B5;
pub const MM_JOY2BUTTONDOWN: UINT = 0x3B6;
pub const MM_JOY1BUTTONUP: UINT = 0x3B7;
pub const MM_JOY2BUTTONUP: UINT = 0x3B8;
pub const MM_MCINOTIFY: UINT = 0x3B9;
pub const MM_WOM_OPEN: UINT = 0x3BB;
pub const MM_WOM_CLOSE: UINT = 0x3BC;
pub const MM_WOM_DONE: UINT = 0x3BD;
pub const MM_WIM_OPEN: UINT = 0x3BE;
pub const MM_WIM_CLOSE: UINT = 0x3BF;
pub const MM_WIM_DATA: UINT = 0x3C0;
pub const MM_MIM_OPEN: UINT = 0x3C1;
pub const MM_MIM_CLOSE: UINT = 0x3C2;
pub const MM_MIM_DATA: UINT = 0x3C3;
pub const MM_MIM_LONGDATA: UINT = 0x3C4;
pub const MM_MIM_ERROR: UINT = 0x3C5;
pub const MM_MIM_LONGERROR: UINT = 0x3C6;
pub const MM_MOM_OPEN: UINT = 0x3C7;
pub const MM_MOM_CLOSE: UINT = 0x3C8;
pub const MM_MOM_DONE: UINT = 0x3C9;
pub const MMSYSERR_BASE: MMRESULT = 0;
pub const WAVERR_BASE: MMRESULT = 32;
pub const MIDIERR_BASE: MMRESULT = 64;
pub const TIMERR_BASE: MMRESULT = 96;
pub const JOYERR_BASE: MMRESULT = 160;
pub const MCIERR_BASE: MMRESULT = 256;
pub const MIXERR_BASE: MMRESULT = 1024;
pub const MMSYSERR_NOERROR: MMRESULT = 0;
pub const MMSYSERR_ERROR: MMRESULT = MMSYSERR_BASE + 1;
pub const MMSYSERR_BADDEVICEID: MMRESULT = MMSYSERR_BASE + 2;
pub const MMSYSERR_NOTENABLED: MMRESULT = MMSYSERR_BASE + 3;
pub const MMSYSERR_ALLOCATED: MMRESULT = MMSYSERR_BASE + 4;
pub const MMSYSERR_INVALHANDLE: MMRESULT = MMSYSERR_BASE + 5;
pub const MMSYSERR_NODRIVER: MMRESULT = MMSYSERR_BASE + 6;
pub const MMSYSERR_NOMEM: MMRESULT = MMSYSERR_BASE + 7;
pub const MMSYSERR_NOTSUPPORTED: MMRESULT = MMSYSERR_BASE + 8;
pub const MMSYSERR_BADERRNUM: MMRESULT = MMSYSERR_BASE + 9;
pub const MMSYSERR_INVALFLAG: MMRESULT = MMSYSERR_BASE + 10;
pub const MMSYSERR_INVALPARAM: MMRESULT = MMSYSERR_BASE + 11;
pub const MMSYSERR_HANDLEBUSY: MMRESULT = MMSYSERR_BASE + 12;
pub const MMSYSERR_INVALIDALIAS: MMRESULT = MMSYSERR_BASE + 13;
pub const MMSYSERR_BADDB: MMRESULT = MMSYSERR_BASE + 14;
pub const MMSYSERR_KEYNOTFOUND: MMRESULT = MMSYSERR_BASE + 15;
pub const MMSYSERR_READERROR: MMRESULT = MMSYSERR_BASE + 16;
pub const MMSYSERR_WRITEERROR: MMRESULT = MMSYSERR_BASE + 17;
pub const MMSYSERR_DELETEERROR: MMRESULT = MMSYSERR_BASE + 18;
pub const MMSYSERR_VALNOTFOUND: MMRESULT = MMSYSERR_BASE + 19;
pub const MMSYSERR_NODRIVERCB: MMRESULT = MMSYSERR_BASE + 20;
pub const MMSYSERR_MOREDATA: MMRESULT = MMSYSERR_BASE + 21;
pub const MMSYSERR_LASTERROR: MMRESULT = MMSYSERR_BASE + 21;
pub const MIDIERR_UNPREPARED: MMRESULT = MIDIERR_BASE + 0;
pub const MIDIERR_STILLPLAYING: MMRESULT = MIDIERR_BASE + 1;
pub const MIDIERR_NOMAP: MMRESULT = MIDIERR_BASE + 2;
pub const MIDIERR_NOTREADY: MMRESULT = MIDIERR_BASE + 3;
pub const MIDIERR_NODEVICE: MMRESULT = MIDIERR_BASE + 4;
pub const MIDIERR_INVALIDSETUP: MMRESULT = MIDIERR_BASE + 5;
pub const MIDIERR_BADOPENMODE: MMRESULT = MIDIERR_BASE + 6;
pub const MIDIERR_DONT_CONTINUE: MMRESULT = MIDIERR_BASE + 7;
pub const MIDIERR_LASTERROR: MMRESULT = MIDIERR_BASE + 7;
pub const CALLBACK_TYPEMASK: DWORD = 0x00070000;
pub const CALLBACK_NULL: DWORD = 0x00000000;
pub const CALLBACK_WINDOW: DWORD = 0x00010000;
pub const CALLBACK_TASK: DWORD = 0x00020000;
pub const CALLBACK_FUNCTION: DWORD = 0x00030000;
pub const CALLBACK_THREAD: DWORD = CALLBACK_TASK;
pub const CALLBACK_EVENT: DWORD = 0x00050000;
//497 (Win 7 SDK)
pub const WAVERR_BADFORMAT: MMRESULT = WAVERR_BASE + 0;
pub const WAVERR_STILLPLAYING: MMRESULT = WAVERR_BASE + 1;
pub const WAVERR_UNPREPARED: MMRESULT = WAVERR_BASE + 2;
pub const WAVERR_SYNC: MMRESULT = WAVERR_BASE + 3;
pub const WAVERR_LASTERROR: MMRESULT = WAVERR_BASE + 3;
DECLARE_HANDLE!{HWAVEIN, HWAVEIN__}
DECLARE_HANDLE!{HWAVEOUT, HWAVEOUT__}
pub type LPHWAVEIN = *mut HWAVEIN;
pub type LPHWAVEOUT = *mut HWAVEOUT;
pub const WOM_OPEN: UINT = MM_WOM_OPEN;
pub const WOM_CLOSE: UINT = MM_WOM_CLOSE;
pub const WOM_DONE: UINT = MM_WOM_DONE;
pub const WIM_OPEN: UINT = MM_WIM_OPEN;
pub const WIM_CLOSE: UINT = MM_WIM_CLOSE;
pub const WIM_DATA: UINT = MM_WIM_DATA;
pub const WAVE_MAPPER: UINT = 0xFFFFFFFF;
pub const WAVE_FORMAT_QUERY: DWORD = 0x0001;
pub const WAVE_ALLOWSYNC: DWORD = 0x0002;
pub const WAVE_MAPPED: DWORD = 0x0004;
pub const WAVE_FORMAT_DIRECT: DWORD = 0x0008;
pub const WAVE_FORMAT_DIRECT_QUERY: DWORD = WAVE_FORMAT_QUERY | WAVE_FORMAT_DIRECT;
pub const WAVE_MAPPED_DEFAULT_COMMUNICATION_DEVICE: DWORD = 0x0010;
STRUCT!{#[repr(packed)] struct WAVEHDR {
    lpData: LPSTR,
    dwBufferLength: DWORD,
    dwBytesRecorded: DWORD,
    dwUser: DWORD_PTR,
    dwFlags: DWORD,
    dwLoops: DWORD,
    lpNext: *mut WAVEHDR,
    reserved: DWORD_PTR,
}}
pub type PWAVEHDR = *mut WAVEHDR;
pub type NPWAVEHDR = *mut WAVEHDR;
pub type LPWAVEHDR = *mut WAVEHDR;
STRUCT!{#[repr(packed)] struct WAVEOUTCAPSW {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [WCHAR; 32],
    dwFormats: DWORD,
    wChannels: WORD,
    wReserved1: WORD,
    dwSupport: DWORD,
}}
pub type PWAVEOUTCAPSW = *mut WAVEOUTCAPSW;
pub type NPWAVEOUTCAPSW = *mut WAVEOUTCAPSW;
pub type LPWAVEOUTCAPSW = *mut WAVEOUTCAPSW;
STRUCT!{#[repr(packed)] struct WAVEINCAPSW {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [WCHAR; 32],
    dwFormats: DWORD,
    wChannels: WORD,
    wReserved1: WORD,
}}
pub type PWAVEINCAPSW = *mut WAVEINCAPSW;
pub type NPWAVEINCAPSW = *mut WAVEINCAPSW;
pub type LPWAVEINCAPSW = *mut WAVEINCAPSW;
pub const WAVE_INVALIDFORMAT: DWORD = 0x00000000;
pub const WAVE_FORMAT_1M08: DWORD = 0x00000001;
pub const WAVE_FORMAT_1S08: DWORD = 0x00000002;
pub const WAVE_FORMAT_1M16: DWORD = 0x00000004;
pub const WAVE_FORMAT_1S16: DWORD = 0x00000008;
pub const WAVE_FORMAT_2M08: DWORD = 0x00000010;
pub const WAVE_FORMAT_2S08: DWORD = 0x00000020;
pub const WAVE_FORMAT_2M16: DWORD = 0x00000040;
pub const WAVE_FORMAT_2S16: DWORD = 0x00000080;
pub const WAVE_FORMAT_4M08: DWORD = 0x00000100;
pub const WAVE_FORMAT_4S08: DWORD = 0x00000200;
pub const WAVE_FORMAT_4M16: DWORD = 0x00000400;
pub const WAVE_FORMAT_4S16: DWORD = 0x00000800;
pub const WAVE_FORMAT_44M08: DWORD = 0x00000100;
pub const WAVE_FORMAT_44S08: DWORD = 0x00000200;
pub const WAVE_FORMAT_44M16: DWORD = 0x00000400;
pub const WAVE_FORMAT_44S16: DWORD = 0x00000800;
pub const WAVE_FORMAT_48M08: DWORD = 0x00001000;
pub const WAVE_FORMAT_48S08: DWORD = 0x00002000;
pub const WAVE_FORMAT_48M16: DWORD = 0x00004000;
pub const WAVE_FORMAT_48S16: DWORD = 0x00008000;
pub const WAVE_FORMAT_96M08: DWORD = 0x00010000;
pub const WAVE_FORMAT_96S08: DWORD = 0x00020000;
pub const WAVE_FORMAT_96M16: DWORD = 0x00040000;
pub const WAVE_FORMAT_96S16: DWORD = 0x00080000;
//782 (Win 7 SDK)
pub type PWAVEFORMATEX = *mut WAVEFORMATEX;
pub type NPWAVEFORMATEX = *mut WAVEFORMATEX;
pub type LPWAVEFORMATEX = *mut WAVEFORMATEX;
pub type LPCWAVEFORMATEX = *const WAVEFORMATEX;
//2170 (Win 7 SDK)
pub const TIMERR_NOERROR: MMRESULT = 0;
pub const TIMERR_NOCANDO: MMRESULT = TIMERR_BASE + 1;
pub const TIMERR_STRUCT: MMRESULT = TIMERR_BASE + 33;
//2198 (Win 7 SDK)
STRUCT!{#[repr(packed)] struct TIMECAPS {
    wPeriodMin: UINT,
    wPeriodMax: UINT,
}}
pub type PTIMECAPS = *mut TIMECAPS;
pub type NPTIMECAPS = *mut TIMECAPS;
pub type LPTIMECAPS = *mut TIMECAPS;
STRUCT!{#[repr(packed)] struct MIDIHDR {
    lpData: LPSTR,
    dwBufferLength: DWORD,
    dwBytesRecorded: DWORD,
    dwUser: DWORD_PTR,
    dwFlags: DWORD,
    lpNext: *mut MIDIHDR,
    reserved: DWORD_PTR,
    dwOffset: DWORD,
    dwReserved: [DWORD_PTR; 8],
}}
pub type PMIDIHDR = *mut MIDIHDR;
pub type NPMIDIHDR = *mut MIDIHDR;
pub type LPMIDIHDR = *mut MIDIHDR;
STRUCT!{#[repr(packed)] struct MIDIINCAPSW {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [WCHAR; 32],
    dwSupport: DWORD,
}}
pub type PMIDIINCAPSW = *mut MIDIINCAPSW;
pub type NPMIDIINCAPSW = *mut MIDIINCAPSW;
pub type LPMIDIINCAPSW = *mut MIDIINCAPSW;
STRUCT!{#[repr(packed)] struct MIDIOUTCAPSW {
    wMid: WORD,
    wPid: WORD,
    vDriverVersion: MMVERSION,
    szPname: [WCHAR; 32],
    wTechnology: WORD,
    wVoices: WORD,
    wNotes: WORD,
    wChannelMask: WORD,
    dwSupport: DWORD,
}}
pub type PMIDIOUTCAPSW = *mut MIDIOUTCAPSW;
pub type NPMIDIOUTCAPSW = *mut MIDIOUTCAPSW;
pub type LPMIDIOUTCAPSW = *mut MIDIOUTCAPSW;
DECLARE_HANDLE!{HMIDIIN, HMIDIIN__}
DECLARE_HANDLE!{HMIDIOUT, HMIDIOUT__}
pub type LPHMIDIIN = *mut HMIDIIN;
pub type LPHMIDIOUT = *mut HMIDIOUT;
DECLARE_HANDLE!{HMIDISTRM, HMIDISTRM__}
DECLARE_HANDLE!{HMIDI, HMIDI__}
pub type LPHMIDISTRM = *mut HMIDISTRM;
pub type LPHMIDI = *mut HMIDI;
