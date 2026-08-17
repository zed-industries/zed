// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_NAVIGATOR_CONCURRENT_HARDWARE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_NAVIGATOR_CONCURRENT_HARDWARE_H_

#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

class CORE_EXPORT NavigatorConcurrentHardware {
 public:
  virtual unsigned hardwareConcurrency() const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_NAVIGATOR_CONCURRENT_HARDWARE_H_
