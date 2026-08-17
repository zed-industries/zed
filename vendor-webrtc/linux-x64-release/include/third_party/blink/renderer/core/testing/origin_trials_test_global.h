// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_ORIGIN_TRIALS_TEST_GLOBAL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_ORIGIN_TRIALS_TEST_GLOBAL_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class LocalDOMWindow;

// This is used for testing that the Window interface can be extended with
// origin trial-enabled IDL members at run-time.
class OriginTrialsTestGlobal final {
  STATIC_ONLY(OriginTrialsTestGlobal);

 public:
  static bool testOriginTrialGlobalAttribute(LocalDOMWindow&) { return true; }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_ORIGIN_TRIALS_TEST_GLOBAL_H_
