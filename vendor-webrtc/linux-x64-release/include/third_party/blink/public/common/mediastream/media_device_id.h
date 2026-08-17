// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_MEDIASTREAM_MEDIA_DEVICE_ID_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_MEDIASTREAM_MEDIA_DEVICE_ID_H_

#include "third_party/blink/public/common/common_export.h"

#include <string>

namespace blink {
BLINK_COMMON_EXPORT bool IsValidMediaDeviceId(const std::string& device_id);
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_MEDIASTREAM_MEDIA_DEVICE_ID_H_
