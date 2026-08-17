// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TIMER_MOCK_TIMER_H_
#define BASE_TIMER_MOCK_TIMER_H_

#include "base/test/simple_test_tick_clock.h"
#include "base/timer/timer.h"

namespace base {

class TestSimpleTaskRunner;

// A mock implementation of base::OneShotTimer which requires being explicitly
// Fire()'d.
// Prefer using TaskEnvironment::MOCK_TIME + FastForward*() to this when
// possible.
class MockOneShotTimer : public OneShotTimer {
 public:
  MockOneShotTimer();
  ~MockOneShotTimer() override;

  // Testing method.
  void Fire();

  // OneShotTimer::FireNow requires no task runner is set. Override FireNow to
  // bypass the check.
  void FireNow() override;

 private:
  // Timer implementation.
  // MockOneShotTimer doesn't support SetTaskRunner. Do not use this.
  void SetTaskRunner(scoped_refptr<SequencedTaskRunner> task_runner) override;

  SimpleTestTickClock clock_;
  scoped_refptr<TestSimpleTaskRunner> test_task_runner_;
};

// See MockOneShotTimer's comment. Prefer using
// TaskEnvironment::MOCK_TIME.
class MockRepeatingTimer : public RepeatingTimer {
 public:
  MockRepeatingTimer();
  ~MockRepeatingTimer() override;

  // Testing method.
  void Fire();

 private:
  // Timer implementation.
  // MockRepeatingTimer doesn't support SetTaskRunner. Do not use this.
  void SetTaskRunner(scoped_refptr<SequencedTaskRunner> task_runner) override;

  SimpleTestTickClock clock_;
  scoped_refptr<TestSimpleTaskRunner> test_task_runner_;
};

// See MockOneShotTimer's comment. Prefer using
// TaskEnvironment::MOCK_TIME.
class MockRetainingOneShotTimer : public RetainingOneShotTimer {
 public:
  MockRetainingOneShotTimer();
  ~MockRetainingOneShotTimer() override;

  // Testing method.
  void Fire();

 private:
  // Timer implementation.
  // MockRetainingOneShotTimer doesn't support SetTaskRunner. Do not use this.
  void SetTaskRunner(scoped_refptr<SequencedTaskRunner> task_runner) override;

  SimpleTestTickClock clock_;
  scoped_refptr<TestSimpleTaskRunner> test_task_runner_;
};

}  // namespace base

#endif  // BASE_TIMER_MOCK_TIMER_H_
