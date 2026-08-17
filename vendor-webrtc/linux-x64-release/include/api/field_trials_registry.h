/*
 *  Copyright 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef API_FIELD_TRIALS_REGISTRY_H_
#define API_FIELD_TRIALS_REGISTRY_H_

#include <string>
#include <utility>

#include "absl/strings/string_view.h"
#include "api/field_trials_view.h"
#include "rtc_base/containers/flat_set.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// Abstract base class for a field trial registry that verifies that any looked
// up key has been pre-registered in accordance with `g3doc/field-trials.md`.
class RTC_EXPORT FieldTrialsRegistry : public FieldTrialsView {
 public:
  FieldTrialsRegistry() = default;

  FieldTrialsRegistry(const FieldTrialsRegistry&) = default;
  FieldTrialsRegistry& operator=(const FieldTrialsRegistry&) = default;

  ~FieldTrialsRegistry() override = default;

  // Verifies that `key` is a registered field trial and then returns the
  // configured value for `key` or an empty string if the field trial isn't
  // configured.
  std::string Lookup(absl::string_view key) const override;

  // Register additional `keys` for testing. This should only be used for
  // imaginary keys that are never used outside test code.
  void RegisterKeysForTesting(flat_set<std::string> keys) {
    test_keys_ = std::move(keys);
  }

 private:
  virtual std::string GetValue(absl::string_view key) const = 0;

  // Imaginary keys only used for testing.
  flat_set<std::string> test_keys_;
};

}  // namespace webrtc

#endif  // API_FIELD_TRIALS_REGISTRY_H_
