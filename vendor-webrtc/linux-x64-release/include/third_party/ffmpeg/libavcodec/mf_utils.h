/*
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVCODEC_MF_UTILS_H
#define AVCODEC_MF_UTILS_H

#include <windows.h>
#include <initguid.h>
#ifdef _MSC_VER
// The official way of including codecapi (via dshow.h) makes the ICodecAPI
// interface unavailable in UWP mode, but including icodecapi.h + codecapi.h
// seems to be equivalent. (These headers conflict with the official way
// of including it though, through strmif.h via dshow.h. And on mingw, the
// mf*.h headers below indirectly include strmif.h.)
#include <icodecapi.h>
#else
#define NO_DSHOW_STRSAFE
#include <dshow.h>
// Older versions of mingw-w64 need codecapi.h explicitly included, while newer
// ones include it implicitly from dshow.h (via uuids.h).
#include <codecapi.h>
#endif
#include <mfapi.h>
#include <mferror.h>
#include <mfobjects.h>
#include <mftransform.h>

#include "avcodec.h"

// Windows N editions does not provide MediaFoundation by default.
// So to avoid DLL loading error, MediaFoundation will be dynamically loaded
// except on UWP build since LoadLibrary is not available on it.
typedef struct MFFunctions {
    HRESULT (WINAPI *MFStartup) (ULONG Version, DWORD dwFlags);
    HRESULT (WINAPI *MFShutdown) (void);
    HRESULT (WINAPI *MFCreateAlignedMemoryBuffer) (DWORD cbMaxLength,
                                                   DWORD cbAligment,
                                                   IMFMediaBuffer **ppBuffer);
    HRESULT (WINAPI *MFCreateSample) (IMFSample **ppIMFSample);
    HRESULT (WINAPI *MFCreateMediaType) (IMFMediaType **ppMFType);
    // MFTEnumEx is missing in Windows Vista's mfplat.dll.
    HRESULT (WINAPI *MFTEnumEx)(GUID guidCategory, UINT32 Flags,
                                const MFT_REGISTER_TYPE_INFO *pInputType,
                                const MFT_REGISTER_TYPE_INFO *pOutputType,
                                IMFActivate ***pppMFTActivate,
                                UINT32 *pnumMFTActivate);
} MFFunctions;

// These functions do exist in mfapi.h, but are only available within
// __cplusplus ifdefs.
HRESULT ff_MFGetAttributeSize(IMFAttributes *pattr, REFGUID guid,
                              UINT32 *pw, UINT32 *ph);
HRESULT ff_MFSetAttributeSize(IMFAttributes *pattr, REFGUID guid,
                              UINT32 uw, UINT32 uh);
#define ff_MFSetAttributeRatio ff_MFSetAttributeSize
#define ff_MFGetAttributeRatio ff_MFGetAttributeSize

// These do exist in mingw-w64's codecapi.h, but they aren't properly defined
// by the header until after mingw-w64 v7.0.0.
DEFINE_GUID(ff_CODECAPI_AVDecVideoThumbnailGenerationMode, 0x2efd8eee,0x1150,0x4328,0x9c,0xf5,0x66,0xdc,0xe9,0x33,0xfc,0xf4);
DEFINE_GUID(ff_CODECAPI_AVDecVideoDropPicWithMissingRef, 0xf8226383,0x14c2,0x4567,0x97,0x34,0x50,0x04,0xe9,0x6f,0xf8,0x87);
DEFINE_GUID(ff_CODECAPI_AVDecVideoSoftwareDeinterlaceMode, 0x0c08d1ce,0x9ced,0x4540,0xba,0xe3,0xce,0xb3,0x80,0x14,0x11,0x09);
DEFINE_GUID(ff_CODECAPI_AVDecVideoFastDecodeMode, 0x6b529f7d,0xd3b1,0x49c6,0xa9,0x99,0x9e,0xc6,0x91,0x1b,0xed,0xbf);
DEFINE_GUID(ff_CODECAPI_AVLowLatencyMode, 0x9c27891a,0xed7a,0x40e1,0x88,0xe8,0xb2,0x27,0x27,0xa0,0x24,0xee);
DEFINE_GUID(ff_CODECAPI_AVDecVideoH264ErrorConcealment, 0xececace8,0x3436,0x462c,0x92,0x94,0xcd,0x7b,0xac,0xd7,0x58,0xa9);
DEFINE_GUID(ff_CODECAPI_AVDecVideoMPEG2ErrorConcealment, 0x9d2bfe18,0x728d,0x48d2,0xb3,0x58,0xbc,0x7e,0x43,0x6c,0x66,0x74);
DEFINE_GUID(ff_CODECAPI_AVDecVideoCodecType, 0x434528e5,0x21f0,0x46b6,0xb6,0x2c,0x9b,0x1b,0x6b,0x65,0x8c,0xd1);
DEFINE_GUID(ff_CODECAPI_AVDecVideoDXVAMode, 0xf758f09e,0x7337,0x4ae7,0x83,0x87,0x73,0xdc,0x2d,0x54,0xe6,0x7d);
DEFINE_GUID(ff_CODECAPI_AVDecVideoDXVABusEncryption, 0x42153c8b,0xfd0b,0x4765,0xa4,0x62,0xdd,0xd9,0xe8,0xbc,0xc3,0x88);
DEFINE_GUID(ff_CODECAPI_AVDecVideoSWPowerLevel, 0xfb5d2347,0x4dd8,0x4509,0xae,0xd0,0xdb,0x5f,0xa9,0xaa,0x93,0xf4);
DEFINE_GUID(ff_CODECAPI_AVDecVideoMaxCodedWidth, 0x5ae557b8,0x77af,0x41f5,0x9f,0xa6,0x4d,0xb2,0xfe,0x1d,0x4b,0xca);
DEFINE_GUID(ff_CODECAPI_AVDecVideoMaxCodedHeight, 0x7262a16a,0xd2dc,0x4e75,0x9b,0xa8,0x65,0xc0,0xc6,0xd3,0x2b,0x13);
DEFINE_GUID(ff_CODECAPI_AVDecNumWorkerThreads, 0x9561c3e8,0xea9e,0x4435,0x9b,0x1e,0xa9,0x3e,0x69,0x18,0x94,0xd8);
DEFINE_GUID(ff_CODECAPI_AVDecSoftwareDynamicFormatChange, 0x862e2f0a,0x507b,0x47ff,0xaf,0x47,0x01,0xe2,0x62,0x42,0x98,0xb7);
DEFINE_GUID(ff_CODECAPI_AVDecDisableVideoPostProcessing, 0xf8749193,0x667a,0x4f2c,0xa9,0xe8,0x5d,0x4a,0xf9,0x24,0xf0,0x8f);

// These are missing from mingw-w64's headers until after mingw-w64 v7.0.0.
DEFINE_GUID(ff_CODECAPI_AVEncCommonRateControlMode,      0x1c0608e9, 0x370c, 0x4710, 0x8a, 0x58, 0xcb, 0x61, 0x81, 0xc4, 0x24, 0x23);
DEFINE_GUID(ff_CODECAPI_AVEncCommonQuality,       0xfcbf57a3, 0x7ea5, 0x4b0c, 0x96, 0x44, 0x69, 0xb4, 0x0c, 0x39, 0xc3, 0x91);
DEFINE_GUID(ff_CODECAPI_AVEncCommonMeanBitRate,   0xf7222374, 0x2144, 0x4815, 0xb5, 0x50, 0xa3, 0x7f, 0x8e, 0x12, 0xee, 0x52);
DEFINE_GUID(ff_CODECAPI_AVEncH264CABACEnable,    0xee6cad62, 0xd305, 0x4248, 0xa5, 0xe, 0xe1, 0xb2, 0x55, 0xf7, 0xca, 0xf8);
DEFINE_GUID(ff_CODECAPI_AVEncVideoForceKeyFrame, 0x398c1b98, 0x8353, 0x475a, 0x9e, 0xf2, 0x8f, 0x26, 0x5d, 0x26, 0x3, 0x45);
DEFINE_GUID(ff_CODECAPI_AVEncMPVDefaultBPictureCount, 0x8d390aac, 0xdc5c, 0x4200, 0xb5, 0x7f, 0x81, 0x4d, 0x04, 0xba, 0xba, 0xb2);
DEFINE_GUID(ff_CODECAPI_AVScenarioInfo, 0xb28a6e64,0x3ff9,0x446a,0x8a,0x4b,0x0d,0x7a,0x53,0x41,0x32,0x36);
DEFINE_GUID(ff_CODECAPI_AVEncCommonBufferSize, 0x0db96574, 0xb6a4, 0x4c8b, 0x81, 0x06, 0x37, 0x73, 0xde, 0x03, 0x10, 0xcd);
DEFINE_GUID(ff_CODECAPI_AVEncCommonMaxBitRate, 0x9651eae4, 0x39b9, 0x4ebf, 0x85, 0xef, 0xd7, 0xf4, 0x44, 0xec, 0x74, 0x65);
DEFINE_GUID(ff_CODECAPI_AVEncCommonQualityVsSpeed, 0x98332df8, 0x03cd, 0x476b, 0x89, 0xfa, 0x3f, 0x9e, 0x44, 0x2d, 0xec, 0x9f);
DEFINE_GUID(ff_CODECAPI_AVEncMPVGOPSize, 0x95f31b26, 0x95a4, 0x41aa, 0x93, 0x03, 0x24, 0x6a, 0x7f, 0xc6, 0xee, 0xf1);
DEFINE_GUID(ff_CODECAPI_AVEncVideoEncodeQP, 0x2cb5696b, 0x23fb, 0x4ce1, 0xa0, 0xf9, 0xef, 0x5b, 0x90, 0xfd, 0x55, 0xca);

DEFINE_GUID(ff_MF_SA_D3D11_BINDFLAGS, 0xeacf97ad, 0x065c, 0x4408, 0xbe, 0xe3, 0xfd, 0xcb, 0xfd, 0x12, 0x8b, 0xe2);
DEFINE_GUID(ff_MF_SA_D3D11_USAGE, 0xe85fe442, 0x2ca3, 0x486e, 0xa9, 0xc7, 0x10, 0x9d, 0xda, 0x60, 0x98, 0x80);
DEFINE_GUID(ff_MF_SA_D3D11_AWARE, 0x206b4fc8, 0xfcf9, 0x4c51, 0xaf, 0xe3, 0x97, 0x64, 0x36, 0x9e, 0x33, 0xa0);
DEFINE_GUID(ff_MF_SA_D3D11_SHARED, 0x7b8f32c3, 0x6d96, 0x4b89, 0x92, 0x3, 0xdd, 0x38, 0xb6, 0x14, 0x14, 0xf3);
DEFINE_GUID(ff_MF_SA_D3D11_SHARED_WITHOUT_MUTEX, 0x39dbd44d, 0x2e44, 0x4931, 0xa4, 0xc8, 0x35, 0x2d, 0x3d, 0xc4, 0x21, 0x15);
DEFINE_GUID(ff_MF_SA_MINIMUM_OUTPUT_SAMPLE_COUNT, 0x851745d5, 0xc3d6, 0x476d, 0x95, 0x27, 0x49, 0x8e, 0xf2, 0xd1, 0xd, 0x18);
DEFINE_GUID(ff_MF_SA_MINIMUM_OUTPUT_SAMPLE_COUNT_PROGRESSIVE, 0xf5523a5, 0x1cb2, 0x47c5, 0xa5, 0x50, 0x2e, 0xeb, 0x84, 0xb4, 0xd1, 0x4a);

DEFINE_MEDIATYPE_GUID(ff_MFVideoFormat_HEVC, 0x43564548); // FCC('HEVC')
DEFINE_MEDIATYPE_GUID(ff_MFVideoFormat_HEVC_ES, 0x53564548); // FCC('HEVS')
DEFINE_MEDIATYPE_GUID(ff_MFVideoFormat_AV1, 0x31305641); // FCC('AV01')


// This enum is missing from mingw-w64's codecapi.h by v7.0.0.
enum ff_eAVEncCommonRateControlMode {
    ff_eAVEncCommonRateControlMode_CBR                 = 0,
    ff_eAVEncCommonRateControlMode_PeakConstrainedVBR  = 1,
    ff_eAVEncCommonRateControlMode_UnconstrainedVBR    = 2,
    ff_eAVEncCommonRateControlMode_Quality             = 3,
    ff_eAVEncCommonRateControlMode_LowDelayVBR         = 4,
    ff_eAVEncCommonRateControlMode_GlobalVBR           = 5,
    ff_eAVEncCommonRateControlMode_GlobalLowDelayVBR   = 6
};

enum ff_eAVScenarioInfo {
    ff_eAVScenarioInfo_Unknown                         = 0,
    ff_eAVScenarioInfo_DisplayRemoting                 = 1,
    ff_eAVScenarioInfo_VideoConference                 = 2,
    ff_eAVScenarioInfo_Archive                         = 3,
    ff_eAVScenarioInfo_LiveStreaming                   = 4,
    ff_eAVScenarioInfo_CameraRecord                    = 5,
    ff_eAVScenarioInfo_DisplayRemotingWithFeatureMap   = 6
};

// These do exist in mingw-w64's mfobjects.idl, but are missing from
// mfobjects.h that is generated from the former, due to incorrect use of
// ifdefs in the IDL file.
enum {
    ff_METransformUnknown = 600,
    ff_METransformNeedInput,
    ff_METransformHaveOutput,
    ff_METransformDrainComplete,
    ff_METransformMarker,
};

// These do exist in all supported headers, but are manually defined here
// to avoid having to include codecapi.h, as there's problems including that
// header when targeting UWP (where including it with MSVC seems to work,
// but fails when built with clang in MSVC mode).
enum ff_eAVEncH264VProfile {
   ff_eAVEncH264VProfile_Base = 66,
   ff_eAVEncH264VProfile_Main = 77,
   ff_eAVEncH264VProfile_High = 100,
};

char *ff_hr_str_buf(char *buf, size_t size, HRESULT hr);
#define ff_hr_str(hr) ff_hr_str_buf((char[80]){0}, 80, hr)

// Possibly compiler-dependent; the MS/MinGW definition for this is just crazy.
#define FF_VARIANT_VALUE(type, contents) &(VARIANT){ .vt = (type), contents }

#define FF_VAL_VT_UI4(v) FF_VARIANT_VALUE(VT_UI4, .ulVal = (v))
#define FF_VAL_VT_BOOL(v) FF_VARIANT_VALUE(VT_BOOL, .boolVal = (v))

IMFSample *ff_create_memory_sample(MFFunctions *f, void *fill_data,
                                   size_t size, size_t align);
enum AVSampleFormat ff_media_type_to_sample_fmt(IMFAttributes *type);
enum AVPixelFormat ff_media_type_to_pix_fmt(IMFAttributes *type);
const GUID *ff_pix_fmt_to_guid(enum AVPixelFormat pix_fmt);
int ff_fourcc_from_guid(const GUID *guid, uint32_t *out_fourcc);
char *ff_guid_str_buf(char *buf, size_t buf_size, const GUID *guid);
#define ff_guid_str(guid) ff_guid_str_buf((char[80]){0}, 80, guid)
void ff_attributes_dump(void *log, IMFAttributes *attrs);
void ff_media_type_dump(void *log, IMFMediaType *type);
const CLSID *ff_codec_to_mf_subtype(enum AVCodecID codec);
int ff_instantiate_mf(void *log, MFFunctions *f, GUID category,
                      MFT_REGISTER_TYPE_INFO *in_type,
                      MFT_REGISTER_TYPE_INFO *out_type,
                      int use_hw, IMFTransform **res);
void ff_free_mf(MFFunctions *f, IMFTransform **mft);

#endif
