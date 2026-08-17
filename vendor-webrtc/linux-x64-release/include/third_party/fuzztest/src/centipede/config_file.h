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

#ifndef THIRD_PARTY_CENTIPEDE_GOOGLE_CONFIG_FILE_H_
#define THIRD_PARTY_CENTIPEDE_GOOGLE_CONFIG_FILE_H_

#include <filesystem>  // NOLINT
#include <functional>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "absl/base/nullability.h"
#include "./centipede/config_init.h"

// TODO(ussuri): Move implementation-only functions to .cc.

namespace fuzztest::internal {

// Constructs an augmented copy of `argv` with any substrings appearing in the
// original elements replaced according to a list replacements.
// TODO(ussuri): Make more robust. What we really want is replace any possible
//  form of --flag=value with an equivalent form of --new_flag=new_value.
// TODO(ussuri): Remove and just use the required bits of logic in .cc.
class AugmentedArgvWithCleanup final {
 public:
  using Replacements = std::vector<std::pair<std::string, std::string>>;
  using BackingResourcesCleanup = std::function<void()>;

  // Ctor. The `orig_argc` and `orig_argv` are compatible with those passed to a
  // main(). Each item in `orig_argv` is first processed with
  // `flag_replacements` if the item has the format "-flag", "-flag=...",
  // "--flag", or "--flag=", and the flag name matches. Then the `replacements`
  // map should map an old substring to a new one. Only simple, one-stage string
  // replacement is performed: no regexes, placeholders, envvars or recursion.
  // The `cleanup` callback should clean up any temporary resources backing the
  // modified flags, such as temporary files.
  AugmentedArgvWithCleanup(const std::vector<std::string>& orig_argv,
                           const Replacements& flag_replacements,
                           const Replacements& replacements,
                           BackingResourcesCleanup&& cleanup);
  // Dtor. Invokes `cleanup_`.
  ~AugmentedArgvWithCleanup();

  // Movable by not copyable to prevent `cleanup_` from running twice.
  AugmentedArgvWithCleanup(const AugmentedArgvWithCleanup&) = delete;
  AugmentedArgvWithCleanup& operator=(const AugmentedArgvWithCleanup&) = delete;
  AugmentedArgvWithCleanup(AugmentedArgvWithCleanup&&) noexcept;
  AugmentedArgvWithCleanup& operator=(AugmentedArgvWithCleanup&&) noexcept;

  // The new argc. Currently, will always match the original argc.
  int argc() const { return static_cast<int>(argv_.size()); }
  // The new, possibly augmented argv. Note that all its char* elements are
  // backed by newly allocated std::strings, so they will all be different from
  // their counterparts in the original argv.
  const std::vector<std::string>& argv() const { return argv_; }
  // Whether the original argv has been augmented from the original, i.e. if any
  // of the requested string replacements actually occurred.
  bool was_augmented() const { return was_augmented_; }

 private:
  std::vector<std::string> argv_;
  bool was_augmented_;
  BackingResourcesCleanup cleanup_;
};

// Replaces any --config=<config_file> in `argv` (or any alternative form of
// that flag) with a --flagfile=<possibly_localized_config_file>, where
// localization means that a remote <config_file> is copied to a temporary local
// mirror. If <config_file> is already local, it is used as-is.
//
// The remote file contents is additionally checked for possible nested
// --config, --save_config and --flagfile: such usage is currently unsupported.
//
// The returned AugmentedArgvWithCleanup deletes the localized files (if any) in
// dtor.
AugmentedArgvWithCleanup LocalizeConfigFilesInArgv(
    const std::vector<std::string>& argv);

// If --save_config=<path> was passed on the command line, saves _all_
// Centipede flags (i.e. those specified on the command line AND the defaulted
// ones) to <path> in the format compatible with --config (defined by
// Centipede), as well as --flagfile (defined by Abseil Flags), and returns
// <path>. Otherwise, returns an empty string. If the <path>'s extension is .sh,
// saves a runnable script instead.
std::filesystem::path MaybeSaveConfigToFile(
    const std::vector<std::string>& leftover_argv);

// Initializes Centipede:
// - Calls `InitRuntime()` at the right time to initialize the runtime
//   subsystems and perform the initial flag parsing.
// - Handles config-related flags: loads the config from --config, if any,
//   and saves it to --save_config (or --update_config), if any.
// - Logs the final resolved config.
// - Returns the runtime state that the caller should take ownership of and
//    keep alive the duration of the process.
[[nodiscard]] std::unique_ptr<RuntimeState> InitCentipede(
    int argc, char** absl_nonnull argv);

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_GOOGLE_CONFIG_FILE_H_
