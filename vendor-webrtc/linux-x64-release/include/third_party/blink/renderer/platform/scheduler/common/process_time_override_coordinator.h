// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_PROCESS_TIME_OVERRIDE_COORDINATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_PROCESS_TIME_OVERRIDE_COORDINATOR_H_

#include "base/containers/flat_map.h"
#include "base/functional/callback.h"
#include "base/memory/ptr_util.h"
#include "base/no_destructor.h"
#include "base/synchronization/lock.h"
#include "base/time/time.h"
#include "base/time/time_override.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink::scheduler {

// This class enables multiple clients (normally, thread scheduler for different
// workers and the main thread) to maintain a synthetic time ticks value that is
// guaranteed to increase monotonically across the set of participating clients:
// each client must call `TryAdvancingTime()` whenever it is ready to advance
// time, and the time is advanced to the value that does not exceed that
// requested by any client.
class PLATFORM_EXPORT ProcessTimeOverrideCoordinator {
 public:
  class PLATFORM_EXPORT ScopedOverride {
   public:
    ~ScopedOverride();

    // Records the time ticks desired by the calling client and advances ticks
    // to the value not exceeding that of any of the clients.
    // Returns new time ticks value.
    // If the time advances to `requested_ticks` later as a result of another
    // client advancing time, |schedule_work_callback_| will be called.
    base::TimeTicks TryAdvancingTime(base::TimeTicks requested_ticks);
    base::TimeTicks NowTicks() const;

   private:
    friend class ProcessTimeOverrideCoordinator;

    explicit ScopedOverride(base::RepeatingClosure schedule_work_callback);

    void ScheduleWork() { schedule_work_callback_.Run(); }

    base::RepeatingClosure schedule_work_callback_;
  };

  ~ProcessTimeOverrideCoordinator() = delete;

  // `requested_time` and `requested_ticks` are only honored when no other
  // overrides are active. If another override exists, the times requested
  // by subsequent callers are ignored and values currently in effect
  // are retained.
  // `schedule_work_callback` will be invoked when another client causes
  // time to be advanced.
  // `requested_ticks` needs to be in the future to provide for monotonous
  // values of overridden tick clock.
  // TODO(caseq): remove `requested_ticks` and use appropriate value based
  // on current tick clock.
  static std::unique_ptr<ScopedOverride> CreateOverride(
      base::Time requested_time,
      base::TimeTicks requested_ticks,
      base::RepeatingClosure schedule_work_callback);

 private:
  friend class base::NoDestructor<ProcessTimeOverrideCoordinator>;

  static ProcessTimeOverrideCoordinator& Instance();

  ProcessTimeOverrideCoordinator();

  static base::Time CurrentTime();
  static base::TimeTicks CurrentTicks();

  void RegisterOverride(ScopedOverride* handle,
                        base::Time requested_time,
                        base::TimeTicks requested_ticks);
  void UnregisterOverride(ScopedOverride* handle);
  // See `CreateOverride` above for the semantics of parameters.
  void EnableOverride(base::Time initial_time, base::TimeTicks initial_ticks)
      EXCLUSIVE_LOCKS_REQUIRED(lock_);
  void DisableOverride() EXCLUSIVE_LOCKS_REQUIRED(lock_);

  base::TimeTicks TryAdvancingTime(ScopedOverride* handle,
                                   base::TimeTicks requested_ticks);

  // We do all operations in terms of ticks, but we can simulate a time clock
  // such that `requested_time` is returned just after override has been
  // enabled by the first client and time is advanced by the same increments
  // as ticks after that.
  // The initial values are only set during `EnableOverride()`, which only
  // happens once in production code, so these are effecvitely consts, with
  // the exception of tests.
  base::Time initial_time_;
  base::TimeTicks initial_ticks_;

  std::atomic<base::TimeTicks> current_ticks_;

  base::Lock lock_;
  std::unique_ptr<base::subtle::ScopedTimeClockOverrides> clock_override_
      GUARDED_BY(lock_);
  // This could be a heap of pairs ordered by requested ticks, but, considering
  // we're normally dealing with a very small number of clients, it is
  // unlikely to bring any tangible performance benefits.
  base::flat_map<ScopedOverride*, base::TimeTicks> requested_ticks_by_client_
      GUARDED_BY(lock_);
};

}  // namespace blink::scheduler
#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_PROCESS_TIME_OVERRIDE_COORDINATOR_H_
