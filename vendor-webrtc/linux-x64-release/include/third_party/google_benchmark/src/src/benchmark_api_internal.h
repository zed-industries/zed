#ifndef BENCHMARK_API_INTERNAL_H
#define BENCHMARK_API_INTERNAL_H

#include <cmath>
#include <iosfwd>
#include <limits>
#include <memory>
#include <string>
#include <vector>

#include "benchmark/benchmark.h"
#include "commandlineflags.h"

namespace benchmark {
namespace internal {

// Information kept per benchmark we may want to run
class BenchmarkInstance {
 public:
  BenchmarkInstance(Benchmark* benchmark, int family_index,
                    int per_family_instance_index,
                    const std::vector<int64_t>& args, int threads);

  const BenchmarkName& name() const { return name_; }
  int family_index() const { return family_index_; }
  int per_family_instance_index() const { return per_family_instance_index_; }
  AggregationReportMode aggregation_report_mode() const {
    return aggregation_report_mode_;
  }
  TimeUnit time_unit() const { return time_unit_; }
  bool measure_process_cpu_time() const { return measure_process_cpu_time_; }
  bool use_real_time() const { return use_real_time_; }
  bool use_manual_time() const { return use_manual_time_; }
  BigO complexity() const { return complexity_; }
  BigOFunc* complexity_lambda() const { return complexity_lambda_; }
  const std::vector<Statistics>& statistics() const { return statistics_; }
  int repetitions() const { return repetitions_; }
  double min_time() const { return min_time_; }
  double min_warmup_time() const { return min_warmup_time_; }
  IterationCount iterations() const { return iterations_; }
  int threads() const { return threads_; }
  void Setup() const;
  void Teardown() const;

  State Run(IterationCount iters, int thread_id, internal::ThreadTimer* timer,
            internal::ThreadManager* manager,
            internal::PerfCountersMeasurement* perf_counters_measurement,
            ProfilerManager* profiler_manager) const;

 private:
  BenchmarkName name_;
  Benchmark& benchmark_;
  const int family_index_;
  const int per_family_instance_index_;
  AggregationReportMode aggregation_report_mode_;
  const std::vector<int64_t>& args_;
  TimeUnit time_unit_;
  bool measure_process_cpu_time_;
  bool use_real_time_;
  bool use_manual_time_;
  BigO complexity_;
  BigOFunc* complexity_lambda_;
  UserCounters counters_;
  const std::vector<Statistics>& statistics_;
  int repetitions_;
  double min_time_;
  double min_warmup_time_;
  IterationCount iterations_;
  int threads_;  // Number of concurrent threads to us

  typedef void (*callback_function)(const benchmark::State&);
  callback_function setup_ = nullptr;
  callback_function teardown_ = nullptr;
};

bool FindBenchmarksInternal(const std::string& re,
                            std::vector<BenchmarkInstance>* benchmarks,
                            std::ostream* Err);

bool IsZero(double n);

BENCHMARK_EXPORT
ConsoleReporter::OutputOptions GetOutputOptions(bool force_no_color = false);

}  // end namespace internal
}  // end namespace benchmark

#endif  // BENCHMARK_API_INTERNAL_H
