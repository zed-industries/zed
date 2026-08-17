// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_INSTRUMENTATION_USE_COUNTER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_INSTRUMENTATION_USE_COUNTER_H_

#include "third_party/blink/public/mojom/use_counter/metrics/web_feature.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/use_counter/metrics/webdx_feature.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

// TODO(yhirano): Remove this.
using WebFeature = mojom::WebFeature;
using WebDXFeature = mojom::blink::WebDXFeature;

// Definition for UseCounter features can be found in:
// third_party/blink/public/mojom/use_counter/metrics/web_feature.mojom and
// third_party/blink/public/mojom/use_counter/metrics/webdx_feature.mojom
//
// UseCounter is used for counting the number of times features of
// Blink are used on real web pages and help us know commonly
// features are used and thus when it's safe to remove or change them. It's
// counting whether a feature is used in a context (e.g., a page), so calling
// a counting function multiple times for the same UseCounter with the same
// feature will be ignored.
//
// The Chromium Content layer controls what is done with this data.
//
// For instance, in Google Chrome, these counts are submitted anonymously
// through the UMA histogram recording system in Chrome for users who have the
// "Automatically send usage statistics and crash reports to Google" setting
// enabled:
// http://www.google.com/chrome/intl/en/privacy.html
//
// This is a pure virtual interface class with some utility static functions.
class UseCounter : public GarbageCollectedMixin {
 public:
  static void Count(UseCounter* use_counter, mojom::WebFeature feature) {
    if (use_counter) {
      use_counter->CountUse(feature);
    }
  }
  static void Count(UseCounter& use_counter, mojom::WebFeature feature) {
    use_counter.CountUse(feature);
  }
  static void CountDeprecation(UseCounter* use_counter,
                               mojom::WebFeature feature) {
    if (use_counter) {
      use_counter->CountDeprecation(feature);
    }
  }
  static void CountWebDXFeature(UseCounter* use_counter, WebDXFeature feature) {
    if (use_counter) {
      use_counter->CountWebDXFeature(feature);
    }
  }
  static void CountWebDXFeature(UseCounter& use_counter, WebDXFeature feature) {
    use_counter.CountWebDXFeature(feature);
  }

  UseCounter() = default;
  UseCounter(const UseCounter&) = delete;
  UseCounter& operator=(const UseCounter&) = delete;
  virtual ~UseCounter() = default;

  // Counts a use of the given feature. Repeated calls are ignored.
  virtual void CountUse(mojom::WebFeature feature) = 0;

  // Counts a use of the given feature which is being deprecated. Repeated
  // calls are ignored.
  virtual void CountDeprecation(mojom::WebFeature feature) = 0;

  // Counts a use of the given feature. Repeated calls are ignored.
  virtual void CountWebDXFeature(WebDXFeature feature) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_INSTRUMENTATION_USE_COUNTER_H_
