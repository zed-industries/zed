/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_EXPLICIT_KEY_VALUE_CONFIG_H_
#define TEST_EXPLICIT_KEY_VALUE_CONFIG_H_

#include <functional>
#include <map>
#include <string>

#include "absl/strings/string_view.h"
#include "api/field_trials_registry.h"

namespace webrtc {
namespace test {

class ExplicitKeyValueConfig : public FieldTrialsRegistry {
 public:
  explicit ExplicitKeyValueConfig(absl::string_view s);

 private:
  std::string GetValue(absl::string_view key) const override;

  // Unlike std::less<std::string>, std::less<> is transparent and allows
  // heterogeneous lookup directly with absl::string_view.
  std::map<std::string, std::string, std::less<>> key_value_map_;
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_EXPLICIT_KEY_VALUE_CONFIG_H_
