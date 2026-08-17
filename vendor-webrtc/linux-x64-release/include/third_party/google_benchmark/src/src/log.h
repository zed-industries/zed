#ifndef BENCHMARK_LOG_H_
#define BENCHMARK_LOG_H_

#include <iostream>
#include <ostream>

// NOTE: this is also defined in benchmark.h but we're trying to avoid a
// dependency.
// The _MSVC_LANG check should detect Visual Studio 2015 Update 3 and newer.
#if __cplusplus >= 201103L || (defined(_MSVC_LANG) && _MSVC_LANG >= 201103L)
#define BENCHMARK_HAS_CXX11
#endif

namespace benchmark {
namespace internal {

typedef std::basic_ostream<char>&(EndLType)(std::basic_ostream<char>&);

class LogType {
  friend LogType& GetNullLogInstance();
  friend LogType& GetErrorLogInstance();

  // FIXME: Add locking to output.
  template <class Tp>
  friend LogType& operator<<(LogType&, Tp const&);
  friend LogType& operator<<(LogType&, EndLType*);

 private:
  LogType(std::ostream* out) : out_(out) {}
  std::ostream* out_;

  // NOTE: we could use BENCHMARK_DISALLOW_COPY_AND_ASSIGN but we shouldn't have
  // a dependency on benchmark.h from here.
#ifndef BENCHMARK_HAS_CXX11
  LogType(const LogType&);
  LogType& operator=(const LogType&);
#else
  LogType(const LogType&) = delete;
  LogType& operator=(const LogType&) = delete;
#endif
};

template <class Tp>
LogType& operator<<(LogType& log, Tp const& value) {
  if (log.out_) {
    *log.out_ << value;
  }
  return log;
}

inline LogType& operator<<(LogType& log, EndLType* m) {
  if (log.out_) {
    *log.out_ << m;
  }
  return log;
}

inline int& LogLevel() {
  static int log_level = 0;
  return log_level;
}

inline LogType& GetNullLogInstance() {
  static LogType null_log(static_cast<std::ostream*>(nullptr));
  return null_log;
}

inline LogType& GetErrorLogInstance() {
  static LogType error_log(&std::clog);
  return error_log;
}

inline LogType& GetLogInstanceForLevel(int level) {
  if (level <= LogLevel()) {
    return GetErrorLogInstance();
  }
  return GetNullLogInstance();
}

}  // end namespace internal
}  // end namespace benchmark

// clang-format off
#define BM_VLOG(x)                                                               \
  (::benchmark::internal::GetLogInstanceForLevel(x) << "-- LOG(" << x << "):" \
                                                                         " ")
// clang-format on
#endif
