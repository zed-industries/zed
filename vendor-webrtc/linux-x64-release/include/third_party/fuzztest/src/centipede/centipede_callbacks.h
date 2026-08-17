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

#ifndef THIRD_PARTY_CENTIPEDE_CENTIPEDE_CALLBACKS_H_
#define THIRD_PARTY_CENTIPEDE_CENTIPEDE_CALLBACKS_H_

#include <cstddef>
#include <filesystem>  // NOLINT
#include <string>
#include <string_view>
#include <vector>

#include "absl/base/nullability.h"
#include "absl/log/check.h"
#include "absl/status/statusor.h"
#include "./centipede/binary_info.h"
#include "./centipede/byte_array_mutator.h"
#include "./centipede/command.h"
#include "./centipede/environment.h"
#include "./centipede/fuzztest_mutator.h"
#include "./centipede/mutation_input.h"
#include "./centipede/runner_result.h"
#include "./centipede/shared_memory_blob_sequence.h"
#include "./centipede/util.h"
#include "./common/defs.h"

namespace fuzztest::internal {

// User must inherit from this class and override at least the
// pure virtual functions.
//
// The classes inherited from this one must be thread-compatible.
// Note: the interface is not yet stable and may change w/o a notice.
class CentipedeCallbacks {
 public:
  // `env` is used to pass flags to `this`, it must outlive `this`.
  CentipedeCallbacks(const Environment &env)
      : env_(env),
        byte_array_mutator_(env.knobs, GetRandomSeed(env.seed)),
        fuzztest_mutator_(env.knobs, GetRandomSeed(env.seed)),
        inputs_blobseq_(shmem_name1_.c_str(), env.shmem_size_mb << 20,
                        env.use_posix_shmem),
        outputs_blobseq_(shmem_name2_.c_str(), env.shmem_size_mb << 20,
                         env.use_posix_shmem) {
    if (env.use_legacy_default_mutator)
      CHECK(byte_array_mutator_.set_max_len(env.max_len));
    else
      CHECK(fuzztest_mutator_.set_max_len(env.max_len));
  }
  virtual ~CentipedeCallbacks() {}

  // Feeds `inputs` into the `binary`, for every input populates `batch_result`.
  // Old contents of `batch_result` are cleared.
  // Returns true on success, false on failure.
  // Post-condition:
  // `batch_result` has results for every `input`, even on failure.
  virtual bool Execute(std::string_view binary,
                       const std::vector<ByteArray> &inputs,
                       BatchResult &batch_result) = 0;

  // Takes non-empty `inputs` and returns at most `num_mutants` mutated inputs.
  virtual std::vector<ByteArray> Mutate(
      const std::vector<MutationInputRef> &inputs, size_t num_mutants) {
    return env_.use_legacy_default_mutator
               ? byte_array_mutator_.MutateMany(inputs, num_mutants)
               : fuzztest_mutator_.MutateMany(inputs, num_mutants);
  }

  // Populates the BinaryInfo using the `symbolizer_path` and `coverage_binary`
  // in `env_`. The tables may not be populated if the PC table cannot be
  // determined from the `coverage_binary` or if symbolization fails. Exits if
  // PC table was not populated and `env_.require_pc_table` is set.
  virtual void PopulateBinaryInfo(BinaryInfo &binary_info);

  // Retrieves at most `num_seeds` seed inputs. Returns the number of seeds
  // available if `num_seeds` had been large enough.
  virtual size_t GetSeeds(size_t num_seeds, std::vector<ByteArray> &seeds) {
    if (num_seeds > 0) seeds = {{0}};
    return 1;
  }

  // Returns the configuration from the test target in the serialized form.
  // Returns an empty string if the test target doesn't provide configuration.
  virtual absl::StatusOr<std::string> GetSerializedTargetConfig() { return ""; }

 protected:
  // Helpers that the user-defined class may use if needed.

  // Same as ExecuteCentipedeSancovBinary, but uses shared memory.
  // Much faster for fast targets since it uses fewer system calls.
  int ExecuteCentipedeSancovBinaryWithShmem(
      std::string_view binary, const std::vector<ByteArray> &inputs,
      BatchResult &batch_result);

  // Constructs a string CENTIPEDE_RUNNER_FLAGS=":flag1:flag2:...",
  // where the flags are determined by `env` and also include `extra_flags`.
  // If `disable_coverage`, coverage options are not added.
  std::string ConstructRunnerFlags(std::string_view extra_flags = "",
                                   bool disable_coverage = false);

  // Uses an external binary `binary` to generate seed inputs. The binary should
  // be linked against :centipede_runner and implement the RunnerCallbacks
  // interface as described in runner_interface.h.
  //
  // Retrieves the first `seeds.size()` inputs (if exist) from `binary`,
  // replacing the existing elements of `seeds`, and shrinking `seeds` if
  // needed. Sets `num_avail_seeds` to the number of available seeds, which may
  // be more than `seeds.size()`.
  //
  // Returns true on success.
  bool GetSeedsViaExternalBinary(std::string_view binary,
                                 size_t &num_avail_seeds,
                                 std::vector<ByteArray> &seeds);

  // Uses an external binary `binary` to get the serialized test target
  // configuration. The binary should be linked against :centipede_runner and
  // implement the RunnerCallbacks interface as described in runner_interface.h.
  //
  // If the binary returns with success and doesn't provide the configuration,
  // sets `serialized_config` to empty string.
  //
  // Returns true on success.
  bool GetSerializedTargetConfigViaExternalBinary(
      std::string_view binary, std::string &serialized_config);

  // Uses an external binary `binary` to mutate `inputs`. The binary
  // should be linked against :centipede_runner and implement the
  // RunnerCallbacks interface as described in runner_interface.h,
  // or implement the legacy Structure-Aware Fuzzing interface described here:
  // github.com/google/fuzzing/blob/master/docs/structure-aware-fuzzing.md
  //
  // Returns a `MutationResult` instance where `exit_code` indicates whether
  // the binary was executed successfully, `has_custom_mutator` indicates
  // whether the binary has a custom mutator, and if it does, `mutants` contains
  // at most `num_mutants` non-empty mutants.
  MutationResult MutateViaExternalBinary(
      std::string_view binary, const std::vector<MutationInputRef> &inputs,
      size_t num_mutants);

  // Loads the dictionary from `dictionary_path`,
  // returns the number of dictionary entries loaded.
  size_t LoadDictionary(std::string_view dictionary_path);

 protected:
  const Environment &env_;
  ByteArrayMutator byte_array_mutator_;
  FuzzTestMutator fuzztest_mutator_;

 private:
  // Returns a Command object with matching `binary` from commands_,
  // creates one if needed.
  Command &GetOrCreateCommandForBinary(std::string_view binary);

  // Prints the execution log from the last executed binary.
  void PrintExecutionLog() const;

  // Variables required for ExecuteCentipedeSancovBinaryWithShmem.
  // They are computed in CTOR, to avoid extra computation in the hot loop.
  std::string temp_dir_ = TemporaryLocalDirPath();
  std::string temp_input_file_path_ =
      std::filesystem::path(temp_dir_).append("temp_input_file");
  const std::string execute_log_path_ =
      std::filesystem::path(temp_dir_).append("log");
  std::string failure_description_path_ =
      std::filesystem::path(temp_dir_).append("failure_description");
  const std::string shmem_name1_ = ProcessAndThreadUniqueID("/ctpd-shm1-");
  const std::string shmem_name2_ = ProcessAndThreadUniqueID("/ctpd-shm2-");

  SharedMemoryBlobSequence inputs_blobseq_;
  SharedMemoryBlobSequence outputs_blobseq_;

  std::vector<Command> commands_;
};

// Abstract class for creating/destroying CentipedeCallbacks objects.
// A typical implementation would simply new/delete objects of appropriate type,
// see DefaultCallbacksFactory below.
// Other implementations (e.g. for tests) may take the object from elsewhere
// and not actually delete it.
class CentipedeCallbacksFactory {
 public:
  virtual CentipedeCallbacks *create(const Environment &env) = 0;
  virtual void destroy(CentipedeCallbacks *callbacks) = 0;
  virtual ~CentipedeCallbacksFactory() {}
};

// This is the typical way to implement a CentipedeCallbacksFactory for a Type.
template <typename Type>
class DefaultCallbacksFactory : public CentipedeCallbacksFactory {
 public:
  CentipedeCallbacks *create(const Environment &env) override {
    return new Type(env);
  }
  void destroy(CentipedeCallbacks *callbacks) override { delete callbacks; }
};

// Creates a CentipedeCallbacks object in CTOR and destroys it in DTOR.
class ScopedCentipedeCallbacks {
 public:
  ScopedCentipedeCallbacks(CentipedeCallbacksFactory &factory,
                           const Environment &env)
      : factory_(factory), callbacks_(factory_.create(env)) {}
  ~ScopedCentipedeCallbacks() { factory_.destroy(callbacks_); }
  CentipedeCallbacks *absl_nonnull callbacks() { return callbacks_; }

 private:
  CentipedeCallbacksFactory &factory_;
  CentipedeCallbacks *callbacks_;
};

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_CENTIPEDE_CALLBACKS_H_
