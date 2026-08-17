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

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHAOTIC_GOOD_TCP_FRAME_TRANSPORT_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHAOTIC_GOOD_TCP_FRAME_TRANSPORT_H

#include <vector>

#include "src/core/ext/transport/chaotic_good/control_endpoint.h"
#include "src/core/ext/transport/chaotic_good/data_endpoints.h"
#include "src/core/ext/transport/chaotic_good/frame_transport.h"
#include "src/core/ext/transport/chaotic_good/pending_connection.h"
#include "src/core/ext/transport/chaotic_good/tcp_frame_header.h"
#include "src/core/ext/transport/chaotic_good/tcp_ztrace_collector.h"
#include "src/core/ext/transport/chaotic_good/transport_context.h"
#include "src/core/lib/promise/inter_activity_latch.h"

namespace grpc_core {
namespace chaotic_good {

inline std::vector<PromiseEndpoint> OneDataEndpoint(PromiseEndpoint endpoint) {
  std::vector<PromiseEndpoint> ep;
  ep.emplace_back(std::move(endpoint));
  return ep;
}

class TcpFrameTransport final : public FrameTransport,
                                public channelz::DataSource {
 public:
  struct Options {
    uint32_t encode_alignment = 64;
    uint32_t decode_alignment = 64;
    uint32_t inlined_payload_size_threshold = 8 * 1024;
    bool enable_tracing = false;
  };

  TcpFrameTransport(Options options, PromiseEndpoint control_endpoint,
                    std::vector<PendingConnection> pending_data_endpoints,
                    TransportContextPtr ctx);

  static RefCountedPtr<channelz::SocketNode> MakeSocketNode(
      const ChannelArgs& args, const PromiseEndpoint& endpoint);

  void Start(Party* party, MpscReceiver<OutgoingFrame> outgoing_frames,
             RefCountedPtr<FrameTransportSink> sink) override;
  void Orphan() override;
  TransportContextPtr ctx() override { return ctx_; }
  std::unique_ptr<channelz::ZTrace> GetZTrace(absl::string_view name) override {
    if (name == "transport_frames") {
      return ztrace_collector_->MakeZTrace();
    }
    return DataSource::GetZTrace(name);
  }
  void AddData(channelz::DataSink& sink) override;

 private:
  auto WriteFrame(const FrameInterface& frame,
                  std::shared_ptr<TcpCallTracer> call_tracer);
  auto WriteLoop(MpscReceiver<OutgoingFrame> frames);
  // Read frame header and payloads for control and data portions of one frame.
  // Resolves to StatusOr<IncomingFrame>.
  auto ReadFrameBytes();
  template <typename Promise>
  auto UntilClosed(Promise promise);

  const TransportContextPtr ctx_;
  std::shared_ptr<TcpZTraceCollector> ztrace_collector_ =
      std::make_shared<TcpZTraceCollector>();
  ControlEndpoint control_endpoint_;
  DataEndpoints data_endpoints_;
  const Options options_;
  InterActivityLatch<void> closed_;
  uint64_t next_payload_tag_ = 1;
};

}  // namespace chaotic_good
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHAOTIC_GOOD_TCP_FRAME_TRANSPORT_H
