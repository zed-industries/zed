// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_HARDWARE_PREFERENCE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_HARDWARE_PREFERENCE_H_

#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

enum class HardwarePreference {
  kNoPreference,
  kPreferSoftware,
  kPreferHardware
};

HardwarePreference StringToHardwarePreference(const String& value);
String HardwarePreferenceToString(HardwarePreference hw_pref);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_HARDWARE_PREFERENCE_H_
