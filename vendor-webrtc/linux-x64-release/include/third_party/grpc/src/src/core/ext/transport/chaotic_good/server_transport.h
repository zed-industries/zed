// Copyright 2022 gRPC authors.
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

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHAOTIC_GOOD_SERVER_TRANSPORT_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHAOTIC_GOOD_SERVER_TRANSPORT_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/event_engine/memory_allocator.h>
#include <grpc/grpc.h>
#include <grpc/slice.h>
#include <grpc/support/port_platform.h>
#include <stdint.h>
#include <stdio.h>

#include <cstdint>
#include <initializer_list>  // IWYU pragma: keep
#include <iostream>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <tuple>
#include <type_traits>
#include <utility>
#include <variant>

#include "absl/base/thread_annotations.h"
#include "absl/container/flat_hash_map.h"
#include "absl/functional/any_invocable.h"
#include "absl/random/random.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "src/core/call/metadata_batch.h"
#include "src/core/ext/transport/chaotic_good/config.h"
#include "src/core/ext/transport/chaotic_good/frame.h"
#include "src/core/ext/transport/chaotic_good/frame_header.h"
#include "src/core/ext/transport/chaotic_good/frame_transport.h"
#include "src/core/ext/transport/chaotic_good/message_chunker.h"
#include "src/core/ext/transport/chaotic_good/message_reassembly.h"
#include "src/core/ext/transport/chaotic_good/pending_connection.h"
#include "src/core/ext/transport/chaotic_good/transport_context.h"
#include "src/core/lib/event_engine/default_event_engine.h"  // IWYU pragma: keep
#include "src/core/lib/promise/activity.h"
#include "src/core/lib/promise/context.h"
#include "src/core/lib/promise/if.h"
#include "src/core/lib/promise/inter_activity_latch.h"
#include "src/core/lib/promise/inter_activity_pipe.h"
#include "src/core/lib/promise/loop.h"
#include "src/core/lib/promise/mpsc.h"
#include "src/core/lib/promise/party.h"
#include "src/core/lib/promise/pipe.h"
#include "src/core/lib/promise/poll.h"
#include "src/core/lib/promise/seq.h"
#include "src/core/lib/promise/try_join.h"
#include "src/core/lib/promise/try_seq.h"
#include "src/core/lib/resource_quota/arena.h"
#include "src/core/lib/resource_quota/memory_quota.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/lib/slice/slice_buffer.h"
#include "src/core/lib/slice/slice_internal.h"
#include "src/core/lib/transport/promise_endpoint.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"

namespace grpc_core {
namespace chaotic_good {

class ChaoticGoodServerTransport final : public ServerTransport {
 public:
  ChaoticGoodServerTransport(const ChannelArgs& args,
                             OrphanablePtr<FrameTransport> frame_transport,
                             MessageChunker message_chunker);

  FilterStackTransport* filter_stack_transport() override { return nullptr; }
  ClientTransport* client_transport() override { return nullptr; }
  ServerTransport* server_transport() override { return this; }
  absl::string_view GetTransportName() const override { return "chaotic_good"; }
  void SetPollset(grpc_stream*, grpc_pollset*) override {}
  void SetPollsetSet(grpc_stream*, grpc_pollset_set*) override {}
  void PerformOp(grpc_transport_op*) override;
  void Orphan() override;
  RefCountedPtr<channelz::SocketNode> GetSocketNode() const override {
    return frame_transport_->ctx()->socket_node;
  }

  void SetCallDestination(
      RefCountedPtr<UnstartedCallDestination> call_destination) override;
  void AbortWithError();

 private:
  struct Stream : public RefCounted<Stream> {
    explicit Stream(CallInitiator call) : call(std::move(call)) {}
    CallInitiator call;
    MessageReassembly message_reassembly;
    Party::SpawnSerializer* spawn_serializer =
        call.party()->MakeSpawnSerializer();
  };
  using StreamMap = absl::flat_hash_map<uint32_t, RefCountedPtr<Stream> >;

  class StreamDispatch : public FrameTransportSink {
   public:
    StreamDispatch(const ChannelArgs& args, FrameTransport* frame_transport,
                   MessageChunker message_chunker,
                   RefCountedPtr<UnstartedCallDestination> call_destination);

    void OnIncomingFrame(IncomingFrame incoming_frame) override;
    void OnFrameTransportClosed(absl::Status status) override;

    void StartConnectivityWatch(
        grpc_connectivity_state state,
        OrphanablePtr<ConnectivityStateWatcherInterface> watcher);
    void StopConnectivityWatch(ConnectivityStateWatcherInterface* watcher);

   private:
    absl::Status NewStream(
        uint32_t stream_id,
        ClientInitialMetadataFrame client_initial_metadata_frame);
    absl::Status AddStream(uint32_t stream_id, CallInitiator call_initiator);
    RefCountedPtr<Stream> LookupStream(uint32_t stream_id);
    RefCountedPtr<Stream> ExtractStream(uint32_t stream_id);

    template <typename T>
    void DispatchFrame(IncomingFrame frame);
    auto PushFrameIntoCall(RefCountedPtr<Stream> stream, MessageFrame frame);
    auto PushFrameIntoCall(RefCountedPtr<Stream> stream,
                           ClientEndOfStream frame);
    auto PushFrameIntoCall(RefCountedPtr<Stream> stream,
                           BeginMessageFrame frame);
    auto PushFrameIntoCall(RefCountedPtr<Stream> stream,
                           MessageChunkFrame frame);
    auto SendCallInitialMetadataAndBody(
        uint32_t stream_id, CallInitiator call_initiator,
        std::shared_ptr<TcpCallTracer> call_tracer);
    auto SendCallBody(uint32_t stream_id, CallInitiator call_initiator,
                      std::shared_ptr<TcpCallTracer> call_tracer);
    auto CallOutboundLoop(uint32_t stream_id, CallInitiator call_initiator);
    auto ProcessNextFrame(IncomingFrame frame);

    Mutex mu_;
    StreamMap stream_map_ ABSL_GUARDED_BY(mu_);
    uint32_t last_seen_new_stream_id_ ABSL_GUARDED_BY(mu_) = 0;
    ConnectivityStateTracker state_tracker_ ABSL_GUARDED_BY(mu_){
        "chaotic_good_server", GRPC_CHANNEL_READY};
    const TransportContextPtr ctx_;
    const RefCountedPtr<CallArenaAllocator> call_arena_allocator_;
    const RefCountedPtr<UnstartedCallDestination> call_destination_;
    Party::SpawnSerializer* incoming_frame_spawner_;
    MessageChunker message_chunker_;
    MpscSender<OutgoingFrame> outgoing_frames_;
    RefCountedPtr<Party> party_;
  };

  struct ConstructionParameters {
    ConstructionParameters(const ChannelArgs& args,
                           MessageChunker message_chunker)
        : args(args), message_chunker(message_chunker) {}
    ChannelArgs args;
    MessageChunker message_chunker;
  };

  // Read different parts of the server frame from control/data endpoints
  // based on frame header.
  // Resolves to a StatusOr<tuple<SliceBuffer, SliceBuffer>>
  auto ReadFrameBody(Slice read_buffer);
  void SendCancel(uint32_t stream_id, absl::Status why);

  struct Orphaned {};
  using State = std::variant<std::unique_ptr<ConstructionParameters>,
                             RefCountedPtr<StreamDispatch>, Orphaned>;
  State state_;
  OrphanablePtr<FrameTransport> frame_transport_;
};

}  // namespace chaotic_good
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHAOTIC_GOOD_SERVER_TRANSPORT_H
