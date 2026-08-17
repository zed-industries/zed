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

#ifndef THIRD_PARTY_CENTIPEDE_ENVIRONMENT_H_
#define THIRD_PARTY_CENTIPEDE_ENVIRONMENT_H_

#include <bitset>
#include <cstddef>
#include <cstdint>
#include <limits>
#include <string>
#include <string_view>
#include <vector>

#include "absl/time/time.h"
#include "./centipede/feature.h"
#include "./centipede/knobs.h"
#include "./fuzztest/internal/configuration.h"

namespace fuzztest::internal {

// Fuzzing environment controlling the behavior of
// CentipedeMain(). Centipede binaries are creating Environment instances using
// the flags defined in environment_flags.cc, while other users can use
// CentipedeMain() as a library function without importing the flags.
struct Environment {
  // Global params. See environment_flags.cc for help on each parameter. -------

  std::string binary;
  std::string coverage_binary;
  std::string clang_coverage_binary;
  std::vector<std::string> extra_binaries;
  std::string workdir;
  std::string merge_from;
  size_t num_runs = std::numeric_limits<size_t>::max();
  size_t total_shards = 1;
  size_t my_shard_index = 0;
  size_t num_threads = 1;
  size_t j = 0;
  size_t max_len = 4000;
  size_t batch_size = 1000;
  size_t mutate_batch_size = 2;
  bool use_legacy_default_mutator = false;
  size_t load_other_shard_frequency = 10;
  bool serialize_shard_loads = false;
  size_t seed = 0;
  size_t prune_frequency = 100;
#ifdef __APPLE__
  // Address space limit is ignored on MacOS.
  // Reference: https://bugs.chromium.org/p/chromium/issues/detail?id=853873#c2
  size_t address_space_limit_mb = 0;
#else   // __APPLE__
  size_t address_space_limit_mb = 8192;
#endif  // __APPLE__
  size_t rss_limit_mb = 4096;
  size_t stack_limit_kb = 0;
  size_t timeout_per_input = 60;
  size_t timeout_per_batch = 0;
  bool ignore_timeout_reports = false;
  absl::Time stop_at = absl::InfiniteFuture();
  bool fork_server = true;
  bool full_sync = false;
  bool use_corpus_weights = true;
  bool use_coverage_frontier = false;
  size_t max_corpus_size = 100000;
  size_t crossover_level = 50;
  bool use_pc_features = true;
  size_t path_level = 0;
  bool use_cmp_features = true;
  size_t callstack_level = 0;
  bool use_auto_dictionary = true;
  bool use_dataflow_features = true;
  bool use_counter_features = false;
  bool use_pcpair_features = false;
  uint64_t user_feature_domain_mask = ~0UL;
  size_t feature_frequency_threshold = 100;
  bool require_pc_table = true;
  bool require_seeds = false;
  int telemetry_frequency = 0;
  bool print_runner_log = false;
  bool distill = false;
  size_t log_features_shards = 0;
  std::string knobs_file;
  std::string corpus_to_files;
  std::string corpus_from_files;
  std::vector<std::string> corpus_dir;
  std::string symbolizer_path = "llvm-symbolizer";
  std::string objdump_path = "objdump";
  std::string runner_dl_path_suffix;
  std::string input_filter;
  std::vector<std::string> dictionary;
  std::string function_filter;
  std::string for_each_blob;
  std::string experiment;
  bool analyze = false;
  bool exit_on_crash = false;
  size_t max_num_crash_reports = 5;
  std::string minimize_crash_file_path;
  bool batch_triage_suspect_only = false;
  size_t shmem_size_mb = 1024;
#ifdef __APPLE__
  bool use_posix_shmem = true;
#else
  bool use_posix_shmem = false;
#endif
  bool dry_run = false;
  bool save_binary_info = false;
  bool populate_binary_info = true;
#ifdef CENTIPEDE_DISABLE_RIEGELI
  bool riegeli = false;
#else
  bool riegeli = true;
#endif  // CENTIPEDE_DISABLE_RIEGELI

  // Internal settings without global flags ------------------------------------

  // If set, treat the first entry of `corpus_dir` as output-only.
  bool first_corpus_dir_output_only = false;
  // If set, load/merge shards without fuzzing new inputs.
  bool load_shards_only = false;
  // If set, operate on the corpus database for a single test specified by
  // FuzzTest instead of all the tests.
  bool fuzztest_single_test_mode = false;
  // If set, deserializes the configuration from the value instead of querying
  // the configuration via runner callbacks.
  std::string fuzztest_configuration;
  // If set, lists the crash IDs of a single test of the binary to
  // the `crash_ids_file` with each crash ID in a single line. If there is no
  // crash for the test, the empty content will be written to the file.
  bool list_crash_ids = false;
  // The path to list the crash IDs for `list_crash_ids`.
  std::string list_crash_ids_file;
  // The crash ID used for `replay_crash` or `export_crash`.
  std::string crash_id;
  // If set, replay `crash_id` in the corpus database.
  bool replay_crash = false;
  // If set, export the input contents of `crash_id` from the corpus database.
  bool export_crash = false;
  // The path to export the input contents of `crash_id` for `export_crash`.
  std::string export_crash_file;

  // Command line-related fields -----------------------------------------------

  std::string exec_name;          // copied from argv[0]
  std::vector<std::string> args;  // copied from argv[1:].
  std::string binary_name;        // Name of `coverage_binary`, w/o directories.
  std::string binary_hash;        // Hash of the `coverage_binary` file.
  bool has_input_wildcards = false;  // Set to true iff `binary` contains "@@".

  // Experiment-related settings -----------------------------------------------

  std::string experiment_name;   // Set by `UpdateForExperiment`.
  std::string experiment_flags;  // Set by `UpdateForExperiment`.

  // Other ---------------------------------------------------------------------

  Knobs knobs;  // Read from a file by `ReadKnobsFileIfSpecified`, see knobs.h.

  // Defines internal logging level. Set to zero to reduce logging in tests.
  // TODO(ussuri): Retire in favor of VLOGs?
  size_t log_level = 1;

  // Path to a file with PCs. This file is created and the field is set in
  // `CentipedeMain()` once per process if trace_pc instrumentation is detected.
  std::string pcs_file_path;

  // APIs ----------------------------------------------------------------------

  // Returns an instance of the environment with default values.
  static const Environment& Default();

  // Should certain actions be performed ---------------------------------------

  // Returns true if we want to log features as symbols in this shard.
  bool LogFeaturesInThisShard() const {
    return my_shard_index < log_features_shards;
  }
  // Returns true if we want to generate the corpus telemetry files (coverage
  // report, corpus stats, etc.) in this shard.
  bool DumpCorpusTelemetryInThisShard() const;
  // Returns true if we want to generate the resource usage report in this
  // shard. See the related RUsageTelemetryScope().
  bool DumpRUsageTelemetryInThisShard() const;
  // Returns true if we want to generate the telemetry files (coverage report,
  // the corpus stats, etc.) after processing `batch_index`-th batch.
  bool DumpTelemetryForThisBatch(size_t batch_index) const;
  // Returns a bitmask indicating which domains Centipede should discard.
  std::bitset<feature_domains::kNumDomains> MakeDomainDiscardMask() const;

  // Experiment-related functions ----------------------------------------------

  // Updates `this` according to the `--experiment` flag.
  // The `--experiment` flag, if not empty, has this form:
  //   foo=1,2,3:bar=10,20
  // where foo and bar are some of the flag names supported for experimentation,
  // see `SetFlag()`.
  // `--experiment` defines the flag values to be set differently in different
  // shards. E.g. in this case,
  //   shard 0 will have {foo=1,bar=10},
  //   shard 1 will have {foo=1,bar=20},
  //   ...
  //   shard 3 will have {foo=2,bar=10},
  //   ...
  //   shard 5 will have {foo=2,bar=30},
  // and so on.
  //
  // CHECK-fails if the `--experiment` flag is not well-formed,
  // or if num_threads is not a multiple of the number of flag combinations
  // (which is 6 in this example).
  //
  // Sets load_other_shard_frequency=0 (experiments should be independent).
  //
  // Sets this->experiment_name to a string like "E01",
  // which means "value #0 is used for foo and value #1 is used for bar".
  void UpdateForExperiment();

  // Sets flag 'name' to `value` for an experiment. CHECK-fails on
  // invalid name/value combination. Used in `UpdateForExperiment()`.
  void SetFlagForExperiment(std::string_view name, std::string_view value);

  // Other ---------------------------------------------------------------------

  // Reads `knobs` from `knobs_file`. Does nothing if the `knobs_file` is empty.
  void ReadKnobsFileIfSpecified();
  // Updates `this` with `config` obtained from the target binary. CHECK-fails
  // if the fields are non-default and inconsistent with the corresponding
  // values in `config`.
  void UpdateWithTargetConfig(const fuzztest::internal::Configuration& config);
  // If `timeout_per_batch` is `val`, computes it as a function of
  // `timeout_per_input` and `batch_size` and updates it. Otherwise, leaves it
  // unchanged.
  void UpdateTimeoutPerBatchIfEqualTo(size_t val);
  // If `binary_hash` is empty, updates it using the file in `coverage_binary`.
  void UpdateBinaryHashIfEmpty();
};

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_ENVIRONMENT_H_
