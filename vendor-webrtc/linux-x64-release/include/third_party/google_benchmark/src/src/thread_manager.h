#ifndef BENCHMARK_THREAD_MANAGER_H
#define BENCHMARK_THREAD_MANAGER_H

#include <atomic>

#include "benchmark/benchmark.h"
#include "mutex.h"

namespace benchmark {
namespace internal {

class ThreadManager {
 public:
  explicit ThreadManager(int num_threads)
      : alive_threads_(num_threads), start_stop_barrier_(num_threads) {}

  Mutex& GetBenchmarkMutex() const RETURN_CAPABILITY(benchmark_mutex_) {
    return benchmark_mutex_;
  }

  bool StartStopBarrier() EXCLUDES(end_cond_mutex_) {
    return start_stop_barrier_.wait();
  }

  void NotifyThreadComplete() EXCLUDES(end_cond_mutex_) {
    start_stop_barrier_.removeThread();
    if (--alive_threads_ == 0) {
      MutexLock lock(end_cond_mutex_);
      end_condition_.notify_all();
    }
  }

  void WaitForAllThreads() EXCLUDES(end_cond_mutex_) {
    MutexLock lock(end_cond_mutex_);
    end_condition_.wait(lock.native_handle(),
                        [this]() { return alive_threads_ == 0; });
  }

  struct Result {
    IterationCount iterations = 0;
    double real_time_used = 0;
    double cpu_time_used = 0;
    double manual_time_used = 0;
    int64_t complexity_n = 0;
    std::string report_label_;
    std::string skip_message_;
    internal::Skipped skipped_ = internal::NotSkipped;
    UserCounters counters;
  };
  GUARDED_BY(GetBenchmarkMutex()) Result results;

 private:
  mutable Mutex benchmark_mutex_;
  std::atomic<int> alive_threads_;
  Barrier start_stop_barrier_;
  Mutex end_cond_mutex_;
  Condition end_condition_;
};

}  // namespace internal
}  // namespace benchmark

#endif  // BENCHMARK_THREAD_MANAGER_H
