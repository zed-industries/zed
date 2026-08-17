/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// RTC_LOG(...) an ostream target that can be used to send formatted
// output to a variety of logging targets, such as debugger console, stderr,
// or any LogSink.
// The severity level passed as the first argument to the logging
// functions is used as a filter, to limit the verbosity of the logging.
// Static members of LogMessage documented below are used to control the
// verbosity and target of the output.
// There are several variations on the RTC_LOG macro which facilitate logging
// of common error conditions, detailed below.

// RTC_LOG(sev) logs the given stream at severity "sev", which must be a
//     compile-time constant of the LoggingSeverity type, without the namespace
//     prefix.
// RTC_LOG_IF(sev, condition) logs the given stream at severity "sev" if
//     "condition" is true.
// RTC_LOG_V(sev) Like RTC_LOG(), but sev is a run-time variable of the
//     LoggingSeverity type (basically, it just doesn't prepend the namespace).
// RTC_LOG_F(sev) Like RTC_LOG(), but includes the name of the current function.
// RTC_LOG_IF_F(sev, condition), Like RTC_LOG_IF(), but includes the name of
//     the current function.
// RTC_LOG_T(sev) Like RTC_LOG(), but includes the this pointer.
// RTC_LOG_T_F(sev) Like RTC_LOG_F(), but includes the this pointer.
// RTC_LOG_GLE(sev [, mod]) attempt to add a string description of the
//     HRESULT returned by GetLastError.
// RTC_LOG_ERRNO(sev) attempts to add a string description of an errno-derived
//     error. errno and associated facilities exist on both Windows and POSIX,
//     but on Windows they only apply to the C/C++ runtime.
// RTC_LOG_ERR(sev) is an alias for the platform's normal error system, i.e.
//     _GLE on Windows and _ERRNO on POSIX.
// (The above three also all have _EX versions that let you specify the error
// code, rather than using the last one.)
// RTC_LOG_E(sev, ctx, err, ...) logs a detailed error interpreted using the
//     specified context.
// RTC_LOG_CHECK_LEVEL(sev) (and RTC_LOG_CHECK_LEVEL_V(sev)) can be used as a
//     test before performing expensive or sensitive operations whose sole
//     purpose is to output logging data at the desired level.

#ifndef RTC_BASE_LOGGING_H_
#define RTC_BASE_LOGGING_H_

#include <errno.h>

#include <atomic>
#include <optional>
#include <sstream>  // no-presubmit-check TODO(webrtc:8982)
#include <string>
#include <type_traits>
#include <utility>

#include "absl/base/attributes.h"
#include "absl/strings/has_absl_stringify.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "api/units/timestamp.h"
#include "rtc_base/platform_thread_types.h"
#include "rtc_base/strings/string_builder.h"
#include "rtc_base/system/inline.h"
#include "rtc_base/type_traits.h"

#if !defined(NDEBUG) || defined(DLOG_ALWAYS_ON)
#define RTC_DLOG_IS_ON 1
#else
#define RTC_DLOG_IS_ON 0
#endif

#if defined(RTC_DISABLE_LOGGING)
#define RTC_LOG_ENABLED() 0
#else
#define RTC_LOG_ENABLED() 1
#endif

namespace webrtc {

//////////////////////////////////////////////////////////////////////
// The meanings of the levels are:
//  LS_VERBOSE: This level is for data which we do not want to appear in the
//   normal debug log, but should appear in diagnostic logs.
//  LS_INFO: Chatty level used in debugging for all sorts of things, the default
//   in debug builds.
//  LS_WARNING: Something that may warrant investigation.
//  LS_ERROR: Something that should not have occurred.
//  LS_NONE: Don't log.
enum LoggingSeverity {
  LS_VERBOSE,
  LS_INFO,
  LS_WARNING,
  LS_ERROR,
  LS_NONE,
};

// LogErrorContext assists in interpreting the meaning of an error value.
enum LogErrorContext {
  ERRCTX_NONE,
  ERRCTX_ERRNO,    // System-local errno
  ERRCTX_HRESULT,  // Windows HRESULT

  // Abbreviations for LOG_E macro
  ERRCTX_EN = ERRCTX_ERRNO,    // LOG_E(sev, EN, x)
  ERRCTX_HR = ERRCTX_HRESULT,  // LOG_E(sev, HR, x)
};

class LogMessage;

// LogLineRef encapsulates all the information required to generate a log line.
// It is used both internally to LogMessage but also as a parameter to
// LogSink::OnLogMessage, allowing custom LogSinks to format the log in
// the most flexible way.
class LogLineRef {
 public:
  absl::string_view message() const { return message_; }
  absl::string_view filename() const { return filename_; }
  int line() const { return line_; }
  std::optional<PlatformThreadId> thread_id() const { return thread_id_; }
  Timestamp timestamp() const { return timestamp_; }
  absl::string_view tag() const { return tag_; }
  LoggingSeverity severity() const { return severity_; }

#if RTC_LOG_ENABLED()
  std::string DefaultLogLine() const;
#else
  std::string DefaultLogLine() const { return ""; }
#endif

 private:
  friend class LogMessage;
  void set_message(std::string message) { message_ = std::move(message); }
  void set_filename(absl::string_view filename) { filename_ = filename; }
  void set_line(int line) { line_ = line; }
  void set_thread_id(std::optional<PlatformThreadId> thread_id) {
    thread_id_ = thread_id;
  }
  void set_timestamp(Timestamp timestamp) { timestamp_ = timestamp; }
  void set_tag(absl::string_view tag) { tag_ = tag; }
  void set_severity(LoggingSeverity severity) { severity_ = severity; }

  std::string message_;
  absl::string_view filename_;
  int line_ = 0;
  std::optional<PlatformThreadId> thread_id_;
  Timestamp timestamp_ = Timestamp::MinusInfinity();
  // The default Android debug output tag.
  absl::string_view tag_ = "libjingle";
  // The severity level of this message
  LoggingSeverity severity_;
};

// Virtual sink interface that can receive log messages.
class LogSink {
 public:
  LogSink() {}
  virtual ~LogSink() {}
  virtual void OnLogMessage(const std::string& msg,
                            LoggingSeverity severity,
                            const char* tag);
  virtual void OnLogMessage(const std::string& message,
                            LoggingSeverity severity);
  virtual void OnLogMessage(const std::string& message) = 0;

  virtual void OnLogMessage(absl::string_view msg,
                            LoggingSeverity severity,
                            const char* tag);
  virtual void OnLogMessage(absl::string_view message,
                            LoggingSeverity severity);
  virtual void OnLogMessage(absl::string_view message);
  virtual void OnLogMessage(const LogLineRef& line);

 private:
  friend class LogMessage;
#if RTC_LOG_ENABLED()
  // Members for LogMessage class to keep linked list of the registered sinks.
  LogSink* next_ = nullptr;
  LoggingSeverity min_severity_;
#endif
};

namespace webrtc_logging_impl {

class LogMetadata {
 public:
  LogMetadata(const char* file, int line, LoggingSeverity severity)
      : file_(file),
        line_and_sev_(static_cast<uint32_t>(line) << 3 | severity) {}
  LogMetadata() = default;

  const char* File() const { return file_; }
  int Line() const { return line_and_sev_ >> 3; }
  LoggingSeverity Severity() const {
    return static_cast<LoggingSeverity>(line_and_sev_ & 0x7);
  }

 private:
  const char* file_;

  // Line number and severity, the former in the most significant 29 bits, the
  // latter in the least significant 3 bits. (This is an optimization; since
  // both numbers are usually compile-time constants, this way we can load them
  // both with a single instruction.)
  uint32_t line_and_sev_;
};
static_assert(std::is_trivial<LogMetadata>::value, "");

struct LogMetadataErr {
  LogMetadata meta;
  LogErrorContext err_ctx;
  int err;
};

#ifdef WEBRTC_ANDROID
struct LogMetadataTag {
  LoggingSeverity severity;
  const char* tag;
};
#endif

enum class LogArgType : int8_t {
  kEnd = 0,
  kInt,
  kLong,
  kLongLong,
  kUInt,
  kULong,
  kULongLong,
  kDouble,
  kLongDouble,
  kCharP,
  kStdString,
  kStringView,
  kVoidP,
  kLogMetadata,
  kLogMetadataErr,
#ifdef WEBRTC_ANDROID
  kLogMetadataTag,
#endif
};

// Wrapper for log arguments. Only ever make values of this type with the
// MakeVal() functions.
template <LogArgType N, typename T>
struct Val {
  static constexpr LogArgType Type() { return N; }
  T GetVal() const { return val; }
  T val;
};

// Case for when we need to construct a temp string and then print that.
// (We can't use Val<CheckArgType::kStdString, const std::string*>
// because we need somewhere to store the temp string.)
struct ToStringVal {
  static constexpr LogArgType Type() { return LogArgType::kStdString; }
  const std::string* GetVal() const { return &val; }
  std::string val;
};

inline Val<LogArgType::kInt, int> MakeVal(int x) {
  return {x};
}
inline Val<LogArgType::kLong, long> MakeVal(long x) {
  return {x};
}
inline Val<LogArgType::kLongLong, long long> MakeVal(long long x) {
  return {x};
}
inline Val<LogArgType::kUInt, unsigned int> MakeVal(unsigned int x) {
  return {x};
}
inline Val<LogArgType::kULong, unsigned long> MakeVal(unsigned long x) {
  return {x};
}
inline Val<LogArgType::kULongLong, unsigned long long> MakeVal(
    unsigned long long x) {
  return {x};
}

inline Val<LogArgType::kDouble, double> MakeVal(double x) {
  return {x};
}

inline Val<LogArgType::kLongDouble, long double> MakeVal(long double x) {
  return {x};
}

inline Val<LogArgType::kCharP, const char*> MakeVal(const char* x) {
  return {x};
}
inline Val<LogArgType::kStdString, const std::string*> MakeVal(
    const std::string& x) {
  return {&x};
}
inline Val<LogArgType::kStringView, const absl::string_view*> MakeVal(
    const absl::string_view& x) {
  return {&x};
}

inline Val<LogArgType::kVoidP, const void*> MakeVal(const void* x) {
  return {x};
}

inline Val<LogArgType::kLogMetadata, LogMetadata> MakeVal(
    const LogMetadata& x) {
  return {x};
}
inline Val<LogArgType::kLogMetadataErr, LogMetadataErr> MakeVal(
    const LogMetadataErr& x) {
  return {x};
}

// The enum class types are not implicitly convertible to arithmetic types.
template <typename T,
          std::enable_if_t<std::is_enum<T>::value &&
                           !absl::HasAbslStringify<T>::value &&
                           !std::is_arithmetic<T>::value>* = nullptr>
inline decltype(MakeVal(std::declval<std::underlying_type_t<T>>())) MakeVal(
    T x) {
  return {static_cast<std::underlying_type_t<T>>(x)};
}

#ifdef WEBRTC_ANDROID
inline Val<LogArgType::kLogMetadataTag, LogMetadataTag> MakeVal(
    const LogMetadataTag& x) {
  return {x};
}
#endif

template <typename T,
          std::enable_if_t<absl::HasAbslStringify<T>::value>* = nullptr>
ToStringVal MakeVal(const T& x) {
  return {absl::StrCat(x)};
}

// Handle arbitrary types other than the above by falling back to stringstream.
// TODO(bugs.webrtc.org/9278): Get rid of this overload when callers don't need
// it anymore. No in-tree caller does, but some external callers still do.
template <typename T,
          typename T1 = std::decay_t<T>,
          std::enable_if_t<std::is_class<T1>::value &&               //
                           !std::is_same<T1, std::string>::value &&  //
                           !std::is_same<T1, LogMetadata>::value &&  //
                           !absl::HasAbslStringify<T1>::value &&
#ifdef WEBRTC_ANDROID
                           !std::is_same<T1, LogMetadataTag>::value &&  //
#endif
                           !std::is_same<T1, LogMetadataErr>::value>* = nullptr>
ToStringVal MakeVal(const T& x) {
  std::ostringstream os;  // no-presubmit-check TODO(webrtc:8982)
  os << x;
  return {os.str()};
}

#if RTC_LOG_ENABLED()
void Log(const LogArgType* fmt, ...);
#else
inline void Log(const LogArgType* fmt, ...) {
  // Do nothing, shouldn't be invoked
}
#endif

// Ephemeral type that represents the result of the logging << operator.
template <typename... Ts>
class LogStreamer;

// Base case: Before the first << argument.
template <>
class LogStreamer<> final {
 public:
  template <typename U, typename V = decltype(MakeVal(std::declval<U>()))>
  RTC_FORCE_INLINE LogStreamer<V> operator<<(const U& arg) const {
    return LogStreamer<V>(MakeVal(arg), this);
  }

  template <typename... Us>
  RTC_FORCE_INLINE static void Call(const Us&... args) {
    static constexpr LogArgType t[] = {Us::Type()..., LogArgType::kEnd};
    Log(t, args.GetVal()...);
  }
};

// Inductive case: We've already seen at least one << argument. The most recent
// one had type `T`, and the earlier ones had types `Ts`.
template <typename T, typename... Ts>
class LogStreamer<T, Ts...> final {
 public:
  RTC_FORCE_INLINE LogStreamer(T arg, const LogStreamer<Ts...>* prior)
      : arg_(arg), prior_(prior) {}

  template <typename U, typename V = decltype(MakeVal(std::declval<U>()))>
  RTC_FORCE_INLINE LogStreamer<V, T, Ts...> operator<<(const U& arg) const {
    return LogStreamer<V, T, Ts...>(MakeVal(arg), this);
  }

  template <typename... Us>
  RTC_FORCE_INLINE void Call(const Us&... args) const {
    prior_->Call(arg_, args...);
  }

 private:
  // The most recent argument.
  T arg_;

  // Earlier arguments.
  const LogStreamer<Ts...>* prior_;
};

class LogCall final {
 public:
  // This can be any binary operator with precedence lower than <<.
  // We return bool here to be able properly remove logging if
  // RTC_DISABLE_LOGGING is defined.
  template <typename... Ts>
  RTC_FORCE_INLINE bool operator&(const LogStreamer<Ts...>& streamer) {
    streamer.Call();
    return true;
  }
};

// This class is used to explicitly ignore values in the conditional
// logging macros.  This avoids compiler warnings like "value computed
// is not used" and "statement has no effect".
class LogMessageVoidify {
 public:
  LogMessageVoidify() = default;
  // This has to be an operator with a precedence lower than << but
  // higher than ?:
  template <typename... Ts>
  void operator&(LogStreamer<Ts...>&& /* streamer */) {}
};

}  // namespace webrtc_logging_impl

// Direct use of this class is deprecated; please use the logging macros
// instead.
// TODO(bugs.webrtc.org/9278): Move this class to an unnamed namespace in the
// .cc file.
class LogMessage {
 public:
  // Same as the above, but using a compile-time constant for the logging
  // severity. This saves space at the call site, since passing an empty struct
  // is generally the same as not passing an argument at all.
  template <LoggingSeverity S>
  RTC_NO_INLINE LogMessage(const char* file,
                           int line,
                           std::integral_constant<LoggingSeverity, S>)
      : LogMessage(file, line, S) {}

#if RTC_LOG_ENABLED()
  LogMessage(const char* file, int line, LoggingSeverity sev);
  LogMessage(const char* file,
             int line,
             LoggingSeverity sev,
             LogErrorContext err_ctx,
             int err);
#if defined(WEBRTC_ANDROID)
  LogMessage(const char* file, int line, LoggingSeverity sev, const char* tag);
#endif
  ~LogMessage();

  LogMessage(const LogMessage&) = delete;
  LogMessage& operator=(const LogMessage&) = delete;

  void AddTag(const char* tag);
  StringBuilder& stream();
  // Returns the time at which this function was called for the first time.
  // The time will be used as the logging start time.
  // If this is not called externally, the LogMessage ctor also calls it, in
  // which case the logging start time will be the time of the first LogMessage
  // instance is created.
  static int64_t LogStartTime();
  // Returns the wall clock equivalent of `LogStartTime`, in seconds from the
  // epoch.
  static uint32_t WallClockStartTime();
  //  LogThreads: Display the thread identifier of the current thread
  static void LogThreads(bool on = true);
  //  LogTimestamps: Display the elapsed time of the program
  static void LogTimestamps(bool on = true);
  // These are the available logging channels
  //  Debug: Debug console on Windows, otherwise stderr
  static void LogToDebug(LoggingSeverity min_sev);
  static LoggingSeverity GetLogToDebug();
  // Sets whether logs will be directed to stderr in debug mode.
  static void SetLogToStderr(bool log_to_stderr);
  // Stream: Any non-blocking stream interface.
  // Installs the `stream` to collect logs with severtiy `min_sev` or higher.
  // `stream` must live until deinstalled by RemoveLogToStream.
  // If `stream` is the first stream added to the system, we might miss some
  // early concurrent log statement happening from another thread happening near
  // this instant.
  static void AddLogToStream(LogSink* stream, LoggingSeverity min_sev);
  // Removes the specified stream, without destroying it. When the method
  // has completed, it's guaranteed that `stream` will receive no more logging
  // calls.
  static void RemoveLogToStream(LogSink* stream);
  // Returns the severity for the specified stream, of if none is specified,
  // the minimum stream severity.
  static int GetLogToStream(LogSink* stream = nullptr);
  // Testing against MinLogSeverity allows code to avoid potentially expensive
  // logging operations by pre-checking the logging level.
  static int GetMinLogSeverity();
  // Parses the provided parameter stream to configure the options above.
  // Useful for configuring logging from the command line.
  static void ConfigureLogging(absl::string_view params);
  // Checks the current global debug severity and if the `streams_` collection
  // is empty. If `severity` is smaller than the global severity and if the
  // `streams_` collection is empty, the LogMessage will be considered a noop
  // LogMessage.
  static bool IsNoop(LoggingSeverity severity);
  // Version of IsNoop that uses fewer instructions at the call site, since the
  // caller doesn't have to pass an argument.
  template <LoggingSeverity S>
  RTC_NO_INLINE static bool IsNoop() {
    return IsNoop(S);
  }
#else
  // Next methods do nothing; no one will call these functions.
  LogMessage(const char* file, int line, LoggingSeverity sev) {}
  LogMessage(const char* file,
             int line,
             LoggingSeverity sev,
             LogErrorContext err_ctx,
             int err) {}
#if defined(WEBRTC_ANDROID)
  LogMessage(const char* file, int line, LoggingSeverity sev, const char* tag) {
  }
#endif
  ~LogMessage() = default;

  inline void AddTag(const char* tag) {}
  inline StringBuilder& stream() { return print_stream_; }
  inline static int64_t LogStartTime() { return 0; }
  inline static uint32_t WallClockStartTime() { return 0; }
  inline static void LogThreads(bool on = true) {}
  inline static void LogTimestamps(bool on = true) {}
  inline static void LogToDebug(LoggingSeverity min_sev) {}
  inline static LoggingSeverity GetLogToDebug() {
    return LoggingSeverity::LS_INFO;
  }
  inline static void SetLogToStderr(bool log_to_stderr) {}
  inline static void AddLogToStream(LogSink* stream, LoggingSeverity min_sev) {}
  inline static void RemoveLogToStream(LogSink* stream) {}
  inline static int GetLogToStream(LogSink* stream = nullptr) { return 0; }
  inline static int GetMinLogSeverity() { return 0; }
  inline static void ConfigureLogging(absl::string_view params) {}
  static constexpr bool IsNoop(LoggingSeverity severity) { return true; }
  template <LoggingSeverity S>
  static constexpr bool IsNoop() {
    return IsNoop(S);
  }
#endif  // RTC_LOG_ENABLED()

 private:
  friend class LogMessageForTesting;

#if RTC_LOG_ENABLED()
  // Updates min_sev_ appropriately when debug sinks change.
  static void UpdateMinLogSeverity();

  // This writes out the actual log messages.
  static void OutputToDebug(const LogLineRef& log_line_ref);

  // Called from the dtor (or from a test) to append optional extra error
  // information to the log stream and a newline character.
  void FinishPrintStream();

  LogLineRef log_line_;

  // String data generated in the constructor, that should be appended to
  // the message before output.
  std::string extra_;

  // The output streams and their associated severities
  static LogSink* streams_;

  // Holds true with high probability if `streams_` is empty, false with high
  // probability otherwise. Operated on with std::memory_order_relaxed because
  // it's ok to lose or log some additional statements near the instant streams
  // are added/removed.
  static std::atomic<bool> streams_empty_;

  // Flags for formatting options and their potential values.
  static bool log_thread_;
  static bool log_timestamp_;

  // Determines if logs will be directed to stderr in debug mode.
  static bool log_to_stderr_;
#else  // RTC_LOG_ENABLED()
  // Next methods do nothing; no one will call these functions.
  inline static void UpdateMinLogSeverity() {}
#if defined(WEBRTC_ANDROID)
  inline static void OutputToDebug(absl::string_view filename,
                                   int line,
                                   absl::string_view msg,
                                   LoggingSeverity severity,
                                   const char* tag) {}
#else
  inline static void OutputToDebug(absl::string_view filename,
                                   int line,
                                   absl::string_view msg,
                                   LoggingSeverity severity) {}
#endif  // defined(WEBRTC_ANDROID)
  inline void FinishPrintStream() {}
#endif  // RTC_LOG_ENABLED()

  // The stringbuilder that buffers the formatted message before output
  StringBuilder print_stream_;
};

//////////////////////////////////////////////////////////////////////
// Logging Helpers
//////////////////////////////////////////////////////////////////////

#define RTC_LOG_FILE_LINE(sev, file, line)           \
  ::webrtc::webrtc_logging_impl::LogCall() &         \
      ::webrtc::webrtc_logging_impl::LogStreamer<>() \
          << ::webrtc::webrtc_logging_impl::LogMetadata(file, line, sev)

#define RTC_LOG(sev)                                \
  !::webrtc::LogMessage::IsNoop<::webrtc::sev>() && \
      RTC_LOG_FILE_LINE(::webrtc::sev, __FILE__, __LINE__)

#define RTC_LOG_IF(sev, condition)                                 \
  !::webrtc::LogMessage::IsNoop<::webrtc::sev>() && (condition) && \
      RTC_LOG_FILE_LINE(::webrtc::sev, __FILE__, __LINE__)

// The _V version is for when a variable is passed in.
#define RTC_LOG_V(sev)                  \
  !::webrtc::LogMessage::IsNoop(sev) && \
      RTC_LOG_FILE_LINE(sev, __FILE__, __LINE__)

// The _F version prefixes the message with the current function name.
#if (defined(__GNUC__) && !defined(NDEBUG)) || defined(WANT_PRETTY_LOG_F)
#define RTC_LOG_F(sev) RTC_LOG(sev) << __PRETTY_FUNCTION__ << ": "
#define RTC_LOG_IF_F(sev, condition) \
  RTC_LOG_IF(sev, condition) << __PRETTY_FUNCTION__ << ": "
#define RTC_LOG_T_F(sev) \
  RTC_LOG(sev) << this << ": " << __PRETTY_FUNCTION__ << ": "
#else
#define RTC_LOG_F(sev) RTC_LOG(sev) << __FUNCTION__ << ": "
#define RTC_LOG_IF_F(sev, condition) \
  RTC_LOG_IF(sev, condition) << __FUNCTION__ << ": "
#define RTC_LOG_T_F(sev) RTC_LOG(sev) << this << ": " << __FUNCTION__ << ": "
#endif

#define RTC_LOG_CHECK_LEVEL(sev) ::webrtc::LogCheckLevel(::webrtc::sev)
#define RTC_LOG_CHECK_LEVEL_V(sev) ::webrtc::LogCheckLevel(sev)

inline bool LogCheckLevel(LoggingSeverity sev) {
  return (LogMessage::GetMinLogSeverity() <= sev);
}

#define RTC_LOG_E(sev, ctx, err)                                       \
  !::webrtc::LogMessage::IsNoop<::webrtc::sev>() &&                    \
      ::webrtc::webrtc_logging_impl::LogCall() &                       \
          ::webrtc::webrtc_logging_impl::LogStreamer<>()               \
              << ::webrtc::webrtc_logging_impl::LogMetadataErr {       \
    {__FILE__, __LINE__, ::webrtc::sev}, ::webrtc::ERRCTX_##ctx, (err) \
  }

#define RTC_LOG_T(sev) RTC_LOG(sev) << this << ": "

#define RTC_LOG_ERRNO_EX(sev, err) RTC_LOG_E(sev, ERRNO, err)
#define RTC_LOG_ERRNO(sev) RTC_LOG_ERRNO_EX(sev, errno)

#if defined(WEBRTC_WIN)
#define RTC_LOG_GLE_EX(sev, err) RTC_LOG_E(sev, HRESULT, err)
#define RTC_LOG_GLE(sev) RTC_LOG_GLE_EX(sev, static_cast<int>(GetLastError()))
#define RTC_LOG_ERR_EX(sev, err) RTC_LOG_GLE_EX(sev, err)
#define RTC_LOG_ERR(sev) RTC_LOG_GLE(sev)
#elif defined(__native_client__) && __native_client__
#define RTC_LOG_ERR_EX(sev, err) RTC_LOG(sev)
#define RTC_LOG_ERR(sev) RTC_LOG(sev)
#elif defined(WEBRTC_POSIX)
#define RTC_LOG_ERR_EX(sev, err) RTC_LOG_ERRNO_EX(sev, err)
#define RTC_LOG_ERR(sev) RTC_LOG_ERRNO(sev)
#endif  // WEBRTC_WIN

#ifdef WEBRTC_ANDROID

namespace webrtc_logging_impl {
// TODO(kwiberg): Replace these with absl::string_view.
inline const char* AdaptString(const char* str) {
  return str;
}
inline const char* AdaptString(const std::string& str) {
  return str.c_str();
}
}  // namespace webrtc_logging_impl

#define RTC_LOG_TAG(sev, tag)                                    \
  !::webrtc::LogMessage::IsNoop(sev) &&                          \
      ::webrtc::webrtc_logging_impl::LogCall() &                 \
          ::webrtc::webrtc_logging_impl::LogStreamer<>()         \
              << ::webrtc::webrtc_logging_impl::LogMetadataTag { \
    sev, ::webrtc::webrtc_logging_impl::AdaptString(tag)         \
  }

#else

// DEPRECATED. This macro is only intended for Android.
#define RTC_LOG_TAG(sev, tag) RTC_LOG_V(sev)

#endif

// The RTC_DLOG macros are equivalent to their RTC_LOG counterparts except that
// they only generate code in debug builds.
#if RTC_DLOG_IS_ON
#define RTC_DLOG(sev) RTC_LOG(sev)
#define RTC_DLOG_IF(sev, condition) RTC_LOG_IF(sev, condition)
#define RTC_DLOG_V(sev) RTC_LOG_V(sev)
#define RTC_DLOG_F(sev) RTC_LOG_F(sev)
#define RTC_DLOG_IF_F(sev, condition) RTC_LOG_IF_F(sev, condition)
#else
#define RTC_DLOG_EAT_STREAM_PARAMS()                   \
  while (false)                                        \
  ::webrtc::webrtc_logging_impl::LogMessageVoidify() & \
      (::webrtc::webrtc_logging_impl::LogStreamer<>())
#define RTC_DLOG(sev) RTC_DLOG_EAT_STREAM_PARAMS()
#define RTC_DLOG_IF(sev, condition) RTC_DLOG_EAT_STREAM_PARAMS()
#define RTC_DLOG_V(sev) RTC_DLOG_EAT_STREAM_PARAMS()
#define RTC_DLOG_F(sev) RTC_DLOG_EAT_STREAM_PARAMS()
#define RTC_DLOG_IF_F(sev, condition) RTC_DLOG_EAT_STREAM_PARAMS()
#endif

}  // namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::LoggingSeverity;
using ::webrtc::LogLineRef;
using ::webrtc::LogMessage;
using ::webrtc::LogSink;
using ::webrtc::LS_ERROR;
using ::webrtc::LS_INFO;
using ::webrtc::LS_NONE;
using ::webrtc::LS_VERBOSE;
using ::webrtc::LS_WARNING;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_LOGGING_H_
