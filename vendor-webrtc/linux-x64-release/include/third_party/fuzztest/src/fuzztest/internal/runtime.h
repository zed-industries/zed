// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef FUZZTEST_FUZZTEST_INTERNAL_RUNTIME_H_
#define FUZZTEST_FUZZTEST_INTERNAL_RUNTIME_H_

#include <atomic>
#include <cstddef>
#include <cstdio>
#include <deque>
#include <iostream>
#include <memory>
#include <optional>
#include <random>
#include <string>
#include <type_traits>
#include <utility>
#include <vector>

#include "absl/base/thread_annotations.h"
#include "absl/functional/any_invocable.h"
#include "absl/functional/function_ref.h"
#include "absl/random/bit_gen_ref.h"
#include "absl/random/discrete_distribution.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/str_format.h"
#include "absl/strings/string_view.h"
#include "absl/time/clock.h"
#include "absl/time/time.h"
#include "absl/types/span.h"
#include "./fuzztest/internal/configuration.h"
#include "./fuzztest/internal/coverage.h"
#include "./fuzztest/internal/domains/domain.h"
#include "./fuzztest/internal/fixture_driver.h"
#include "./fuzztest/internal/logging.h"
#include "./fuzztest/internal/registration.h"
#include "./fuzztest/internal/seed_seq.h"

namespace fuzztest {

// The mode in which we are running the fuzz tests.
enum class RunMode {
  // Run without instrumentation and coverage-guidance for a short time.
  kUnitTest,

  // Run coverage-guided fuzzing until a failure is detected or the test is
  // manually terminated.
  kFuzz
};

namespace internal {

class FuzzTestFuzzer {
 public:
  virtual ~FuzzTestFuzzer() = default;
  // Returns ture if no error were detected by the FuzzTest, false otherwise.
  virtual bool RunInUnitTestMode(const Configuration& configuration) = 0;
  // Returns ture if no error were detected by the FuzzTest, false otherwise.
  virtual bool RunInFuzzingMode(int* argc, char*** argv,
                                const Configuration& configuration) = 0;
};

class FuzzTest;

using FuzzTestFuzzerFactory =
    absl::AnyInvocable<std::unique_ptr<FuzzTestFuzzer>(const FuzzTest&) const>;

class FuzzTest {
 public:
  FuzzTest(BasicTestInfo test_info, FuzzTestFuzzerFactory factory)
      : test_info_(std::move(test_info)), make_(std::move(factory)) {}

  const std::string& suite_name() const { return test_info_.suite_name; }
  const std::string& test_name() const { return test_info_.test_name; }
  std::string full_name() const {
    return absl::StrCat(test_info_.suite_name, ".", test_info_.test_name);
  }
  const std::string& file() const { return test_info_.file; }
  int line() const { return test_info_.line; }
  bool uses_fixture() const { return test_info_.uses_fixture; }
  auto make() const { return make_(*this); }

 private:
  BasicTestInfo test_info_;
  FuzzTestFuzzerFactory make_;
};

struct RuntimeStats {
  absl::Time start_time;
  size_t runs;
  size_t edges_covered;
  size_t total_edges;
  // Number of executed inputs that increase coverage.
  size_t useful_inputs;
  size_t max_stack_used;
};

void InstallSignalHandlers(FILE* report_out);

// A function that is called when crash metadata is available.
using CrashMetadataListener =
    absl::AnyInvocable<void(absl::string_view crash_type,
                            absl::Span<const std::string> stack_frames) const>;
using CrashMetadataListenerRef =
    absl::FunctionRef<void(absl::string_view crash_type,
                           absl::Span<const std::string> stack_frames) const>;

// This class encapsulates the runtime state that is global by necessity.
// The state is accessed by calling `Runtime::instance()`, which handles the
// necessary initialization steps.
class Runtime {
 public:
  static Runtime& instance() {
    static auto* runtime = new Runtime();
    return *runtime;
  }

  void SetExternalFailureDetected(bool v) {
    external_failure_was_detected_.store(v, std::memory_order_relaxed);
  }
  bool external_failure_detected() const {
    return external_failure_was_detected_.load(std::memory_order_relaxed);
  }

  void SetSkippingRequested(bool requested) {
    skipping_requested_.store(requested, std::memory_order_relaxed);
  }

  bool skipping_requested() const {
    return skipping_requested_.load(std::memory_order_relaxed);
  }

  void SetShouldTerminateOnNonFatalFailure(bool v) {
    should_terminate_on_non_fatal_failure_ = v;
  }

  bool should_terminate_on_non_fatal_failure() const {
    return should_terminate_on_non_fatal_failure_;
  }

  void SetTerminationRequested() {
    termination_requested_.store(true, std::memory_order_relaxed);
  }

  bool termination_requested() const {
    return termination_requested_.load(std::memory_order_relaxed);
  }

  void SetRunMode(RunMode run_mode) { run_mode_ = run_mode; }
  RunMode run_mode() const { return run_mode_; }

  // Enables the crash reporter.
  // REQUIRES: `SetCurrentTest()` has been called with non-null arguments.
  void EnableReporter(const RuntimeStats* stats, absl::Time (*clock_fn)()) {
    reporter_enabled_ = true;
    stats_ = stats;
    clock_fn_ = clock_fn;
    // In case we have not installed them yet, do so now.
    InstallSignalHandlers(GetStderr());
    ResetCrashType();
  }
  void DisableReporter() { reporter_enabled_ = false; }

  struct Args {
    const GenericDomainCorpusType& corpus_value;
    UntypedDomain& domain;
  };

  // Sets the current test and configuration.
  // REQUIRES: Before passing null arguments, the reporter must be disabled by
  // calling `DisableReporter()`.
  void SetCurrentTest(const FuzzTest* test, const Configuration* configuration);

  void OnTestIterationStart(const absl::Time& start_time);
  void OnTestIterationEnd();

  void SetCurrentArgs(Args* args) { current_args_ = args; }
  void UnsetCurrentArgs() { current_args_ = nullptr; }

  void PrintFinalStats(absl::FormatRawSink out) const;
  void PrintFinalStatsOnDefaultSink() const;
  void PrintReport(absl::FormatRawSink out) const;
  void PrintReportOnDefaultSink() const;

  // Registers a crash metadata listener that will be called when crash metadata
  // is available.
  void RegisterCrashMetadataListener(CrashMetadataListener listener) {
    crash_metadata_listeners_.push_back(std::move(listener));
  }
  void SetCrashTypeIfUnset(std::string crash_type) {
    if (!crash_type_.has_value()) {
      crash_type_ = std::move(crash_type);
    }
  }
  void ResetCrashType() { crash_type_ = std::nullopt; }

  class Watchdog;
  // Returns a watchdog that periodically checks the time and memory limits in a
  // separate thread. The watchdog handles the logic of starting and joining the
  // thread. The runtime must outlive the watchdog.
  Watchdog CreateWatchdog();

 private:
  Runtime();

  // Checks time and memory limits. Aborts the process if any limit is exceeded.
  void CheckWatchdogLimits();

  // Returns the file path of the reproducer.
  // Returns empty string if writing the file failed.
  std::string DumpReproducer(absl::string_view outdir) const;

  // Some failures are not necessarily detected by signal handlers or by
  // sanitizers. For example, we could have test framework failures like
  // `EXPECT_EQ` failures from GoogleTest.
  // If such a failure is detected, the external system can set
  // `external_failure_was_detected` to true to bubble it up.
  // Note: Even though failures should happen within the code under test, they
  // could be set from other threads at any moment. We make it an atomic to
  // avoid a race condition.
  std::atomic<bool> external_failure_was_detected_ = false;

  // To support in-process minimization for non-fatal failures we signal
  // suppress termination until we believe minimization is complete.
  bool should_terminate_on_non_fatal_failure_ = true;

  // If set to true in fixture setup, skips calling property functions
  // utill the matching teardown is called; If set to true in a property
  // function, skip adding the current input to the corpus when fuzzing.
  std::atomic<bool> skipping_requested_ = false;

  // If true, fuzzing should terminate as soon as possible.
  // Atomic because it is set from signal handlers.
  std::atomic<bool> termination_requested_ = false;

  RunMode run_mode_ = RunMode::kUnitTest;

  absl::Time creation_time_ = absl::Now();
  size_t test_counter_ = 0;

  bool reporter_enabled_ = false;
  Args* current_args_ = nullptr;
  const FuzzTest* current_test_ = nullptr;
  const Configuration* current_configuration_;
  const RuntimeStats* stats_ = nullptr;
  absl::Time (*clock_fn_)() = nullptr;

  // We use a simple custom spinlock instead of absl::Mutex to reduce
  // dependencies and avoid potential issues with code instrumentation.
  class ABSL_LOCKABLE Spinlock {
   public:
    Spinlock() = default;
    Spinlock(const Spinlock&) = delete;
    Spinlock& operator=(const Spinlock&) = delete;

    void Lock() ABSL_EXCLUSIVE_LOCK_FUNCTION();
    void Unlock() ABSL_UNLOCK_FUNCTION();

   private:
    std::atomic_flag locked_ = ATOMIC_FLAG_INIT;
  };

  Spinlock watchdog_spinlock_;
  absl::Time current_iteration_start_time_ ABSL_GUARDED_BY(watchdog_spinlock_);
  bool test_iteration_started_ ABSL_GUARDED_BY(watchdog_spinlock_) = false;
  bool watchdog_limit_exceeded_ ABSL_GUARDED_BY(watchdog_spinlock_) = false;

  // A registry of crash metadata listeners.
  std::vector<CrashMetadataListener> crash_metadata_listeners_;
  // In case of a crash, contains the crash type.
  std::optional<std::string> crash_type_;
};

extern void (*crash_handler_hook)();

template <typename Arg, size_t I, typename Tuple>
decltype(auto) GetDomainOrArbitrary(const Tuple& t) {
  if constexpr (I < std::tuple_size_v<Tuple>) {
    return std::get<I>(t);
  } else {
    return Arbitrary<std::decay_t<Arg>>();
  }
}

class FuzzTestExternalEngineAdaptor;

class FuzzTestFuzzerImpl : public FuzzTestFuzzer {
 public:
  explicit FuzzTestFuzzerImpl(
      const FuzzTest& test,
      std::unique_ptr<UntypedFixtureDriver> fixture_driver);
  ~FuzzTestFuzzerImpl();

 private:
  // TODO(fniksic): Refactor to reduce code complexity and improve readability.
  bool RunInUnitTestMode(const Configuration& configuration) override;

  // TODO(fniksic): Refactor to reduce code complexity and improve readability.
  bool RunInFuzzingMode(int* argc, char*** argv,
                        const Configuration& configuration) override;

  // Use the standard PRNG instead of absl::BitGen because Abseil doesn't
  // guarantee seed stability
  // (https://abseil.io/docs/cpp/guides/random#seed-stability).
  using PRNG = std::mt19937;
  using corpus_type = GenericDomainCorpusType;

  struct Input {
    corpus_type args;
    size_t depth = 0;
    absl::Duration run_time = absl::ZeroDuration();
  };
  struct RunResult {
    bool new_coverage;
    absl::Duration run_time;
  };

  void PopulateFromSeeds(const std::vector<std::string>& corpus_files);

  bool ReplayInputsIfAvailable(const Configuration& configuration);

  std::optional<std::vector<std::string>> GetFilesToReplay();

  std::optional<corpus_type> ReadReproducerToMinimize();

  absl::StatusOr<corpus_type> TryParse(absl::string_view data);

  void MutateValue(Input& input, absl::BitGenRef prng,
                   const domain_implementor::MutationMetadata& metadata);

  void UpdateCorpusDistribution();

  void MinimizeNonFatalFailureLocally(absl::BitGenRef prng);

  // Runs on `sample` and returns new coverage and run time. If there's new
  // coverage, outputs updated runtime stats. Additionally, if `write_to_file`
  // is true, tries to write the sample to a file.
  RunResult TrySample(const Input& sample, bool write_to_file = true);

  // Runs on `sample` and records it into the in-memory corpus if it finds new
  // coverage. If `write_to_file` is set, tries to write the corpus data to a
  // file when recording it. Updates the memory dictionary on new coverage, and
  // occasionally even if there is no new coverage.
  void TrySampleAndUpdateInMemoryCorpus(Input sample,
                                        bool write_to_file = true);

  // Iterates over inputs in `files` and calls `consume` on each input.
  // `consume` is a function that takes a file path, an optional blob index in
  // the file (for blob files with multiple blobs), and an input in the given
  // file at the given blob index (if applicable). When `timeout` is reached
  // before calling the `consume` on an input, the iteration stops and the rest
  // of the inputs won't be consumed.
  void ForEachInput(
      absl::Span<const std::string> files,
      absl::FunctionRef<void(absl::string_view file_path,
                             std::optional<int> blob_idx, Input input)>
          consume,
      absl::Duration timeout = absl::InfiniteDuration());

  // Returns true if we're in minimization mode.
  bool MinimizeCorpusIfInMinimizationMode(absl::BitGenRef prng);

  std::vector<Input> TryReadCorpusFromFiles();

  void TryWriteCorpusFile(const Input& input);

  void InitializeCorpus(absl::BitGenRef prng);

  RunResult RunOneInput(const Input& input);

  bool ShouldStop();

  // Prints a message indicating that we're replaying an input from `file_path`
  // at `blob_idx` (if applicable) and then runs `input`.
  void ReplayInput(absl::string_view file_path, std::optional<int> blob_idx,
                   const Input& input);

  const FuzzTest& test_;
  std::unique_ptr<UntypedFixtureDriver> fixture_driver_;
  UntypedDomain params_domain_;
  std::seed_seq seed_sequence_ = GetFromEnvOrMakeSeedSeq(std::cerr);
  ExecutionCoverage* execution_coverage_;
  CorpusCoverage corpus_coverage_;
  std::deque<Input> corpus_;
  // Corpus distribution is only used in Fuzzing mode.
  absl::discrete_distribution<> corpus_distribution_;

  absl::string_view corpus_out_dir_;
  RuntimeStats stats_{};
  std::optional<size_t> runs_limit_;
  absl::Time time_limit_ = absl::InfiniteFuture();
  std::optional<Input> minimal_non_fatal_counterexample_;

  Runtime& runtime_ = Runtime::instance();

#ifdef FUZZTEST_COMPATIBILITY_MODE
  friend class FuzzTestExternalEngineAdaptor;
#endif  // FUZZTEST_COMPATIBILITY_MODE
  // Defined in centipede_adaptor.cc
  friend class CentipedeFuzzerAdaptor;
  friend class CentipedeAdaptorRunnerCallbacks;
  friend class CentipedeAdaptorEngineCallbacks;
};

size_t GetStackLimitFromEnvOrConfiguration(const Configuration& configuration);

// A reproduction command template will include these placeholders. These
// placeholders then will be replaced by the proper test filter when creating
// the final reproduction command from the template.
static constexpr absl::string_view kTestFilterPlaceholder = "$TEST_FILTER";
static constexpr absl::string_view kExtraArgsPlaceholder = "$EXTRA_ARGS";

}  // namespace internal
}  // namespace fuzztest

#endif  // FUZZTEST_FUZZTEST_INTERNAL_RUNTIME_H_
