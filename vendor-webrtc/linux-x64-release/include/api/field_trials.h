/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_FIELD_TRIALS_H_
#define API_FIELD_TRIALS_H_

#include <memory>
#include <string>

#include "absl/strings/string_view.h"
#include "api/field_trials_registry.h"
#include "rtc_base/containers/flat_map.h"

namespace webrtc {

// The FieldTrials class is used to inject field trials into webrtc.
//
// Field trials allow webrtc clients (such as Chromium) to turn on feature code
// in binaries out in the field and gather information with that.
//
// They are designed to be easy to use with Chromium field trials and to speed
// up developers by reducing the need to wire up APIs to control whether a
// feature is on/off.
//
// The field trials are injected into objects that use them at creation time.
//
// NOTE: Creating multiple FieldTrials-object is currently prohibited
// until we remove the global string (TODO(bugs.webrtc.org/10335))
// (unless using CreateNoGlobal):
class FieldTrials : public FieldTrialsRegistry {
 public:
  explicit FieldTrials(absl::string_view s);
  ~FieldTrials();

  // Create a FieldTrials object that is not reading/writing from
  // global variable (i.e can not be used for all parts of webrtc).
  static std::unique_ptr<FieldTrials> CreateNoGlobal(absl::string_view s);

 private:
  explicit FieldTrials(absl::string_view s, bool);

  std::string GetValue(absl::string_view key) const override;

  const bool uses_global_;
  const std::string field_trial_string_;
  const char* const previous_field_trial_string_;
  const flat_map<std::string, std::string> key_value_map_;
};

}  // namespace webrtc

#endif  // API_FIELD_TRIALS_H_
