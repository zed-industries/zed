// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SPECULATION_RULES_AUTO_SPECULATION_RULES_CONFIG_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SPECULATION_RULES_AUTO_SPECULATION_RULES_CONFIG_H_

#include <string>

#include "third_party/blink/public/mojom/loader/javascript_framework_detection.mojom-shared.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/speculation_rules/speculation_rule_set.h"
#include "third_party/blink/renderer/platform/json/json_values.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

namespace test {
class AutoSpeculationRulesConfigOverride;
}

// Parses the auto speculation rules config string and provides methods to query
// the resulting configuration. See https://crbug.com/1472970 for more details
// on auto speculation rules.
class CORE_EXPORT AutoSpeculationRulesConfig {
 public:
  explicit AutoSpeculationRulesConfig(const String& config_string);
  ~AutoSpeculationRulesConfig() = default;

  static const AutoSpeculationRulesConfig& GetInstance();

  String ForFramework(mojom::JavaScriptFramework) const;
  Vector<std::pair<String, BrowserInjectedSpeculationRuleOptOut>> ForUrl(
      const KURL&) const;

 private:
  void ParseUrlMatchPatternConfig(const JSONObject* config,
                                  const String& json_key_name,
                                  BrowserInjectedSpeculationRuleOptOut opt_out);

  friend class test::AutoSpeculationRulesConfigOverride;

  // Makes GetInstance() return the given value. Returns the previous override,
  // if any. The caller is responsible for calling OverrideInstanceForTesting()
  // again with that returned value once they're done.
  static AutoSpeculationRulesConfig* OverrideInstanceForTesting(
      AutoSpeculationRulesConfig* new_override);

  Vector<std::pair<mojom::JavaScriptFramework, String>>
      framework_to_speculation_rules_;

  // Because we will eventually be passing the first element of this pair to
  // base::MatchPattern, which takes a std::string, we just store it as
  // std::string from the beginning, instead of using WTF::String and converting
  // at test time.
  Vector<std::pair<std::string,
                   std::pair<String, BrowserInjectedSpeculationRuleOptOut>>>
      url_match_pattern_to_speculation_rules_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SPECULATION_RULES_AUTO_SPECULATION_RULES_CONFIG_H_
