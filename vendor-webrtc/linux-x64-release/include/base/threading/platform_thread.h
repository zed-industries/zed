// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// WARNING: You should *NOT* be using this class directly.  PlatformThread is
// the low-level platform-specific abstraction to the OS's threading interface.
// You should instead be using a message-loop driven Thread, see thread.h.

#ifndef BASE_THREADING_PLATFORM_THREAD_H_
#define BASE_THREADING_PLATFORM_THREAD_H_

#include <stddef.h>

#include <iosfwd>
#include <limits>
#include <optional>
#include <type_traits>

#include "base/base_export.h"
#include "base/message_loop/message_pump_type.h"
#include "base/process/process_handle.h"
#include "base/threading/platform_thread_ref.h"
#include "base/time/time.h"
#include "base/trace_event/base_tracing_forward.h"
#include "base/types/strong_alias.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_WIN)
#include "base/win/windows_types.h"
#elif BUILDFLAG(IS_FUCHSIA)
#include <zircon/types.h>
#elif BUILDFLAG(IS_POSIX)
#include <pthread.h>
#include <unistd.h>
#endif

#if BUILDFLAG(IS_CHROMEOS)
#include "base/feature_list.h"
#endif

namespace base {

// Used for uniquely identifying a thread.
//
// Wraps a platform-specific integer value with platform-specific size,
// guaranteed to have a maximum bitness of 64-bit. Getting a 32-bit value is
// possible only if we either know the platform-specific size (because we're in
// platform-specific code), or if we are ok with truncation of the value (e.g.
// because we are logging and the occasional false match is not catastrophic).
class BASE_EXPORT PlatformThreadId {
 public:
#if BUILDFLAG(IS_WIN)
  using UnderlyingType = DWORD;
#elif BUILDFLAG(IS_FUCHSIA)
  using UnderlyingType = zx_koid_t;
#elif BUILDFLAG(IS_APPLE)
  using UnderlyingType = uint64_t;
#elif BUILDFLAG(IS_POSIX)
  using UnderlyingType = pid_t;
#endif
  static_assert(std::is_integral_v<UnderlyingType>, "Always an integer value.");

  constexpr PlatformThreadId() = default;

  // Special templated constructor which prevents implicit conversion of the
  // integer argument.
  template <typename T>
  explicit constexpr PlatformThreadId(T value)
    requires(std::is_same_v<T, UnderlyingType>)
      : value_(value) {}

  static constexpr PlatformThreadId ForTest(int value) {
    return PlatformThreadId(static_cast<UnderlyingType>(value));
  }

  // Allow conversion to u/int64_t, whether the underlying type is signed or
  // not, and whether it is 32-bit or 64-bit.
  explicit constexpr operator uint64_t() const {
    static_assert(sizeof(uint64_t) >= sizeof(UnderlyingType));
    return static_cast<uint64_t>(value_);
  }
  explicit constexpr operator int64_t() const {
    static_assert(sizeof(int64_t) >= sizeof(UnderlyingType));
    return static_cast<int64_t>(value_);
  }
  // Forbid conversion to u/int32_t, since we might have a 64-bit
  // value -- use truncate_to_int32_for_display_only() or raw() instead.
  explicit constexpr operator uint32_t() const = delete;
  explicit constexpr operator int32_t() const = delete;

  // Truncating getter for an int32 representation of the id.
  //
  // AVOID: This should only be used in cases where truncation is not
  // catastrophic, e.g. displaying the thread id in traces or logs. It will
  // always be preferable to display the full, untruncated thread id.
  constexpr int32_t truncate_to_int32_for_display_only() const {
    return static_cast<int32_t>(value_);
  }

  // Getter for the underlying raw value. Should only be used when
  // exposing the UnderlyingType, e.g. passing into system APIs or passing into
  // functions overloaded on different integer sizes like NumberToString.
  constexpr UnderlyingType raw() const { return value_; }

  constexpr friend auto operator<=>(const PlatformThreadId& lhs,
                                    const PlatformThreadId& rhs) = default;
  constexpr friend bool operator==(const PlatformThreadId& lhs,
                                   const PlatformThreadId& rhs) = default;

  // Allow serialising into a trace.
  void WriteIntoTrace(perfetto::TracedValue&& context) const;

 private:
  // TODO(crbug.com/393384253): Use a system-specific invalid value, which might
  // be 0, -1, or some other value from a system header.
  UnderlyingType value_ = 0;
};

inline std::ostream& operator<<(std::ostream& stream,
                                const PlatformThreadId& id) {
  return stream << id.raw();
}

// Used to operate on threads.
class PlatformThreadHandle {
 public:
#if BUILDFLAG(IS_WIN)
  typedef void* Handle;
#elif BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)
  typedef pthread_t Handle;
#endif

  constexpr PlatformThreadHandle() : handle_(0) {}

  explicit constexpr PlatformThreadHandle(Handle handle) : handle_(handle) {}

  bool is_equal(const PlatformThreadHandle& other) const {
    return handle_ == other.handle_;
  }

  bool is_null() const { return !handle_; }

  Handle platform_handle() const { return handle_; }

 private:
  Handle handle_;
};

static constexpr PlatformThreadId kInvalidThreadId = PlatformThreadId();

// Valid values for `thread_type` of Thread::Options, SimpleThread::Options,
// and SetCurrentThreadType(), listed in increasing order of importance.
//
// It is up to each platform-specific implementation what these translate to.
// Callers should avoid setting different ThreadTypes on different platforms
// (ifdefs) at all cost, instead the platform differences should be encoded in
// the platform-specific implementations. Some implementations may treat
// adjacent ThreadTypes in this enum as equivalent.
//
// Reach out to //base/task/OWNERS (scheduler-dev@chromium.org) before changing
// thread type assignments in your component, as such decisions affect the whole
// of Chrome.
//
// Refer to PlatformThreadTest.SetCurrentThreadTypeTest in
// platform_thread_unittest.cc for the most up-to-date state of each platform's
// handling of ThreadType.
enum class ThreadType : int {
  // Suitable for threads that have the least urgency and lowest priority, and
  // can be interrupted or delayed by other types.
  kBackground,
  // Suitable for threads that are less important than normal type, and can be
  // interrupted or delayed by threads with kDefault type.
  kUtility,
  // Suitable for threads that produce user-visible artifacts but aren't
  // latency sensitive. The underlying platform will try to be economic
  // in its usage of resources for this thread, if possible.
  kResourceEfficient,
  // Default type. The thread priority or quality of service will be set to
  // platform default. In Chrome, this is suitable for handling user
  // interactions (input), only display and audio can get a higher priority.
  kDefault,
  // Suitable for display critical threads, ie. threads critical to compositing
  // and presenting the foreground content.
  kDisplayCritical,
  // Suitable for low-latency, glitch-resistant audio.
  kRealtimeAudio,
  kMaxValue = kRealtimeAudio,
};

// Cross-platform mapping of physical thread priorities. Used by tests to verify
// the underlying effects of SetCurrentThreadType.
enum class ThreadPriorityForTest : int {
  kBackground,
  kUtility,
  kResourceEfficient,
  kNormal,
  kCompositing,
  kDisplay,
  kRealtimeAudio,
  kMaxValue = kRealtimeAudio,
};

// A namespace for low-level thread functions.
class BASE_EXPORT PlatformThreadBase {
 public:
  // Implement this interface to run code on a background thread.  Your
  // ThreadMain method will be called on the newly created thread.
  class BASE_EXPORT Delegate {
   public:
    virtual void ThreadMain() = 0;

#if BUILDFLAG(IS_APPLE)
    // TODO: Move this to the PlatformThreadApple class.
    // The interval at which the thread expects to have work to do. Zero if
    // unknown. (Example: audio buffer duration for real-time audio.) Is used to
    // optimize the thread real-time behavior. Is called on the newly created
    // thread before ThreadMain().
    virtual TimeDelta GetRealtimePeriod();
#endif

   protected:
    virtual ~Delegate() = default;
  };

  PlatformThreadBase() = delete;
  PlatformThreadBase(const PlatformThreadBase&) = delete;
  PlatformThreadBase& operator=(const PlatformThreadBase&) = delete;

  // Gets the current thread id, which may be useful for logging purposes.
  static PlatformThreadId CurrentId();

  // Gets the current thread reference, which can be used to check if
  // we're on the right thread quickly.
  static PlatformThreadRef CurrentRef();

  // Get the handle representing the current thread. On Windows, this is a
  // pseudo handle constant which will always represent the thread using it and
  // hence should not be shared with other threads nor be used to differentiate
  // the current thread from another.
  static PlatformThreadHandle CurrentHandle();

  // Yield the current thread so another thread can be scheduled.
  //
  // Note: this is likely not the right call to make in most situations. If this
  // is part of a spin loop, consider base::Lock, which likely has better tail
  // latency. Yielding the thread has different effects depending on the
  // platform, system load, etc., and can result in yielding the CPU for less
  // than 1us, or many tens of ms.
  static void YieldCurrentThread();

  // Sleeps for the specified duration (real-time; ignores time overrides).
  // Note: The sleep duration may be in base::Time or base::TimeTicks, depending
  // on platform. If you're looking to use this in unit tests testing delayed
  // tasks, this will be unreliable - instead, use
  // base::test::TaskEnvironment with MOCK_TIME mode.
  static void Sleep(base::TimeDelta duration);

  // Sets the thread name visible to debuggers/tools. This will try to
  // initialize the context for current thread unless it's a WorkerThread.
  static void SetName(const std::string& name);

  // Gets the thread name, if previously set by SetName.
  static const char* GetName();

  // Creates a new thread.  The `stack_size` parameter can be 0 to indicate
  // that the default stack size should be used.  Upon success,
  // `*thread_handle` will be assigned a handle to the newly created thread,
  // and `delegate`'s ThreadMain method will be executed on the newly created
  // thread.
  // NOTE: When you are done with the thread handle, you must call Join to
  // release system resources associated with the thread.  You must ensure that
  // the Delegate object outlives the thread.
  static bool Create(size_t stack_size,
                     Delegate* delegate,
                     PlatformThreadHandle* thread_handle) {
    return CreateWithType(stack_size, delegate, thread_handle,
                          ThreadType::kDefault);
  }

  // CreateWithType() does the same thing as Create() except the priority and
  // possibly the QoS of the thread is set based on `thread_type`.
  // `pump_type_hint` must be provided if the thread will be using a
  // MessagePumpForUI or MessagePumpForIO as this affects the application of
  // `thread_type`.
  static bool CreateWithType(
      size_t stack_size,
      Delegate* delegate,
      PlatformThreadHandle* thread_handle,
      ThreadType thread_type,
      MessagePumpType pump_type_hint = MessagePumpType::DEFAULT);

  // CreateNonJoinable() does the same thing as Create() except the thread
  // cannot be Join()'d.  Therefore, it also does not output a
  // PlatformThreadHandle.
  static bool CreateNonJoinable(size_t stack_size, Delegate* delegate);

  // CreateNonJoinableWithType() does the same thing as CreateNonJoinable()
  // except the type of the thread is set based on `type`. `pump_type_hint` must
  // be provided if the thread will be using a MessagePumpForUI or
  // MessagePumpForIO as this affects the application of `thread_type`.
  static bool CreateNonJoinableWithType(
      size_t stack_size,
      Delegate* delegate,
      ThreadType thread_type,
      MessagePumpType pump_type_hint = MessagePumpType::DEFAULT);

  // Joins with a thread created via the Create function.  This function blocks
  // the caller until the designated thread exits.  This will invalidate
  // `thread_handle`.
  static void Join(PlatformThreadHandle thread_handle);

  // Detaches and releases the thread handle. The thread is no longer joinable
  // and `thread_handle` is invalidated after this call.
  static void Detach(PlatformThreadHandle thread_handle);

  // Returns true if SetCurrentThreadType() should be able to change the type
  // of a thread in current process from `from` to `to`.
  static bool CanChangeThreadType(ThreadType from, ThreadType to);

  // Declares the type of work running on the current thread. This will affect
  // things like thread priority and thread QoS (Quality of Service) to the best
  // of the current platform's abilities.
  static void SetCurrentThreadType(ThreadType thread_type);

  // Get the last `thread_type` set by SetCurrentThreadType, no matter if the
  // underlying priority successfully changed or not.
  static ThreadType GetCurrentThreadType();

  // Returns a realtime period provided by `delegate`.
  static TimeDelta GetRealtimePeriod(Delegate* delegate);

  // Returns the override of task leeway if any.
  static std::optional<TimeDelta> GetThreadLeewayOverride();

  // Returns the default thread stack size set by chrome. If we do not
  // explicitly set default size then returns 0.
  static size_t GetDefaultThreadStackSize();

  static ThreadPriorityForTest GetCurrentThreadPriorityForTest();

 protected:
  static void SetNameCommon(const std::string& name);
};

#if BUILDFLAG(IS_APPLE)
class BASE_EXPORT PlatformThreadApple : public PlatformThreadBase {
 public:
  // Stores the period value in TLS.
  static void SetCurrentThreadRealtimePeriodValue(TimeDelta realtime_period);

  static TimeDelta GetCurrentThreadRealtimePeriodForTest();

  // Initializes features for this class. See `base::features::Init()`.
  static void InitializeFeatures();
};
#endif  // BUILDFLAG(IS_APPLE)

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)
class ThreadTypeDelegate;
using IsViaIPC = base::StrongAlias<class IsViaIPCTag, bool>;

class BASE_EXPORT PlatformThreadLinux : public PlatformThreadBase {
 public:
  static constexpr struct sched_param kRealTimeAudioPrio = {8};
  static constexpr struct sched_param kRealTimeDisplayPrio = {6};

  // Sets a delegate which handles thread type changes for this process. This
  // must be externally synchronized with any call to SetCurrentThreadType.
  static void SetThreadTypeDelegate(ThreadTypeDelegate* delegate);

  // Toggles a specific thread's type at runtime. This can be used to
  // change the priority of a thread in a different process and will fail
  // if the calling process does not have proper permissions. The
  // SetCurrentThreadType() function above is preferred in favor of
  // security but on platforms where sandboxed processes are not allowed to
  // change priority this function exists to allow a non-sandboxed process
  // to change the priority of sandboxed threads for improved performance.
  // Warning: Don't use this for a main thread because that will change the
  // whole thread group's (i.e. process) priority.
  static void SetThreadType(ProcessId process_id,
                            PlatformThreadId thread_id,
                            ThreadType thread_type,
                            IsViaIPC via_ipc);

  // For a given thread id and thread type, setup the cpuset and schedtune
  // CGroups for the thread.
  static void SetThreadCgroupsForThreadType(PlatformThreadId thread_id,
                                            ThreadType thread_type);

  // Determine if thread_id is a background thread by looking up whether
  // it is in the urgent or non-urgent cpuset
  static bool IsThreadBackgroundedForTest(PlatformThreadId thread_id);
};
#endif  // BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)

#if BUILDFLAG(IS_CHROMEOS)
BASE_EXPORT BASE_DECLARE_FEATURE(kSetRtForDisplayThreads);

class CrossProcessPlatformThreadDelegate;

class BASE_EXPORT PlatformThreadChromeOS : public PlatformThreadLinux {
 public:
  // Sets a delegate which handles thread type changes for threads of another
  // process. This must be externally synchronized with any call to
  // SetCurrentThreadType.
  static void SetCrossProcessPlatformThreadDelegate(
      CrossProcessPlatformThreadDelegate* delegate);

  // Initializes features for this class. See `base::features::Init()`.
  static void InitializeFeatures();

  // Toggles a specific thread's type at runtime. This is the ChromeOS-specific
  // version and includes Linux's functionality but does slightly more. See
  // PlatformThreadLinux's SetThreadType() header comment for Linux details.
  static void SetThreadType(ProcessId process_id,
                            PlatformThreadId thread_id,
                            ThreadType thread_type,
                            IsViaIPC via_ipc);

  // Returns true if the feature for backgrounding of threads is enabled.
  static bool IsThreadsBgFeatureEnabled();

  // Returns true if the feature for setting display threads to RT is enabled.
  static bool IsDisplayThreadsRtFeatureEnabled();

  // Set a specific thread as backgrounded. This is called when the process
  // moves to and from the background and changes have to be made to each of its
  // thread's scheduling attributes.
  static void SetThreadBackgrounded(ProcessId process_id,
                                    PlatformThreadId thread_id,
                                    bool backgrounded);

  // Returns the thread type of a thread given its thread id.
  static std::optional<ThreadType> GetThreadTypeFromThreadId(
      ProcessId process_id,
      PlatformThreadId thread_id);

  // DCHECKs that the caller is on the correct sequence to perform cross-process
  // priority changes without races.
  //
  // This does not simply return a `SequenceChecker&` and let the caller do the
  // check, because doing so requires an `#include` of sequence_checker.h (since
  // `SequenceChecker` is an alias rather than a forward-declarable class),
  // which complicates life for other base/ headers trying to avoid circular
  // dependencies.
  static void DcheckCrossProcessThreadPrioritySequence();
};
#endif  // BUILDFLAG(IS_CHROMEOS)

// Alias to the correct platform-specific class based on preprocessor directives
#if BUILDFLAG(IS_APPLE)
using PlatformThread = PlatformThreadApple;
#elif BUILDFLAG(IS_CHROMEOS)
using PlatformThread = PlatformThreadChromeOS;
#elif BUILDFLAG(IS_LINUX)
using PlatformThread = PlatformThreadLinux;
#else
using PlatformThread = PlatformThreadBase;
#endif

namespace internal {

void SetCurrentThreadType(ThreadType thread_type,
                          MessagePumpType pump_type_hint);

void SetCurrentThreadTypeImpl(ThreadType thread_type,
                              MessagePumpType pump_type_hint);

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)
void SetThreadTypeLinux(ProcessId process_id,
                        PlatformThreadId thread_id,
                        ThreadType thread_type,
                        IsViaIPC via_ipc);
#endif
#if BUILDFLAG(IS_CHROMEOS)
void SetThreadTypeChromeOS(ProcessId process_id,
                           PlatformThreadId thread_id,
                           ThreadType thread_type,
                           IsViaIPC via_ipc);
#endif
#if BUILDFLAG(IS_CHROMEOS)
inline constexpr auto SetThreadType = SetThreadTypeChromeOS;
#elif BUILDFLAG(IS_LINUX)
inline constexpr auto SetThreadType = SetThreadTypeLinux;
#endif

}  // namespace internal

}  // namespace base

#endif  // BASE_THREADING_PLATFORM_THREAD_H_
