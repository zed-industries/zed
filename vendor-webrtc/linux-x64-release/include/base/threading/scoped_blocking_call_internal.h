// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_THREADING_SCOPED_BLOCKING_CALL_INTERNAL_H_
#define BASE_THREADING_SCOPED_BLOCKING_CALL_INTERNAL_H_

#include <optional>

#include "base/auto_reset.h"
#include "base/base_export.h"
#include "base/functional/callback_forward.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/ref_counted.h"
#include "base/synchronization/lock.h"
#include "base/thread_annotations.h"
#include "base/time/time.h"
#include "base/types/strong_alias.h"

namespace base {

// Forward-declare types from scoped_blocking_call.h to break cyclic dependency.
enum class BlockingType;
using IOJankReportingCallback = RepeatingCallback<void(int, int)>;
using OnlyObservedThreadsForTest =
    StrongAlias<class OnlyObservedThreadsTag, bool>;
void BASE_EXPORT EnableIOJankMonitoringForProcess(IOJankReportingCallback,
                                                  OnlyObservedThreadsForTest);

// Implementation details of types in scoped_blocking_call.h and classes for a
// few key //base types to observe and react to blocking calls.
namespace internal {

// Interface for an observer to be informed when a thread enters or exits
// the scope of ScopedBlockingCall objects.
class BASE_EXPORT BlockingObserver {
 public:
  virtual ~BlockingObserver() = default;

  // Invoked when a ScopedBlockingCall is instantiated on the observed thread
  // where there wasn't an existing ScopedBlockingCall.
  virtual void BlockingStarted(BlockingType blocking_type) = 0;

  // Invoked when a WILL_BLOCK ScopedBlockingCall is instantiated on the
  // observed thread where there was a MAY_BLOCK ScopedBlockingCall but not a
  // WILL_BLOCK ScopedBlockingCall.
  virtual void BlockingTypeUpgraded() = 0;

  // Invoked when the last ScopedBlockingCall on the observed thread is
  // destroyed.
  virtual void BlockingEnded() = 0;
};

// Registers |new_blocking_observer| on the current thread. It is invalid to
// call this on a thread where there is an active ScopedBlockingCall.
BASE_EXPORT void SetBlockingObserverForCurrentThread(
    BlockingObserver* new_blocking_observer);

BASE_EXPORT void ClearBlockingObserverForCurrentThread();

// An IOJankMonitoringWindow instruments 1-minute of runtime. Any I/O jank > 1
// second happening during that period will be reported to it. It will then
// report via the IOJankReportingCallback in |reporting_callback_storage()| if
// it's non-null. https://bit.ly/chrome-io-jank-metric.
class BASE_EXPORT [[maybe_unused, nodiscard]] IOJankMonitoringWindow
    : public RefCountedThreadSafe<IOJankMonitoringWindow> {
 public:
  explicit IOJankMonitoringWindow(TimeTicks start_time);

  IOJankMonitoringWindow(const IOJankMonitoringWindow&) = delete;
  IOJankMonitoringWindow& operator=(const IOJankMonitoringWindow&) = delete;

  // Cancels monitoring and clears this class' static state.
  static void CancelMonitoringForTesting();

  class [[maybe_unused, nodiscard]] ScopedMonitoredCall {
   public:
    // Stores a ref to the current IOJankMonitoringWindow if monitoring is
    // active, keeping it alive at least until the monitored call completes or
    // Cancel() is invoked.
    ScopedMonitoredCall();

    // Reports to |assigned_jank_window_| if it's non-null.
    ~ScopedMonitoredCall();

    ScopedMonitoredCall(const ScopedMonitoredCall&) = delete;
    ScopedMonitoredCall& operator=(const ScopedMonitoredCall&) = delete;

    // Cancels monitoring of this call.
    void Cancel();

   private:
    TimeTicks call_start_;
    scoped_refptr<IOJankMonitoringWindow> assigned_jank_window_;
  };

  static constexpr TimeDelta kIOJankInterval = Seconds(1);
  static constexpr TimeDelta kMonitoringWindow = Minutes(1);
  static constexpr TimeDelta kTimeDiscrepancyTimeout = kIOJankInterval * 10;
  static constexpr int kNumIntervals = kMonitoringWindow / kIOJankInterval;

  // kIOJankIntervals must integrally fill kMonitoringWindow
  static_assert((kMonitoringWindow % kIOJankInterval).is_zero(), "");

  // Cancelation is simple because it can only affect the current window.
  static_assert(kTimeDiscrepancyTimeout < kMonitoringWindow, "");

 private:
  friend class base::RefCountedThreadSafe<IOJankMonitoringWindow>;
  friend void base::EnableIOJankMonitoringForProcess(
      IOJankReportingCallback,
      OnlyObservedThreadsForTest);

  // No-op if reporting_callback_storage() is null (i.e. unless
  // EnableIOJankMonitoringForProcess() was called).
  // When reporting_callback_storage() is non-null : Ensures that there's an
  // active IOJankMonitoringWindow for Now(), connects it via |next_| to the
  // previous IOJankMonitoringWindow to let ScopedMonitoredCalls that span
  // multiple windows report to each window they cover. In the event that Now()
  // is farther ahead than expected (> 10s), the previous window is |canceled_|
  // as it was likely interrupted by a system sleep and a new
  // IOJankMonitoringWindow chain is started from Now(). In all cases, returns a
  // live reference to the current (old or new) IOJankMonitoringWindow as a
  // helper so callers that need it don't need to re-acquire
  // current_jank_window_lock() after calling this.
  // |recent_now| is a recent sampling of TimeTicks::Now(), avoids
  // double-sampling Now() from most callers.
  static scoped_refptr<IOJankMonitoringWindow> MonitorNextJankWindowIfNecessary(
      TimeTicks recent_now);

  // An IOJankMonitoringWindow is destroyed when all refs to it are gone, i.e.:
  //  1) The window it covers has elapsed and MonitorNextJankWindowIfNecessary()
  //     has replaced it.
  //  2) All pending ScopedMonitoredCall's in their range have completed
  //     (including the ones that transitively have it in their |next_| chain).
  ~IOJankMonitoringWindow();

  // Called from ~ScopedMonitoredCall().
  void OnBlockingCallCompleted(TimeTicks call_start, TimeTicks call_end);

  // Helper for OnBlockingCallCompleted(). Records |num_janky_intervals|
  // starting at |local_jank_start_index|. Having this logic separately helps
  // sane management of |intervals_lock_| when recursive calls through |next_|
  // pointers are necessary.
  void AddJank(int local_jank_start_index, int num_janky_intervals);

  static Lock& current_jank_window_lock();
  static scoped_refptr<IOJankMonitoringWindow>& current_jank_window_storage()
      EXCLUSIVE_LOCKS_REQUIRED(current_jank_window_lock());

  // Storage for callback used to report monitoring results.
  // NullCallback if monitoring was not enabled for this process.
  static IOJankReportingCallback& reporting_callback_storage()
      EXCLUSIVE_LOCKS_REQUIRED(current_jank_window_lock());

  Lock intervals_lock_;
  size_t intervals_jank_count_[kNumIntervals] GUARDED_BY(intervals_lock_) = {};

  const TimeTicks start_time_;

  // Set only once per window, in MonitorNextJankWindowIfNecessary(). Any read
  // of this value must be ordered after that call in memory and in time.
  scoped_refptr<IOJankMonitoringWindow> next_;

  // Set to true if ~IOJankMonitoringWindow() shouldn't record metrics.
  // Modifications of this variable must be synchronized with each other and
  // happen-before ~IOJankMonitoringWindow().
  bool canceled_ = false;
};

// Common implementation class for both ScopedBlockingCall and
// ScopedBlockingCallWithBaseSyncPrimitives without assertions.
class BASE_EXPORT [[maybe_unused, nodiscard]] UncheckedScopedBlockingCall {
 public:
  enum class BlockingCallType {
    kRegular,
    kBaseSyncPrimitives,
  };

  UncheckedScopedBlockingCall(BlockingType blocking_type,
                              BlockingCallType blocking_call_type);

  UncheckedScopedBlockingCall(const UncheckedScopedBlockingCall&) = delete;
  UncheckedScopedBlockingCall& operator=(const UncheckedScopedBlockingCall&) =
      delete;

  ~UncheckedScopedBlockingCall();

 private:
  const raw_ptr<BlockingObserver> blocking_observer_;

  // Previous ScopedBlockingCall instantiated on this thread.
  const raw_ptr<UncheckedScopedBlockingCall> previous_scoped_blocking_call_;

  const base::AutoReset<UncheckedScopedBlockingCall*> resetter_;

  // Whether the BlockingType of the current thread was WILL_BLOCK after this
  // ScopedBlockingCall was instantiated.
  const bool is_will_block_;

  // Non-nullopt for non-nested blocking calls of type MAY_BLOCK on foreground
  // threads which we monitor for I/O jank.
  std::optional<IOJankMonitoringWindow::ScopedMonitoredCall> monitored_call_;
};

}  // namespace internal
}  // namespace base

#endif  // BASE_THREADING_SCOPED_BLOCKING_CALL_INTERNAL_H_
