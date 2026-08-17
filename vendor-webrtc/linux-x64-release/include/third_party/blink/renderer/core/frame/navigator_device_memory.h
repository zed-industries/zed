// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_NAVIGATOR_DEVICE_MEMORY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_NAVIGATOR_DEVICE_MEMORY_H_

#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

class CORE_EXPORT NavigatorDeviceMemory {
 public:
  float deviceMemory() const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_NAVIGATOR_DEVICE_MEMORY_H_
