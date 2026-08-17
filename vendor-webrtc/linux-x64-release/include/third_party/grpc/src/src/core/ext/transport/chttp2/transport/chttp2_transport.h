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

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_CHTTP2_TRANSPORT_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_CHTTP2_TRANSPORT_H

#include <grpc/slice.h>
#include <grpc/support/port_platform.h>

#include <cstdint>
#include <optional>
#include <string>

#include "src/core/channelz/channelz.h"
#include "src/core/ext/transport/chttp2/transport/flow_control.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/debug/trace.h"
#include "src/core/lib/iomgr/buffer_list.h"
#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/endpoint.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/telemetry/call_tracer.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/time.h"

/// Creates a CHTTP2 Transport. This takes ownership of a \a resource_user ref
/// from the caller; if the caller still needs the resource_user after creating
/// a transport, the caller must take another ref.
grpc_core::Transport* grpc_create_chttp2_transport(
    const grpc_core::ChannelArgs& channel_args,
    grpc_core::OrphanablePtr<grpc_endpoint> ep, bool is_client);

/// Takes ownership of \a read_buffer, which (if non-NULL) contains
/// leftover bytes previously read from the endpoint (e.g., by handshakers).
/// If non-null, \a notify_on_receive_settings will be scheduled when
/// HTTP/2 settings are received from the peer.
/// If non-null, the endpoint will be removed from
/// interested_parties_until_recv_settings before
/// notify_on_receive_settings is invoked.
void grpc_chttp2_transport_start_reading(
    grpc_core::Transport* transport, grpc_slice_buffer* read_buffer,
    grpc_closure* notify_on_receive_settings,
    grpc_pollset_set* interested_parties_until_recv_settings,
    grpc_closure* notify_on_close);

namespace grpc_core {
typedef void (*TestOnlyGlobalHttp2TransportInitCallback)();
typedef void (*TestOnlyGlobalHttp2TransportDestructCallback)();

void TestOnlySetGlobalHttp2TransportInitCallback(
    TestOnlyGlobalHttp2TransportInitCallback callback);

void TestOnlySetGlobalHttp2TransportDestructCallback(
    TestOnlyGlobalHttp2TransportDestructCallback callback);

// If \a disable is true, the HTTP2 transport will not update the connectivity
// state tracker to TRANSIENT_FAILURE when a goaway is received. This prevents
// the watchers (eg. client_channel) from noticing the GOAWAY, thereby allowing
// us to test the racy behavior when a call is sent down the stack around the
// same time that a GOAWAY is received.
void TestOnlyGlobalHttp2TransportDisableTransientFailureStateNotification(
    bool disable);

typedef void (*WriteTimestampsCallback)(void*, Timestamps*,
                                        grpc_error_handle error);
typedef void* (*CopyContextFn)(Arena*);

void GrpcHttp2SetWriteTimestampsCallback(WriteTimestampsCallback fn);
void GrpcHttp2SetCopyContextFn(CopyContextFn fn);

WriteTimestampsCallback GrpcHttp2GetWriteTimestampsCallback();
CopyContextFn GrpcHttp2GetCopyContextFn();

// Interprets the passed arg as a ContextList type and for each entry in the
// passed ContextList, it executes the function set using
// GrpcHttp2SetWriteTimestampsCallback method with each context in the list
// and \a ts. It also deletes/frees up the passed ContextList after this
// operation.
void ForEachContextListEntryExecute(void* arg, Timestamps* ts,
                                    grpc_error_handle error);

class HttpAnnotation : public CallTracerAnnotationInterface::Annotation {
 public:
  enum class Type : uint8_t {
    kUnknown = 0,
    // When the first byte enters the HTTP transport.
    kStart,
    // When the first byte leaves the HTTP transport.
    kHeadWritten,
    // When the last byte leaves the HTTP transport.
    kEnd,
  };

  // A snapshot of write stats to export.
  struct WriteStats {
    size_t target_write_size;
  };

  HttpAnnotation(Type type, gpr_timespec time);

  HttpAnnotation& Add(const chttp2::TransportFlowControl::Stats& stats) {
    transport_stats_ = stats;
    return *this;
  }

  HttpAnnotation& Add(const chttp2::StreamFlowControl::Stats& stats) {
    stream_stats_ = stats;
    return *this;
  }

  HttpAnnotation& Add(const WriteStats& stats) {
    write_stats_ = stats;
    return *this;
  }

  std::string ToString() const override;

  Type http_type() const { return type_; }
  gpr_timespec time() const { return time_; }
  std::optional<chttp2::TransportFlowControl::Stats> transport_stats() const {
    return transport_stats_;
  }
  std::optional<chttp2::StreamFlowControl::Stats> stream_stats() const {
    return stream_stats_;
  }
  std::optional<WriteStats> write_stats() const { return write_stats_; }

 private:
  const Type type_;
  const gpr_timespec time_;
  std::optional<chttp2::TransportFlowControl::Stats> transport_stats_;
  std::optional<chttp2::StreamFlowControl::Stats> stream_stats_;
  std::optional<WriteStats> write_stats_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_CHTTP2_TRANSPORT_H
