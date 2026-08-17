#ifndef BENCHMARK_THREAD_TIMER_H
#define BENCHMARK_THREAD_TIMER_H

#include "check.h"
#include "timers.h"

namespace benchmark {
namespace internal {

class ThreadTimer {
  explicit ThreadTimer(bool measure_process_cpu_time_)
      : measure_process_cpu_time(measure_process_cpu_time_) {}

 public:
  static ThreadTimer Create() {
    return ThreadTimer(/*measure_process_cpu_time_=*/false);
  }
  static ThreadTimer CreateProcessCpuTime() {
    return ThreadTimer(/*measure_process_cpu_time_=*/true);
  }

  // Called by each thread
  void StartTimer() {
    running_ = true;
    start_real_time_ = ChronoClockNow();
    start_cpu_time_ = ReadCpuTimerOfChoice();
  }

  // Called by each thread
  void StopTimer() {
    BM_CHECK(running_);
    running_ = false;
    real_time_used_ += ChronoClockNow() - start_real_time_;
    // Floating point error can result in the subtraction producing a negative
    // time. Guard against that.
    cpu_time_used_ +=
        std::max<double>(ReadCpuTimerOfChoice() - start_cpu_time_, 0);
  }

  // Called by each thread
  void SetIterationTime(double seconds) { manual_time_used_ += seconds; }

  bool running() const { return running_; }

  // REQUIRES: timer is not running
  double real_time_used() const {
    BM_CHECK(!running_);
    return real_time_used_;
  }

  // REQUIRES: timer is not running
  double cpu_time_used() const {
    BM_CHECK(!running_);
    return cpu_time_used_;
  }

  // REQUIRES: timer is not running
  double manual_time_used() const {
    BM_CHECK(!running_);
    return manual_time_used_;
  }

 private:
  double ReadCpuTimerOfChoice() const {
    if (measure_process_cpu_time) return ProcessCPUUsage();
    return ThreadCPUUsage();
  }

  // should the thread, or the process, time be measured?
  const bool measure_process_cpu_time;

  bool running_ = false;        // Is the timer running
  double start_real_time_ = 0;  // If running_
  double start_cpu_time_ = 0;   // If running_

  // Accumulated time so far (does not contain current slice if running_)
  double real_time_used_ = 0;
  double cpu_time_used_ = 0;
  // Manually set iteration time. User sets this with SetIterationTime(seconds).
  double manual_time_used_ = 0;
};

}  // namespace internal
}  // namespace benchmark

#endif  // BENCHMARK_THREAD_TIMER_H
