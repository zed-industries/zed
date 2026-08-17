// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! DSound procedure declarations, constant definitions and macros
use shared::guiddef::{GUID, LPCGUID, LPGUID};
use shared::minwindef::{DWORD, LPDWORD, LPLONG, LPVOID};
use shared::windef::HWND;
use shared::winerror::{E_FAIL, S_OK};
use um::mmsystem::{LPCWAVEFORMATEX, LPWAVEFORMATEX};
use um::unknwnbase::{IUnknown, IUnknownVtbl, LPUNKNOWN};
use um::winnt::{HRESULT, LONG};
DEFINE_GUID!{CLSID_DirectSound,
    0x47d4d946, 0x62e8, 0x11cf, 0x93, 0xbc, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00}
DEFINE_GUID!{CLSID_DirectSound8,
    0x3901cc3f, 0x84b5, 0x4fa4, 0xba, 0x35, 0xaa, 0x81, 0x72, 0xb8, 0xa0, 0x9b}
DEFINE_GUID!{CLSID_DirectSoundCapture,
    0xb0210780, 0x89cd, 0x11d0, 0xaf, 0x08, 0x00, 0xa0, 0xc9, 0x25, 0xcd, 0x16}
DEFINE_GUID!{CLSID_DirectSoundCapture8,
    0xe4bcac13, 0x7f99, 0x4908, 0x9a, 0x8e, 0x74, 0xe3, 0xbf, 0x24, 0xb6, 0xe1}
DEFINE_GUID!{CLSID_DirectSoundFullDuplex,
    0xfea4300c, 0x7959, 0x4147, 0xb2, 0x6a, 0x23, 0x77, 0xb9, 0xe7, 0xa9, 0x1d}
DEFINE_GUID!{DSDEVID_DefaultPlayback,
    0xdef00000, 0x9c6d, 0x47ed, 0xaa, 0xf1, 0x4d, 0xda, 0x8f, 0x2b, 0x5c, 0x03}
DEFINE_GUID!{DSDEVID_DefaultCapture,
    0xdef00001, 0x9c6d, 0x47ed, 0xaa, 0xf1, 0x4d, 0xda, 0x8f, 0x2b, 0x5c, 0x03}
DEFINE_GUID!{DSDEVID_DefaultVoicePlayback,
    0xdef00002, 0x9c6d, 0x47ed, 0xaa, 0xf1, 0x4d, 0xda, 0x8f, 0x2b, 0x5c, 0x03}
DEFINE_GUID!{DSDEVID_DefaultVoiceCapture,
    0xdef00003, 0x9c6d, 0x47ed, 0xaa, 0xf1, 0x4d, 0xda, 0x8f, 0x2b, 0x5c, 0x03}
STRUCT!{struct DSCAPS {
    dwSize: DWORD,
    dwFlags: DWORD,
    dwMinSecondarySampleRate: DWORD,
    dwMaxSecondarySampleRate: DWORD,
    dwPrimaryBuffers: DWORD,
    dwMaxHwMixingAllBuffers: DWORD,
    dwMaxHwMixingStaticBuffers: DWORD,
    dwMaxHwMixingStreamingBuffers: DWORD,
    dwFreeHwMixingAllBuffers: DWORD,
    dwFreeHwMixingStaticBuffers: DWORD,
    dwFreeHwMixingStreamingBuffers: DWORD,
    dwMaxHw3DAllBuffers: DWORD,
    dwMaxHw3DStaticBuffers: DWORD,
    dwMaxHw3DStreamingBuffers: DWORD,
    dwFreeHw3DAllBuffers: DWORD,
    dwFreeHw3DStaticBuffers: DWORD,
    dwFreeHw3DStreamingBuffers: DWORD,
    dwTotalHwMemBytes: DWORD,
    dwFreeHwMemBytes: DWORD,
    dwMaxContigFreeHwMemBytes: DWORD,
    dwUnlockTransferRateHwBuffers: DWORD,
    dwPlayCpuOverheadSwBuffers: DWORD,
    dwReserved1: DWORD,
    dwReserved2: DWORD,
}}
pub type LPDSCAPS = *mut DSCAPS;
STRUCT!{struct DSBCAPS {
    dwSize: DWORD,
    dwFlags: DWORD,
    dwBufferBytes: DWORD,
    dwUnlockTransferRate: DWORD,
    dwPlayCpuOverhead: DWORD,
}}
pub type LPDSBCAPS = *mut DSBCAPS;
STRUCT!{struct DSBUFFERDESC {
    dwSize: DWORD,
    dwFlags: DWORD,
    dwBufferBytes: DWORD,
    dwReserved: DWORD,
    lpwfxFormat: LPWAVEFORMATEX,
    guid3DAlgorithm: GUID,
}}
pub type LPCDSBUFFERDESC = *const DSBUFFERDESC;
RIDL!{#[uuid(0x279afa85, 0x4981, 0x11ce, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60)]
interface IDirectSoundBuffer(IDirectSoundBufferVtbl): IUnknown(IUnknownVtbl) {
    fn GetCaps(
        pDSBufferCaps: LPDSBCAPS,
    ) -> HRESULT,
    fn GetCurrentPosition(
        pdwCurrentPlayCursor: LPDWORD,
        pdwCurrentWriteCursor: LPDWORD,
    ) -> HRESULT,
    fn GetFormat(
        pwfxFormat: LPWAVEFORMATEX,
        dwSizeAllocated: DWORD,
        pdwSizeWritten: LPDWORD,
    ) -> HRESULT,
    fn GetVolume(
        plVolume: LPLONG,
    ) -> HRESULT,
    fn GetPan(
        plPan: LPLONG,
    ) -> HRESULT,
    fn GetFrequency(
        pdwFrequency: LPDWORD,
    ) -> HRESULT,
    fn GetStatus(
        pdwStatus: LPDWORD,
    ) -> HRESULT,
    fn Initialize(
        pDirectSound: LPDIRECTSOUND,
        pcDSBufferDesc: LPCDSBUFFERDESC,
    ) -> HRESULT,
    fn Lock(
        dwOffset: DWORD,
        dwBytes: DWORD,
        ppvAudioPtr1: *mut LPVOID,
        pdwAudioBytes1: LPDWORD,
        ppvAudioPtr2: *mut LPVOID,
        pdwAudioBytes2: LPDWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn Play(
        dwReserved1: DWORD,
        dwPriority: DWORD,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn SetCurrentPosition(
        dwNewPosition: DWORD,
    ) -> HRESULT,
    fn SetFormat(
        pcfxFormat: LPCWAVEFORMATEX,
    ) -> HRESULT,
    fn SetVolume(
        lVolume: LONG,
    ) -> HRESULT,
    fn SetPan(
        lPan: LONG,
    ) -> HRESULT,
    fn SetFrequency(
        dwFrequency: DWORD,
    ) -> HRESULT,
    fn Stop() -> HRESULT,
    fn Unlock(
        pvAudioPtr1: LPVOID,
        dwAudioBytes1: DWORD,
        pvAudioPtr2: LPVOID,
        dwAudioBytes2: DWORD,
    ) -> HRESULT,
    fn Restore() -> HRESULT,
}}
pub type LPDIRECTSOUNDBUFFER = *mut IDirectSoundBuffer;
DEFINE_GUID!{IID_IReferenceClock,
    0x56a86897, 0x0ad4, 0x11ce, 0xb0, 0x3a, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70}
DEFINE_GUID!{IID_IDirectSound,
    0x279afa83, 0x4981, 0x11ce, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60}
RIDL!{#[uuid(0x279afa83, 0x4981, 0x11ce, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60)]
interface IDirectSound(IDirectSoundVtbl): IUnknown(IUnknownVtbl) {
    fn CreateSoundBuffer(
        pcDSBufferDesc: LPCDSBUFFERDESC,
        ppDSBuffer: *mut LPDIRECTSOUNDBUFFER,
        pUnkOuter: LPUNKNOWN,
    ) -> HRESULT,
    fn GetCaps(
        pDSCaps: LPDSCAPS,
    ) -> HRESULT,
    fn DuplicateSoundBuffer(
        pDSBufferOriginal: LPDIRECTSOUNDBUFFER,
        ppDSBufferDuplicate: *mut LPDIRECTSOUNDBUFFER,
    ) -> HRESULT,
    fn SetCooperativeLevel(
        hWnd: HWND,
        dwLevel: DWORD,
    ) -> HRESULT,
    fn Compact() -> HRESULT,
    fn GetSpeakerConfig(
        pdwSpeakerConfig: LPDWORD,
    ) -> HRESULT,
    fn SetSpeakerConfig(
        dwSpeakerConfig: DWORD,
    ) -> HRESULT,
    fn Initialize(
        pcGuidDevice: LPCGUID,
    ) -> HRESULT,
}}
pub type LPDIRECTSOUND = *mut IDirectSound;
DEFINE_GUID!{IID_IDirectSound8,
    0xc50a7e93, 0xf395, 0x4834, 0x9e, 0xf6, 0x7f, 0xa9, 0x9d, 0xe5, 0x09, 0x66}
DEFINE_GUID!{IID_IDirectSoundBuffer,
    0x279afa85, 0x4981, 0x11ce, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60}
DEFINE_GUID!{IID_IDirectSoundBuffer8,
    0x6825a449, 0x7524, 0x4d82, 0x92, 0x0f, 0x50, 0xe3, 0x6a, 0xb3, 0xab, 0x1e}
DEFINE_GUID!{GUID_All_Objects,
    0xaa114de5, 0xc262, 0x4169, 0xa1, 0xc8, 0x23, 0xd6, 0x98, 0xcc, 0x73, 0xb5}
DEFINE_GUID!{IID_IDirectSound3DListener,
    0x279afa84, 0x4981, 0x11ce, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60}
DEFINE_GUID!{IID_IDirectSound3DBuffer,
    0x279afa86, 0x4981, 0x11ce, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60}
DEFINE_GUID!{IID_IDirectSoundCapture,
    0xb0210781, 0x89cd, 0x11d0, 0xaf, 0x08, 0x00, 0xa0, 0xc9, 0x25, 0xcd, 0x16}
DEFINE_GUID!{IID_IDirectSoundCaptureBuffer,
    0xb0210782, 0x89cd, 0x11d0, 0xaf, 0x08, 0x00, 0xa0, 0xc9, 0x25, 0xcd, 0x16}
DEFINE_GUID!{IID_IDirectSoundCaptureBuffer8,
    0x00990df4, 0x0dbb, 0x4872, 0x83, 0x3e, 0x6d, 0x30, 0x3e, 0x80, 0xae, 0xb6}
DEFINE_GUID!{IID_IDirectSoundNotify,
    0xb0210783, 0x89cd, 0x11d0, 0xaf, 0x08, 0x00, 0xa0, 0xc9, 0x25, 0xcd, 0x16}
DEFINE_GUID!{IID_IKsPropertySet,
    0x31efac30, 0x515c, 0x11d0, 0xa9, 0xaa, 0x00, 0xaa, 0x00, 0x61, 0xbe, 0x93}
DEFINE_GUID!{IID_IDirectSoundFXGargle,
    0xd616f352, 0xd622, 0x11ce, 0xaa, 0xc5, 0x00, 0x20, 0xaf, 0x0b, 0x99, 0xa3}
DEFINE_GUID!{IID_IDirectSoundFXChorus,
    0x880842e3, 0x145f, 0x43e6, 0xa9, 0x34, 0xa7, 0x18, 0x06, 0xe5, 0x05, 0x47}
DEFINE_GUID!{IID_IDirectSoundFXFlanger,
    0x903e9878, 0x2c92, 0x4072, 0x9b, 0x2c, 0xea, 0x68, 0xf5, 0x39, 0x67, 0x83}
DEFINE_GUID!{IID_IDirectSoundFXEcho,
    0x8bd28edf, 0x50db, 0x4e92, 0xa2, 0xbd, 0x44, 0x54, 0x88, 0xd1, 0xed, 0x42}
DEFINE_GUID!{IID_IDirectSoundFXDistortion,
    0x8ecf4326, 0x455f, 0x4d8b, 0xbd, 0xa9, 0x8d, 0x5d, 0x3e, 0x9e, 0x3e, 0x0b}
DEFINE_GUID!{IID_IDirectSoundFXCompressor,
    0x4bbd1154, 0x62f6, 0x4e2c, 0xa1, 0x5c, 0xd3, 0xb6, 0xc4, 0x17, 0xf7, 0xa0}
DEFINE_GUID!{IID_IDirectSoundFXParamEq,
    0xc03ca9fe, 0xfe90, 0x4204, 0x80, 0x78, 0x82, 0x33, 0x4c, 0xd1, 0x77, 0xda}
DEFINE_GUID!{IID_IDirectSoundFXI3DL2Reverb,
    0x4b166a6a, 0x0d66, 0x43f3, 0x80, 0xe3, 0xee, 0x62, 0x80, 0xde, 0xe1, 0xa4}
DEFINE_GUID!{IID_IDirectSoundFXWavesReverb,
    0x46858c3a, 0x0dc6, 0x45e3, 0xb7, 0x60, 0xd4, 0xee, 0xf1, 0x6c, 0xb3, 0x25}
DEFINE_GUID!{IID_IDirectSoundCaptureFXAec,
    0xad74143d, 0x903d, 0x4ab7, 0x80, 0x66, 0x28, 0xd3, 0x63, 0x03, 0x6d, 0x65}
DEFINE_GUID!{IID_IDirectSoundCaptureFXNoiseSuppress,
    0xed311e41, 0xfbae, 0x4175, 0x96, 0x25, 0xcd, 0x08, 0x54, 0xf6, 0x93, 0xca}
DEFINE_GUID!{IID_IDirectSoundFullDuplex,
    0xedcb4c7a, 0xdaab, 0x4216, 0xa4, 0x2e, 0x6c, 0x50, 0x59, 0x6d, 0xdc, 0x1d}
pub const DS_OK: HRESULT = S_OK;
pub const DSERR_GENERIC: HRESULT = E_FAIL;
pub const DSSCL_NORMAL: DWORD = 0x00000001;
pub const DSSCL_PRIORITY: DWORD = 0x00000002;
pub const DSSCL_EXCLUSIVE: DWORD = 0x00000003;
pub const DSSCL_WRITEPRIMARY: DWORD = 0x00000004;
pub const DSBCAPS_PRIMARYBUFFER: DWORD = 0x00000001;
pub const DSBCAPS_STATIC: DWORD = 0x00000002;
pub const DSBCAPS_LOCHARDWARE: DWORD = 0x00000004;
pub const DSBCAPS_LOCSOFTWARE: DWORD = 0x00000008;
pub const DSBCAPS_CTRL3D: DWORD = 0x00000010;
pub const DSBCAPS_CTRLFREQUENCY: DWORD = 0x00000020;
pub const DSBCAPS_CTRLPAN: DWORD = 0x00000040;
pub const DSBCAPS_CTRLVOLUME: DWORD = 0x00000080;
pub const DSBCAPS_CTRLPOSITIONNOTIFY: DWORD = 0x00000100;
pub const DSBCAPS_CTRLFX: DWORD = 0x00000200;
pub const DSBCAPS_STICKYFOCUS: DWORD = 0x00004000;
pub const DSBCAPS_GLOBALFOCUS: DWORD = 0x00008000;
pub const DSBCAPS_GETCURRENTPOSITION2: DWORD = 0x00010000;
pub const DSBCAPS_MUTE3DATMAXDISTANCE: DWORD = 0x00020000;
pub const DSBCAPS_LOCDEFER: DWORD = 0x00040000;
pub const DSBCAPS_TRUEPLAYPOSITION: DWORD = 0x00080000;
pub const DSBPLAY_LOOPING: DWORD = 0x00000001;
pub const DSBPLAY_LOCHARDWARE: DWORD = 0x00000002;
pub const DSBPLAY_LOCSOFTWARE: DWORD = 0x00000004;
pub const DSBPLAY_TERMINATEBY_TIME: DWORD = 0x00000008;
pub const DSBPLAY_TERMINATEBY_DISTANCE: DWORD = 0x000000010;
pub const DSBPLAY_TERMINATEBY_PRIORITY: DWORD = 0x000000020;
extern "system" {
    pub fn DirectSoundCreate(
        pcGuidDevice: LPCGUID,
        ppDS: *mut LPDIRECTSOUND,
        pUnkOuter: LPUNKNOWN,
    ) -> HRESULT;
    // pub fn DirectSoundEnumerateA(
    //     pDSEnumCallback: LPDSENUMCALLBACKA,
    //     pContext: LPVOID,
    // ) -> HRESULT;
    // pub fn DirectSoundEnumerateW(
    //     pDSEnumCallback: LPDSENUMCALLBACKW,
    //     pContext: LPVOID,
    // ) -> HRESULT;
    // pub fn DirectSoundCaptureCreate(
    //     pcGuidDevice: LPCGUID,
    //     ppDSC: *mut LPDIRECTSOUNDCAPTURE,
    //     pUnkOuter: LPUNKNOWN,
    // ) -> HRESULT;
    // pub fn DirectSoundCaptureEnumerateA(
    //     pDSEnumCallback: LPDSENUMCALLBACKA,
    //     pContext: LPVOID,
    // ) -> HRESULT;
    // pub fn DirectSoundCaptureEnumerateW(
    //     pDSEnumCallback: LPDSENUMCALLBACKW,
    //     pContext: LPVOID,
    // ) -> HRESULT;
    // pub fn DirectSoundCreate8(
    //     pcGuidDevice: LPCGUID,
    //     ppDS8: *mut LPDIRECTSOUND8,
    //     pUnkOuter: LPUNKNOWN,
    // ) -> HRESULT;
    // pub fn DirectSoundCaptureCreate8(
    //     pcGuidDevice: LPCGUID,
    //     ppDSC8: *mut LPDIRECTSOUNDCAPTURE8,
    //     pUnkOuter: LPUNKNOWN,
    // ) -> HRESULT;
    // pub fn DirectSoundFullDuplexCreate(
    //     pcGuidCaptureDevice: LPCGUID,
    //     pcGuidRenderDevice: LPCGUID,
    //     pcDSCBufferDesc: LPCDSCBUFFERDESC,
    //     pcDSBufferDesc: LPCDSBUFFERDESC,
    //     hWnd: HWND,
    //     dwLevel: DWORD,
    //     ppDSFD: *mut LPDIRECTSOUNDFULLDUPLEX,
    //     ppDSCBuffer8: *mut LPDIRECTSOUNDCAPTUREBUFFER8,
    //     ppDSBuffer8: *mut LPDIRECTSOUNDBUFFER8,
    //     pUnkOuter: LPUNKNOWN,
    // ) -> HRESULT;
    pub fn GetDeviceID(
        pGuidSrc: LPCGUID,
        pGuidDest: LPGUID,
    ) -> HRESULT;
}
DEFINE_GUID!{DS3DALG_NO_VIRTUALIZATION,
    0xc241333f, 0x1c1b, 0x11d2, 0x94, 0xf5, 0x00, 0xc0, 0x4f, 0xc2, 0x8a, 0xca}
DEFINE_GUID!{DS3DALG_HRTF_FULL,
    0xc2413340, 0x1c1b, 0x11d2, 0x94, 0xf5, 0x00, 0xc0, 0x4f, 0xc2, 0x8a, 0xca}
DEFINE_GUID!{DS3DALG_HRTF_LIGHT,
    0xc2413342, 0x1c1b, 0x11d2, 0x94, 0xf5, 0x00, 0xc0, 0x4f, 0xc2, 0x8a, 0xca}
DEFINE_GUID!{GUID_DSFX_STANDARD_GARGLE,
    0xdafd8210, 0x5711, 0x4b91, 0x9f, 0xe3, 0xf7, 0x5b, 0x7a, 0xe2, 0x79, 0xbf}
DEFINE_GUID!{GUID_DSFX_STANDARD_CHORUS,
    0xefe6629c, 0x81f7, 0x4281, 0xbd, 0x91, 0xc9, 0xd6, 0x04, 0xa9, 0x5a, 0xf6}
DEFINE_GUID!{GUID_DSFX_STANDARD_FLANGER,
    0xefca3d92, 0xdfd8, 0x4672, 0xa6, 0x03, 0x74, 0x20, 0x89, 0x4b, 0xad, 0x98}
DEFINE_GUID!{GUID_DSFX_STANDARD_ECHO,
    0xef3e932c, 0xd40b, 0x4f51, 0x8c, 0xcf, 0x3f, 0x98, 0xf1, 0xb2, 0x9d, 0x5d}
DEFINE_GUID!{GUID_DSFX_STANDARD_DISTORTION,
    0xef114c90, 0xcd1d, 0x484e, 0x96, 0xe5, 0x09, 0xcf, 0xaf, 0x91, 0x2a, 0x21}
DEFINE_GUID!{GUID_DSFX_STANDARD_COMPRESSOR,
    0xef011f79, 0x4000, 0x406d, 0x87, 0xaf, 0xbf, 0xfb, 0x3f, 0xc3, 0x9d, 0x57}
DEFINE_GUID!{GUID_DSFX_STANDARD_PARAMEQ,
    0x120ced89, 0x3bf4, 0x4173, 0xa1, 0x32, 0x3c, 0xb4, 0x06, 0xcf, 0x32, 0x31}
DEFINE_GUID!{GUID_DSFX_STANDARD_I3DL2REVERB,
    0xef985e71, 0xd5c7, 0x42d4, 0xba, 0x4d, 0x2d, 0x07, 0x3e, 0x2e, 0x96, 0xf4}
DEFINE_GUID!{GUID_DSFX_WAVES_REVERB,
    0x87fc0268, 0x9a55, 0x4360, 0x95, 0xaa, 0x00, 0x4a, 0x1d, 0x9d, 0xe2, 0x6c}
DEFINE_GUID!{GUID_DSCFX_CLASS_AEC,
    0xbf963d80, 0xc559, 0x11d0, 0x8a, 0x2b, 0x00, 0xa0, 0xc9, 0x25, 0x5a, 0xc1}
DEFINE_GUID!{GUID_DSCFX_MS_AEC,
    0xcdebb919, 0x379a, 0x488a, 0x87, 0x65, 0xf5, 0x3c, 0xfd, 0x36, 0xde, 0x40}
DEFINE_GUID!{GUID_DSCFX_SYSTEM_AEC,
    0x1c22c56d, 0x9879, 0x4f5b, 0xa3, 0x89, 0x27, 0x99, 0x6d, 0xdc, 0x28, 0x10}
DEFINE_GUID!{GUID_DSCFX_CLASS_NS,
    0xe07f903f, 0x62fd, 0x4e60, 0x8c, 0xdd, 0xde, 0xa7, 0x23, 0x66, 0x65, 0xb5}
DEFINE_GUID!{GUID_DSCFX_MS_NS,
    0x11c5c73b, 0x66e9, 0x4ba1, 0xa0, 0xba, 0xe8, 0x14, 0xc6, 0xee, 0xd9, 0x2d}
DEFINE_GUID!{GUID_DSCFX_SYSTEM_NS,
    0x5ab0882e, 0x7274, 0x4516, 0x87, 0x7d, 0x4e, 0xee, 0x99, 0xba, 0x4f, 0xd0}
