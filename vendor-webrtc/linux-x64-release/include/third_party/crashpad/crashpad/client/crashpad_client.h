// Copyright 2014 The Crashpad Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef CRASHPAD_CLIENT_CRASHPAD_CLIENT_H_
#define CRASHPAD_CLIENT_CRASHPAD_CLIENT_H_

#include <functional>
#include <map>
#include <set>
#include <string>
#include <vector>

#include <stdint.h>

#include "base/files/file_path.h"
#include "build/build_config.h"
#include "util/file/file_io.h"

#if !BUILDFLAG(IS_FUCHSIA)
#include "util/misc/capture_context.h"
#endif  // !BUILDFLAG(IS_FUCHSIA)

#if BUILDFLAG(IS_APPLE)
#include "base/apple/scoped_mach_port.h"
#elif BUILDFLAG(IS_WIN)
#include <windows.h>
#include "util/win/scoped_handle.h"
#elif BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_ANDROID)
#include <signal.h>
#include <ucontext.h>
#endif

#if BUILDFLAG(IS_IOS)
#include "client/upload_behavior_ios.h"
#include "handler/user_stream_data_source.h"  // nogncheck
#endif

namespace crashpad {

//! \brief The primary interface for an application to have Crashpad monitor
//!     it for crashes.
class CrashpadClient {
 public:
  CrashpadClient();

  CrashpadClient(const CrashpadClient&) = delete;
  CrashpadClient& operator=(const CrashpadClient&) = delete;

  ~CrashpadClient();

  //! \brief Starts a Crashpad handler process, performing any necessary
  //!     handshake to configure it.
  //!
  //! This method directs crashes to the Crashpad handler. On macOS, this is
  //! applicable to this process and all subsequent child processes. On Windows,
  //! child processes must also register by using SetHandlerIPCPipe().
  //!
  //! On macOS, this method starts a Crashpad handler and obtains a Mach send
  //! right corresponding to a receive right held by the handler process. The
  //! handler process runs an exception server on this port. This method sets
  //! the task’s exception port for `EXC_CRASH`, `EXC_RESOURCE`, and `EXC_GUARD`
  //! exceptions to the Mach send right obtained. The handler will be installed
  //! with behavior `EXCEPTION_STATE_IDENTITY | MACH_EXCEPTION_CODES` and thread
  //! state flavor `MACHINE_THREAD_STATE`. Exception ports are inherited, so a
  //! Crashpad handler started here will remain the handler for any child
  //! processes created after StartHandler() is called. These child processes do
  //! not need to call StartHandler() or be aware of Crashpad in any way. The
  //! Crashpad handler will receive crashes from child processes that have
  //! inherited it as their exception handler even after the process that called
  //! StartHandler() exits.
  //!
  //! On Windows, if \a asynchronous_start is `true`, this function will not
  //! directly call `CreateProcess()`, making it suitable for use in a
  //! `DllMain()`. In that case, the handler is started from a background
  //! thread, deferring the handler's startup. Nevertheless, regardless of the
  //! value of \a asynchronous_start, after calling this method, the global
  //! unhandled exception filter is set up, and all crashes will be handled by
  //! Crashpad. Optionally, use WaitForHandlerStart() to join with the
  //! background thread and retrieve the status of handler startup.
  //!
  //! On Fuchsia, this method binds to the exception port of the current default
  //! job, and starts a Crashpad handler to monitor that port.
  //!
  //! On Linux, this method starts a Crashpad handler, connected to this process
  //! via an `AF_UNIX` socket pair and installs signal handlers to request crash
  //! dumps on the client's socket end.
  //!
  //! \param[in] handler The path to a Crashpad handler executable.
  //! \param[in] database The path to a Crashpad database. The handler will be
  //!     started with this path as its `--database` argument.
  //! \param[in] metrics_dir The path to an already existing directory where
  //!     metrics files can be stored. The handler will be started with this
  //!     path as its `--metrics-dir` argument.
  //! \param[in] url The URL of an upload server. The handler will be started
  //!     with this URL as its `--url` argument.
  //! \param[in] annotations Process annotations to set in each crash report.
  //!     The handler will be started with an `--annotation` argument for each
  //!     element in this map.
  //! \param[in] arguments Additional arguments to pass to the Crashpad handler.
  //!     Arguments passed in other parameters and arguments required to perform
  //!     the handshake are the responsibility of this method, and must not be
  //!     specified in this parameter.
  //! \param[in] restartable If `true`, the handler will be restarted if it
  //!     dies, if this behavior is supported. This option is not available on
  //!     all platforms, and does not function on all OS versions. If it is
  //!     not supported, it will be ignored.
  //! \param[out] asynchronous_start If `true`, the handler will be started from
  //!     a background thread. Optionally, WaitForHandlerStart() can be used at
  //!     a suitable time to retreive the result of background startup. This
  //!     option is only used on Windows.
  //! \param[in] attachments Vector that stores file paths that should be
  //!     captured with each report at the time of the crash.
  //!
  //! \return `true` on success, `false` on failure with a message logged.
  bool StartHandler(const base::FilePath& handler,
                    const base::FilePath& database,
                    const base::FilePath& metrics_dir,
                    const std::string& url,
                    const std::map<std::string, std::string>& annotations,
                    const std::vector<std::string>& arguments,
                    bool restartable,
                    bool asynchronous_start,
                    const std::vector<base::FilePath>& attachments = {});

#if BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || \
    DOXYGEN
  //! \brief Retrieve the socket and process ID for the handler.
  //!
  //! `StartHandler()` must have successfully been called before calling this
  //!     method.
  //!
  //! \param[out] sock The socket connected to the handler, if not `nullptr`.
  //! \param[out] pid The handler's process ID, if not `nullptr`.
  //! \return `true` on success. Otherwise `false` with a message logged.
  static bool GetHandlerSocket(int* sock, pid_t* pid);

  //! \brief Sets the socket to a presumably-running Crashpad handler process
  //!      which was started with StartHandler().
  //!
  //! This method installs a signal handler to request crash dumps on \a sock.
  //!
  //! \param[in] sock A socket connected to a Crashpad handler.
  //! \param[in] pid The process ID of the handler, used to set the handler as
  //!     this process' ptracer. 0 indicates it is not necessary to set the
  //!     handler as this process' ptracer. -1 indicates that the handler's
  //!     process ID should be determined by communicating over the socket.
  bool SetHandlerSocket(ScopedFileHandle sock, pid_t pid);

  //! \brief Uses `sigaltstack()` to allocate a signal stack for the calling
  //!     thread.
  //!
  //! This method allocates an alternate stack to handle signals delivered to
  //! the calling thread and should be called early in the lifetime of each
  //! thread. Installing an alternate stack allows signals to be delivered in
  //! the event that the call stack's stack pointer points to invalid memory,
  //! as in the case of stack overflow.
  //!
  //! This method is called automatically by SetHandlerSocket() and
  //! the various StartHandler() methods. It is harmless to call multiple times.
  //! A new signal stack will be allocated only if there is no existing stack or
  //! the existing stack is too small. The stack will be automatically freed
  //! when the thread exits.
  //!
  //! An application might choose to diligently call this method from the start
  //! routine for each thread, call it from a `pthread_create()` wrapper which
  //! the application provides, or link the provided "client:pthread_create"
  //! target.
  //!
  //! \return `true` on success. Otherwise `false` with a message logged.
  static bool InitializeSignalStackForThread();
#endif  // BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_LINUX) ||
        // BUILDFLAG(IS_CHROMEOS) || DOXYGEN

#if BUILDFLAG(IS_ANDROID) || DOXYGEN
  //! \brief Installs a signal handler to execute `/system/bin/app_process` and
  //!     load a Java class in response to a crash.
  //!
  //! \param[in] class_name The fully qualified class name to load, which must
  //!     define a `main()` method to be invoked by `app_process`. Arguments
  //!     will be passed to this method as though it were the Crashpad handler.
  //!     This class is expected to load a native library defining
  //!     crashpad::HandlerMain() and pass the arguments to it.
  //! \param[in] env A vector of environment variables of the form `var=value`
  //!     defining the environment in which to execute `app_process`. If this
  //!     value is `nullptr`, the application's environment at the time of the
  //!     crash will be used.
  //! \param[in] database The path to a Crashpad database. The handler will be
  //!     started with this path as its `--database` argument.
  //! \param[in] metrics_dir The path to an already existing directory where
  //!     metrics files can be stored. The handler will be started with this
  //!     path as its `--metrics-dir` argument.
  //! \param[in] url The URL of an upload server. The handler will be started
  //!     with this URL as its `--url` argument.
  //! \param[in] annotations Process annotations to set in each crash report.
  //!     The handler will be started with an `--annotation` argument for each
  //!     element in this map.
  //! \param[in] arguments Additional arguments to pass to the Crashpad handler.
  //!     Arguments passed in other parameters and arguments required to perform
  //!     the handshake are the responsibility of this method, and must not be
  //!     specified in this parameter.
  //!
  //! \return `true` on success, `false` on failure with a message logged.
  bool StartJavaHandlerAtCrash(
      const std::string& class_name,
      const std::vector<std::string>* env,
      const base::FilePath& database,
      const base::FilePath& metrics_dir,
      const std::string& url,
      const std::map<std::string, std::string>& annotations,
      const std::vector<std::string>& arguments);

  //! \brief Executes `/system/bin/app_process` and loads a Java class.
  //!
  //! \param[in] class_name The fully qualified class name to load, which must
  //!     define a `main()` method to be invoked by `app_process`. Arguments
  //!     will be passed to this method as though it were the Crashpad handler.
  //!     This class is expected to load a native library defining
  //!     crashpad::HandlerMain() and pass the arguments to it.
  //! \param[in] env A vector of environment variables of the form `var=value`
  //!     defining the environment in which to execute `app_process`. If this
  //!     value is `nullptr`, the application's current environment will be
  //!     used.
  //! \param[in] database The path to a Crashpad database. The handler will be
  //!     started with this path as its `--database` argument.
  //! \param[in] metrics_dir The path to an already existing directory where
  //!     metrics files can be stored. The handler will be started with this
  //!     path as its `--metrics-dir` argument.
  //! \param[in] url The URL of an upload server. The handler will be started
  //!     with this URL as its `--url` argument.
  //! \param[in] annotations Process annotations to set in each crash report.
  //!     The handler will be started with an `--annotation` argument for each
  //!     element in this map.
  //! \param[in] arguments Additional arguments to pass to the Crashpad handler.
  //!     Arguments passed in other parameters and arguments required to perform
  //!     the handshake are the responsibility of this method, and must not be
  //!     specified in this parameter.
  //! \param[in] socket The server end of a socket pair. The client end should
  //!     be used with an ExceptionHandlerClient.
  //!
  //! \return `true` on success, `false` on failure with a message logged.
  static bool StartJavaHandlerForClient(
      const std::string& class_name,
      const std::vector<std::string>* env,
      const base::FilePath& database,
      const base::FilePath& metrics_dir,
      const std::string& url,
      const std::map<std::string, std::string>& annotations,
      const std::vector<std::string>& arguments,
      int socket);

  //! \brief Installs a signal handler to start a Crashpad handler process by
  //!     loading it with `/system/bin/linker`.
  //!
  //! This method is only supported by Android Q+.
  //!
  //! \param[in] handler_trampoline The path to a Crashpad handler trampoline
  //!     executable, possibly located within an apk, e.g.
  //!     "/data/app/myapk.apk!/myabi/libcrashpad_handler_trampoline.so".
  //! \param[in] handler_library The name of a library exporting the symbol
  //!     `CrashpadHandlerMain()`. The path to this library must be present in
  //!     `LD_LIBRARY_PATH`.
  //! \param[in] is_64_bit `true` if \a handler_trampoline and \a
  //!     handler_library are 64-bit objects. They must have the same bitness.
  //! \param[in] env A vector of environment variables of the form `var=value`
  //!     defining the environment in which to execute `app_process`. If this
  //!     value is `nullptr`, the application's environment at the time of the
  //!     crash will be used.
  //! \param[in] database The path to a Crashpad database. The handler will be
  //!     started with this path as its `--database` argument.
  //! \param[in] metrics_dir The path to an already existing directory where
  //!     metrics files can be stored. The handler will be started with this
  //!     path as its `--metrics-dir` argument.
  //! \param[in] url The URL of an upload server. The handler will be started
  //!     with this URL as its `--url` argument.
  //! \param[in] annotations Process annotations to set in each crash report.
  //!     The handler will be started with an `--annotation` argument for each
  //!     element in this map.
  //! \param[in] arguments Additional arguments to pass to the Crashpad handler.
  //!     Arguments passed in other parameters and arguments required to perform
  //!     the handshake are the responsibility of this method, and must not be
  //!     specified in this parameter.
  //!
  //! \return `true` on success, `false` on failure with a message logged.
  bool StartHandlerWithLinkerAtCrash(
      const std::string& handler_trampoline,
      const std::string& handler_library,
      bool is_64_bit,
      const std::vector<std::string>* env,
      const base::FilePath& database,
      const base::FilePath& metrics_dir,
      const std::string& url,
      const std::map<std::string, std::string>& annotations,
      const std::vector<std::string>& arguments);

  //! \brief Starts a Crashpad handler process with an initial client by loading
  //!     it with `/system/bin/linker`.
  //!
  //! This method is only supported by Android Q+.
  //!
  //! \param[in] handler_trampoline The path to a Crashpad handler trampoline
  //!     executable, possibly located within an apk, e.g.
  //!     "/data/app/myapk.apk!/myabi/libcrashpad_handler_trampoline.so".
  //! \param[in] handler_library The name of a library exporting the symbol
  //!     `CrashpadHandlerMain()`. The path to this library must be present in
  //!     `LD_LIBRARY_PATH`.
  //! \param[in] is_64_bit `true` if \a handler_trampoline and \a
  //!     handler_library are 64-bit objects. They must have the same bitness.
  //! \param[in] env A vector of environment variables of the form `var=value`
  //!     defining the environment in which to execute `app_process`. If this
  //!     value is `nullptr`, the application's current environment will be
  //!     used.
  //! \param[in] database The path to a Crashpad database. The handler will be
  //!     started with this path as its `--database` argument.
  //! \param[in] metrics_dir The path to an already existing directory where
  //!     metrics files can be stored. The handler will be started with this
  //!     path as its `--metrics-dir` argument.
  //! \param[in] url The URL of an upload server. The handler will be started
  //!     with this URL as its `--url` argument.
  //! \param[in] annotations Process annotations to set in each crash report.
  //!     The handler will be started with an `--annotation` argument for each
  //!     element in this map.
  //! \param[in] arguments Additional arguments to pass to the Crashpad handler.
  //!     Arguments passed in other parameters and arguments required to perform
  //!     the handshake are the responsibility of this method, and must not be
  //!     specified in this parameter.
  //! \param[in] socket The server end of a socket pair. The client end should
  //!     be used with an ExceptionHandlerClient.
  //!
  //! \return `true` on success, `false` on failure with a message logged.
  static bool StartHandlerWithLinkerForClient(
      const std::string& handler_trampoline,
      const std::string& handler_library,
      bool is_64_bit,
      const std::vector<std::string>* env,
      const base::FilePath& database,
      const base::FilePath& metrics_dir,
      const std::string& url,
      const std::map<std::string, std::string>& annotations,
      const std::vector<std::string>& arguments,
      int socket);
#endif  // BUILDFLAG(IS_ANDROID) || DOXYGEN

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_CHROMEOS) || \
    DOXYGEN
  //! \brief Installs a signal handler to launch a handler process in reponse to
  //!     a crash.
  //!
  //! The handler process will create a crash dump for this process and exit.
  //!
  //! \param[in] handler The path to a Crashpad handler executable.
  //! \param[in] database The path to a Crashpad database. The handler will be
  //!     started with this path as its `--database` argument.
  //! \param[in] metrics_dir The path to an already existing directory where
  //!     metrics files can be stored. The handler will be started with this
  //!     path as its `--metrics-dir` argument.
  //! \param[in] url The URL of an upload server. The handler will be started
  //!     with this URL as its `--url` argument.
  //! \param[in] annotations Process annotations to set in each crash report.
  //!     The handler will be started with an `--annotation` argument for each
  //!     element in this map.
  //! \param[in] arguments Additional arguments to pass to the Crashpad handler.
  //!     Arguments passed in other parameters and arguments required to perform
  //!     the handshake are the responsibility of this method, and must not be
  //!     specified in this parameter.
  //! \param[in] attachments Attachment paths to pass to the Crashpad handler.
  //!     The handler will be started with an `--attachment` argument for each
  //!     path in this vector.
  //!
  //! \return `true` on success, `false` on failure with a message logged.
  bool StartHandlerAtCrash(
      const base::FilePath& handler,
      const base::FilePath& database,
      const base::FilePath& metrics_dir,
      const std::string& url,
      const std::map<std::string, std::string>& annotations,
      const std::vector<std::string>& arguments,
      const std::vector<base::FilePath>& attachments = {});

  //! \brief Starts a handler process with an initial client.
  //!
  //! This method allows a process to launch the handler process on behalf of
  //! another process.
  //!
  //! \param[in] handler The path to a Crashpad handler executable.
  //! \param[in] database The path to a Crashpad database. The handler will be
  //!     started with this path as its `--database` argument.
  //! \param[in] metrics_dir The path to an already existing directory where
  //!     metrics files can be stored. The handler will be started with this
  //!     path as its `--metrics-dir` argument.
  //! \param[in] url The URL of an upload server. The handler will be started
  //!     with this URL as its `--url` argument.
  //! \param[in] annotations Process annotations to set in each crash report.
  //!     The handler will be started with an `--annotation` argument for each
  //!     element in this map.
  //! \param[in] arguments Additional arguments to pass to the Crashpad handler.
  //!     Arguments passed in other parameters and arguments required to perform
  //!     the handshake are the responsibility of this method, and must not be
  //!     specified in this parameter.
  //! \param[in] socket The server end of a socket pair. The client end should
  //!     be used with an ExceptionHandlerClient.
  //!
  //! \return `true` on success, `false` on failure with a message logged.
  static bool StartHandlerForClient(
      const base::FilePath& handler,
      const base::FilePath& database,
      const base::FilePath& metrics_dir,
      const std::string& url,
      const std::map<std::string, std::string>& annotations,
      const std::vector<std::string>& arguments,
      int socket);

  //! \brief Requests that the handler capture a dump even though there hasn't
  //!     been a crash.
  //!
  //! A handler must have already been installed before calling this method.
  //!
  //! TODO(jperaza): Floating point information in the context is zeroed out
  //! until CaptureContext() supports collecting that information.
  //!
  //! \param[in] context A NativeCPUContext, generally captured by
  //!     CaptureContext() or similar.
  static void DumpWithoutCrash(NativeCPUContext* context);

  //! \brief Disables any installed crash handler, not including any
  //!     FirstChanceHandler and crashes the current process.
  //!
  //! \param[in] message A message to be logged before crashing.
  [[noreturn]] static void CrashWithoutDump(const std::string& message);

  //! \brief The type for custom handlers installed by clients.
  using FirstChanceHandler = bool (*)(int, siginfo_t*, ucontext_t*);

  //! \brief Installs a custom crash signal handler which runs before the
  //!     currently installed Crashpad handler.
  //!
  //! Handling signals appropriately can be tricky and use of this method
  //! should be avoided, if possible.
  //!
  //! A handler must have already been installed before calling this method.
  //!
  //! The custom handler runs in a signal handler context and must be safe for
  //! that purpose.
  //!
  //! If the custom handler returns `true`, the signal is considered handled and
  //! the signal handler returns. Otherwise, the currently installed Crashpad
  //! signal handler is run.
  //!
  //! \param[in] handler The custom crash signal handler to install.
  static void SetFirstChanceExceptionHandler(FirstChanceHandler handler);

  //! \brief Installs a custom crash signal handler which runs after the
  //!     currently installed Crashpad handler.
  //!
  //! Handling signals appropriately can be tricky and use of this method
  //! should be avoided, if possible.
  //!
  //! A handler must have already been installed before calling this method.
  //!
  //! The custom handler runs in a signal handler context and must be safe for
  //! that purpose.
  //!
  //! If the custom handler returns `true`, the signal is not reraised.
  //!
  //! \param[in] handler The custom crash signal handler to install.
  static void SetLastChanceExceptionHandler(bool (*handler)(int,
                                                            siginfo_t*,
                                                            ucontext_t*));

  //! \brief Configures a set of signals that shouldn't have Crashpad signal
  //!     handlers installed.
  //!
  //! This method should be called before calling StartHandler(),
  //! SetHandlerSocket(), or other methods that install Crashpad signal
  //! handlers.
  //!
  //! \param[in] unhandled_signals The set of unhandled signals
  void SetUnhandledSignals(const std::set<int>& unhandled_signals);
#endif  // BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_ANDROID) ||
        // BUILDFLAG(IS_CHROMEOS) || DOXYGEN

#if BUILDFLAG(IS_IOS) || DOXYGEN
  //! \brief Observation callback invoked each time this object finishes
  //!     processing and attempting to upload on-disk crash reports (whether or
  //!     not the uploads succeeded).
  //!
  //! This callback is copied into this object. Any references or pointers
  //! inside must outlive this object.
  //!
  //! The callback might be invoked on a background thread, so clients must
  //! synchronize appropriately.
  using ProcessPendingReportsObservationCallback = std::function<void()>;

  //! \brief Configures the process to direct its crashes to the iOS in-process
  //! Crashpad handler.
  //!
  //! This method is only defined on iOS.
  //!
  //! \param[in] database The path to a Crashpad database.
  //! \param[in] url The URL of an upload server.
  //! \param[in] annotations Process annotations to set in each crash report.
  //! \param[in] callback Optional callback invoked zero or more times
  //!     on a background thread each time the handler finishes
  //!     processing and attempting to upload on-disk crash reports.
  //!     If this callback is empty, it is not invoked.
  //! \return `true` on success, `false` on failure with a message logged.
  static bool StartCrashpadInProcessHandler(
      const base::FilePath& database,
      const std::string& url,
      const std::map<std::string, std::string>& annotations,
      ProcessPendingReportsObservationCallback callback);

  //! \brief Requests that the handler convert intermediate dumps into
  //!     minidumps and trigger an upload if possible.
  //!
  //! A handler must have already been installed before calling this method.
  //! This method should be called when an application is ready to start
  //! processing previously created intermediate dumps. Processing will block,
  //! so this should not be called on the main UI thread. No intermediate dumps
  //! will be processed until this method is called.
  //!
  //! \param[in] annotations Process annotations to set in each crash report.
  //!     Useful when adding crash annotations detected on the next run after a
  //!     crash but before upload.
  //! \param[in] user_stream_sources An optional vector containing the
  //!     extensibility data sources to call on crash. Each time a minidump is
  //!     created, the sources are called in turn. Any streams returned are
  //!     added to the minidump.
  static void ProcessIntermediateDumps(
      const std::map<std::string, std::string>& annotations = {},
      const UserStreamDataSources* user_stream_sources = nullptr);

  //! \brief Requests that the handler convert a single intermediate dump at \a
  //!     file generated by DumpWithoutCrashAndDeferProcessingAtPath into a
  //!     minidump and trigger an upload if possible.
  //!
  //! A handler must have already been installed before calling this method.
  //! This method should be called when an application is ready to start
  //! processing previously created intermediate dumps. Processing will block,
  //! so this should not be called on the main UI thread.
  //!
  //! \param[in] file The intermediate dump to process.
  //! \param[in] annotations Process annotations to set in each crash report.
  //!     Useful when adding crash annotations detected on the next run after a
  //!     crash but before upload.
  static void ProcessIntermediateDump(
      const base::FilePath& file,
      const std::map<std::string, std::string>& annotations = {});

  //! \brief Requests that the handler begin in-process uploading of any
  //!     pending reports.
  //!
  //! Once called the handler will start looking for pending reports to upload
  //! on another thread. This method does not block.
  //!
  //! A handler must have already been installed before calling this method.
  //!
  //! \param[in] upload_behavior Controls when the upload thread will run and
  //!     process pending reports. By default, only uploads pending reports
  //!     when the application is active.
  static void StartProcessingPendingReports(
      UploadBehavior upload_behavior = UploadBehavior::kUploadWhenAppIsActive);

  //! \brief Requests that the handler capture an intermediate dump even though
  //!     there hasn't been a crash. The intermediate dump will be converted
  //!     to a mindump immediately. If StartProcessingPendingReports() has been
  //!     called, this will also trigger an upload.
  //!
  //! For internal use only. Clients should use CRASHPAD_SIMULATE_CRASH().
  //!
  //! A handler must have already been installed before calling this method.
  //!
  //! \param[in] context A NativeCPUContext, generally captured by
  //!     CaptureContext() or similar.
  static void DumpWithoutCrash(NativeCPUContext* context);

  //! \brief Requests that the handler capture an intermediate dump even though
  //!     there hasn't been a crash. The intermediate dump will not be converted
  //!     to a mindump until ProcessIntermediateDumps() is called.
  //!
  //! For internal use only. Clients should use
  //! CRASHPAD_SIMULATE_CRASH_AND_DEFER_PROCESSING().
  //!
  //! A handler must have already been installed before calling this method.
  //!
  //! \param[in] context A NativeCPUContext, generally captured by
  //!     CaptureContext() or similar.
  static void DumpWithoutCrashAndDeferProcessing(NativeCPUContext* context);

  //! \brief Requests that the handler capture an intermediate dump and store it
  //!     in path, even though there hasn't been a crash. The intermediate dump
  //!     will not be converted to a mindump until ProcessIntermediateDump() is
  //!     called.
  //!
  //! For internal use only. Clients should use
  //! CRASHPAD_SIMULATE_CRASH_AND_DEFER_PROCESSING_AT_PATH().
  //!
  //! A handler must have already been installed before calling this method.
  //!
  //! \param[in] context A NativeCPUContext, generally captured by
  //!     CaptureContext() or similar.
  //! \param[in] path The path for writing the intermediate dump.
  static void DumpWithoutCrashAndDeferProcessingAtPath(
      NativeCPUContext* context,
      const base::FilePath path);

  //! \brief Unregister the Crashpad client. Intended to be used by tests so
  //!     multiple Crashpad clients can be started and stopped. Not expected to
  //!     be used in a shipping application.
  static void ResetForTesting();

  //! \brief Inject a callback into the Mach exception and signal handling
  //!     mechanisms. Intended to be used by tests to trigger a reentrant
  //      exception.
  static void SetExceptionCallbackForTesting(void (*callback)());

  //! \brief Returns the thread id of the Mach exception thread, used by tests.
  static uint64_t GetThreadIdForTesting();
#endif

#if BUILDFLAG(IS_APPLE) || DOXYGEN
  //! \brief Sets the process’ crash handler to a Mach service registered with
  //!     the bootstrap server.
  //!
  //! This method is only defined on macOS.
  //!
  //! See StartHandler() for more detail on how the port and handler are
  //! configured.
  //!
  //! \param[in] service_name The service name of a Crashpad exception handler
  //!     service previously registered with the bootstrap server.
  //!
  //! \return `true` on success, `false` on failure with a message logged.
  bool SetHandlerMachService(const std::string& service_name);

  //! \brief Sets the process’ crash handler to a Mach port.
  //!
  //! This method is only defined on macOS.
  //!
  //! See StartHandler() for more detail on how the port and handler are
  //! configured.
  //!
  //! \param[in] exception_port An `exception_port_t` corresponding to a
  //!     Crashpad exception handler service.
  //!
  //! \return `true` on success, `false` on failure with a message logged.
  bool SetHandlerMachPort(base::apple::ScopedMachSendRight exception_port);

  //! \brief Retrieves a send right to the process’ crash handler Mach port.
  //!
  //! This method is only defined on macOS.
  //!
  //! This method can be used to obtain the crash handler Mach port when a
  //! Crashpad client process wishes to provide a send right to this port to
  //! another process. The IPC mechanism used to convey the right is under the
  //! application’s control. If the other process wishes to become a client of
  //! the same crash handler, it can provide the transferred right to
  //! SetHandlerMachPort().
  //!
  //! See StartHandler() for more detail on how the port and handler are
  //! configured.
  //!
  //! \return The Mach port set by SetHandlerMachPort(), possibly indirectly by
  //!     a call to another method such as StartHandler() or
  //!     SetHandlerMachService(). This method must only be called after a
  //!     successful call to one of those methods. `MACH_PORT_NULL` on failure
  //!     with a message logged.
  base::apple::ScopedMachSendRight GetHandlerMachPort() const;
#endif

#if BUILDFLAG(IS_WIN) || DOXYGEN
  //! \brief Sets the IPC pipe of a presumably-running Crashpad handler process
  //!     which was started with StartHandler() or by other compatible means
  //!     and does an IPC message exchange to register this process with the
  //!     handler. Crashes will be serviced once this method returns.
  //!
  //! This method is only defined on Windows.
  //!
  //! This method sets the unhandled exception handler to a local
  //! function that when reached will "signal and wait" for the crash handler
  //! process to create the dump.
  //!
  //! \param[in] ipc_pipe The full name of the crash handler IPC pipe. This is
  //!     a string of the form `&quot;\\.\pipe\NAME&quot;`.
  //!
  //! \return `true` on success and `false` on failure.
  bool SetHandlerIPCPipe(const std::wstring& ipc_pipe);

  //! \brief Retrieves the IPC pipe name used to register with the Crashpad
  //!     handler.
  //!
  //! This method is only defined on Windows.
  //!
  //! This method retrieves the IPC pipe name set by SetHandlerIPCPipe(), or a
  //! suitable IPC pipe name chosen by StartHandler(). It must only be called
  //! after a successful call to one of those methods. It is intended to be used
  //! to obtain the IPC pipe name so that it may be passed to other processes,
  //! so that they may register with an existing Crashpad handler by calling
  //! SetHandlerIPCPipe().
  //!
  //! \return The full name of the crash handler IPC pipe, a string of the form
  //!     `&quot;\\.\pipe\NAME&quot;`.
  std::wstring GetHandlerIPCPipe() const;

  //! \brief When `asynchronous_start` is used with StartHandler(), this method
  //!     can be used to block until the handler launch has been completed to
  //!     retrieve status information.
  //!
  //! This method should not be used unless `asynchronous_start` was `true`.
  //!
  //! \param[in] timeout_ms The number of milliseconds to wait for a result from
  //!     the background launch, or `0xffffffff` to block indefinitely.
  //!
  //! \return `true` if the hander startup succeeded, `false` otherwise, and an
  //!     error message will have been logged.
  bool WaitForHandlerStart(unsigned int timeout_ms);

  //! \brief Register a DLL using WerRegisterExceptionModule().
  //!
  //! This method should only be called after a successful call to
  //! SetHandlerIPCPipe() or StartHandler(). The registration is valid for the
  //! lifetime of this object.
  //!
  //! \param[in] full_path The full path to the DLL that will be registered.
  //!     The DLL path should also be set in an appropriate
  //!     `Windows Error Reporting` registry key.
  //!
  //! \return `true` if the DLL was registered. Note: Windows just stashes the
  //!     path somewhere so this returns `true` even if the DLL is not yet
  //!     set in an appropriate registry key, or does not exist.
  bool RegisterWerModule(const std::wstring& full_path);

  //! \brief Requests that the handler capture a dump even though there hasn't
  //!     been a crash.
  //!
  //! \param[in] context A `CONTEXT`, generally captured by CaptureContext() or
  //!     similar.
  static void DumpWithoutCrash(const CONTEXT& context);

  //! \brief Requests that the handler capture a dump using the given \a
  //!     exception_pointers to get the `EXCEPTION_RECORD` and `CONTEXT`.
  //!
  //! This function is not necessary in general usage as an unhandled exception
  //! filter is installed by StartHandler() or SetHandlerIPCPipe().
  //!
  //! \param[in] exception_pointers An `EXCEPTION_POINTERS`, as would generally
  //!     passed to an unhandled exception filter.
  static void DumpAndCrash(EXCEPTION_POINTERS* exception_pointers);

  //! \brief Requests that the handler capture a dump of a different process.
  //!
  //! The target process must be an already-registered Crashpad client. An
  //! exception will be triggered in the target process, and the regular dump
  //! mechanism used. This function will block until the exception in the target
  //! process has been handled by the Crashpad handler.
  //!
  //! This function is unavailable when running on Windows XP and will return
  //! `false`.
  //!
  //! \param[in] process A `HANDLE` identifying the process to be dumped.
  //! \param[in] blame_thread If non-null, a `HANDLE` valid in the caller's
  //!     process, referring to a thread in the target process. If this is
  //!     supplied, instead of the exception referring to the location where the
  //!     exception was injected, an exception record will be fabricated that
  //!     refers to the current location of the given thread.
  //! \param[in] exception_code If \a blame_thread is non-null, this will be
  //!     used as the exception code in the exception record.
  //!
  //! \return `true` if the exception was triggered successfully.
  static bool DumpAndCrashTargetProcess(HANDLE process,
                                        HANDLE blame_thread,
                                        DWORD exception_code);
#endif

#if BUILDFLAG(IS_APPLE) || DOXYGEN
  //! \brief Configures the process to direct its crashes to the default handler
  //!     for the operating system.
  //!
  //! On macOS, this sets the task’s exception port as in SetHandlerMachPort(),
  //! but the exception handler used is obtained from
  //! SystemCrashReporterHandler(). If the system’s crash reporter handler
  //! cannot be determined or set, the task’s exception ports for crash-type
  //! exceptions are cleared.
  //!
  //! Use of this function is strongly discouraged.
  //!
  //! \warning After a call to this function, Crashpad will no longer monitor
  //!     the process for crashes until a subsequent call to
  //!     SetHandlerMachPort().
  //!
  //! \note This is provided as a static function to allow it to be used in
  //!     situations where a CrashpadClient object is not otherwise available.
  //!     This may be useful when a child process inherits its parent’s Crashpad
  //!     handler, but wants to sever this tie.
  static void UseSystemDefaultHandler();
#endif

#if BUILDFLAG(IS_CHROMEOS)
  //! \brief Sets a timestamp on the signal handler to be passed on to
  //!     crashpad_handler and then eventually Chrome OS's crash_reporter.
  //!
  //! \note This method is used by clients that use `StartHandler()` to start
  //!     a handler and not by clients that use any other handler starting
  //!     methods.
  static void SetCrashLoopBefore(uint64_t crash_loop_before_time);
#endif

 private:
#if BUILDFLAG(IS_WIN) || DOXYGEN
  //!  \brief Registers process handlers for the client.
  void RegisterHandlers();
#endif

#if BUILDFLAG(IS_APPLE)
  base::apple::ScopedMachSendRight exception_port_;
#elif BUILDFLAG(IS_WIN)
  std::wstring ipc_pipe_;
  ScopedKernelHANDLE handler_start_thread_;
  ScopedVectoredExceptionRegistration vectored_handler_;
#elif BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_ANDROID)
  std::set<int> unhandled_signals_;
#endif  // BUILDFLAG(IS_APPLE)
};

}  // namespace crashpad

#endif  // CRASHPAD_CLIENT_CRASHPAD_CLIENT_H_
