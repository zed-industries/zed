/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_FIELD_TRIAL_H_
#define TEST_FIELD_TRIAL_H_

#include <string>

#include "absl/strings/string_view.h"

namespace webrtc {
namespace test {

// This class is used to override field-trial configs within specific tests.
// After this class goes out of scope previous field trials will be restored.
class ScopedFieldTrials {
 public:
  explicit ScopedFieldTrials(absl::string_view config);
  ScopedFieldTrials(const ScopedFieldTrials&) = delete;
  ScopedFieldTrials& operator=(const ScopedFieldTrials&) = delete;
  ~ScopedFieldTrials();

 private:
  std::string current_field_trials_;
  const char* previous_field_trials_;
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_FIELD_TRIAL_H_
