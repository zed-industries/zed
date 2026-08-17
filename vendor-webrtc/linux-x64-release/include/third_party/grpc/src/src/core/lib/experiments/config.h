// Copyright 2022 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_EXPERIMENTS_CONFIG_H
#define GRPC_SRC_CORE_LIB_EXPERIMENTS_CONFIG_H

#include <grpc/support/port_platform.h>
#include <stddef.h>
#include <stdint.h>

#include <atomic>

#include "absl/functional/any_invocable.h"
#include "absl/strings/string_view.h"

// #define GRPC_EXPERIMENTS_ARE_FINAL

namespace grpc_core {

struct ExperimentMetadata {
  const char* name;
  const char* description;
  const char* additional_constaints;
  const uint8_t* required_experiments;
  uint8_t num_required_experiments;
  bool default_value;
  bool allow_in_fuzzing_config;
};

#ifndef GRPC_EXPERIMENTS_ARE_FINAL
class ExperimentFlags {
 public:
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static bool IsExperimentEnabled(
      size_t experiment_id) {
    auto bit = experiment_id % kFlagsPerWord;
    auto word = experiment_id / kFlagsPerWord;
    auto cur = experiment_flags_[word].load(std::memory_order_relaxed);
    if (cur & (1ull << bit)) return true;
    if (cur & kLoadedFlag) return false;
    return LoadFlagsAndCheck(experiment_id);
  }

  template <size_t kExperimentId>
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static bool IsExperimentEnabled() {
    auto bit = kExperimentId % kFlagsPerWord;
    auto word = kExperimentId / kFlagsPerWord;
    auto cur = experiment_flags_[word].load(std::memory_order_relaxed);
    if (cur & (1ull << bit)) return true;
    if (cur & kLoadedFlag) return false;
    return LoadFlagsAndCheck(kExperimentId);
  }

  static void TestOnlyClear();

 private:
  static bool LoadFlagsAndCheck(size_t experiment_id);

  // We layout experiment flags in groups of 63... each 64 bit word contains
  // 63 enablement flags (one per experiment), and the high bit which indicates
  // whether the flags have been loaded from the configuration.
  // Consequently, with one load, we can tell if the experiment is definitely
  // enabled (the bit is set), or definitely disabled (the bit is clear, and the
  // loaded flag is set), or if we need to load the flags and re-check.

  static constexpr size_t kNumExperimentFlagsWords = 8;
  static constexpr size_t kFlagsPerWord = 63;
  static constexpr uint64_t kLoadedFlag = 0x8000000000000000ull;
  static std::atomic<uint64_t> experiment_flags_[kNumExperimentFlagsWords];
};

// Return true if experiment \a experiment_id is enabled.
// Experiments are numbered by their order in the g_experiment_metadata array
// declared in experiments.h.
inline bool IsExperimentEnabled(size_t experiment_id) {
  return ExperimentFlags::IsExperimentEnabled(experiment_id);
}

template <size_t kExperimentId>
inline bool IsExperimentEnabled() {
  return ExperimentFlags::IsExperimentEnabled<kExperimentId>();
}

// Given a test experiment id, returns true if the test experiment is enabled.
// Test experiments can be loaded using the LoadTestOnlyExperimentsFromMetadata
// method.
bool IsTestExperimentEnabled(size_t experiment_id);

template <size_t kExperimentId>
inline bool IsTestExperimentEnabled() {
  return IsTestExperimentEnabled(kExperimentId);
}

// Slow check for if a named experiment is enabled.
// Parses the configuration and looks up the experiment in that, so it does not
// affect any global state, but it does require parsing the configuration every
// call!
bool IsExperimentEnabledInConfiguration(size_t experiment_id);

// Reload experiment state from config variables.
// Does not change ForceEnableExperiment state.
// Expects the caller to handle global thread safety - so really only
// appropriate for carefully written tests.
void TestOnlyReloadExperimentsFromConfigVariables();

// Reload experiment state from passed metadata.
// Does not change ForceEnableExperiment state.
// Expects the caller to handle global thread safety - so really only
// appropriate for carefully written tests.
void LoadTestOnlyExperimentsFromMetadata(
    const ExperimentMetadata* experiment_metadata, size_t num_experiments);
#endif

// Print out a list of all experiments that are built into this binary.
void PrintExperimentsList();

// Force an experiment to be on or off.
// Must be called before experiments are configured (the first
// IsExperimentEnabled call).
// If the experiment does not exist, emits a warning but continues execution.
// If this is called twice for the same experiment, both calls must agree.
void ForceEnableExperiment(absl::string_view experiment_name, bool enable);

// Register a function to be called to validate the value an experiment can
// take subject to additional constraints.
// The function will take the ExperimentMetadata as its argument. It will return
// a bool value indicating the actual value the experiment should take.
void RegisterExperimentConstraintsValidator(
    absl::AnyInvocable<bool(struct ExperimentMetadata)> check_constraints_cb);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_EXPERIMENTS_CONFIG_H
