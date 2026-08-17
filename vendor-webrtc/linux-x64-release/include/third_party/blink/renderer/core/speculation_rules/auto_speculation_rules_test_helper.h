// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SPECULATION_RULES_AUTO_SPECULATION_RULES_TEST_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SPECULATION_RULES_AUTO_SPECULATION_RULES_TEST_HELPER_H_

#include "third_party/blink/renderer/core/speculation_rules/auto_speculation_rules_config.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink::test {

class AutoSpeculationRulesConfigOverride {
 public:
  explicit AutoSpeculationRulesConfigOverride(const String& config_string);
  ~AutoSpeculationRulesConfigOverride();

 private:
  std::unique_ptr<AutoSpeculationRulesConfig> current_override_;
  AutoSpeculationRulesConfig* previous_override_;
};

}  // namespace blink::test

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SPECULATION_RULES_AUTO_SPECULATION_RULES_TEST_HELPER_H_
