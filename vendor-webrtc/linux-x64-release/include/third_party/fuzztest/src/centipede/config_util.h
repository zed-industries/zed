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

#ifndef THIRD_PARTY_CENTIPEDE_CONFIG_UTIL_H_
#define THIRD_PARTY_CENTIPEDE_CONFIG_UTIL_H_

#include <map>
#include <set>
#include <string>
#include <string_view>
#include <vector>

namespace fuzztest::internal {

// A set of overloads to cast argv between vector<string> and main()-compatible
// vector<char*> or argc/argv pair in both directions. The result can be used
// like this:
//   AugmentedArgvWithCleanup new_argv{CastArgv(argc, argv), ...};
//   std::vector<std::string> leftover_argv =
//       CastArgv(absl::ParseCommandLine(
//           new_argv.argc(), CastArgv(new_argv.argv()).data());
std::vector<std::string> CastArgv(int argc, char** argv);
std::vector<std::string> CastArgv(const std::vector<char*>& argv);
// WARNING: Beware of the lifetimes. The returned vector<char*> referenced the
// passed `argv`, so `argv` must outlive it.
std::vector<char*> CastArgv(const std::vector<std::string>& argv);

// Types returned from GetFlagsPerSource().
struct FlagInfo {
  const std::string_view name;
  const std::string value;
  const std::string default_value;
  const std::string help;

  friend bool operator<(const FlagInfo& x, const FlagInfo& y) {
    return x.name < y.name;
  }
};
using FlagInfosPerSource =
    std::map<std::string /*source_filename*/, std::set<FlagInfo>>;

// Returns a per-source map of all compiled-in flags defined by sources whose
// relative workspace paths contain `source_fragment`. An empty
// `source_fragment` returns flags from all sources.
FlagInfosPerSource GetFlagsPerSource(
    std::string_view source_fragment = "",
    const std::set<std::string_view>& exclude_flags = {});

// Returns a string with newline-separated --flag=value tokens for all
// compiled-in flags defined by sources whose relative workspace paths start
// with `source_prefix`. An empty `source_prefix` returns flags from all
// sources. Flag names in `exclude_flags` are excluded from the result.
//
// The flags are grouped by the source filename, and sorted within each group.
//
//
//   # Flags from centipede/environment.cc:
//
//   --binary="unicorn_x86_64_sancov"
//   # --rss_limit_mb="4096"
//   --use_pc_features="true"
//
//   # Flags from third_party/absl/log/flags.cc:
//
//   --alsologtostderr="true"
//   # --log_backtrace_at=""
//
// (See config_util_test.cc for more examples of the output).
//
// The returned value is compatible with the standard Abseil's --flagfile flag
// and its remote-enabled Centipede's equivalents --config and --save_config.
enum class DefaultedFlags {
  kIncluded = 0,      // Include flags with value == default.
  kExcluded = 1,      // Exclude flags with value == default.
  kCommentedOut = 2,  // Comment out flags with value == default.
};
enum class FlagComments {
  kNone = 0,            // Do not add any comments.
  kDefault = 1,         // Add a comment with the flag's default.
  kHelpAndDefault = 2,  // Add a comment with the flag's help and default.
};
std::string FormatFlagfileString(
    const FlagInfosPerSource& flags,
    DefaultedFlags defaulted = DefaultedFlags::kIncluded,
    FlagComments comments = FlagComments::kNone);

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_CONFIG_UTIL_H_
