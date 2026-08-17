// Copyright 2024 gRPC authors.
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

#ifndef GRPC_SRC_CORE_CALL_METADATA_H
#define GRPC_SRC_CORE_CALL_METADATA_H

#include <grpc/support/port_platform.h>

#include "src/core/call/metadata_batch.h"
#include "src/core/lib/promise/try_seq.h"

namespace grpc_core {

// Server metadata type
// TODO(ctiller): This should be a bespoke instance of MetadataMap<>
using ServerMetadata = grpc_metadata_batch;
using ServerMetadataHandle = Arena::PoolPtr<ServerMetadata>;

// Client initial metadata type
// TODO(ctiller): This should be a bespoke instance of MetadataMap<>
using ClientMetadata = grpc_metadata_batch;
using ClientMetadataHandle = Arena::PoolPtr<ClientMetadata>;

template <typename T>
class ServerMetadataOrHandle {
 public:
  using ValueType = Arena::PoolPtr<T>;

  static ServerMetadataOrHandle Ok(ValueType value) {
    return ServerMetadataOrHandle{nullptr, std::move(value)};
  }
  static ServerMetadataOrHandle Failure(ServerMetadataHandle server_metadata) {
    return ServerMetadataOrHandle{std::move(server_metadata), nullptr};
  }

  bool ok() const { return server_metadata_ == nullptr; }
  ServerMetadataHandle& metadata() {
    CHECK(!ok());
    return server_metadata_;
  }
  ValueType& operator*() {
    CHECK(ok());
    return value_;
  }
  const ServerMetadataHandle& metadata() const {
    CHECK(!ok());
    return server_metadata_;
  }
  const ValueType& operator*() const {
    CHECK(ok());
    return value_;
  }

  ServerMetadataHandle TakeMetadata() && {
    CHECK(!ok());
    return std::move(server_metadata_);
  }

  ValueType TakeValue() && {
    CHECK(ok());
    return std::move(value_);
  }

 private:
  ServerMetadataOrHandle(ServerMetadataHandle server_metadata, ValueType value)
      : server_metadata_(std::move(server_metadata)),
        value_(std::move(value)) {}

  ServerMetadataHandle server_metadata_;
  ValueType value_;
};

template <typename T>
struct FailureStatusCastImpl<ServerMetadataOrHandle<T>, ServerMetadataHandle> {
  static ServerMetadataOrHandle<T> Cast(ServerMetadataHandle t) {
    return ServerMetadataOrHandle<T>::Failure(std::move(t));
  }
};

template <typename T>
struct FailureStatusCastImpl<ServerMetadataOrHandle<T>, ServerMetadataHandle&> {
  static ServerMetadataOrHandle<T> Cast(ServerMetadataHandle& t) {
    return ServerMetadataOrHandle<T>::Failure(std::move(t));
  }
};

template <>
struct FailureStatusCastImpl<ServerMetadataHandle, ServerMetadataHandle&> {
  static ServerMetadataHandle Cast(ServerMetadataHandle& t) {
    return std::move(t);
  }
};

template <typename T>
inline bool IsStatusOk(const ServerMetadataOrHandle<T>& x) {
  return x.ok();
}

namespace promise_detail {
template <typename T>
struct AllowGenericTrySeqTraits<ServerMetadataOrHandle<T>> {
  static constexpr bool value = false;
};
template <typename T>
struct TrySeqTraitsWithSfinae<ServerMetadataOrHandle<T>> {
  using UnwrappedType = Arena::PoolPtr<T>;
  using WrappedType = ServerMetadataOrHandle<T>;
  template <typename Next>
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static auto CallFactory(
      Next* next, ServerMetadataOrHandle<T>&& status) {
    return next->Make(std::move(*status));
  }
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static bool IsOk(
      const ServerMetadataOrHandle<T>& status) {
    return status.ok();
  }
  static std::string ErrorString(const ServerMetadataOrHandle<T>& status) {
    return status.metadata()->DebugString();
  }
  template <typename R>
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static R ReturnValue(
      ServerMetadataOrHandle<T>&& status) {
    return FailureStatusCast<R>(status.metadata());
  }
};
}  // namespace promise_detail

// TODO(ctiller): separate when we have different types for client/server
// metadata.
template <typename Sink>
void AbslStringify(Sink& sink, const Arena::PoolPtr<grpc_metadata_batch>& md) {
  if (md == nullptr) {
    sink.Append("nullptr");
    return;
  }
  sink.Append("ServerMetadata{");
  sink.Append(md->DebugString());
  sink.Append("}");
}

// Ok/not-ok check for trailing metadata, so that it can be used as result types
// for TrySeq.
inline bool IsStatusOk(const ServerMetadataHandle& m) {
  return m->get(GrpcStatusMetadata()).value_or(GRPC_STATUS_UNKNOWN) ==
         GRPC_STATUS_OK;
}

// Convert absl::Status to ServerMetadata
ServerMetadataHandle ServerMetadataFromStatus(const absl::Status& status);
// Convert absl::Status to ServerMetadata, and set GrpcCallWasCancelled() to
// true
ServerMetadataHandle CancelledServerMetadataFromStatus(
    const absl::Status& status);
// Server metadata with status code set
inline ServerMetadataHandle ServerMetadataFromStatus(grpc_status_code code) {
  auto hdl = Arena::MakePooledForOverwrite<ServerMetadata>();
  hdl->Set(GrpcStatusMetadata(), code);
  return hdl;
}
inline ServerMetadataHandle CancelledServerMetadataFromStatus(
    grpc_status_code code) {
  auto hdl = Arena::MakePooledForOverwrite<ServerMetadata>();
  hdl->Set(GrpcStatusMetadata(), code);
  hdl->Set(GrpcCallWasCancelled(), true);
  return hdl;
}
// The same, but with an error string
ServerMetadataHandle ServerMetadataFromStatus(grpc_status_code code,
                                              absl::string_view message);
ServerMetadataHandle CancelledServerMetadataFromStatus(
    grpc_status_code code, absl::string_view message);

template <>
struct StatusCastImpl<ServerMetadataHandle, absl::Status> {
  static ServerMetadataHandle Cast(const absl::Status& m) {
    return ServerMetadataFromStatus(m);
  }
};

template <>
struct StatusCastImpl<ServerMetadataHandle, const absl::Status&> {
  static ServerMetadataHandle Cast(const absl::Status& m) {
    return ServerMetadataFromStatus(m);
  }
};

template <>
struct StatusCastImpl<ServerMetadataHandle, absl::Status&> {
  static ServerMetadataHandle Cast(const absl::Status& m) {
    return ServerMetadataFromStatus(m);
  }
};

// Anything that can be first cast to absl::Status can then be cast to
// ServerMetadataHandle.
template <typename T>
struct StatusCastImpl<
    ServerMetadataHandle, T,
    absl::void_t<decltype(&StatusCastImpl<absl::Status, T>::Cast)>> {
  static ServerMetadataHandle Cast(const T& m) {
    return ServerMetadataFromStatus(StatusCast<absl::Status>(m));
  }
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CALL_METADATA_H
