// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_PAINT_TEST_CONFIGURATIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_PAINT_TEST_CONFIGURATIONS_H_

#include <gtest/gtest.h>

#include "base/task/thread_pool/thread_pool_instance.h"
#include "base/test/scoped_feature_list.h"
#include "base/test/task_environment.h"
#include "cc/base/features.h"
#include "third_party/blink/public/common/features.h"
#include "third_party/blink/public/web/web_heap.h"
#include "third_party/blink/renderer/platform/testing/runtime_enabled_features_test_helpers.h"
#include "ui/native_theme/features/native_theme_features.h"

namespace blink {

class CullRectTestConfig {
 public:
  CullRectTestConfig() {
    feature_.InitAndEnableFeatureWithParameters(
        features::kExpandCompositedCullRect,
        {{"pixels", "4000"}, {"changed_enough", "512"}});
  }

 private:
  base::test::ScopedFeatureList feature_;
};

inline constexpr unsigned kUnderInvalidationChecking = 1 << 0;
inline constexpr unsigned kFluentScrollbar = 1 << 1;
inline constexpr unsigned kElementCapture = 1 << 2;
inline constexpr unsigned kRasterInducingScroll = 1 << 3;
inline constexpr unsigned kSpeculativeImageDecodes = 1 << 4;

class PaintTestConfigurations
    : public testing::WithParamInterface<unsigned>,
      private ScopedPaintUnderInvalidationCheckingForTest,
      private ScopedElementCaptureForTest,
      private ScopedRasterInducingScrollForTest,
      private CullRectTestConfig {
 public:
  PaintTestConfigurations()
      : ScopedPaintUnderInvalidationCheckingForTest(GetParam() &
                                                    kUnderInvalidationChecking),
        ScopedElementCaptureForTest(GetParam() & kElementCapture),
        ScopedRasterInducingScrollForTest(GetParam() & kRasterInducingScroll) {
    std::vector<base::test::FeatureRef> enabled_features = {};
    std::vector<base::test::FeatureRef> disabled_features = {};
    if (GetParam() & kFluentScrollbar) {
      enabled_features.push_back(::features::kFluentScrollbar);
    } else {
      disabled_features.push_back(::features::kFluentScrollbar);
    }
    if (GetParam() & kSpeculativeImageDecodes) {
      enabled_features.push_back(features::kSpeculativeImageDecodes);
      enabled_features.push_back(
          ::features::kSendExplicitDecodeRequestsImmediately);
      enabled_features.push_back(::features::kPreventDuplicateImageDecodes);
    }
    feature_list_.InitWithFeatures(enabled_features, disabled_features);
  }
  ~PaintTestConfigurations() override {
    // Must destruct all objects before toggling back feature flags.
    std::unique_ptr<base::test::TaskEnvironment> task_environment;
    if (!base::ThreadPoolInstance::Get()) {
      // Create a TaskEnvironment for the garbage collection below.
      task_environment = std::make_unique<base::test::TaskEnvironment>();
    }
    feature_list_.Reset();
    WebHeap::CollectAllGarbageForTesting();
  }

 private:
  base::test::ScopedFeatureList feature_list_;
};

#define PAINT_TEST_SUITE_P_VALUES \
  0, kFluentScrollbar, kRasterInducingScroll, kSpeculativeImageDecodes

#define INSTANTIATE_PAINT_TEST_SUITE_P(test_class) \
  INSTANTIATE_TEST_SUITE_P(All, test_class,        \
                           ::testing::Values(PAINT_TEST_SUITE_P_VALUES))

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_PAINT_TEST_CONFIGURATIONS_H_
