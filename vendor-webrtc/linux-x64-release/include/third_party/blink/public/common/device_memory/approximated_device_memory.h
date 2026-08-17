// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_DEVICE_MEMORY_APPROXIMATED_DEVICE_MEMORY_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_DEVICE_MEMORY_APPROXIMATED_DEVICE_MEMORY_H_

#include <stdint.h>

#include "third_party/blink/public/common/common_export.h"

namespace blink {

class ApproximatedDeviceMemory {
 public:
  // Caches the device's physical memory in static members.
  static void BLINK_COMMON_EXPORT Initialize();

  // Returns an approximation of the physical memory rounded to the most
  // significant bit. This information is provided to web-developers to allow
  // them to customize the experience of their page to the possible available
  // device memory.
  static float BLINK_COMMON_EXPORT GetApproximatedDeviceMemory();

  // Override the value of the physical memory for testing.
  static void BLINK_COMMON_EXPORT SetPhysicalMemoryMBForTesting(int64_t);

 private:
  static void CalculateAndSetApproximatedDeviceMemory();

  static float approximated_device_memory_gb_;
  static int64_t physical_memory_mb_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_DEVICE_MEMORY_APPROXIMATED_DEVICE_MEMORY_H_
