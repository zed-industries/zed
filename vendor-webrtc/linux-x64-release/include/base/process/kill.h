// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file contains routines to kill processes and get the exit code and
// termination status.

#ifndef BASE_PROCESS_KILL_H_
#define BASE_PROCESS_KILL_H_

#include "base/base_export.h"
#include "base/files/file_path.h"
#include "base/process/process.h"
#include "base/process/process_handle.h"
#include "base/time/time.h"
#include "build/build_config.h"

namespace base {

class ProcessFilter;

#if BUILDFLAG(IS_WIN)
namespace win {

// See definition in sandbox/win/src/sandbox_types.h
const DWORD kSandboxFatalMemoryExceeded = 7012;

// Exit codes with special meanings on Windows.
const DWORD kNormalTerminationExitCode = 0;
const DWORD kDebuggerInactiveExitCode = 0xC0000354;
const DWORD kKeyboardInterruptExitCode = 0xC000013A;
const DWORD kDebuggerTerminatedExitCode = 0x40010004;
const DWORD kStatusInvalidImageHashExitCode = 0xC0000428;

// This exit code is used by the Windows task manager when it kills a
// process.  It's value is obviously not that unique, and it's
// surprising to me that the task manager uses this value, but it
// seems to be common practice on Windows to test for it as an
// indication that the task manager has killed something if the
// process goes away.
const DWORD kProcessKilledExitCode = 1;

}  // namespace win

#endif  // BUILDFLAG(IS_WIN)

// Return status values from GetTerminationStatus. Don't use these as exit code
// arguments to KillProcess*(), use platform/application specific values
// instead.
//
// Used for metrics. Keep in sync with the "TerminationStatus" histogram enum.
// Do not repurpose previously used indexes.
// GENERATED_JAVA_ENUM_PACKAGE: org.chromium.base
// GENERATED_JAVA_PREFIX_TO_STRIP: TERMINATION_STATUS_
enum TerminationStatus : int {
  // Zero exit status.
  TERMINATION_STATUS_NORMAL_TERMINATION = 0,
  // Other abnormal termination reason.
  TERMINATION_STATUS_ABNORMAL_TERMINATION = 1,
  // E.g. SIGKILL or task manager kill.
  TERMINATION_STATUS_PROCESS_WAS_KILLED = 2,
  // E.g. Segmentation fault.
  TERMINATION_STATUS_PROCESS_CRASHED = 3,
  // Child hasn't exited yet.
  TERMINATION_STATUS_STILL_RUNNING = 4,
#if BUILDFLAG(IS_CHROMEOS)
  // OOM-killer killed the process on ChromeOS.
  TERMINATION_STATUS_PROCESS_WAS_KILLED_BY_OOM = 5,
#endif
#if BUILDFLAG(IS_ANDROID)
  // On Android processes are spawned from the system Zygote and we do not get
  // the termination status. We can't know if the termination was a crash or an
  // oom kill for sure, but we can use status of the strong process bindings as
  // a hint.
  TERMINATION_STATUS_OOM_PROTECTED = 6,
#endif
  // Child process never launched.
  TERMINATION_STATUS_LAUNCH_FAILED = 7,
  // Out of memory.
  TERMINATION_STATUS_OOM = 8,
#if BUILDFLAG(IS_WIN)
  // On Windows, the OS terminated process due to code integrity failure.
  TERMINATION_STATUS_INTEGRITY_FAILURE = 9,
#endif
  TERMINATION_STATUS_MAX_ENUM = 10,
};

// Attempts to kill all the processes on the current machine that were launched
// from the given executable name, ending them with the given exit code.  If
// filter is non-null, then only processes selected by the filter are killed.
// Returns true if all processes were able to be killed off, false if at least
// one couldn't be killed.
BASE_EXPORT bool KillProcesses(const FilePath::StringType& executable_name,
                               int exit_code,
                               const ProcessFilter* filter);

// Get the termination status of the process by interpreting the
// circumstances of the child process' death. |exit_code| is set to
// the status returned by waitpid() on POSIX, and from GetExitCodeProcess() on
// Windows, and may not be null.  Note that on Linux, this function
// will only return a useful result the first time it is called after
// the child exits (because it will reap the child and the information
// will no longer be available).
BASE_EXPORT TerminationStatus GetTerminationStatus(ProcessHandle handle,
                                                   int* exit_code);

#if BUILDFLAG(IS_POSIX)
// Send a kill signal to the process and then wait for the process to exit
// and get the termination status.
//
// This is used in situations where it is believed that the process is dead
// or dying (because communication with the child process has been cut).
// In order to avoid erroneously returning that the process is still running
// because the kernel is still cleaning it up, this will wait for the process
// to terminate. In order to avoid the risk of hanging while waiting for the
// process to terminate, send a SIGKILL to the process before waiting for the
// termination status.
//
// Note that it is not an option to call WaitForExitCode and then
// GetTerminationStatus as the child will be reaped when WaitForExitCode
// returns, and this information will be lost.
//
BASE_EXPORT TerminationStatus
GetKnownDeadTerminationStatus(ProcessHandle handle, int* exit_code);

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)
// Spawns a thread to wait asynchronously for the child |process| to exit
// and then reaps it.
BASE_EXPORT void EnsureProcessGetsReaped(Process process);
#endif  // BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)
#endif  // BUILDFLAG(IS_POSIX)

// Registers |process| to be asynchronously monitored for termination, forcibly
// terminated if necessary, and reaped on exit. The caller should have signalled
// |process| to exit before calling this API. The API will allow a couple of
// seconds grace period before forcibly terminating |process|.
// TODO(crbug.com/41367359): The Mac implementation currently blocks the
// calling thread for up to two seconds.
BASE_EXPORT void EnsureProcessTerminated(Process process);

// These are only sparingly used, and not needed on Fuchsia or iOS. They could
// be implemented if necessary.
#if !BUILDFLAG(IS_FUCHSIA) && !BUILDFLAG(IS_IOS)
// Wait for all the processes based on the named executable to exit.  If filter
// is non-null, then only processes selected by the filter are waited on.
// Returns after all processes have exited or wait_milliseconds have expired.
// Returns true if all the processes exited, false otherwise.
BASE_EXPORT bool WaitForProcessesToExit(
    const FilePath::StringType& executable_name,
    base::TimeDelta wait,
    const ProcessFilter* filter);

// Waits a certain amount of time (can be 0) for all the processes with a given
// executable name to exit, then kills off any of them that are still around.
// If filter is non-null, then only processes selected by the filter are waited
// on.  Killed processes are ended with the given exit code.  Returns false if
// any processes needed to be killed, true if they all exited cleanly within
// the wait_milliseconds delay.
BASE_EXPORT bool CleanupProcesses(const FilePath::StringType& executable_name,
                                  base::TimeDelta wait,
                                  int exit_code,
                                  const ProcessFilter* filter);
#endif  // !BUILDFLAG(IS_FUCHSIA) && !BUILDFLAG(IS_IOS)

#if BUILDFLAG(IS_MAC) || (BUILDFLAG(IS_IOS) && TARGET_OS_SIMULATOR)
// This is common code used by kill_ios.cc when building with iOS simulator it
// does not need to be exported.
void WaitForChildToDie(pid_t child, int timeout_seconds);
#endif  // BUILDFLAG(IS_MAC) || (BUILDFLAG(IS_IOS) && TARGET_OS_SIMULATOR)

}  // namespace base

#endif  // BASE_PROCESS_KILL_H_
