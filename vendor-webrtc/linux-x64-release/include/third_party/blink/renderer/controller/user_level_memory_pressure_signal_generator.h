// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CONTROLLER_USER_LEVEL_MEMORY_PRESSURE_SIGNAL_GENERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CONTROLLER_USER_LEVEL_MEMORY_PRESSURE_SIGNAL_GENERATOR_H_

#include <optional>

#include "base/memory/raw_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "build/build_config.h"
#include "third_party/blink/renderer/controller/controller_export.h"
#include "third_party/blink/renderer/controller/memory_usage_monitor.h"
#include "third_party/blink/renderer/platform/scheduler/public/rail_mode_observer.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

#if BUILDFLAG(IS_ANDROID)

namespace base {
class TickClock;
}

namespace blink {

class Platform;
class MainThreadScheduler;

namespace user_level_memory_pressure_signal_generator_test {
class MockUserLevelMemoryPressureSignalGenerator;
}  // namespace user_level_memory_pressure_signal_generator_test

// Generates extra memory pressure signals (on top of the OS generated ones)
// when the memory usage goes over a threshold.
class CONTROLLER_EXPORT UserLevelMemoryPressureSignalGenerator
    : public RAILModeObserver {
  USING_FAST_MALLOC(UserLevelMemoryPressureSignalGenerator);

 public:
  static UserLevelMemoryPressureSignalGenerator* Instance();

  // Returns the shared instance.
  static void Initialize(
      Platform* platform,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);

  void RequestMemoryPressureSignal();

 private:
  friend class user_level_memory_pressure_signal_generator_test::
      MockUserLevelMemoryPressureSignalGenerator;

  explicit UserLevelMemoryPressureSignalGenerator(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner,
      std::pair<base::TimeDelta, base::TimeDelta> inert_and_minimum_interval);
  UserLevelMemoryPressureSignalGenerator(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner,
      base::TimeDelta inert_interval,
      base::TimeDelta minimum_interval,
      const base::TickClock* clock,
      MainThreadScheduler* main_thread_scheduler);

  ~UserLevelMemoryPressureSignalGenerator() override;

  // RAILModeObserver:
  void OnRAILModeChanged(RAILMode rail_mode) override;

  // This is only virtual to override in tests.
  virtual void Generate(base::TimeTicks now);

  void OnTimerFired();

  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
  base::TimeDelta inert_interval_ = base::TimeDelta();
  base::TimeDelta minimum_interval_ = base::TimeDelta();
  raw_ptr<const base::TickClock> clock_;

  bool is_loading_ = false;
  std::optional<base::TimeTicks> last_loaded_;
  bool has_pending_request_ = false;
  base::TimeTicks last_requested_;
  std::optional<base::TimeTicks> last_generated_;
  raw_ptr<MainThreadScheduler> main_thread_scheduler_ = nullptr;
};

}  // namespace blink

#endif  // BUILDFLAG(IS_ANDROID)

#endif  // THIRD_PARTY_BLINK_RENDERER_CONTROLLER_USER_LEVEL_MEMORY_PRESSURE_SIGNAL_GENERATOR_H_
