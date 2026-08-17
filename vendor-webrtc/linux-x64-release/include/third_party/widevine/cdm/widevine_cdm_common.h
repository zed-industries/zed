// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef WIDEVINE_CDM_WIDEVINE_CDM_COMMON_H_
#define WIDEVINE_CDM_WIDEVINE_CDM_COMMON_H_

#include "build/build_config.h"
#include "media/cdm/cdm_type.h"  // nogncheck

// Default constants common to all Widevine CDMs.

// "alpha" is a temporary name until a convention is defined.
inline constexpr char kWidevineKeySystem[] = "com.widevine.alpha";

#if BUILDFLAG(IS_WIN)
// A sub key system of `kWidevineKeySystem` only used in experiments.
inline constexpr char kWidevineExperimentKeySystem[] =
    "com.widevine.alpha.experiment";
#endif  // BUILDFLAG(IS_WIN)

// Widevine CDM files are in a directory with this name. This path is also
// hardcoded in some build files and changing it requires changing the build
// files as well.
inline constexpr char kWidevineCdmBaseDirectory[] = "WidevineCdm";

// Media Foundation Widevine CDM files are in a directory with this name.
inline constexpr char kMediaFoundationWidevineCdmBaseDirection[] =
    "MediaFoundationWidevineCdm";

// This name is used by UMA. Do not change it!
inline constexpr char kWidevineKeySystemNameForUMA[] = "Widevine";

// Name of the CDM library.
inline constexpr char kWidevineCdmLibraryName[] = "widevinecdm";

inline constexpr char kWidevineCdmDisplayName[] =
    "Widevine Content Decryption Module";

// Identifier used for CDM process site isolation.
inline constexpr media::CdmType kWidevineCdmType{0x05d908e5dcca9960ull,
                                                 0xcd92d30eac98157aull};

// Constants specific to Windows MediaFoundation-based Widevine CDM library.
#if BUILDFLAG(IS_WIN)
inline constexpr char kMediaFoundationWidevineCdmLibraryName[] =
    "Google.Widevine.CDM";
inline constexpr char kMediaFoundationWidevineCdmDisplayName[] =
    "Google Widevine Windows CDM";
// Identifier used for CDM process site isolation.
inline constexpr media::CdmType kMediaFoundationWidevineCdmType{
    0x8e73dec793bf5adcull, 0x27e572c9a1fd930eull};
#endif  // BUILDFLAG(IS_WIN)

// UUID from http://dashif.org/identifiers/content_protection/. UUIDs are used
// in Android for creating MediaDRM objects that support the DRM scheme required
// by content.
#if BUILDFLAG(IS_ANDROID)
inline constexpr uint8_t kWidevineUuid[16] = {
    0xED, 0xEF, 0x8B, 0xA9, 0x79, 0xD6, 0x4A, 0xCE,  //
    0xA3, 0xC8, 0x27, 0xDC, 0xD5, 0x1D, 0x21, 0xED};
#endif  // BUILDFLAG(IS_ANDROID)
#endif  // WIDEVINE_CDM_WIDEVINE_CDM_COMMON_H_
