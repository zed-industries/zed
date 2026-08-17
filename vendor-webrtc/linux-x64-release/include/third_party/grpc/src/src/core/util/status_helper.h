//
//
// Copyright 2021 the gRPC authors.
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

#ifndef GRPC_SRC_CORE_UTIL_STATUS_HELPER_H
#define GRPC_SRC_CORE_UTIL_STATUS_HELPER_H

#include <grpc/support/port_platform.h>
#include <stdint.h>

#include <optional>
#include <string>
#include <vector>

#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "absl/time/time.h"
#include "src/core/util/debug_location.h"

extern "C" {
struct google_rpc_Status;
struct upb_Arena;
}

#define GRPC_RETURN_IF_ERROR(expr)      \
  do {                                  \
    const absl::Status status = (expr); \
    if (!status.ok()) return status;    \
  } while (0)

namespace grpc_core {

/// This enum should have the same value of grpc_error_ints
enum class StatusIntProperty {
  /// stream identifier: for errors that are associated with an individual
  /// wire stream
  // TODO(tjagtap): Remove this when the PH2 migration is done.
  kStreamId,
  /// grpc status code representing this error
  // TODO(roth): Remove this after error_flatten experiment is removed.
  kRpcStatus,
  /// http2 error code associated with the error (see the HTTP2 RFC)
  // TODO(tjagtap): Remove this as part of creating a new HTTP/2 error type.
  kHttp2Error,
  /// channel connectivity state associated with the error
  // TODO(roth): Remove this when the promise migration is done.
  ChannelConnectivityState,
  /// LB policy drop
  // TODO(roth): Replace this with something else, possibly a call
  // context element.
  kLbPolicyDrop,
};

/// This enum should have the same value of grpc_error_strs
// TODO(roth): Remove this after error_flatten experiment is removed.
enum class StatusStrProperty {
  /// peer that we were trying to communicate when this error occurred
  kGrpcMessage,
};

/// Creates a status with given additional information
absl::Status StatusCreate(absl::StatusCode code, absl::string_view msg,
                          const DebugLocation& location,
                          std::vector<absl::Status> children);

/// Sets the int property to the status
void StatusSetInt(absl::Status* status, StatusIntProperty key, intptr_t value);

/// Gets the int property from the status
GRPC_MUST_USE_RESULT
std::optional<intptr_t> StatusGetInt(const absl::Status& status,
                                     StatusIntProperty key);

/// Sets the str property to the status
void StatusSetStr(absl::Status* status, StatusStrProperty key,
                  absl::string_view value);

/// Gets the str property from the status
GRPC_MUST_USE_RESULT std::optional<std::string> StatusGetStr(
    const absl::Status& status, StatusStrProperty key);

/// Adds a child status to status
void StatusAddChild(absl::Status* status, absl::Status child);

/// Returns all children status from a status
GRPC_MUST_USE_RESULT std::vector<absl::Status> StatusGetChildren(
    absl::Status status);

/// Returns a string representation from status
/// Error status will be like
///   STATUS[:MESSAGE] [{PAYLOADS[, children:[CHILDREN-STATUS-LISTS]]}]
/// e.g.
///   CANCELLATION:SampleMessage {errno:'2021', line:'54', children:[ABORTED]}
GRPC_MUST_USE_RESULT std::string StatusToString(const absl::Status& status);

/// Adds prefix to the message of status.
absl::Status AddMessagePrefix(absl::string_view prefix,
                              const absl::Status& status);

namespace internal {

/// Builds a upb message, google_rpc_Status from a status
/// This is for internal implementation & test only
GRPC_MUST_USE_RESULT google_rpc_Status* StatusToProto(
    const absl::Status& status, upb_Arena* arena);

/// Builds a status from a upb message, google_rpc_Status
/// This is for internal implementation & test only
absl::Status StatusFromProto(google_rpc_Status* msg);

/// Returns ptr that is allocated in the heap memory and the given status is
/// copied into. This ptr can be used to get Status later and should be
/// freed by StatusFreeHeapPtr. This can be 0 in case of OkStatus.
uintptr_t StatusAllocHeapPtr(absl::Status s);

/// Frees the allocated status at heap ptr.
void StatusFreeHeapPtr(uintptr_t ptr);

/// Get the status from a heap ptr.
absl::Status StatusGetFromHeapPtr(uintptr_t ptr);

/// Move the status from a heap ptr. (GetFrom & FreeHeap)
absl::Status StatusMoveFromHeapPtr(uintptr_t ptr);

}  // namespace internal

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_STATUS_HELPER_H
