/*
 * Copyright (C) 2020 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef INCLUDE_PERFETTO_EXT_BASE_SUBPROCESS_H_
#define INCLUDE_PERFETTO_EXT_BASE_SUBPROCESS_H_

#include <condition_variable>
#include <functional>
#include <initializer_list>
#include <memory>
#include <mutex>
#include <optional>
#include <string>
#include <thread>
#include <vector>

#include "perfetto/base/build_config.h"
#include "perfetto/base/logging.h"
#include "perfetto/base/platform_handle.h"
#include "perfetto/base/proc_utils.h"
#include "perfetto/ext/base/event_fd.h"
#include "perfetto/ext/base/pipe.h"
#include "perfetto/ext/base/scoped_file.h"

namespace perfetto {
namespace base {

// Handles creation and lifecycle management of subprocesses, taking care of
// all subtleties involved in handling processes on UNIX.
// This class allows to deal with macro two use-cases:
// 1) fork() + exec() equivalent: for spawning a brand new process image.
//    This happens when |args.exec_cmd| is not empty.
//    This is safe to use even in a multi-threaded environment.
// 2) fork(): for spawning a process and running a function.
//    This happens when |args.posix_entrypoint_for_testing| is not empty.
//    This is intended only for tests as it is extremely subtle.
//    This mode must be used with extreme care. Before the entrypoint is
//    invoked all file descriptors other than stdin/out/err and the ones
//    specified in |args.preserve_fds| will be closed, to avoid each process
//    retaining a dupe of other subprocesses pipes. This however means that
//    any non trivial calls (including logging) must be avoided as they might
//    refer to FDs that are now closed. The entrypoint should really be used
//    just to signal a pipe or similar for synchronizing sequencing in tests.

//
// This class allows to control stdin/out/err pipe redirection and takes care
// of keeping all the pipes pumped (stdin) / drained (stdout/err), in a similar
// fashion of python's subprocess.Communicate()
// stdin: is always piped and closed once the |args.input| buffer is written.
// stdout/err can be either:
//   - dup()ed onto the parent process stdout/err.
//   - redirected onto /dev/null.
//   - piped onto a buffer (see output() method). There is only one output
//     buffer in total. If both stdout and stderr are set to kBuffer mode, they
//     will be merged onto the same. There doesn't seem any use case where they
//     are needed distinctly.
//
// Some caveats worth mentioning:
// - It always waitpid()s, to avoid leaving zombies around. If the process is
//   not terminated by the time the destructor is reached, the dtor will
//   send a SIGKILL and wait for the termination.
// - After fork()-ing it will close all file descriptors, preserving only
//   stdin/out/err and the fds listed in |args.preserve_fds|.
// - On Linux/Android, the child process will be SIGKILL-ed if the calling
//   thread exists, even if the Subprocess is std::move()-d onto another thread.
//   This happens by virtue PR_SET_PDEATHSIG, which is used to avoid that
//   child processes are leaked in the case of a crash of the parent (frequent
//   in tests). However, the child process might still be leaked if execing
//   a setuid/setgid binary (see man 2 prctl).
//
// Usage:
// base::Subprocess p({"/bin/cat", "-"});
// (or equivalently:
//     base::Subprocess p;
//     p.args.exec_cmd.push_back("/bin/cat");
//     p.args.exec_cmd.push_back("-");
//  )
// p.args.stdout_mode = base::Subprocess::kBuffer;
// p.args.stderr_mode = base::Subprocess::kInherit;
// p.args.input = "stdin contents";
// p.Call();
// (or equivalently:
//     p.Start();
//     p.Wait();
// )
// EXPECT_EQ(p.status(), base::Subprocess::kTerminated);
// EXPECT_EQ(p.returncode(), 0);
class Subprocess {
 public:
  enum Status {
    kNotStarted = 0,  // Before calling Start() or Call().
    kRunning,         // After calling Start(), before Wait().
    kTerminated,      // The subprocess terminated, either successfully or not.
                      // This includes crashes or other signals on UNIX.
  };

  enum class OutputMode {
    kInherit = 0,  // Inherit's the caller process stdout/stderr.
    kDevNull,      // dup() onto /dev/null.
    kBuffer,       // dup() onto a pipe and move it into the output() buffer.
    kFd,           // dup() onto the passed args.fd.
  };

  enum class InputMode {
    kBuffer = 0,  // dup() onto a pipe and write args.input on it.
    kDevNull,     // dup() onto /dev/null.
  };

  // Input arguments for configuring the subprocess behavior.
  struct Args {
    Args(std::initializer_list<std::string> _cmd = {}) : exec_cmd(_cmd) {}
    Args(Args&&) noexcept;
    Args& operator=(Args&&);
    // If non-empty this will cause an exec() when Start()/Call() are called.
    std::vector<std::string> exec_cmd;

#if !PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)
    // If non-empty, it changes the argv[0] argument passed to exec. If
    // unset, argv[0] == exec_cmd[0]. This is to handle cases like:
    // exec_cmd = {"/proc/self/exec"}, argv0: "my_custom_test_override".
    std::string posix_argv0_override_for_testing;

    // If non-empty this will be invoked on the fork()-ed child process, after
    // stdin/out/err has been redirected and all other file descriptor are
    // closed. It is valid to specify both |exec_cmd| AND
    // |posix_entrypoint_for_testing|. In this case the latter will be invoked
    // just before the exec() call, but after having closed all fds % stdin/o/e.
    // This is for synchronization barriers in tests.
    std::function<void()> posix_entrypoint_for_testing;

    // When set, will will move the process to the given process group. If set
    // and zero, it will create a new process group. Effectively this calls
    // setpgid(0 /*self_pid*/, posix_proc_group_id).
    // This can be used to avoid that subprocesses receive CTRL-C from the
    // terminal, while still living in the same session.
    std::optional<pid_t> posix_proc_group_id{};
#endif

    // If non-empty, replaces the environment passed to exec().
    std::vector<std::string> env;

    // The file descriptors in this list will not be closed.
    std::vector<int> preserve_fds;

    // The data to push in the child process stdin, if input_mode ==
    // InputMode::kBuffer.
    std::string input;

    InputMode stdin_mode = InputMode::kBuffer;
    OutputMode stdout_mode = OutputMode::kInherit;
    OutputMode stderr_mode = OutputMode::kInherit;

    base::ScopedPlatformHandle out_fd;

    // Returns " ".join(exec_cmd), quoting arguments.
    std::string GetCmdString() const;
  };

  struct ResourceUsage {
    uint32_t cpu_utime_ms = 0;
    uint32_t cpu_stime_ms = 0;
    uint32_t max_rss_kb = 0;
    uint32_t min_page_faults = 0;
    uint32_t maj_page_faults = 0;
    uint32_t vol_ctx_switch = 0;
    uint32_t invol_ctx_switch = 0;

    uint32_t cpu_time_ms() const { return cpu_utime_ms + cpu_stime_ms; }
  };

  explicit Subprocess(std::initializer_list<std::string> exec_cmd = {});
  Subprocess(Subprocess&&) noexcept;
  Subprocess& operator=(Subprocess&&);
  ~Subprocess();  // It will KillAndWaitForTermination() if still alive.

  // Starts the subprocess but doesn't wait for its termination. The caller
  // is expected to either call Wait() or Poll() after this call.
  void Start();

  // Wait for process termination. Can be called more than once.
  // Args:
  //   |timeout_ms| = 0: wait indefinitely.
  //   |timeout_ms| > 0: wait for at most |timeout_ms|.
  // Returns:
  //  True: The process terminated. See status() and returncode().
  //  False: Timeout reached, the process is still running. In this case the
  //         process will be left in the kRunning state.
  bool Wait(int timeout_ms = 0);

  // Equivalent of Start() + Wait();
  // Returns true if the process exited cleanly with return code 0. False in
  // any othe case.
  bool Call(int timeout_ms = 0);

  Status Poll();

  // Sends a signal (SIGKILL if not specified) and wait for process termination.
  void KillAndWaitForTermination(int sig_num = 0);

  PlatformProcessId pid() const { return s_->pid; }

  // The accessors below are updated only after a call to Poll(), Wait() or
  // KillAndWaitForTermination().
  // In most cases you want to call Poll() rather than these accessors.

  Status status() const { return s_->status; }
  int returncode() const { return s_->returncode; }
  bool timed_out() const { return s_->timed_out; }

  // This contains both stdout and stderr (if the corresponding _mode ==
  // OutputMode::kBuffer). It's non-const so the caller can std::move() it.
  std::string& output() { return s_->output; }
  const std::string& output() const { return s_->output; }

  const ResourceUsage& posix_rusage() const { return *s_->rusage; }

  Args args;

 private:
  // The signal/exit code used when killing the process in case of a timeout.
  static const int kTimeoutSignal;

  Subprocess(const Subprocess&) = delete;
  Subprocess& operator=(const Subprocess&) = delete;

  // This is to deal robustly with the move operators, without having to
  // manually maintain member-wise move instructions.
  struct MovableState {
    base::Pipe stdin_pipe;
    base::Pipe stdouterr_pipe;
    PlatformProcessId pid;
    Status status = kNotStarted;
    int returncode = -1;
    std::string output;  // Stdin+stderr. Only when OutputMode::kBuffer.
    std::unique_ptr<ResourceUsage> rusage{new ResourceUsage()};
    bool timed_out = false;
#if PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)
    std::thread stdouterr_thread;
    std::thread stdin_thread;
    ScopedPlatformHandle win_proc_handle;
    ScopedPlatformHandle win_thread_handle;

    base::EventFd stdouterr_done_event;
    std::mutex mutex;  // Protects locked_outerr_buf and the two pipes.
    std::string locked_outerr_buf;
#else
    base::Pipe exit_status_pipe;
    size_t input_written = 0;
    std::thread waitpid_thread;
#endif
  };

#if PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)
  static void StdinThread(MovableState*, std::string input);
  static void StdoutErrThread(MovableState*);
#else
  void TryPushStdin();
  void TryReadStdoutAndErr();
  void TryReadExitStatus();
  bool PollInternal(int poll_timeout_ms);
#endif

  std::unique_ptr<MovableState> s_;
};

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_SUBPROCESS_H_
