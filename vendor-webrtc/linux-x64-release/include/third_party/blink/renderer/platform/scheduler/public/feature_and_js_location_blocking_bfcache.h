// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_FEATURE_AND_JS_LOCATION_BLOCKING_BFCACHE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_FEATURE_AND_JS_LOCATION_BLOCKING_BFCACHE_H_

#include "third_party/blink/renderer/platform/bindings/source_location.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/public/scheduling_policy.h"

namespace blink {

// A location in JS code.
// BackForwardCacheDisablingFeatureTracker in
// scheduler/common/back_forward_cache_disabling_feature_tracker.h creates
// FeatureAndJSLocationBlockingBFCache from SourceLocation and send it to the
// browser.
class PLATFORM_EXPORT FeatureAndJSLocationBlockingBFCache {
 public:
  FeatureAndJSLocationBlockingBFCache() = default;
  FeatureAndJSLocationBlockingBFCache(SchedulingPolicy::Feature feature,
                                      const String& url,
                                      const String& function,
                                      unsigned line_number,
                                      unsigned column_number);
  FeatureAndJSLocationBlockingBFCache(SchedulingPolicy::Feature feature,
                                      const SourceLocation* source_location);
  FeatureAndJSLocationBlockingBFCache(
      const FeatureAndJSLocationBlockingBFCache&) = default;
  FeatureAndJSLocationBlockingBFCache& operator=(
      const FeatureAndJSLocationBlockingBFCache&) = default;
  ~FeatureAndJSLocationBlockingBFCache();
  bool operator==(const FeatureAndJSLocationBlockingBFCache& other) const;
  SchedulingPolicy::Feature Feature() const { return feature_; }
  const String& Url() const { return url_; }
  const String& Function() const { return function_; }
  unsigned LineNumber() const { return line_number_; }
  unsigned ColumnNumber() const { return column_number_; }

 private:
  SchedulingPolicy::Feature feature_ = SchedulingPolicy::Feature::kMinValue;
  String url_;
  String function_;
  unsigned line_number_;
  unsigned column_number_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_FEATURE_AND_JS_LOCATION_BLOCKING_BFCACHE_H_
