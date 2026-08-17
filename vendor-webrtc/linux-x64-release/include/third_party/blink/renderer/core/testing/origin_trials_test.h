// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_ORIGIN_TRIALS_TEST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_ORIGIN_TRIALS_TEST_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_origin_trials_test_dictionary.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class ExceptionState;
class ScriptState;

// OriginTrialsTest is a very simple interface used for testing
// origin-trial-enabled features which are attached directly to interfaces at
// run-time.
class OriginTrialsTest : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Required by IDL constructor.
  static OriginTrialsTest* Create();

  OriginTrialsTest() = default;
  ~OriginTrialsTest() override = default;

  bool normalAttribute() { return true; }
  static bool staticAttribute() { return true; }
  bool normalMethod() { return true; }
  static bool staticMethod() { return true; }
  static const uint16_t kConstant = 1;

  bool throwingAttribute(ScriptState*, ExceptionState&);
  OriginTrialsTestDictionary* getDictionaryMethod() {
    return OriginTrialsTestDictionary::Create();
  }
  void checkDictionaryMethod(const OriginTrialsTestDictionary*) {}

  bool unconditionalAttribute() { return true; }
  static bool staticUnconditionalAttribute() { return true; }
  bool unconditionalMethod() { return true; }
  void unconditionalDictionaryMethod(const OriginTrialsTestDictionary* dict) {}
  static bool staticUnconditionalMethod() { return true; }
  static const uint16_t kUnconditionalConstant = 99;

  bool secureUnconditionalAttribute() { return true; }
  static bool secureStaticUnconditionalAttribute() { return true; }
  bool secureUnconditionalMethod() { return true; }
  static bool secureStaticUnconditionalMethod() { return true; }

  bool secureAttribute() { return true; }
  static bool secureStaticAttribute() { return true; }
  bool secureMethod() { return true; }
  static bool secureStaticMethod() { return true; }

  bool deprecationAttribute() { return true; }

  bool impliedAttribute() { return true; }

  bool invalidOSAttribute() { return true; }

  bool navigationMethod() { return true; }

  bool thirdPartyAttribute() { return true; }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_ORIGIN_TRIALS_TEST_H_
