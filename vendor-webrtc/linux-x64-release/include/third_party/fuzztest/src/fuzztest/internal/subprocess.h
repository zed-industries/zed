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

#ifndef FUZZTEST_FUZZTEST_INTERNAL_SUBPROCESS_H_
#define FUZZTEST_FUZZTEST_INTERNAL_SUBPROCESS_H_

#include <iostream>
#include <string>
#include <variant>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/time/time.h"

namespace fuzztest::internal {

enum class ExitCodeT : int;
inline ExitCodeT ExitCode(int i) { return static_cast<ExitCodeT>(i); }
inline std::ostream& operator<<(std::ostream& os, ExitCodeT code) {
  return os << "ExitCode: " << static_cast<int>(code);
}
enum class SignalT : int;
inline SignalT Signal(int i) { return static_cast<SignalT>(i); }
inline std::ostream& operator<<(std::ostream& os, SignalT code) {
  return os << "Signal: " << static_cast<int>(code);
}

// Represents the termination status of a process.
class TerminationStatus {
  using StatusT = std::variant<ExitCodeT, SignalT>;

 public:
  // Constructs TerminationStatus from a raw `status` value, e.g., returned by
  // the waitpid() system call.
  TerminationStatus(int status);
  // True iff the process exited (wasn't terminated by a signal).
  bool Exited() const;
  // True iff the process was terminated by a signal.
  bool Signaled() const;

  friend std::ostream& operator<<(std::ostream& os, TerminationStatus self) {
    std::visit([&os](auto v) { os << v; }, self.Status());
    return os;
  }

  template <typename Sink>
  friend void AbslStringify(Sink& sink, const TerminationStatus self) {
    std::stringstream ss;
    ss << self;
    sink.Append(ss.str());
  }

  // TerminationStatus can be compared to ExitCodeT and SignalT.
  friend bool operator==(TerminationStatus self, StatusT res) {
    return self.Status() == res;
  }
  friend bool operator==(StatusT res, TerminationStatus self) {
    return self.Status() == res;
  }
  friend bool operator!=(TerminationStatus self, StatusT res) {
    return self.Status() != res;
  }
  friend bool operator!=(StatusT res, TerminationStatus self) {
    return self.Status() != res;
  }

  // If Exited, returns an ExitCodeT.
  // If Signaled, returns a SignalT.
  StatusT Status() const;

 private:
  // The raw status.
  int status_;
};

struct RunResults {
  // Termination status.
  TerminationStatus status;
  // Contents of stdout.
  std::string stdout_output;
  // Contents of stderr.
  std::string stderr_output;
};

// Runs `command_line` in a subprocess. Environment variables can be set via
// `environment`. If optional `timeout` is provided, the process is terminated
// after the given timeout. The timeout will be rounded up to seconds.
RunResults RunCommand(
    const std::vector<std::string>& command_line,
    const absl::flat_hash_map<std::string, std::string>& environment = {},
    absl::Duration timeout = absl::InfiniteDuration());

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_SUBPROCESS_H_
