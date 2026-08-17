// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_SEQUENCE_MANAGER_TEST_MOCK_TIME_DOMAIN_H_
#define BASE_TASK_SEQUENCE_MANAGER_TEST_MOCK_TIME_DOMAIN_H_

#include <optional>

#include "base/task/sequence_manager/time_domain.h"
#include "base/time/tick_clock.h"

namespace base {
namespace sequence_manager {

// TimeDomain with a mock clock and not invoking SequenceManager.
// NOTE: All methods are main thread only.
class MockTimeDomain : public TimeDomain {
 public:
  explicit MockTimeDomain(TimeTicks initial_now_ticks);
  MockTimeDomain(const MockTimeDomain&) = delete;
  MockTimeDomain& operator=(const MockTimeDomain&) = delete;
  ~MockTimeDomain() override;

  void SetNowTicks(TimeTicks now_ticks);

  // TickClock implementation:
  TimeTicks NowTicks() const override;

  // TimeDomain implementation:
  bool MaybeFastForwardToWakeUp(std::optional<WakeUp> next_wake_up,
                                bool quit_when_idle_requested) override;
  const char* GetName() const override;

 private:
  TimeTicks now_ticks_;
};

}  // namespace sequence_manager
}  // namespace base

#endif  // BASE_TASK_SEQUENCE_MANAGER_TEST_MOCK_TIME_DOMAIN_H_
