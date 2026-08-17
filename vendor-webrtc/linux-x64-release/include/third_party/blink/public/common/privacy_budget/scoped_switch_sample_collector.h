// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_SCOPED_SWITCH_SAMPLE_COLLECTOR_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_SCOPED_SWITCH_SAMPLE_COLLECTOR_H_

#include "base/component_export.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/privacy_budget/identifiability_sample_collector.h"

namespace blink {
namespace test {

// ***NOTE ***
//    If you are looking for an intropspectable `IdentifiabilitySampleCollector`
//    for testing, you are probably looking for
//    `ScopedIdentifiabilityTestSampleCollector` instead.
//
// `IdentifiabilitySampleCollector` is a per-process singleton meant to be
// accessible from anywhere and from any thread. For testing purposes, however,
// it would be convenient to swap out the default collector and instead use
// a test stand-in.
//
// `ScopedSwitchSampleCollector` sets the default sample collector to
// a replacement object for the duration of the `ScopedSwitchSampleCollector`'s
// lifetime.
//
// Example usage:
//
//     TEST(MyTest, Something) {
//       MyFakeSampleCollector collector;
//       ScopedSwitchSampleCollector scoped_default(&collector);
//       ...
//     }
//
// `ScopedSwitchSampleCollector` does not nest. Instantiating a new object while
// there's an active `ScopedSwitchSampleCollector` isn't allowed and will
// `DCHECK` on debug builds.
class COMPONENT_EXPORT(PRIVACY_BUDGET_TEST_SUPPORT)
    ScopedSwitchSampleCollector {
 public:
  // Sets `replacement` as the default `IdentifiabilitySampleCollector` for the
  // duration of this object's lifetime. `replacement` must outlive this object.
  explicit ScopedSwitchSampleCollector(
      IdentifiabilitySampleCollector* replacement);

  ~ScopedSwitchSampleCollector();
};

}  // namespace test
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_SCOPED_SWITCH_SAMPLE_COLLECTOR_H_
