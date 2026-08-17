// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_DEVICE_INFO_H_
#define BASE_ANDROID_DEVICE_INFO_H_

#include <string>

namespace base::android::device_info {
const char* gms_version_code();

void set_gms_version_code_for_test(const std::string& gms_version_code);

bool is_tv();
bool is_automotive();
bool is_foldable();
bool is_desktop();
// Available only on Android T+.
int32_t vulkan_deqp_level();

}  // namespace base::android::device_info

#endif  // BASE_ANDROID_DEVICE_INFO_H_
