// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_THREADING_HANG_WATCHER_H_
#define BASE_THREADING_HANG_WATCHER_H_

#include <atomic>
#include <cstdint>
#include <memory>
#include <type_traits>
#include <vector>

#include "base/auto_reset.h"
#include "base/base_export.h"
#include "base/bits.h"
#include "base/compiler_specific.h"
#include "base/dcheck_is_on.h"
#include "base/debug/crash_logging.h"
#include "base/functional/callback.h"
#include "base/functional/callback_forward.h"
#include "base/functional/callback_helpers.h"
#include "base/gtest_prod_util.h"
#include "base/memory/memory_pressure_listener.h"
#include "base/memory/raw_ptr.h"
#include "base/synchronization/lock.h"
#include "base/synchronization/waitable_event.h"
#include "base/thread_annotations.h"
#include "base/threading/platform_thread.h"
#include "base/threading/simple_thread.h"
#include "base/threading/thread_checker.h"
#include "base/time/tick_clock.h"
#include "base/time/time.h"
#include "build/build_config.h"

namespace base {
class WatchHangsInScope;
namespace internal {
class HangWatchState;
}  // namespace internal
}  // namespace base

namespace base {

// Instantiate a WatchHangsInScope in a code scope to register to be
// watched for hangs of more than |timeout| by the HangWatcher.
//
// Example usage:
//
//  void FooBar(){
//    WatchHangsInScope scope(base::Seconds(5));
//    DoWork();
//  }
//
// If DoWork() takes more than 5s to run and the HangWatcher
// inspects the thread state before Foobar returns a hang will be
// reported.
//
// WatchHangsInScopes are typically meant to live on the stack. In some
// cases it's necessary to keep a WatchHangsInScope instance as a class
// member but special care is required when doing so as a WatchHangsInScope
// that stays alive longer than intended will generate non-actionable hang
// reports.
class BASE_EXPORT [[maybe_unused, nodiscard]] WatchHangsInScope {
 public:
  // A good default value needs to be large enough to represent a significant
  // hang and avoid noise while being small enough to not exclude too many
  // hangs. The nature of the work that gets executed on the thread is also
  // important. We can be much stricter when monitoring a UI thread compared to
  // a ThreadPool thread for example.
  static constexpr base::TimeDelta kDefaultHangWatchTime = base::Seconds(10);

  // Constructing/destructing thread must be the same thread.
  explicit WatchHangsInScope(TimeDelta timeout = kDefaultHangWatchTime);
  ~WatchHangsInScope();

  WatchHangsInScope(const WatchHangsInScope&) = delete;
  WatchHangsInScope& operator=(const WatchHangsInScope&) = delete;

 private:
  // Will be true if the object actually set a deadline and false if not.
  bool took_effect_ = true;

  // This object should always be constructed and destructed on the same thread.
  THREAD_CHECKER(thread_checker_);

  // The deadline set by the previous WatchHangsInScope created on this
  // thread. Stored so it can be restored when this WatchHangsInScope is
  // destroyed.
  TimeTicks previous_deadline_;

  // Indicates whether the kIgnoreCurrentWatchHangsInScope flag must be set upon
  // exiting this WatchHangsInScope if a call to InvalidateActiveExpectations()
  // previously suspended hang watching.
  bool set_hangs_ignored_on_exit_ = false;

#if DCHECK_IS_ON()
  // The previous WatchHangsInScope created on this thread.
  raw_ptr<WatchHangsInScope> previous_watch_hangs_in_scope_;
#endif
};

// Monitors registered threads for hangs by inspecting their associated
// HangWatchStates for deadline overruns. This happens at a regular interval on
// a separate thread. Only one instance of HangWatcher can exist at a time
// within a single process. This instance must outlive all monitored threads.
class BASE_EXPORT HangWatcher : public DelegateSimpleThread::Delegate {
 public:
  // Describes the type of a process for logging purposes.
  enum class ProcessType {
    kUnknownProcess = 0,
    kBrowserProcess = 1,
    kGPUProcess = 2,
    kRendererProcess = 3,
    kUtilityProcess = 4,
    kMax = kUtilityProcess
  };

  // Describes the type of a thread for logging purposes.
  enum class ThreadType {
    kIOThread = 0,
    kMainThread = 1,
    kThreadPoolThread = 2,
    kMax = kThreadPoolThread
  };

  // Notes on lifetime:
  //   1) The first invocation of the constructor will set the global instance
  //      accessible through GetInstance().
  //   2) In production HangWatcher is always purposefuly leaked.
  //   3) If not leaked HangWatcher is always constructed and destructed from
  //      the same thread.
  //   4) There can never be more than one instance of HangWatcher at a time.
  //      The class is not base::Singleton derived because it needs to destroyed
  //      in tests.
  HangWatcher();

  // Clears the global instance for the class.
  ~HangWatcher() override;

  HangWatcher(const HangWatcher&) = delete;
  HangWatcher& operator=(const HangWatcher&) = delete;

  static void CreateHangWatcherInstance();

  // Returns a non-owning pointer to the global HangWatcher instance.
  static HangWatcher* GetInstance();

  // Initializes HangWatcher. Must be called once on the main thread during
  // startup while single-threaded.
  static void InitializeOnMainThread(ProcessType process_type,
                                     bool emit_crashes);

  // Returns the values that were set through InitializeOnMainThread() to their
  // default value. Used for testing since in prod initialization should happen
  // only once.
  static void UnitializeOnMainThreadForTesting();

  // Thread safe functions to verify if hang watching is activated. If called
  // before InitializeOnMainThread returns the default value which is false.
  static bool IsEnabled();
  static bool IsThreadPoolHangWatchingEnabled();
  static bool IsIOThreadHangWatchingEnabled();

  // Returns true if crash dump reporting is configured for any thread type.
  static bool IsCrashReportingEnabled();

  // Use to avoid capturing hangs for operations known to take unbounded time
  // like waiting for user input. WatchHangsInScope objects created after this
  // call will take effect. To resume watching for hangs create a new
  // WatchHangsInScope after the unbounded operation finishes.
  //
  // Example usage:
  //  {
  //    WatchHangsInScope scope_1;
  //    {
  //      WatchHangsInScope scope_2;
  //      InvalidateActiveExpectations();
  //      WaitForUserInput();
  //    }
  //
  //    WatchHangsInScope scope_4;
  //  }
  //
  // WatchHangsInScope scope_5;
  //
  // In this example hang watching is disabled for WatchHangsInScopes 1 and 2
  // since they were both active at the time of the invalidation.
  // WatchHangsInScopes 4 and 5 are unaffected since they were created after the
  // end of the WatchHangsInScope that was current at the time of invalidation.
  //
  static void InvalidateActiveExpectations();

  // Marks the current process as "shutting down". This changes the histograms
  // emitted every interval for all threads.
  static void SetShuttingDown();

  // Sets up the calling thread to be monitored for threads. Returns a
  // ScopedClosureRunner that unregisters the thread. This closure has to be
  // called from the registered thread before it's joined. Returns a null
  // closure in the case where there is no HangWatcher instance to register the
  // thread with.
  [[nodiscard]] static ScopedClosureRunner RegisterThread(
      ThreadType thread_type);

  // Choose a closure to be run at the end of each call to Monitor(). Use only
  // for testing. Reentering the HangWatcher in the closure must be done with
  // care. It should only be done through certain testing functions because
  // deadlocks are possible.
  void SetAfterMonitorClosureForTesting(base::RepeatingClosure closure);

  // Choose a closure to be run instead of recording the hang. Used to test
  // that certain conditions hold true at the time of recording. Use only
  // for testing. Reentering the HangWatcher in the closure must be done with
  // care. It should only be done through certain testing functions because
  // deadlocks are possible.
  void SetOnHangClosureForTesting(base::RepeatingClosure closure);

  // Set a monitoring period other than the default. Use only for
  // testing.
  void SetMonitoringPeriodForTesting(base::TimeDelta period);

  // Choose a callback to invoke right after waiting to monitor in Wait(). Use
  // only for testing.
  void SetAfterWaitCallbackForTesting(
      RepeatingCallback<void(TimeTicks)> callback);

  // Force the monitoring loop to resume and evaluate whether to continue.
  // This can trigger a call to Monitor() or not depending on why the
  // HangWatcher thread is sleeping. Use only for testing.
  void SignalMonitorEventForTesting();

  // Call to make sure no more monitoring takes place. The
  // function is thread-safe and can be called at anytime but won't stop
  // monitoring that is currently taking place. Use only for testing.
  static void StopMonitoringForTesting();

  // Replace the clock used when calculating time spent
  // sleeping. Use only for testing.
  void SetTickClockForTesting(const base::TickClock* tick_clock);

  // Use to block until the hang is recorded. Allows the caller to halt
  // execution so it does not overshoot the hang watch target and result in a
  // non-actionable stack trace in the crash recorded.
  void BlockIfCaptureInProgress();

  // Begin executing the monitoring loop on the HangWatcher thread.
  void Start();

  // Returns true if Start() has been called and Stop() has not been called
  // since.
  bool IsStarted() const { return thread_started_; }

  // Returns the value of the crash key with the time since last system power
  // resume.
  std::string GetTimeSinceLastSystemPowerResumeCrashKeyValue() const;

 private:
  // See comment of ::RegisterThread() for details.
  [[nodiscard]] ScopedClosureRunner RegisterThreadInternal(
      ThreadType thread_type) LOCKS_EXCLUDED(watch_state_lock_);

  // Use to assert that functions are called on the monitoring thread.
  THREAD_CHECKER(hang_watcher_thread_checker_);

  // Use to assert that functions are called on the constructing thread.
  THREAD_CHECKER(constructing_thread_checker_);

  // Invoked on memory pressure signal.
  void OnMemoryPressure(
      base::MemoryPressureListener::MemoryPressureLevel memory_pressure_level);

#if !BUILDFLAG(IS_NACL)
  // Returns a ScopedCrashKeyString that sets the crash key with the time since
  // last critical memory pressure signal.
  [[nodiscard]] debug::ScopedCrashKeyString
  GetTimeSinceLastCriticalMemoryPressureCrashKey();
#endif

  // Invoke base::debug::DumpWithoutCrashing() insuring that the stack frame
  // right under it in the trace belongs to HangWatcher for easier attribution.
  NOINLINE static void RecordHang();

  using HangWatchStates =
      std::vector<std::unique_ptr<internal::HangWatchState>>;

  // Used to save a snapshots of the state of hang watching during capture.
  // Only the state of hung threads is retained.
  class BASE_EXPORT WatchStateSnapShot {
   public:
    struct WatchStateCopy {
      base::TimeTicks deadline;
      base::PlatformThreadId thread_id;
    };

    WatchStateSnapShot();
    WatchStateSnapShot(const WatchStateSnapShot& other);
    ~WatchStateSnapShot();

    // Initialize the snapshot from provided data. |snapshot_time| can be
    // different than now() to be coherent with other operations recently done
    // on |watch_states|. |hung_watch_state_copies_| can be empty after
    // initialization for a number of reasons:
    // 1. If any deadline in |watch_states| is before
    // |deadline_ignore_threshold|.
    // 2. If some of the hung threads could not be marked as blocking on
    // capture.
    // 3. If none of the hung threads are of a type configured to trigger a
    // crash dump.
    //
    // This function cannot be called more than once without an associated call
    // to Clear().
    void Init(const HangWatchStates& watch_states,
              base::TimeTicks deadline_ignore_threshold,
              base::TimeDelta monitoring_period);

    // Reset the snapshot object to be reused. Can only be called after Init().
    void Clear();

    // Returns a string that contains the ids of the hung threads separated by a
    // '|'. The size of the string is capped at debug::CrashKeySize::Size256. If
    // no threads are hung returns an empty string. Can only be invoked if
    // IsActionable(). Can only be called after Init().
    std::string PrepareHungThreadListCrashKey() const;

    // Return the highest deadline included in this snapshot. Can only be called
    // if IsActionable(). Can only be called after Init().
    base::TimeTicks GetHighestDeadline() const;

    // Returns true if the snapshot can be used to record an actionable hang
    // report and false if not. Can only be called after Init().
    bool IsActionable() const;

   private:
    bool initialized_ = false;
    std::vector<WatchStateCopy> hung_watch_state_copies_;
  };

  // Return a watch state snapshot taken Now() to be inspected in tests.
  // NO_THREAD_SAFETY_ANALYSIS is needed because the analyzer can't figure out
  // that calls to this function done from |on_hang_closure_| are properly
  // locked.
  WatchStateSnapShot GrabWatchStateSnapshotForTesting() const
      NO_THREAD_SAFETY_ANALYSIS;

  // Inspects the state of all registered threads to check if they are hung and
  // invokes the appropriate closure if so.
  void Monitor() LOCKS_EXCLUDED(watch_state_lock_);

  // Record the hang crash dump and perform the necessary housekeeping before
  // and after.
  void DoDumpWithoutCrashing(const WatchStateSnapShot& watch_state_snapshot)
      EXCLUSIVE_LOCKS_REQUIRED(watch_state_lock_) LOCKS_EXCLUDED(capture_lock_);

  // Stop all monitoring and join the HangWatcher thread.
  void Stop();

  // Wait until it's time to monitor.
  void Wait();

  // Run the loop that periodically monitors the registered thread at a
  // set time interval.
  void Run() override;

  base::TimeDelta monitoring_period_;

  // Use to make the HangWatcher thread wake or sleep to schedule the
  // appropriate monitoring frequency.
  WaitableEvent should_monitor_;

  bool IsWatchListEmpty() LOCKS_EXCLUDED(watch_state_lock_);

  // Stops hang watching on the calling thread by removing the entry from the
  // watch list.
  void UnregisterThread() LOCKS_EXCLUDED(watch_state_lock_);

  Lock watch_state_lock_;

  std::vector<std::unique_ptr<internal::HangWatchState>> watch_states_
      GUARDED_BY(watch_state_lock_);

  // Snapshot to be reused across hang captures. The point of keeping it
  // around is reducing allocations during capture.
  WatchStateSnapShot watch_state_snapshot_
      GUARDED_BY_CONTEXT(hang_watcher_thread_checker_);

  base::DelegateSimpleThread thread_;
  bool thread_started_ = false;

  RepeatingClosure after_monitor_closure_for_testing_;
  RepeatingClosure on_hang_closure_for_testing_;
  RepeatingCallback<void(TimeTicks)> after_wait_callback_;

  base::Lock capture_lock_ ACQUIRED_AFTER(watch_state_lock_);
  std::atomic<bool> capture_in_progress_{false};

  raw_ptr<const base::TickClock> tick_clock_;

  // Registration to receive memory pressure signals.
  base::MemoryPressureListener memory_pressure_listener_;

  // The last time at which a critical memory pressure signal was received, or
  // null if no signal was ever received. Atomic because it's set and read from
  // different threads.
  std::atomic<base::TimeTicks> last_critical_memory_pressure_{
      base::TimeTicks()};

  // The time after which all deadlines in |watch_states_| need to be for a hang
  // to be reported.
  base::TimeTicks deadline_ignore_threshold_;

  FRIEND_TEST_ALL_PREFIXES(HangWatcherTest, NestedScopes);
  FRIEND_TEST_ALL_PREFIXES(HangWatcherSnapshotTest, HungThreadIDs);
  FRIEND_TEST_ALL_PREFIXES(HangWatcherSnapshotTest, NonActionableReport);
};

// Classes here are exposed in the header only for testing. They are not
// intended to be used outside of base.
namespace internal {

// Threadsafe class that manages a deadline of type TimeTicks alongside hang
// watching specific flags. The flags are stored in the higher bits of the
// underlying TimeTicks deadline. This enables setting the flags on thread T1 in
// a way that's resilient to concurrent deadline or flag changes from thread T2.
// Flags can be queried separately from the deadline and users of this class
// should not have to care about them when doing so.
class BASE_EXPORT HangWatchDeadline {
 public:
  // Masks to set flags by flipping a single bit in the TimeTicks value. There
  // are two types of flags. Persistent flags remain set through a deadline
  // change and non-persistent flags are cleared when the deadline changes.
  enum class Flag : uint64_t {
    // Minimum value for validation purposes. Not currently used.
    kMinValue = bits::LeftmostBit<uint64_t>() >> 7,
    // Persistent because if hang detection is disabled on a thread it should
    // be re-enabled manually.
    kIgnoreCurrentWatchHangsInScope = bits::LeftmostBit<uint64_t>() >> 1,
    // Non-persistent because a new value means a new WatchHangsInScope started
    // after the beginning of capture. It can't be implicated in the hang so we
    // don't want it to block.
    kShouldBlockOnHang = bits::LeftmostBit<uint64_t>() >> 0,
    kMaxValue = kShouldBlockOnHang
  };

  HangWatchDeadline();
  ~HangWatchDeadline();

  // HangWatchDeadline should never be copied. To keep a copy of the deadline or
  // flags use the appropriate accessors.
  HangWatchDeadline(const HangWatchDeadline&) = delete;
  HangWatchDeadline& operator=(const HangWatchDeadline&) = delete;

  // Returns the underlying TimeTicks deadline. WARNING: The deadline and flags
  // can change concurrently. To inspect both, use GetFlagsAndDeadline() to get
  // a coherent race-free view of the state.
  TimeTicks GetDeadline() const;

  // Returns a mask containing the flags and the deadline as a pair. Use to
  // inspect the flags and deadline and then optionally call
  // SetShouldBlockOnHang() .
  std::pair<uint64_t, TimeTicks> GetFlagsAndDeadline() const;

  // Returns true if the flag is set and false if not. WARNING: The deadline and
  // flags can change concurrently. To inspect both, use GetFlagsAndDeadline()
  // to get a coherent race-free view of the state.
  bool IsFlagSet(Flag flag) const;

  // Returns true if a flag is set in |flags| and false if not. Use to inspect
  // the flags mask returned by GetFlagsAndDeadline(). WARNING: The deadline and
  // flags can change concurrently. If you need to inspect both you need to use
  // GetFlagsAndDeadline() to get a coherent race-free view of the state.
  static bool IsFlagSet(Flag flag, uint64_t flags);

  // Replace the deadline value. |new_value| needs to be within [0,
  // Max()]. This function can never fail.
  void SetDeadline(TimeTicks new_value);

  // Sets the kShouldBlockOnHang flag and returns true if current flags and
  // deadline are still equal to |old_flags| and  |old_deadline|. Otherwise does
  // not set the flag and returns false.
  bool SetShouldBlockOnHang(uint64_t old_flags, TimeTicks old_deadline);

  // Sets the kIgnoreCurrentWatchHangsInScope flag.
  void SetIgnoreCurrentWatchHangsInScope();

  // Clears the kIgnoreCurrentWatchHangsInScope flag.
  void UnsetIgnoreCurrentWatchHangsInScope();

  // Use to simulate the value of |bits_| changing between the calling a
  // Set* function and the moment of atomically switching the values. The
  // callback should return a value containing the desired flags and deadline
  // bits. The flags that are already set will be preserved upon applying. Use
  // only for testing.
  void SetSwitchBitsClosureForTesting(
      RepeatingCallback<uint64_t(void)> closure);

  // Remove the deadline modification callback for when testing is done. Use
  // only for testing.
  void ResetSwitchBitsClosureForTesting();

 private:
  using TimeTicksInternalRepresentation =
      std::invoke_result<decltype(&TimeTicks::ToInternalValue),
                         TimeTicks>::type;
  static_assert(std::is_same_v<TimeTicksInternalRepresentation, int64_t>,
                "Bit manipulations made by HangWatchDeadline need to be"
                "adapted if internal representation of TimeTicks changes.");

  // Replace the bits with the ones provided through the callback. Preserves the
  // flags that were already set. Returns the switched in bits. Only call if
  // |switch_bits_callback_for_testing_| is installed.
  uint64_t SwitchBitsForTesting();

  // Atomically sets persitent flag |flag|. Cannot fail.
  void SetPersistentFlag(Flag flag);

  // Atomically clears persitent flag |flag|. Cannot fail.
  void ClearPersistentFlag(Flag flag);

  // Converts bits to TimeTicks with some sanity checks. Use to return the
  // deadline outside of this class.
  static TimeTicks DeadlineFromBits(uint64_t bits);

  // Returns the largest representable deadline.
  static TimeTicks Max();

  // Extract the flag bits from |bits|.
  static uint64_t ExtractFlags(uint64_t bits);

  // Extract the deadline bits from |bits|.
  static uint64_t ExtractDeadline(uint64_t bits);

  // BitsType is uint64_t. This type is chosen for having
  // std::atomic<BitsType>{}.is_lock_free() true on many platforms and having no
  // undefined behaviors with regards to bit shift operations. Throughout this
  // class this is the only type that is used to store, retrieve and manipulate
  // the bits. When returning a TimeTicks value outside this class it's
  // necessary to run the proper checks to insure correctness of the conversion
  // that has to go through int_64t. (See DeadlineFromBits()).
  using BitsType = uint64_t;
  static_assert(std::is_same_v<std::underlying_type<Flag>::type, BitsType>,
                "Flag should have the same underlying type as bits_ to "
                "simplify thinking about bit operations");

  // Holds the bits of both the flags and the TimeTicks deadline.
  // TimeTicks values represent a count of microseconds since boot which may or
  // may not include suspend time depending on the platform. Using the seven
  // highest order bits and the sign bit to store flags still enables the
  // storing of TimeTicks values that can represent up to ~1142 years of uptime
  // in the remaining bits. Should never be directly accessed from outside the
  // class. Starts out at Max() to provide a base-line deadline that will not be
  // reached during normal execution.
  //
  // Binary format: 0xFFDDDDDDDDDDDDDDDD
  // F = Flags
  // D = Deadline
  std::atomic<BitsType> bits_{static_cast<uint64_t>(Max().ToInternalValue())};

  RepeatingCallback<uint64_t(void)> switch_bits_callback_for_testing_;

  THREAD_CHECKER(thread_checker_);

  FRIEND_TEST_ALL_PREFIXES(HangWatchDeadlineTest, BitsPreservedThroughExtract);
};

// Contains the information necessary for hang watching a specific
// thread. Instances of this class are accessed concurrently by the associated
// thread and the HangWatcher. The HangWatcher owns instances of this
// class and outside of it they are accessed through
// GetHangWatchStateForCurrentThread().
class BASE_EXPORT HangWatchState {
 public:
  // |thread_type| is the type of thread the watch state will
  // be associated with. It's the responsibility of the creating
  // code to choose the correct type.
  explicit HangWatchState(HangWatcher::ThreadType thread_type);
  ~HangWatchState();

  HangWatchState(const HangWatchState&) = delete;
  HangWatchState& operator=(const HangWatchState&) = delete;

  // Allocates a new state object bound to the calling thread and returns an
  // owning pointer to it.
  static std::unique_ptr<HangWatchState> CreateHangWatchStateForCurrentThread(
      HangWatcher::ThreadType thread_type);

  // Retrieves the hang watch state associated with the calling thread.
  // Returns nullptr if no HangWatchState exists for the current thread (see
  // CreateHangWatchStateForCurrentThread()).
  static HangWatchState* GetHangWatchStateForCurrentThread();

  // Returns the current deadline. Use this function if you need to
  // store the value. To test if the deadline has expired use IsOverDeadline().
  // WARNING: The deadline and flags can change concurrently. If you need to
  // inspect both you need to use GetFlagsAndDeadline() to get a coherent
  // race-free view of the state.
  TimeTicks GetDeadline() const;

  // Returns a mask containing the hang watching flags and the value as a pair.
  // Use to inspect the flags and deadline and optionally call
  // SetShouldBlockOnHang(flags, deadline).
  std::pair<uint64_t, TimeTicks> GetFlagsAndDeadline() const;

  // Sets the deadline to a new value.
  void SetDeadline(TimeTicks deadline);

  // Mark this thread as ignored for hang watching. This means existing
  // WatchHangsInScope will not trigger hangs.
  void SetIgnoreCurrentWatchHangsInScope();

  // Reactivate hang watching on this thread. Should be called when all
  // WatchHangsInScope instances that were ignored have completed.
  void UnsetIgnoreCurrentWatchHangsInScope();

  // Mark the current state as having to block in its destruction until hang
  // capture completes.
  bool SetShouldBlockOnHang(uint64_t old_flags, TimeTicks old_deadline);

  // Returns true if |flag| is set and false if not. WARNING: The deadline and
  // flags can change concurrently. If you need to inspect both you need to use
  // GetFlagsAndDeadline() to get a coherent race-free view of the state.
  bool IsFlagSet(HangWatchDeadline::Flag flag);

  // Tests whether the associated thread's execution has gone over the deadline.
  bool IsOverDeadline() const;

#if DCHECK_IS_ON()
  // Saves the supplied WatchHangsInScope as the currently active
  // WatchHangsInScope.
  void SetCurrentWatchHangsInScope(WatchHangsInScope* scope);

  // Retrieve the currently active scope.
  WatchHangsInScope* GetCurrentWatchHangsInScope();
#endif

  PlatformThreadId GetThreadID() const;

  // Retrieve the current hang watch deadline directly. For testing only.
  HangWatchDeadline* GetHangWatchDeadlineForTesting();

  // Returns the current nesting level.
  int nesting_level() { return nesting_level_; }

  // Increase the nesting level by 1;
  void IncrementNestingLevel();

  // Reduce the nesting level by 1;
  void DecrementNestingLevel();

  // Returns the type of the thread under watch.
  HangWatcher::ThreadType thread_type() const { return thread_type_; }

 private:
  // The thread that creates the instance should be the class that updates
  // the deadline.
  THREAD_CHECKER(thread_checker_);

  const AutoReset<HangWatchState*> resetter_;

  // If the deadline fails to be updated before TimeTicks::Now() ever
  // reaches the value contained in it this constistutes a hang.
  HangWatchDeadline deadline_;

  // A unique ID of the thread under watch. Used for logging in crash reports
  // only.
  PlatformThreadId thread_id_ = kInvalidThreadId;

  // Number of active HangWatchScopeEnables on this thread.
  int nesting_level_ = 0;

  // The type of the thread under watch.
  const HangWatcher::ThreadType thread_type_;

#if DCHECK_IS_ON()
  // Used to keep track of the current WatchHangsInScope and detect improper
  // usage. Scopes should always be destructed in reverse order from the one
  // they were constructed in. Example of improper use:
  //
  // {
  //   std::unique_ptr<Scope> scope = std::make_unique<Scope>(...);
  //   Scope other_scope;
  //   |scope| gets deallocated first, violating reverse destruction order.
  //   scope.reset();
  // }
  raw_ptr<WatchHangsInScope> current_watch_hangs_in_scope_{nullptr};
#endif
};

}  // namespace internal
}  // namespace base

#endif  // BASE_THREADING_HANG_WATCHER_H_
