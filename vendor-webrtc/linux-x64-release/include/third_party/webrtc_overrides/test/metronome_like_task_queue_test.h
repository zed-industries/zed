// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_WEBRTC_OVERRIDES_TEST_METRONOME_LIKE_TASK_QUEUE_TEST_H_
#define THIRD_PARTY_WEBRTC_OVERRIDES_TEST_METRONOME_LIKE_TASK_QUEUE_TEST_H_

#include <functional>
#include <memory>

#include "base/test/task_environment.h"
#include "base/time/time.h"
#include "testing/gtest/include/gtest/gtest.h"
#include "third_party/webrtc/api/task_queue/task_queue_base.h"
#include "third_party/webrtc/rtc_base/system/rtc_export.h"

namespace blink {

class RTC_EXPORT MetronomeLikeTaskQueueProvider {
 public:
  virtual ~MetronomeLikeTaskQueueProvider() = default;

  virtual void Initialize() = 0;
  virtual base::TimeDelta DeltaToNextTick() const = 0;
  virtual base::TimeDelta MetronomeTick() const = 0;
  virtual webrtc::TaskQueueBase* TaskQueue() const = 0;
};

class RTC_EXPORT MetronomeLikeTaskQueueTest
    : public ::testing::TestWithParam<
          std::function<std::unique_ptr<MetronomeLikeTaskQueueProvider>()>> {
 public:
  MetronomeLikeTaskQueueTest()
      : task_environment_(
            base::test::TaskEnvironment::ThreadingMode::MULTIPLE_THREADS,
            base::test::TaskEnvironment::TimeSource::MOCK_TIME),
        provider_(GetParam()()) {}

  void SetUp() override {
    provider_->Initialize();
    task_environment_.FastForwardBy(provider_->DeltaToNextTick());
  }
  void TearDown() override { provider_.reset(); }

 protected:
  base::test::TaskEnvironment task_environment_;
  std::unique_ptr<MetronomeLikeTaskQueueProvider> provider_;
};

}  // namespace blink

#endif  // THIRD_PARTY_WEBRTC_OVERRIDES_TEST_METRONOME_LIKE_TASK_QUEUE_TEST_H_
