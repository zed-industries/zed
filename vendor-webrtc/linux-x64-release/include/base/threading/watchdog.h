// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// The Watchdog class creates a second thread that can Alarm if a specific
// duration of time passes without proper attention.  The duration of time is
// specified at construction time.  The Watchdog may be used many times by
// simply calling Arm() (to start timing) and Disarm() (to reset the timer).
// The Watchdog is typically used under a debugger, where the stack traces on
// other threads can be examined if/when the Watchdog alarms.

// Some watchdogs will be enabled or disabled via command line switches. To
// facilitate such code, an "enabled" argument for the constuctor can be used
// to permanently disable the watchdog.  Disabled watchdogs don't even spawn
// a second thread, and their methods call (Arm() and Disarm()) return very
// quickly.

#ifndef BASE_THREADING_WATCHDOG_H_
#define BASE_THREADING_WATCHDOG_H_

#include <string>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/memory/raw_ptr.h"
#include "base/synchronization/condition_variable.h"
#include "base/synchronization/lock.h"
#include "base/threading/platform_thread.h"
#include "base/time/time.h"

namespace base {

class BASE_EXPORT Watchdog {
 public:
  class Delegate {
   public:
    virtual ~Delegate() = default;

    // Called on the watchdog thread.
    virtual void Alarm() = 0;
  };

  // Constructor specifies how long the Watchdog will wait before alarming. If
  // `delegate` is non-null, `Alarm` on the delegate will be called instead of
  // the default behavior.
  Watchdog(const TimeDelta& duration,
           const std::string& thread_watched_name,
           bool enabled,
           Delegate* delegate = nullptr);

  Watchdog(const Watchdog&) = delete;
  Watchdog& operator=(const Watchdog&) = delete;

  ~Watchdog();

  // Notify watchdog thread to finish up. Sets the state_ to SHUTDOWN.
  void Cleanup();

  // Returns true if we state_ is JOINABLE (which indicates that Watchdog has
  // exited).
  bool IsJoinable();

  // Start timing, and alarm when time expires (unless we're disarm()ed.)
  void Arm();  // Arm  starting now.
  void ArmSomeTimeDeltaAgo(const TimeDelta& time_delta);
  void ArmAtStartTime(const TimeTicks start_time);

  // Reset time, and do not set off the alarm.
  void Disarm();

  // Alarm is called if the time expires after an Arm() without someone calling
  // Disarm().
  void Alarm();

  // Reset static data to initial state. Useful for tests, to ensure
  // they are independent.
  static void ResetStaticData();

  // The default behavior of Alarm() if a delegate is not provided.
  void DefaultAlarm();

 private:
  class ThreadDelegate : public PlatformThread::Delegate {
   public:
    explicit ThreadDelegate(Watchdog* watchdog) : watchdog_(watchdog) {}
    void ThreadMain() override;

   private:
    void SetThreadName() const;

    raw_ptr<Watchdog> watchdog_;
  };

  enum State { ARMED, DISARMED, SHUTDOWN, JOINABLE };

  bool enabled_;

  Lock lock_;
  ConditionVariable condition_variable_;
  State state_ GUARDED_BY(lock_);
  const TimeDelta duration_;  // How long after start_time_ do we alarm?
  const std::string thread_watched_name_;
  PlatformThreadHandle handle_;
  ThreadDelegate thread_delegate_;  // Must outlive the thread.

  raw_ptr<Delegate> delegate_;
  TimeTicks start_time_;  // Start of epoch, and alarm after duration_.
};

}  // namespace base

#endif  // BASE_THREADING_WATCHDOG_H_
