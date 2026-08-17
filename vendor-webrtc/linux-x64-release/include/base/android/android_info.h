// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_ANDROID_INFO_H_
#define BASE_ANDROID_ANDROID_INFO_H_

#include "base/base_export.h"

namespace base::android::android_info {

// This enumeration maps to the values returned by AndroidInfo::sdk_int(),
// indicating the Android release associated with a given SDK version.
enum SdkVersion {
  SDK_VERSION_JELLY_BEAN = 16,
  SDK_VERSION_JELLY_BEAN_MR1 = 17,
  SDK_VERSION_JELLY_BEAN_MR2 = 18,
  SDK_VERSION_KITKAT = 19,
  SDK_VERSION_KITKAT_WEAR = 20,
  SDK_VERSION_LOLLIPOP = 21,
  SDK_VERSION_LOLLIPOP_MR1 = 22,
  SDK_VERSION_MARSHMALLOW = 23,
  SDK_VERSION_NOUGAT = 24,
  SDK_VERSION_NOUGAT_MR1 = 25,
  SDK_VERSION_OREO = 26,
  SDK_VERSION_O_MR1 = 27,
  SDK_VERSION_P = 28,
  SDK_VERSION_Q = 29,
  SDK_VERSION_R = 30,
  SDK_VERSION_S = 31,
  SDK_VERSION_Sv2 = 32,
  SDK_VERSION_T = 33,
  SDK_VERSION_U = 34,
  SDK_VERSION_V = 35,
  SDK_VERSION_BAKLAVA = 36,
};

const char* device();

const char* manufacturer();

const char* model();

BASE_EXPORT const char* brand();

const char* android_build_id();

const char* build_type();

const char* board();

const char* android_build_fp();

BASE_EXPORT int sdk_int();

bool is_debug_android();

const char* version_incremental();

BASE_EXPORT const char* hardware();

bool is_at_least_u();

const char* codename();

// Available only on android S+. For S-, this method returns empty string.
const char* soc_manufacturer();

bool is_at_least_t();

const char* abi_name();

}  // namespace base::android::android_info

#endif  // BASE_ANDROID_ANDROID_INFO_H_
