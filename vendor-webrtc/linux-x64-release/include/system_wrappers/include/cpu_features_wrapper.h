/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SYSTEM_WRAPPERS_INCLUDE_CPU_FEATURES_WRAPPER_H_
#define SYSTEM_WRAPPERS_INCLUDE_CPU_FEATURES_WRAPPER_H_

#include <stdint.h>

namespace webrtc {

// List of features in x86.
typedef enum { kSSE2, kSSE3, kAVX2, kFMA3 } CPUFeature;

// List of features in ARM.
enum {
  kCPUFeatureARMv7 = (1 << 0),
  kCPUFeatureVFPv3 = (1 << 1),
  kCPUFeatureNEON = (1 << 2),
  kCPUFeatureLDREXSTREX = (1 << 3)
};

// Returns true if the CPU supports the feature.
int GetCPUInfo(CPUFeature feature);

// No CPU feature is available => straight C path.
int GetCPUInfoNoASM(CPUFeature feature);

// Return the features in an ARM device.
// It detects the features in the hardware platform, and returns supported
// values in the above enum definition as a bitmask.
uint64_t GetCPUFeaturesARM(void);

}  // namespace webrtc

#endif  // SYSTEM_WRAPPERS_INCLUDE_CPU_FEATURES_WRAPPER_H_
