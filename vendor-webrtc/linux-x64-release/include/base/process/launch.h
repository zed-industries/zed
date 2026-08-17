// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file contains functions for launching subprocesses.

#ifndef BASE_PROCESS_LAUNCH_H_
#define BASE_PROCESS_LAUNCH_H_

#include <limits.h>
#include <stddef.h>

#include <string>
#include <string_view>
#include <utility>
#include <vector>

#include "base/base_export.h"
#include "base/command_line.h"
#include "base/environment.h"
#include "base/files/file_path.h"
#include "base/functional/function_ref.h"
#include "base/memory/raw_ptr.h"
#include "base/process/process.h"
#include "base/process/process_handle.h"
#include "base/threading/thread_restrictions.h"
#include "base/time/time.h"
#include "build/blink_buildflags.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_WIN)
#include "base/win/windows_types.h"
#elif BUILDFLAG(IS_FUCHSIA)
#include <lib/fdio/spawn.h>
#include <zircon/types.h>
#endif

#if BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)
#include "base/posix/file_descriptor_shuffle.h"
#endif

#if BUILDFLAG(IS_ANDROID)
#include "base/android/binder.h"
#endif

#if BUILDFLAG(IS_MAC)
#include "base/mac/process_requirement.h"
#endif

namespace base {

enum TerminationStatus : int;

#if BUILDFLAG(IS_POSIX)
// Some code (e.g. the sandbox) relies on PTHREAD_STACK_MIN
// being async-signal-safe, which is no longer guaranteed by POSIX
// (it may call sysconf, which is not safe).
// To work around this, use a hardcoded value unless it's already
// defined as a constant.

// These constants are borrowed from glibc's (arch)/bits/pthread_stack_min.h.
#if defined(ARCH_CPU_ARM64) || defined(ARCH_CPU_LOONGARCH64)
#define PTHREAD_STACK_MIN_CONST \
  (__builtin_constant_p(PTHREAD_STACK_MIN) ? PTHREAD_STACK_MIN : 131072)
#else
#define PTHREAD_STACK_MIN_CONST \
  (__builtin_constant_p(PTHREAD_STACK_MIN) ? PTHREAD_STACK_MIN : 16384)
#endif  // defined(ARCH_CPU_ARM64)

static_assert(__builtin_constant_p(PTHREAD_STACK_MIN_CONST),
              "must be constant");

// Make sure our hardcoded value is large enough to accommodate the
// actual minimum stack size. This function will run a one-time CHECK
// and so should be called at some point, preferably during startup.
BASE_EXPORT void CheckPThreadStackMinIsSafe();
#endif  // BUILDFLAG(IS_POSIX)

#if BUILDFLAG(IS_APPLE) && !BUILDFLAG(IS_IOS_TVOS)
class MachRendezvousPort;
using MachPortsForRendezvous = std::map<uint32_t, MachRendezvousPort>;
#endif

#if BUILDFLAG(IS_WIN)
typedef std::vector<HANDLE> HandlesToInheritVector;
#elif BUILDFLAG(IS_FUCHSIA)
struct PathToTransfer {
  base::FilePath path;
  zx_handle_t handle;
};
struct HandleToTransfer {
  uint32_t id;
  zx_handle_t handle;
};
typedef std::vector<HandleToTransfer> HandlesToTransferVector;
typedef std::vector<std::pair<int, int>> FileHandleMappingVector;
#elif BUILDFLAG(IS_POSIX)
typedef std::vector<std::pair<int, int>> FileHandleMappingVector;
#endif  // BUILDFLAG(IS_WIN)

// Options for launching a subprocess that are passed to LaunchProcess().
// The default constructor constructs the object with default options.
struct BASE_EXPORT LaunchOptions {
#if (BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)) && !BUILDFLAG(IS_APPLE)
  // Delegate to be run in between fork and exec in the subprocess (see
  // pre_exec_delegate below)
  class BASE_EXPORT PreExecDelegate {
   public:
    PreExecDelegate() = default;

    PreExecDelegate(const PreExecDelegate&) = delete;
    PreExecDelegate& operator=(const PreExecDelegate&) = delete;

    virtual ~PreExecDelegate() = default;

    // Since this is to be run between fork and exec, and fork may have happened
    // while multiple threads were running, this function needs to be async
    // safe.
    virtual void RunAsyncSafe() = 0;
  };
#endif  // BUILDFLAG(IS_POSIX)

  LaunchOptions();
  LaunchOptions(const LaunchOptions&);
  ~LaunchOptions();

  // If true, wait for the process to complete.
  bool wait = false;

  // If not empty, change to this directory before executing the new process.
  base::FilePath current_directory;

#if BUILDFLAG(IS_WIN)
  bool start_hidden = false;

  // Process will be started using ShellExecuteEx instead of CreateProcess so
  // that it is elevated. LaunchProcess with this flag will have different
  // behaviour due to ShellExecuteEx. Some common operations like OpenProcess
  // will fail. Currently the only other supported LaunchOptions are
  // |start_hidden| and |wait|.
  bool elevated = false;

  // Sets STARTF_FORCEOFFFEEDBACK so that the feedback cursor is forced off
  // while the process is starting.
  bool feedback_cursor_off = false;

  // Windows can inherit handles when it launches child processes.
  // See https://blogs.msdn.microsoft.com/oldnewthing/20111216-00/?p=8873
  // for a good overview of Windows handle inheritance.
  //
  // Implementation note: it might be nice to implement in terms of
  // std::optional<>, but then the natural default state (vector not present)
  // would be "all inheritable handles" while we want "no inheritance."
  enum class Inherit {
    // Only those handles in |handles_to_inherit| vector are inherited. If the
    // vector is empty, no handles are inherited. The handles in the vector must
    // all be inheritable.
    kSpecific,

    // All handles in the current process which are inheritable are inherited.
    // In production code this flag should be used only when running
    // short-lived, trusted binaries, because open handles from other libraries
    // and subsystems will leak to the child process, causing errors such as
    // open socket hangs. There are also race conditions that can cause handle
    // over-sharing.
    //
    // |handles_to_inherit| must be null.
    //
    // DEPRECATED. THIS SHOULD NOT BE USED. Explicitly map all handles that
    // need to be shared in new code.
    // TODO(brettw) bug 748258: remove this.
    kAll
  };
  Inherit inherit_mode = Inherit::kSpecific;
  HandlesToInheritVector handles_to_inherit;

  // If non-null, runs as if the user represented by the token had launched it.
  // Whether the application is visible on the interactive desktop depends on
  // the token belonging to an interactive logon session.
  //
  // To avoid hard to diagnose problems, when specified this loads the
  // environment variables associated with the user and if this operation fails
  // the entire call fails as well.
  UserTokenHandle as_user = nullptr;

  // If true, use an empty string for the desktop name.
  bool empty_desktop_name = false;

  // If non-null, launches the application in that job object. The process will
  // be terminated immediately and LaunchProcess() will fail if assignment to
  // the job object fails.
  HANDLE job_handle = nullptr;

  // Handles for the redirection of stdin, stdout and stderr. The caller should
  // either set all three of them or none (i.e. there is no way to redirect
  // stderr without redirecting stdin).
  //
  // The handles must be inheritable. Pseudo handles are used when stdout and
  // stderr redirect to the console. In that case, GetFileType() will return
  // FILE_TYPE_CHAR and they're automatically inherited by child processes. See
  // https://msdn.microsoft.com/en-us/library/windows/desktop/ms682075.aspx
  // Otherwise, the caller must ensure that the |inherit_mode| and/or
  // |handles_to_inherit| set so that the handles are inherited.
  HANDLE stdin_handle = nullptr;
  HANDLE stdout_handle = nullptr;
  HANDLE stderr_handle = nullptr;

  // If set to true, ensures that the child process is launched with the
  // CREATE_BREAKAWAY_FROM_JOB flag which allows it to breakout of the parent
  // job if any.
  bool force_breakaway_from_job_ = false;

  // If set to true, permission to bring windows to the foreground is passed to
  // the launched process if the current process has such permission.
  bool grant_foreground_privilege = false;

  // If set to true, sets a process mitigation flag to disable Hardware-enforced
  // Stack Protection for the process.
  // This overrides /cetcompat if set on the executable. See:
  // https://docs.microsoft.com/en-us/cpp/build/reference/cetcompat?view=msvc-160
  // If not supported by Windows, has no effect. This flag weakens security by
  // turning off ROP protection.
  bool disable_cetcompat = false;
#elif BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)
  // Remap file descriptors according to the mapping of src_fd->dest_fd to
  // propagate FDs into the child process.
  FileHandleMappingVector fds_to_remap;
#endif  // BUILDFLAG(IS_WIN)

#if BUILDFLAG(IS_ANDROID)
  // Set of strong IBinder references to be passed to the child process. These
  // make their way to ChildProcessServiceDelegate.onConnectionSetup (Java)
  // within the new child process.
  std::vector<android::BinderRef> binders;
#endif

#if BUILDFLAG(IS_WIN) || BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)
  // Set/unset environment variables. These are applied on top of the parent
  // process environment.  Empty (the default) means to inherit the same
  // environment. See internal::AlterEnvironment().
  EnvironmentMap environment;

  // Clear the environment for the new process before processing changes from
  // |environment|.
  bool clear_environment = false;
#endif  // BUILDFLAG(IS_WIN) || BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)
  // If non-zero, start the process using clone(), using flags as provided.
  // Unlike in clone, clone_flags may not contain a custom termination signal
  // that is sent to the parent when the child dies. The termination signal will
  // always be set to SIGCHLD.
  int clone_flags = 0;

  // By default, child processes will have the PR_SET_NO_NEW_PRIVS bit set. If
  // true, then this bit will not be set in the new child process.
  bool allow_new_privs = false;

  // Sets parent process death signal to SIGKILL.
  bool kill_on_parent_death = false;

  // File descriptors of the parent process with FD_CLOEXEC flag to be removed
  // before calling exec*().
  std::vector<int> fds_to_remove_cloexec;
#endif  // BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)

#if BUILDFLAG(IS_MAC) || (BUILDFLAG(IS_IOS) && BUILDFLAG(USE_BLINK))
#if !BUILDFLAG(IS_IOS_TVOS)
  // Mach ports that will be accessible to the child process. These are not
  // directly inherited across process creation, but they are stored by a Mach
  // IPC server that a child process can communicate with to retrieve them.
  //
  // After calling LaunchProcess(), any rights that were transferred with MOVE
  // dispositions will be consumed, even on failure.
  //
  // See base/apple/mach_port_rendezvous.h for details.
  MachPortsForRendezvous mach_ports_for_rendezvous;
#endif  // !BUILDFLAG(IS_IOS_TVOS)

  // Apply a process scheduler policy to enable mitigations against CPU side-
  // channel attacks.
  bool enable_cpu_security_mitigations = false;
#endif  // BUILDFLAG(IS_MAC) || (BUILDFLAG(IS_IOS) && BUILDFLAG(USE_BLINK))

#if BUILDFLAG(IS_MAC)
  // When a child process is launched, the system tracks the parent process
  // with a concept of "responsibility". The responsible process will be
  // associated with any requests for private data stored on the system via
  // the TCC subsystem. When launching processes that run foreign/third-party
  // code, the responsibility for the child process should be disclaimed so
  // that any TCC requests are not associated with the parent.
  bool disclaim_responsibility = false;

  // A `ProcessRequirement` that will be used to validate the launched process
  // before it can retrieve `mach_ports_for_rendezvous`.
  std::optional<mac::ProcessRequirement> process_requirement;
#endif  // BUILDFLAG(IS_MAC)

#if BUILDFLAG(IS_FUCHSIA)
  // If valid, launches the application in that job object.
  zx_handle_t job_handle = ZX_HANDLE_INVALID;

  // Specifies additional handles to transfer (not duplicate) to the child
  // process. Each entry is an <id,handle> pair, with an |id| created using the
  // PA_HND() macro. The child retrieves the handle
  // |zx_take_startup_handle(id)|. The supplied handles are consumed by
  // LaunchProcess() even on failure.
  // Note that PA_USER1 ids are reserved for use by AddHandleToTransfer(), below
  // and by convention PA_USER0 is reserved for use by the embedding
  // application.
  HandlesToTransferVector handles_to_transfer;

  // Allocates a unique id for |handle| in |handles_to_transfer|, inserts it,
  // and returns the generated id.
  static uint32_t AddHandleToTransfer(
      HandlesToTransferVector* handles_to_transfer,
      zx_handle_t handle);

  // Specifies which basic capabilities to grant to the child process.
  // By default the child process will receive the caller's complete namespace,
  // access to the current base::GetDefaultJob(), handles for stdio and access
  // to the dynamic library loader.
  // Note that the child is always provided access to the loader service.
  uint32_t spawn_flags = FDIO_SPAWN_CLONE_NAMESPACE | FDIO_SPAWN_CLONE_STDIO |
                         FDIO_SPAWN_CLONE_JOB;

  // Specifies paths to clone from the calling process' namespace into that of
  // the child process. If |paths_to_clone| is empty then the process will
  // receive either a full copy of the parent's namespace, or an empty one,
  // depending on whether FDIO_SPAWN_CLONE_NAMESPACE is set.
  // Process launch will fail if `paths_to_clone` and `paths_to_transfer`
  // together contain conflicting paths (e.g. overlaps or duplicates).
  std::vector<FilePath> paths_to_clone;

  // Specifies handles which will be installed as files or directories in the
  // child process' namespace.
  // Process launch will fail if `paths_to_clone` and `paths_to_transfer`
  // together contain conflicting paths (e.g. overlaps or duplicates).
  std::vector<PathToTransfer> paths_to_transfer;

  // Suffix that will be added to the process name. When specified process name
  // will be set to "<binary_name><process_suffix>".
  std::string process_name_suffix;
#endif  // BUILDFLAG(IS_FUCHSIA)

#if BUILDFLAG(IS_POSIX)
  // If not empty, launch the specified executable instead of
  // cmdline.GetProgram(). This is useful when it is necessary to pass a custom
  // argv[0].
  base::FilePath real_path;

#if !BUILDFLAG(IS_APPLE)
  // If non-null, a delegate to be run immediately prior to executing the new
  // program in the child process.
  //
  // WARNING: If LaunchProcess is called in the presence of multiple threads,
  // code running in this delegate essentially needs to be async-signal safe
  // (see man 7 signal for a list of allowed functions).
  raw_ptr<PreExecDelegate> pre_exec_delegate = nullptr;
#endif  // !BUILDFLAG(IS_APPLE)

  // Each element is an RLIMIT_* constant that should be raised to its
  // rlim_max.  This pointer is owned by the caller and must live through
  // the call to LaunchProcess().
  raw_ptr<const std::vector<int>> maximize_rlimits = nullptr;

  // If true, start the process in a new process group, instead of
  // inheriting the parent's process group.  The pgid of the child process
  // will be the same as its pid.
  bool new_process_group = false;
#endif  // BUILDFLAG(IS_POSIX)

#if BUILDFLAG(IS_CHROMEOS)
  // If non-negative, the specified file descriptor will be set as the launched
  // process' controlling terminal.
  int ctrl_terminal_fd = -1;
#endif  // BUILDFLAG(IS_CHROMEOS)
};

// Launch a process via the command line |cmdline|.
// See the documentation of LaunchOptions for details on |options|.
//
// Returns a valid Process upon success.
//
// Unix-specific notes:
// - All file descriptors open in the parent process will be closed in the
//   child process except for any preserved by options::fds_to_remap, and
//   stdin, stdout, and stderr. If not remapped by options::fds_to_remap,
//   stdin is reopened as /dev/null, and the child is allowed to inherit its
//   parent's stdout and stderr.
// - If the first argument on the command line does not contain a slash,
//   PATH will be searched.  (See man execvp.)
BASE_EXPORT Process LaunchProcess(const CommandLine& cmdline,
                                  const LaunchOptions& options);

#if BUILDFLAG(IS_WIN)
// Windows-specific LaunchProcess that takes the command line as a
// string.  Useful for situations where you need to control the
// command line arguments directly, but prefer the CommandLine version
// if launching Chrome itself. Also prefer the CommandLine version if
// `options.elevated` is set because `cmdline` needs to be parsed for
// ShellExecuteEx.
//
// The first command line argument should be the path to the process,
// and don't forget to quote it.
//
// Example (including literal quotes)
//  cmdline = "c:\windows\explorer.exe" -foo "c:\bar\"
BASE_EXPORT Process LaunchProcess(const CommandLine::StringType& cmdline,
                                  const LaunchOptions& options);

#elif BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)
// A POSIX-specific version of LaunchProcess that takes an argv array
// instead of a CommandLine.  Useful for situations where you need to
// control the command line arguments directly, but prefer the
// CommandLine version if launching Chrome itself.
BASE_EXPORT Process LaunchProcess(const std::vector<std::string>& argv,
                                  const LaunchOptions& options);

#if !BUILDFLAG(IS_APPLE)
// Close all file descriptors, except those which are a destination in the
// given multimap. Only call this function in a child process where you know
// that there aren't any other threads.
BASE_EXPORT void CloseSuperfluousFds(const InjectiveMultimap& saved_map);
#endif  // BUILDFLAG(IS_APPLE)
#endif  // BUILDFLAG(IS_WIN)

#if BUILDFLAG(IS_WIN)
// Set |job_object|'s JOBOBJECT_EXTENDED_LIMIT_INFORMATION
// BasicLimitInformation.LimitFlags to |limit_flags|.
BASE_EXPORT bool SetJobObjectLimitFlags(HANDLE job_object, DWORD limit_flags);

// Output multi-process printf, cout, cerr, etc to the cmd.exe console that ran
// chrome. This is not thread-safe: only call from main thread.
BASE_EXPORT void RouteStdioToConsole(bool create_console_if_not_found);
#endif  // BUILDFLAG(IS_WIN)

// Executes the application specified by |cl| and wait for it to exit. Stores
// the output (stdout) in |output|. Redirects stderr to /dev/null. Returns true
// on success (application launched and exited cleanly, with exit code
// indicating success).
BASE_EXPORT bool GetAppOutput(const CommandLine& cl, std::string* output);

// Like GetAppOutput, but also includes stderr.
BASE_EXPORT bool GetAppOutputAndError(const CommandLine& cl,
                                      std::string* output);

// A version of |GetAppOutput()| which also returns the exit code of the
// executed command. Returns true if the application runs and exits cleanly. If
// this is the case the exit code of the application is available in
// |*exit_code|.
BASE_EXPORT bool GetAppOutputWithExitCode(const CommandLine& cl,
                                          std::string* output,
                                          int* exit_code);

#if BUILDFLAG(IS_WIN)
// A Windows-specific version of GetAppOutput that takes a command line string
// instead of a CommandLine object. Useful for situations where you need to
// control the command line arguments directly.
BASE_EXPORT bool GetAppOutput(CommandLine::StringViewType cl,
                              std::string* output);

// A Windows-specific version of `GetAppOutput` that allows the ability to
// specify:
// * an optional `output` providing the complete output of `cl`.
// * an optional `timeout` if `cl` does not complete in time.
// * an optional `LaunchOptions`.
// * an optional `FunctionRef`, called multiple times while waiting, with
//   streaming partial output received since the last call to the `FunctionRef`
//   from stdout/stderr of the running `cl` process. The implementation of the
//   `FunctionRef` can log the output, or concatenate the partial outputs over
//   successive calls to effectively produce the full `output` from the `cl`
//   process.
// * an optional `final_status` `TerminationStatus` value on function return.
// Note that the expected use cases for this function do not expect `cl` to
// produce a lot of output. This function will not work optimally with lots of
// output from the `cl` process, since it waits a second each time between
// reading the output.
BASE_EXPORT bool GetAppOutputWithExitCodeAndTimeout(
    CommandLine::StringViewType cl,
    bool include_stderr,
    std::string* output,
    int* exit_code,
    TimeDelta timeout = TimeDelta::Max(),
    const LaunchOptions& options = {},
    FunctionRef<void(std::string_view)> still_waiting =
        [](std::string_view partial_output) {},
    TerminationStatus* final_status = nullptr);

#elif BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)
// A POSIX-specific version of GetAppOutput that takes an argv array
// instead of a CommandLine.  Useful for situations where you need to
// control the command line arguments directly.
BASE_EXPORT bool GetAppOutput(const std::vector<std::string>& argv,
                              std::string* output);

// Like the above POSIX-specific version of GetAppOutput, but also includes
// stderr.
BASE_EXPORT bool GetAppOutputAndError(const std::vector<std::string>& argv,
                                      std::string* output);
#endif  // BUILDFLAG(IS_WIN)

// If supported on the platform, and the user has sufficent rights, increase
// the current process's scheduling priority to a high priority.
BASE_EXPORT void RaiseProcessToHighPriority();

// Creates a LaunchOptions object suitable for launching processes in a test
// binary. This should not be called in production/released code.
BASE_EXPORT LaunchOptions LaunchOptionsForTest();

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)
// A wrapper for clone with fork-like behavior, meaning that it returns the
// child's pid in the parent and 0 in the child. |flags|, |ptid|, and |ctid| are
// as in the clone system call (the CLONE_VM flag is not supported).
//
// This function uses the libc clone wrapper (which updates libc's pid cache)
// internally, so callers may expect things like getpid() to work correctly
// after in both the child and parent.
//
// As with fork(), callers should be extremely careful when calling this while
// multiple threads are running, since at the time the fork happened, the
// threads could have been in any state (potentially holding locks, etc.).
// Callers should most likely call execve() in the child soon after calling
// this.
//
// It is unsafe to use any pthread APIs after ForkWithFlags().
// However, performing an exec() will lift this restriction.
BASE_EXPORT pid_t ForkWithFlags(int flags, pid_t* ptid, pid_t* ctid);
#endif

namespace internal {

// Friend and derived class of ScopedAllowBaseSyncPrimitives which allows
// GetAppOutputInternal() to join a process. GetAppOutputInternal() can't itself
// be a friend of ScopedAllowBaseSyncPrimitives because it is in the anonymous
// namespace.
class [[maybe_unused, nodiscard]] GetAppOutputScopedAllowBaseSyncPrimitives
    : public base::ScopedAllowBaseSyncPrimitives {};

}  // namespace internal

}  // namespace base

#endif  // BASE_PROCESS_LAUNCH_H_
