/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_SCOPED_KEY_VALUE_CONFIG_H_
#define TEST_SCOPED_KEY_VALUE_CONFIG_H_

#include <functional>
#include <map>
#include <memory>
#include <string>

#include "absl/strings/string_view.h"
#include "api/field_trials_registry.h"
#include "test/field_trial.h"

namespace webrtc {
namespace test {

class ScopedKeyValueConfig : public FieldTrialsRegistry {
 public:
  virtual ~ScopedKeyValueConfig();
  ScopedKeyValueConfig();
  explicit ScopedKeyValueConfig(absl::string_view s);
  ScopedKeyValueConfig(ScopedKeyValueConfig& parent, absl::string_view s);

 private:
  ScopedKeyValueConfig(ScopedKeyValueConfig* parent, absl::string_view s);
  ScopedKeyValueConfig* GetRoot(ScopedKeyValueConfig* n);
  std::string GetValue(absl::string_view key) const override;
  std::string LookupRecurse(absl::string_view key) const;

  ScopedKeyValueConfig* const parent_;

  // The leaf in a list of stacked ScopedKeyValueConfig.
  // Only set on root (e.g with parent_ == nullptr).
  const ScopedKeyValueConfig* leaf_;

  // Unlike std::less<std::string>, std::less<> is transparent and allows
  // heterogeneous lookup directly with absl::string_view.
  std::map<std::string, std::string, std::less<>> key_value_map_;
  std::unique_ptr<ScopedFieldTrials> scoped_field_trials_;
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_SCOPED_KEY_VALUE_CONFIG_H_
