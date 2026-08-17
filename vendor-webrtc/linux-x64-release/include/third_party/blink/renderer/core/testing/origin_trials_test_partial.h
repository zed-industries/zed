// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_ORIGIN_TRIALS_TEST_PARTIAL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_ORIGIN_TRIALS_TEST_PARTIAL_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class OriginTrialsTest;

// This interface is used for testing that partial interfaces can  extend other
// interfaces at run-time, if the origin trial is enabled.
class OriginTrialsTestPartial final {
  STATIC_ONLY(OriginTrialsTestPartial);

 public:
  static bool normalAttributePartial(OriginTrialsTest&) { return true; }
  static bool staticAttributePartial() { return true; }
  static bool normalMethodPartial(OriginTrialsTest&) { return true; }
  static bool staticMethodPartial() { return true; }
  static const unsigned kConstantPartial = 2;
  static bool secureAttributePartial(OriginTrialsTest&) { return true; }
  static bool secureStaticAttributePartial() { return true; }
  static bool secureMethodPartial(OriginTrialsTest&) { return true; }
  static bool secureStaticMethodPartial() { return true; }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_ORIGIN_TRIALS_TEST_PARTIAL_H_
