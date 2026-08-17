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

#ifndef GRPC_TEST_CORE_TRANSPORT_CHAOTIC_GOOD_MOCK_FRAME_TRANSPORT_H
#define GRPC_TEST_CORE_TRANSPORT_CHAOTIC_GOOD_MOCK_FRAME_TRANSPORT_H

#include <google/protobuf/text_format.h>

#include <queue>

#include "src/core/ext/transport/chaotic_good/frame_transport.h"
#include "src/core/lib/promise/inter_activity_latch.h"

namespace grpc_core {
namespace chaotic_good {
namespace testing {

class MockFrameTransport final : public FrameTransport {
 public:
  explicit MockFrameTransport(
      std::shared_ptr<grpc_event_engine::experimental::EventEngine>
          event_engine)
      : ctx_(MakeRefCounted<TransportContext>(
            std::move(event_engine),
            MakeRefCounted<channelz::SocketNode>(
                "local", "remote", "chaotic_good remote", nullptr))) {}
  ~MockFrameTransport() override;

  void Start(Party* party, MpscReceiver<OutgoingFrame> outgoing_frames,
             RefCountedPtr<FrameTransportSink> sink) override;

  void ExpectWrite(Frame frame, SourceLocation whence = {}) {
    expected_writes_.emplace(std::move(frame), whence);
  }
  void Read(Frame frame);
  void Close() {
    if (sink_ != nullptr) {
      sink_->OnFrameTransportClosed(absl::UnavailableError("tschüß!"));
      sink_.reset();
    }
    closed_.Set();
  }
  void Orphan() override {
    Close();
    Unref();
  }
  TransportContextPtr ctx() override { return ctx_; }

 private:
  struct ExpectedWrite {
    ExpectedWrite(Frame frame, SourceLocation whence)
        : frame(std::move(frame)), whence(whence) {}
    Frame frame;
    SourceLocation whence;
  };
  TransportContextPtr ctx_;
  std::queue<ExpectedWrite> expected_writes_;
  RefCountedPtr<FrameTransportSink> sink_;
  InterActivityLatch<void> closed_;
};

template <typename T>
Frame MakeProtoFrame(std::string body) {
  T frame;
  CHECK(google::protobuf::TextFormat::ParseFromString(body, &frame.body));
  return frame;
}

template <typename T>
Frame MakeProtoFrame(uint32_t stream_id, std::string body) {
  T frame;
  CHECK(google::protobuf::TextFormat::ParseFromString(body, &frame.body));
  frame.stream_id = stream_id;
  return frame;
}

inline Frame MakeMessageFrame(uint32_t stream_id, absl::string_view payload) {
  MessageFrame frame;
  frame.message = Arena::MakePooled<Message>(
      SliceBuffer(Slice::FromCopiedString(payload)), 0);
  frame.stream_id = stream_id;
  return frame;
}

}  // namespace testing
}  // namespace chaotic_good
}  // namespace grpc_core

#endif  // GRPC_TEST_CORE_TRANSPORT_CHAOTIC_GOOD_MOCK_FRAME_TRANSPORT_H
