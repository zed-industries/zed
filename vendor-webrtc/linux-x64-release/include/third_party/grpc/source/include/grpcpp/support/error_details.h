//
//
// Copyright 2017 gRPC authors.
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

#ifndef GRPCPP_SUPPORT_ERROR_DETAILS_H
#define GRPCPP_SUPPORT_ERROR_DETAILS_H

#include <grpcpp/support/status.h>

namespace grpc {

/// Map a \a grpc::Status to a \a google::rpc::Status.
/// The given \a to object will be cleared.
/// On success, returns status with OK.
/// Returns status with \a INVALID_ARGUMENT, if failed to deserialize.
/// Returns status with \a FAILED_PRECONDITION, if \a to is nullptr.
///
/// \note
/// This function is a template to avoid a build dep on \a status.proto.
/// However, this function still requires that \tparam T is of type
/// \a google::rpc::Status, which is defined at
/// https://github.com/googleapis/googleapis/blob/master/google/rpc/status.proto
template <typename T>
grpc::Status ExtractErrorDetails(const grpc::Status& from, T* to) {
  if (to == nullptr) {
    return grpc::Status(grpc::StatusCode::FAILED_PRECONDITION, "");
  }
  if (!to->ParseFromString(from.error_details())) {
    return grpc::Status(grpc::StatusCode::INVALID_ARGUMENT, "");
  }
  return grpc::Status::OK;
}
inline grpc::Status ExtractErrorDetails(const grpc::Status&, std::nullptr_t) {
  return grpc::Status(grpc::StatusCode::FAILED_PRECONDITION, "");
}

/// Map \a google::rpc::Status to a \a grpc::Status.
/// Returns OK on success.
/// Returns status with \a FAILED_PRECONDITION if \a to is nullptr.
///
/// \note
/// This function is a template to avoid a build dep on \a status.proto.
/// However, this function still requires that \tparam T is of type
/// \a google::rpc::Status, which is defined at
/// https://github.com/googleapis/googleapis/blob/master/google/rpc/status.proto
template <typename T>
grpc::Status SetErrorDetails(const T& from, grpc::Status* to) {
  if (to == nullptr) {
    return grpc::Status(grpc::StatusCode::FAILED_PRECONDITION, "");
  }
  grpc::StatusCode code = grpc::StatusCode::UNKNOWN;
  if (from.code() >= grpc::StatusCode::OK &&
      from.code() <= grpc::StatusCode::UNAUTHENTICATED) {
    code = static_cast<grpc::StatusCode>(from.code());
  }
  *to = grpc::Status(code, from.message(), from.SerializeAsString());
  return grpc::Status::OK;
}

}  // namespace grpc

#endif  // GRPCPP_SUPPORT_ERROR_DETAILS_H
