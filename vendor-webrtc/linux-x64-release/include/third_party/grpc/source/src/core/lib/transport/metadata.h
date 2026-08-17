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

#ifndef GRPC_SRC_CORE_LIB_TRANSPORT_METADATA_H
#define GRPC_SRC_CORE_LIB_TRANSPORT_METADATA_H

#include <grpc/support/port_platform.h>

#include "src/core/lib/transport/metadata_batch.h"

namespace grpc_core {

// Server metadata type
// TODO(ctiller): This should be a bespoke instance of MetadataMap<>
using ServerMetadata = grpc_metadata_batch;
using ServerMetadataHandle = Arena::PoolPtr<ServerMetadata>;

// Client initial metadata type
// TODO(ctiller): This should be a bespoke instance of MetadataMap<>
using ClientMetadata = grpc_metadata_batch;
using ClientMetadataHandle = Arena::PoolPtr<ClientMetadata>;

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

#endif  // GRPC_SRC_CORE_LIB_TRANSPORT_METADATA_H
