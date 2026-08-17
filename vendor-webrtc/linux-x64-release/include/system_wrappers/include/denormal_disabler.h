/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SYSTEM_WRAPPERS_INCLUDE_DENORMAL_DISABLER_H_
#define SYSTEM_WRAPPERS_INCLUDE_DENORMAL_DISABLER_H_

#include "rtc_base/system/arch.h"

namespace webrtc {

// Activates the hardware (HW) way to flush denormals (see [1]) to zero as they
// can very seriously impact performance. At destruction time restores the
// denormals handling state read by the ctor; hence, supports nested calls.
// Equals a no-op if the architecture is not x86 or ARM or if the compiler is
// not CLANG.
// [1] https://en.wikipedia.org/wiki/Denormal_number
//
// Example usage:
//
// void Foo() {
//   DenormalDisabler d;
//   ...
// }
class DenormalDisabler {
 public:
  // Ctor. If architecture and compiler are supported, stores the HW settings
  // for denormals, disables denormals and sets `disabling_activated_` to true.
  // Otherwise, only sets `disabling_activated_` to false.
  DenormalDisabler();
  // Ctor. Same as above, but also requires `enabled` to be true to disable
  // denormals.
  explicit DenormalDisabler(bool enabled);
  DenormalDisabler(const DenormalDisabler&) = delete;
  DenormalDisabler& operator=(const DenormalDisabler&) = delete;
  // Dtor. If `disabling_activated_` is true, restores the denormals HW settings
  // read by the ctor before denormals were disabled. Otherwise it's a no-op.
  ~DenormalDisabler();

  // Returns true if architecture and compiler are supported.
  static bool IsSupported();

 private:
  const int status_word_;
  const bool disabling_activated_;
};

}  // namespace webrtc

#endif  // SYSTEM_WRAPPERS_INCLUDE_DENORMAL_DISABLER_H_
