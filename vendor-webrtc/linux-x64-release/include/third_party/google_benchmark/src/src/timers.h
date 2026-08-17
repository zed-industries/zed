#ifndef BENCHMARK_TIMERS_H
#define BENCHMARK_TIMERS_H

#include <chrono>
#include <string>

namespace benchmark {

// Return the CPU usage of the current process
double ProcessCPUUsage();

// Return the CPU usage of the children of the current process
double ChildrenCPUUsage();

// Return the CPU usage of the current thread
double ThreadCPUUsage();

#if defined(BENCHMARK_OS_QURT)

// std::chrono::now() can return 0 on some Hexagon devices;
// this reads the value of a 56-bit, 19.2MHz hardware counter
// and converts it to seconds. Unlike std::chrono, this doesn't
// return an absolute time, but since ChronoClockNow() is only used
// to compute elapsed time, this shouldn't matter.
struct QuRTClock {
  typedef uint64_t rep;
  typedef std::ratio<1, 19200000> period;
  typedef std::chrono::duration<rep, period> duration;
  typedef std::chrono::time_point<QuRTClock> time_point;
  static const bool is_steady = false;

  static time_point now() {
    unsigned long long count;
    asm volatile(" %0 = c31:30 " : "=r"(count));
    return time_point(static_cast<duration>(count));
  }
};

#else

#if defined(HAVE_STEADY_CLOCK)
template <bool HighResIsSteady = std::chrono::high_resolution_clock::is_steady>
struct ChooseSteadyClock {
  typedef std::chrono::high_resolution_clock type;
};

template <>
struct ChooseSteadyClock<false> {
  typedef std::chrono::steady_clock type;
};
#endif  // HAVE_STEADY_CLOCK

#endif

struct ChooseClockType {
#if defined(BENCHMARK_OS_QURT)
  typedef QuRTClock type;
#elif defined(HAVE_STEADY_CLOCK)
  typedef ChooseSteadyClock<>::type type;
#else
  typedef std::chrono::high_resolution_clock type;
#endif
};

inline double ChronoClockNow() {
  typedef ChooseClockType::type ClockType;
  using FpSeconds = std::chrono::duration<double, std::chrono::seconds::period>;
  return FpSeconds(ClockType::now().time_since_epoch()).count();
}

std::string LocalDateTimeString();

}  // end namespace benchmark

#endif  // BENCHMARK_TIMERS_H
