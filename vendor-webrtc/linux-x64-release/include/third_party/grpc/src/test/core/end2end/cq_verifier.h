//
//
// Copyright 2015 gRPC authors.
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
//
//

#ifndef GRPC_TEST_CORE_END2END_CQ_VERIFIER_H
#define GRPC_TEST_CORE_END2END_CQ_VERIFIER_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/grpc.h>
#include <grpc/slice.h>
#include <grpc/support/time.h>
#include <stdint.h>

#include <functional>
#include <string>
#include <variant>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/functional/any_invocable.h"
#include "src/core/util/debug_location.h"
#include "src/core/util/time.h"

namespace grpc_core {

// A CqVerifier can verify that expected events arrive in a timely fashion
// on a single completion queue
class CqVerifier {
 public:
  // ExpectedResult - if the tag is received, set *seen to true (if seen is
  // non-null).
  struct Maybe {
    bool* seen = nullptr;
  };
  // ExpectedResult - expect the tag, but its result may be true or false.
  // Store the result in result (if result is non-null).
  struct AnyStatus {
    bool* result = nullptr;
  };
  // PerformAction - expect the tag, and run a function based on the result
  struct PerformAction {
    std::function<void(bool success)> action;
  };
  // MaybePerformAction - run a function if a tag is seen
  struct MaybePerformAction {
    std::function<void(bool success)> action;
  };

  using ExpectedResult =
      std::variant<bool, Maybe, AnyStatus, PerformAction, MaybePerformAction>;

  // Captures information about one failure
  struct Failure {
    SourceLocation location;
    std::string message;
    std::vector<std::string> expected;
    std::vector<std::string> message_details;
  };

  // Produces a string upon the successful (but unexpected) completion of an
  // expectation.
  class SuccessfulStateString {
   public:
    virtual std::string GetSuccessfulStateString() = 0;

   protected:
    ~SuccessfulStateString() = default;
  };

  static void FailUsingGprCrash(const Failure& failure);
  static void FailUsingGprCrashWithStdio(const Failure& failure);
  static void FailUsingGtestFail(const Failure& failure);

  // Allow customizing the failure handler
  // For legacy tests we should use FailUsingGprCrash (the default)
  // For gtest based tests we should start migrating to FailUsingGtestFail which
  // will produce nicer failure messages.
  explicit CqVerifier(
      grpc_completion_queue* cq,
      absl::AnyInvocable<void(Failure) const> fail = FailUsingGprCrash,
      absl::AnyInvocable<
          void(grpc_event_engine::experimental::EventEngine::Duration) const>
          step_fn = nullptr);
  ~CqVerifier();

  CqVerifier(const CqVerifier&) = delete;
  CqVerifier& operator=(const CqVerifier&) = delete;

  // Ensure all expected events (and only those events) are present on the
  // bound completion queue within \a timeout.
  void Verify(Duration timeout = Duration::Seconds(10),
              SourceLocation location = SourceLocation());

  // Ensure that the completion queue is empty, waiting up to \a timeout.
  void VerifyEmpty(Duration timeout = Duration::Seconds(1),
                   SourceLocation location = SourceLocation());

  void ClearSuccessfulStateStrings(void* tag);
  void AddSuccessfulStateString(void* tag,
                                SuccessfulStateString* successful_state_string);

  // Match an expectation about a status.
  // location must be DEBUG_LOCATION.
  // result can be any of the types in ExpectedResult - a plain bool means
  // 'expect success to be true/false'.
  void Expect(void* tag, ExpectedResult result,
              SourceLocation location = SourceLocation());

  std::string ToString() const;
  std::vector<std::string> ToStrings() const;
  std::string ToShortString() const;
  std::vector<std::string> ToShortStrings() const;

  // Logging verifications helps debug CI problems a lot.
  // Only disable if the logging prevents a stress test like scenario from
  // passing.
  void SetLogVerifications(bool log_verifications) {
    log_verifications_ = log_verifications;
  }

  static void* tag(intptr_t t) { return reinterpret_cast<void*>(t); }

 private:
  struct Expectation {
    SourceLocation location;
    void* tag;
    ExpectedResult result;

    std::string ToString() const;
    std::string ToShortString() const;
  };

  void FailNoEventReceived(const SourceLocation& location) const;
  void FailUnexpectedEvent(grpc_event* ev,
                           const SourceLocation& location) const;
  bool AllMaybes() const;
  grpc_event Step(gpr_timespec deadline);

  grpc_completion_queue* const cq_;
  std::vector<Expectation> expectations_;
  absl::AnyInvocable<void(Failure) const> fail_;
  absl::AnyInvocable<void(
      grpc_event_engine::experimental::EventEngine::Duration) const>
      step_fn_;
  absl::flat_hash_map<void*, std::vector<SuccessfulStateString*>>
      successful_state_strings_;
  bool log_verifications_ = true;
  bool added_expectations_ = false;
};

}  // namespace grpc_core

int byte_buffer_eq_slice(grpc_byte_buffer* bb, grpc_slice b);
int byte_buffer_eq_string(grpc_byte_buffer* bb, const char* str);
int contains_metadata(grpc_metadata_array* array, const char* key,
                      const char* value);
int contains_metadata_slices(grpc_metadata_array* array, grpc_slice key,
                             grpc_slice value);

#endif  // GRPC_TEST_CORE_END2END_CQ_VERIFIER_H
