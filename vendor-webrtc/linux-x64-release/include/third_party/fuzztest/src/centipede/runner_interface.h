// Copyright 2022 The Centipede Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// WARNING: this interface is not yet stable and may change at any point.

#ifndef THIRD_PARTY_CENTIPEDE_RUNNER_INTERFACE_H_
#define THIRD_PARTY_CENTIPEDE_RUNNER_INTERFACE_H_

#include <cstddef>
#include <cstdint>
#include <functional>
#include <memory>
#include <string>
#include <string_view>
#include <vector>

#include "absl/base/nullability.h"
#include "./centipede/mutation_input.h"
#include "./common/defs.h"

// Typedefs for the libFuzzer API, https://llvm.org/docs/LibFuzzer.html
using FuzzerTestOneInputCallback = int (*)(const uint8_t *data, size_t size);
using FuzzerInitializeCallback = int (*)(int *argc, char ***argv);
using FuzzerCustomMutatorCallback = size_t (*)(uint8_t *data, size_t size,
                                               size_t max_size,
                                               unsigned int seed);
using FuzzerCustomCrossOverCallback = size_t (*)(
    const uint8_t *data1, size_t size1, const uint8_t *data2, size_t size2,
    uint8_t *out, size_t max_out_size, unsigned int seed);

// This is the header-less interface of libFuzzer, see
// https://llvm.org/docs/LibFuzzer.html.
extern "C" {
int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size);
__attribute__((weak)) int LLVMFuzzerInitialize(int *absl_nonnull argc,
                                               char ***absl_nonnull argv);
__attribute__((weak)) size_t LLVMFuzzerCustomMutator(uint8_t *data, size_t size,
                                                     size_t max_size,
                                                     unsigned int seed);
__attribute__((weak)) size_t LLVMFuzzerCustomCrossOver(
    const uint8_t *data1, size_t size1, const uint8_t *data2, size_t size2,
    uint8_t *out, size_t max_out_size, unsigned int seed);
}  // extern "C"

// https://llvm.org/docs/LibFuzzer.html#using-libfuzzer-as-a-library
extern "C" int LLVMFuzzerRunDriver(
    int *absl_nonnull argc, char ***absl_nonnull argv,
    FuzzerTestOneInputCallback test_one_input_cb);

// This interface can be used to detect presence of Centipede in the binary.
// Also pretend we are LibFuzzer for compatibility.
// This API can be used by other pieces of fuzzing infrastructure,
// but should not be used by end-users of fuzz targets
// (consider using FUZZING_BUILD_MODE_UNSAFE_FOR_PRODUCTION macro).
extern "C" __attribute__((weak)) void CentipedeIsPresent();
extern "C" __attribute__((weak)) void __libfuzzer_is_present();

// Reconfigures the RSS limit to `rss_limit_mb` - 0 indicates no limit.
extern "C" void CentipedeSetRssLimit(size_t rss_limit_mb);

// Reconfigures the stack limit to `stack_limit_kb` - 0 indicates no limit.
extern "C" void CentipedeSetStackLimit(size_t stack_limit_kb);

// Reconfigures `timeout_per_input` accordingly in seconds - 0 means no timeout.
extern "C" void CentipedeSetTimeoutPerInput(uint64_t timeout_per_input);

// An overridable function to get the runner flags for configuring the runner
// during the initialization. The default implementation (as a weak function)
// gets the flags from CENTIPEDE_RUNNER_FLAGS env var.
//
// It should return either a nullptr or a constant string that is valid
// throughout the entire process life-time.
extern "C" const char *absl_nullable CentipedeGetRunnerFlags();

// An overridable function to override `LLVMFuzzerMutate` behavior.
extern "C" size_t CentipedeLLVMFuzzerMutateCallback(uint8_t *data, size_t size,
                                                    size_t max_size);

// Prepares to run a batch of test executions that ends with calling
// `CentipedeEndExecutionBatch`.
//
// `CentipedeBeginExecutionBatch` would abort if it was previously called
// without a matching `CentipedeEndExecutionBatch` call.
extern "C" void CentipedeBeginExecutionBatch();

// Finalizes the current batch of test executions. It would abort if no
// `CentipedeBeginExecutionBatch` was called before without a matching
// `CentipedeEndExecutionBatch` call.
extern "C" void CentipedeEndExecutionBatch();

// Resets the internal state of the runner to process a new input.
extern "C" void CentipedePrepareProcessing();

// Finalizes the processing of an input and stores the state internally.
//
// For tool integration, it can be called inside `RunnerCallbacks::Execute()` to
// finalize the execution early before extra cleanups.
extern "C" void CentipedeFinalizeProcessing();

// Retrieves the execution results (including coverage information) after
// processing an input. This function saves the data to the provided buffer and
// returns the size of the saved data. It may be called after
// CentipedeFinalizeProcessing().
extern "C" size_t CentipedeGetExecutionResult(uint8_t *data, size_t capacity);

// Retrieves the coverage data collected during the processing of an input.
// This function saves the raw coverage data to the provided buffer and returns
// the size of the saved data. It may be called after
// CentipedeFinalizeProcessing().
extern "C" size_t CentipedeGetCoverageData(uint8_t *data, size_t capacity);

// Set the current execution result to the opaque memory `data` with `size`.
// Such data is retrieved using `CentipedeGetExecutionResult`, possibly from
// another process. When `data` is `nullptr`, will set the execution result to
// "empty" with no features or metadata.
extern "C" void CentipedeSetExecutionResult(const uint8_t *data, size_t size);

namespace fuzztest::internal {

// Callbacks interface implemented by the fuzzer and called by the runner.
//
// WARNING: This interface is designed for FuzzTest/Centipede integration -
// no stability is guaranteed for other usages.
class RunnerCallbacks {
 public:
  // Attempts to execute the test logic using `input`, and returns false if the
  // input should be ignored from the corpus, true otherwise.
  virtual bool Execute(ByteSpan input) = 0;
  // Generates seed inputs by calling `seed_callback` for each input.
  // The default implementation generates a single-byte input {0}.
  virtual void GetSeeds(std::function<void(ByteSpan)> seed_callback);
  // Returns the serialized configuration from the test target. The default
  // implementation returns the empty string.
  virtual std::string GetSerializedTargetConfig();
  // Returns true if and only if the test target has a custom mutator.
  virtual bool HasCustomMutator() const = 0;
  // Generates at most `num_mutants` mutants by calling `new_mutant_callback`
  // for each mutant. Returns true on success, false otherwise.
  //
  // TODO(xinhaoyuan): Consider supporting only_shrink to speed up
  // input shrinking.
  virtual bool Mutate(const std::vector<MutationInputRef> &inputs,
                      size_t num_mutants,
                      std::function<void(ByteSpan)> new_mutant_callback);
  // Registers a function to be called when a failure happens. If the
  // implementation supports this functionality, it will call the function with
  // a description of the failure. Otherwise, it will do nothing.
  virtual void OnFailure(
      std::function<void(std::string_view)> failure_description_callback);
  virtual ~RunnerCallbacks() = default;
};

// Wraps legacy fuzzer callbacks into a `RunnerCallbacks` instance.
std::unique_ptr<RunnerCallbacks> CreateLegacyRunnerCallbacks(
    FuzzerTestOneInputCallback test_one_input_cb,
    FuzzerCustomMutatorCallback custom_mutator_cb,
    FuzzerCustomCrossOverCallback custom_crossover_cb);

// The main Centipede Runner function.
// It performs actions prescribed by argc/argv and environment variables
// and returns EXIT_SUCCESS or EXIT_FAILURE.
// Normally, the runner itself calls this function (LLVMFuzzerRunDriver).
//
// As an *experiment* we want to allow user code to call RunnerMain().
// This is not a guaranteed public interface (yet) and may disappear w/o notice.
int RunnerMain(int argc, char **argv, RunnerCallbacks &callbacks);

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_RUNNER_INTERFACE_H_
