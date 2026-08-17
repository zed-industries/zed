//
//
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
//
//

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_HTTP2_SERVER_TRANSPORT_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_HTTP2_SERVER_TRANSPORT_H

#include <cstdint>
#include <type_traits>

#include "absl/container/flat_hash_map.h"
#include "src/core/call/call_destination.h"
#include "src/core/call/call_spine.h"
#include "src/core/ext/transport/chttp2/transport/frame.h"
#include "src/core/ext/transport/chttp2/transport/hpack_encoder.h"
#include "src/core/ext/transport/chttp2/transport/hpack_parser.h"
#include "src/core/ext/transport/chttp2/transport/http2_settings.h"
#include "src/core/lib/promise/mpsc.h"
#include "src/core/lib/promise/party.h"
#include "src/core/lib/transport/promise_endpoint.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"

namespace grpc_core {
namespace http2 {

// Experimental : This is just the initial skeleton of class
// and it is functions. The code will be written iteratively.
// Do not use or edit any of these functions unless you are
// familiar with the PH2 project (Moving chttp2 to promises.)
// TODO(tjagtap) : [PH2][P3] : Delete this comment when http2
// rollout begins
class Http2ServerTransport final : public ServerTransport {
  // TODO(tjagtap) : [PH2][P3] Move the definitions to the header for better
  // inlining. For now definitions are in the cc file to
  // reduce cognitive load in the header.
 public:
  Http2ServerTransport(
      PromiseEndpoint endpoint, GRPC_UNUSED const ChannelArgs& channel_args,
      std::shared_ptr<grpc_event_engine::experimental::EventEngine>
          event_engine);
  ~Http2ServerTransport() override;

  FilterStackTransport* filter_stack_transport() override { return nullptr; }
  ClientTransport* client_transport() override { return nullptr; }
  ServerTransport* server_transport() override { return this; }
  absl::string_view GetTransportName() const override { return "http2"; }

  // TODO(tjagtap) : [PH2][EXT] : These can be removed when event engine rollout
  // is complete.
  void SetPollset(grpc_stream*, grpc_pollset*) override {}
  void SetPollsetSet(grpc_stream*, grpc_pollset_set*) override {}

  void SetCallDestination(
      RefCountedPtr<UnstartedCallDestination> call_destination) override;

  void PerformOp(grpc_transport_op*) override;

  void Orphan() override;
  void AbortWithError();

  RefCountedPtr<channelz::SocketNode> GetSocketNode() const override {
    return nullptr;
  }

 private:
  // Reading from the endpoint.

  // Returns a promise to keep reading in a Loop till a fail/close is received.
  auto ReadLoop();

  // Returns a promise that will read and process one HTTP2 frame.
  auto ReadAndProcessOneFrame();

  // Returns a promise that will process one HTTP2 frame.
  auto ProcessOneFrame(Http2Frame frame);

  // Returns a promise that will do the cleanup after the ReadLoop ends.
  auto OnReadLoopEnded();

  // Writing to the endpoint.

  // Read from the MPSC queue and write it.
  auto WriteFromQueue();

  // Returns a promise to keep writing in a Loop till a fail/close is received.
  auto WriteLoop();

  // Returns a promise that will do the cleanup after the WriteLoop ends.
  auto OnWriteLoopEnded();

  RefCountedPtr<Party> general_party_;

  PromiseEndpoint endpoint_;
  Http2SettingsManager settings_;

  // TODO(tjagtap) : [PH2][P3] : This is not nice. Fix by using Stapler.
  Http2FrameHeader current_frame_header_;

  struct Stream : public RefCounted<Stream> {
    explicit Stream(CallInitiator call) : call(std::move(call)) {}
    // Transport holds one CallHandler object for each Stream.
    CallInitiator call;
    // TODO(tjagtap) : [PH2][P2] : Add more members as necessary
    // TODO(tjagtap) : [PH2][P2] : May be add state of Stream - Idle , Open etc
    // https://datatracker.ietf.org/doc/html/rfc9113#name-stream-identifiers
  };
  RefCountedPtr<UnstartedCallDestination> call_destination_;

  MpscReceiver<Http2Frame> outgoing_frames_;

  Mutex transport_mutex_;
  // TODO(tjagtap) : [PH2][P2] : Add to map in SetCallDestination and clean this
  // mapping up in the on_done of the CallInitiator or CallHandler
  absl::flat_hash_map<uint32_t, RefCountedPtr<Stream>> stream_list_
      ABSL_GUARDED_BY(transport_mutex_);

  // TODO(tjagtap) : [PH2][P1] : Either use this in code or delete it.
  // uint32_t next_stream_id_ ABSL_GUARDED_BY(transport_mutex_) = 1;
  // TODO(tjagtap) : [PH2][P1] : Either use this in code or delete it. This was
  // copied from Chaotic Good.
  // uint32_t last_seen_new_stream_id_ = 0;

  // TODO(tjagtap) : [PH2][P1] : Implement if needed.
  // uint32_t MakeStream(CallHandler call_handler);
  // TODO(tjagtap) : [PH2][P1] : Implement if needed.
  // RefCountedPtr<Http2ServerTransport::Stream> LookupStream(uint32_t
  // stream_id);
};

GRPC_CHECK_CLASS_SIZE(Http2ServerTransport, 240);

}  // namespace http2
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_HTTP2_SERVER_TRANSPORT_H
