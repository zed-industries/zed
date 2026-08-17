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

#ifndef GRPCPP_IMPL_STATUS_H
#define GRPCPP_IMPL_STATUS_H

// IWYU pragma: private, include <grpcpp/support/status.h>

#include <grpc/status.h>
#include <grpc/support/port_platform.h>
#include <grpcpp/support/config.h>
#include <grpcpp/support/status_code_enum.h>

namespace grpc {

/// Did it work? If it didn't, why?
///
/// See \a grpc::StatusCode for details on the available code and their meaning.
class GRPC_MUST_USE_RESULT_WHEN_USE_STRICT_WARNING GRPCXX_DLL Status {
 public:
  /// Construct an OK instance.
  Status() : code_(StatusCode::OK) {
    // Static assertions to make sure that the C++ API value correctly
    // maps to the core surface API value
    static_assert(StatusCode::OK == static_cast<StatusCode>(GRPC_STATUS_OK),
                  "Mismatched status code");
    static_assert(
        StatusCode::CANCELLED == static_cast<StatusCode>(GRPC_STATUS_CANCELLED),
        "Mismatched status code");
    static_assert(
        StatusCode::UNKNOWN == static_cast<StatusCode>(GRPC_STATUS_UNKNOWN),
        "Mismatched status code");
    static_assert(StatusCode::INVALID_ARGUMENT ==
                      static_cast<StatusCode>(GRPC_STATUS_INVALID_ARGUMENT),
                  "Mismatched status code");
    static_assert(StatusCode::DEADLINE_EXCEEDED ==
                      static_cast<StatusCode>(GRPC_STATUS_DEADLINE_EXCEEDED),
                  "Mismatched status code");
    static_assert(
        StatusCode::NOT_FOUND == static_cast<StatusCode>(GRPC_STATUS_NOT_FOUND),
        "Mismatched status code");
    static_assert(StatusCode::ALREADY_EXISTS ==
                      static_cast<StatusCode>(GRPC_STATUS_ALREADY_EXISTS),
                  "Mismatched status code");
    static_assert(StatusCode::PERMISSION_DENIED ==
                      static_cast<StatusCode>(GRPC_STATUS_PERMISSION_DENIED),
                  "Mismatched status code");
    static_assert(StatusCode::UNAUTHENTICATED ==
                      static_cast<StatusCode>(GRPC_STATUS_UNAUTHENTICATED),
                  "Mismatched status code");
    static_assert(StatusCode::RESOURCE_EXHAUSTED ==
                      static_cast<StatusCode>(GRPC_STATUS_RESOURCE_EXHAUSTED),
                  "Mismatched status code");
    static_assert(StatusCode::FAILED_PRECONDITION ==
                      static_cast<StatusCode>(GRPC_STATUS_FAILED_PRECONDITION),
                  "Mismatched status code");
    static_assert(
        StatusCode::ABORTED == static_cast<StatusCode>(GRPC_STATUS_ABORTED),
        "Mismatched status code");
    static_assert(StatusCode::OUT_OF_RANGE ==
                      static_cast<StatusCode>(GRPC_STATUS_OUT_OF_RANGE),
                  "Mismatched status code");
    static_assert(StatusCode::UNIMPLEMENTED ==
                      static_cast<StatusCode>(GRPC_STATUS_UNIMPLEMENTED),
                  "Mismatched status code");
    static_assert(
        StatusCode::INTERNAL == static_cast<StatusCode>(GRPC_STATUS_INTERNAL),
        "Mismatched status code");
    static_assert(StatusCode::UNAVAILABLE ==
                      static_cast<StatusCode>(GRPC_STATUS_UNAVAILABLE),
                  "Mismatched status code");
    static_assert(
        StatusCode::DATA_LOSS == static_cast<StatusCode>(GRPC_STATUS_DATA_LOSS),
        "Mismatched status code");
  }

  /// Construct an instance with associated \a code and \a error_message.
  /// It is an error to construct an OK status with non-empty \a error_message.
  /// Note that \a message is intentionally accepted as a const reference
  /// instead of a value (which results in a copy instead of a move) to allow
  /// for easy transition to absl::Status in the future which accepts an
  /// absl::string_view as a parameter.
  Status(StatusCode code, const std::string& error_message)
      : code_(code), error_message_(error_message) {}

  /// Construct an instance with \a code,  \a error_message and
  /// \a error_details. It is an error to construct an OK status with non-empty
  /// \a error_message and/or \a error_details.
  Status(StatusCode code, const std::string& error_message,
         const std::string& error_details)
      : code_(code),
        error_message_(error_message),
        binary_error_details_(error_details) {}

  // Pre-defined special status objects.
  /// An OK pre-defined instance.
  static const Status& OK;
  /// A CANCELLED pre-defined instance.
  static const Status& CANCELLED;

  /// Return the instance's error code.
  StatusCode error_code() const { return code_; }
  /// Return the instance's error message.
  std::string error_message() const { return error_message_; }
  /// Return the (binary) error details.
  // Usually it contains a serialized google.rpc.Status proto.
  std::string error_details() const { return binary_error_details_; }

  /// Is the status OK?
  bool ok() const { return code_ == StatusCode::OK; }

  // Ignores any errors. This method does nothing except potentially suppress
  // complaints from any tools that are checking that errors are not dropped on
  // the floor.
  void IgnoreError() const {}

 private:
  StatusCode code_;
  std::string error_message_;
  std::string binary_error_details_;
};

}  // namespace grpc

#endif  // GRPCPP_IMPL_STATUS_H
