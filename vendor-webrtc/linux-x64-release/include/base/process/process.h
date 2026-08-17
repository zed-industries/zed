// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROCESS_PROCESS_H_
#define BASE_PROCESS_PROCESS_H_

#include <string_view>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/process/process_handle.h"
#include "base/time/time.h"
#include "build/blink_buildflags.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_WIN)
#include "base/win/scoped_handle.h"
#endif

#if BUILDFLAG(IS_FUCHSIA)
#include <lib/zx/process.h>
#endif

#if BUILDFLAG(IS_APPLE) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_WIN)
#include "base/feature_list.h"
#endif  // BUILDFLAG(IS_APPLE) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_WIN)

#if BUILDFLAG(IS_APPLE)
#include "base/process/port_provider_mac.h"
#endif  // BUILDFLAG(IS_APPLE)

namespace base {

#if BUILDFLAG(IS_CHROMEOS)
// OneGroupPerRenderer feature places each foreground renderer process into
// its own cgroup. This will cause the scheduler to use the aggregate runtime
// of all threads in the process when deciding on the next thread to schedule.
// It will help guarantee fairness between renderers.
BASE_EXPORT BASE_DECLARE_FEATURE(kOneGroupPerRenderer);

// Set all threads of a background process as backgrounded, which changes the
// thread attributes including c-group, latency sensitivity. But the nice value
// is unchanged, since background process is under the spell of the background
// CPU c-group (via cgroup.procs).
BASE_EXPORT BASE_DECLARE_FEATURE(kSetThreadBgForBgProcess);

// FlattenCpuCgroups feature uses /sys/fs/cgroup/cpu/chrome_renderers and
// /sys/fs/cgroup/cpu/chrome_renderers_background cpu cgroups for renderer
// processes instead of nested cpu cgroups. Nested cpu cgroups has an overhead
// on task scheduling.
BASE_EXPORT BASE_DECLARE_FEATURE(kFlattenCpuCgroups);

class ProcessPriorityDelegate;
#endif

// Provides a move-only encapsulation of a process.
//
// This object is not tied to the lifetime of the underlying process: the
// process may be killed and this object may still around, and it will still
// claim to be valid. The actual behavior in that case is OS dependent like so:
//
// Windows: The underlying ProcessHandle will be valid after the process dies
// and can be used to gather some information about that process, but most
// methods will obviously fail.
//
// POSIX: The underlying ProcessHandle is not guaranteed to remain valid after
// the process dies, and it may be reused by the system, which means that it may
// end up pointing to the wrong process.
class BASE_EXPORT Process {
 public:
  // On Windows, this takes ownership of |handle|. On POSIX, this does not take
  // ownership of |handle|.
  explicit Process(ProcessHandle handle = kNullProcessHandle);

  Process(Process&& other);

  Process(const Process&) = delete;
  Process& operator=(const Process&) = delete;

  // The destructor does not terminate the process.
  ~Process();

  Process& operator=(Process&& other);

  // Returns an object for the current process.
  static Process Current();

  // Returns a Process for the given |pid|.
  static Process Open(ProcessId pid);

  // Returns a Process for the given |pid|. On Windows the handle is opened
  // with more access rights and must only be used by trusted code (can read the
  // address space and duplicate handles).
  static Process OpenWithExtraPrivileges(ProcessId pid);

#if BUILDFLAG(IS_WIN)
  // Returns a Process for the given |pid|, using some |desired_access|.
  // See ::OpenProcess documentation for valid |desired_access|.
  static Process OpenWithAccess(ProcessId pid, DWORD desired_access);
#endif

  // Returns true if changing the priority of processes through `SetPriority()`
  // is possible.
  static bool CanSetPriority();

  // Terminates the current process immediately with |exit_code|.
  [[noreturn]] static void TerminateCurrentProcessImmediately(int exit_code);

  // Returns true if this objects represents a valid process.
  bool IsValid() const;

  // Returns a handle for this process. There is no guarantee about when that
  // handle becomes invalid because this object retains ownership.
  ProcessHandle Handle() const;

  // Returns a second object that represents this process.
  Process Duplicate() const;

  // Relinquishes ownership of the handle and sets this to kNullProcessHandle.
  // The result may be a pseudo-handle, depending on the OS and value stored in
  // this.
  [[nodiscard]] ProcessHandle Release();

  // Get the PID for this process.
  ProcessId Pid() const;

  // Get the creation time for this process. Since the Pid can be reused after a
  // process dies, it is useful to use both the Pid and the creation time to
  // uniquely identify a process.
  //
  // On Android, works only if |this| is the current process, as security
  // features prevent an application from getting data about other processes,
  // even if they belong to us. Otherwise, returns Time().
  Time CreationTime() const;

  // Returns true if this process is the current process.
  bool is_current() const;

#if BUILDFLAG(IS_CHROMEOS)
  // A unique token generated for each process, this is used to create a unique
  // cgroup for each renderer.
  const std::string& unique_token() const LIFETIME_BOUND {
    return unique_token_;
  }
#endif

  // Close the process handle. This will not terminate the process.
  void Close();

  // Returns true if this process is still running. This is only safe on Windows
  // (and maybe Fuchsia?), because the ProcessHandle will keep the zombie
  // process information available until itself has been released. But on Posix,
  // the OS may reuse the ProcessId.
#if BUILDFLAG(IS_WIN)
  bool IsRunning() const {
    return !WaitForExitWithTimeout(base::TimeDelta(), nullptr);
  }
#endif

  // Terminates the process with extreme prejudice. The given |exit_code| will
  // be the exit code of the process. If |wait| is true, this method will wait
  // for up to one minute for the process to actually terminate.
  // Returns true if the process terminates within the allowed time.
  // NOTE: |exit_code| is only used on OS_WIN.
  bool Terminate(int exit_code, bool wait) const;

#if BUILDFLAG(IS_WIN)
  enum class WaitExitStatus {
    PROCESS_EXITED,
    STOP_EVENT_SIGNALED,
    FAILED,
  };

  // Waits for the process to exit, or the specified |stop_event_handle| to be
  // set. Returns value indicating which event was set. The given |exit_code|
  // will be the exit code of the process.
  WaitExitStatus WaitForExitOrEvent(
      const base::win::ScopedHandle& stop_event_handle,
      int* exit_code) const;
#endif  // BUILDFLAG(IS_WIN)

  // Waits for the process to exit. Returns true on success.
  // On POSIX, if the process has been signaled then |exit_code| is set to -1.
  // On Linux this must be a child process, however on Mac and Windows it can be
  // any process.
  // NOTE: |exit_code| is optional, nullptr can be passed if the exit code is
  // not required.
  bool WaitForExit(int* exit_code) const;

  // Same as WaitForExit() but only waits for up to |timeout|.
  // NOTE: |exit_code| is optional, nullptr can be passed if the exit code
  // is not required.
  bool WaitForExitWithTimeout(TimeDelta timeout, int* exit_code) const;

  // Indicates that the process has exited with the specified |exit_code|.
  // This should be called if process exit is observed outside of this class.
  // (i.e. Not because Terminate or WaitForExit, above, was called.)
  // Note that nothing prevents this being called multiple times for a dead
  // process though that should be avoided.
  void Exited(int exit_code) const;

  // The different priorities that a process can have.
  // TODO(pmonette): Consider merging with base::TaskPriority when the API is
  //                 stable.
  enum class Priority {
    // The process does not contribute to content that is currently important
    // to the user. Lowest priority.
    kBestEffort,

    // The process contributes to content that is visible to the user, but the
    // work don't have significant performance or latency requirement, so it can
    // run in energy efficient manner. Moderate priority.
    kUserVisible,

    // The process contributes to content that is of the utmost importance to
    // the user, like producing audible content, or visible content in the
    // main frame. High priority.
    kUserBlocking,

    kMaxValue = kUserBlocking,
  };

#if (BUILDFLAG(IS_MAC) || (BUILDFLAG(IS_IOS) && BUILDFLAG(USE_BLINK))) && \
    !BUILDFLAG(IS_IOS_TVOS)
  // The Mac needs a Mach port in order to manipulate a process's priority,
  // and there's no good way to get that from base given the pid. These Mac
  // variants of the `GetPriority()` and `SetPriority()` API take a port
  // provider for this reason. See crbug.com/460102.

  // Retrieves the priority of the process. Defaults to Priority::kUserBlocking
  // if the priority could not be retrieved, or if `port_provider` is null.
  Priority GetPriority(PortProvider* port_provider) const;

  // Sets the priority of the process process. Returns true if the priority was
  // changed, false otherwise. If `port_provider` is null, this is a no-op and
  // it returns false.
  bool SetPriority(PortProvider* port_provider, Priority priority);
#else
  // Retrieves the priority of the process. Defaults to Priority::kUserBlocking
  // if the priority could not be retrieved.
  Priority GetPriority() const;

  // Sets the priority of the process process. Returns true if the priority was
  // changed, false otherwise.
  bool SetPriority(Priority priority);
#endif  // BUILDFLAG(IS_MAC) || (BUILDFLAG(IS_IOS) && BUILDFLAG(USE_BLINK))

  // Returns an integer representing the priority of a process. The meaning
  // of this value is OS dependent.
  int GetOSPriority() const;

#if BUILDFLAG(IS_CHROMEOS)
  // Get the PID in its PID namespace.
  // If the process is not in a PID namespace or /proc/<pid>/status does not
  // report NSpid, kNullProcessId is returned.
  ProcessId GetPidInNamespace() const;
#endif

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)
  // Returns true if the process has any seccomp policy applied.
  bool IsSeccompSandboxed();
#endif  // BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)

#if BUILDFLAG(IS_CHROMEOS)
  // Sets a delegate which handles process priority changes. This
  // must be externally synchronized with any call to base::Process methods.
  static void SetProcessPriorityDelegate(ProcessPriorityDelegate* delegate);

  // Exposes OneGroupPerRendererEnabled() to unit tests.
  static bool OneGroupPerRendererEnabledForTesting();

  // If OneGroupPerRenderer is enabled, runs at process startup to clean up
  // any stale cgroups that were left behind from any unclean exits of the
  // browser process.
  static void CleanUpStaleProcessStates();

  // Initializes the process's priority.
  //
  // This should be called before SetPriority().
  //
  // If SchedQoSOnResourcedForChrome is enabled, this creates a cache entry for
  // the process priority. The returned `base::Process::PriorityEntry` should be
  // freed when the process is terminated so that the cached entry is freed from
  // the internal map.
  //
  // If OneGroupPerRenderer is enabled, it also creates a unique cgroup for the
  // process.
  // This is a no-op if the Process is not valid or if it has already been
  // called.
  void InitializePriority();

  // Clears the entities initialized by InitializePriority().
  //
  // This is no-op if SchedQoSOnResourcedForChrome is disabled.
  void ForgetPriority();
#endif  // BUILDFLAG(IS_CHROMEOS)

#if BUILDFLAG(IS_APPLE)
  // Sets the priority of the current process to its default value.
  // Ironically, non-App Nap compliant processes do not get this by default on
  // Mac.
  static void SetCurrentTaskDefaultRole();
#endif  // BUILDFLAG(IS_MAC)

#if BUILDFLAG(IS_IOS) && BUILDFLAG(USE_BLINK)
  using TerminateCallback = bool (*)(ProcessHandle handle);
  using WaitForExitCallback = bool (*)(ProcessHandle handle,
                                       int* exit_code,
                                       base::TimeDelta timeout);
  // Function ptrs to implement termination without polluting //base with
  // BrowserEngineKit APIs.
  static void SetTerminationHooks(TerminateCallback terminate_callback,
                                  WaitForExitCallback wait_callback);
#if TARGET_OS_SIMULATOR
  // Methods for supporting both "content processes" and traditional
  // forked processes. For non-simulator builds on iOS every process would
  // be a "content process" so we don't need the conditionals.
  void SetIsContentProcess();
  bool IsContentProcess() const;
#endif
#endif

 private:
#if BUILDFLAG(IS_CHROMEOS)
  // Cleans up process state. If OneGroupPerRenderer is enabled, it cleans up
  // the cgroup created by InitializePriority(). If the process has not
  // fully terminated yet, it will post a background task to try again.
  void CleanUpProcess(int remaining_retries) const;

  // Calls CleanUpProcess() on a background thread.
  void CleanUpProcessAsync() const;

  // Used to call CleanUpProcess() on a background thread because Process is not
  // refcounted.
  static void CleanUpProcessScheduled(Process process, int remaining_retries);
#endif  // BUILDFLAG(IS_CHROMEOS)

#if !BUILDFLAG(IS_IOS) || (BUILDFLAG(IS_IOS) && TARGET_OS_SIMULATOR)
  bool TerminateInternal(int exit_code, bool wait) const;
  bool WaitForExitWithTimeoutImpl(base::ProcessHandle handle,
                                  int* exit_code,
                                  base::TimeDelta timeout) const;
#endif

#if BUILDFLAG(IS_WIN)
  win::ScopedHandle process_;
#elif BUILDFLAG(IS_FUCHSIA)
  zx::process process_;
#else
  ProcessHandle process_;
#endif

#if BUILDFLAG(IS_WIN) || BUILDFLAG(IS_FUCHSIA)
  bool is_current_process_;
#endif

#if BUILDFLAG(IS_IOS) && BUILDFLAG(USE_BLINK) && TARGET_OS_SIMULATOR
  // A flag indicating that this is a "content process". iOS does not support
  // generic process invocation but it does support some types of well defined
  // processes. These types of processes are defined at the //content layer so
  // for termination we defer to some globally initialized callbacks.
  bool content_process_ = false;
#endif

#if BUILDFLAG(IS_CHROMEOS)
  // A unique token per process not per class instance (`base::Process`). This
  // is similar to the PID of a process but should not be reused after the
  // process's termination. The token will be copied during Duplicate()
  // and move semantics as is the PID/ProcessHandle.
  std::string unique_token_;
#endif
};

#if BUILDFLAG(IS_CHROMEOS)
// Exposed for testing.
// Given the contents of the /proc/<pid>/cgroup file, determine whether the
// process is backgrounded or not.
BASE_EXPORT Process::Priority GetProcessPriorityCGroup(
    std::string_view cgroup_contents);
#endif  // BUILDFLAG(IS_CHROMEOS)

}  // namespace base

#endif  // BASE_PROCESS_PROCESS_H_
