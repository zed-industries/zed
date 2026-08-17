// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_FRAMEWORK_INPUT_STREAM_SHARD_H_
#define MEDIAPIPE_FRAMEWORK_INPUT_STREAM_SHARD_H_

#include <queue>
#include <string>

#include "mediapipe/framework/input_stream.h"
#include "mediapipe/framework/packet.h"

namespace mediapipe {

// For testing
class MediaPipeProfilerTestPeer;

// InputStreamShard, a subclass of InputStream, holds a header packet, a FIFO
// queue of input packets, and a bool variable to indicate if the stream is
// completely done. Each call to Calculator::Open(), Calculator::Process(), and
// Calculator::Close() can only access its own InputStreamShard(s).
//
// The input stream handler makes sure exactly one packet is added to each shard
// per Calculator::Process call. This is done by pushing empty packets when
// necessary to guarantee alignment with the corresponding timestamps. Every
// call to ClearCurrentPacket() must remove a packet from the queue and every
// call to Value() must successfully return the front element of the queue.
class InputStreamShard : public InputStream {
 public:
  InputStreamShard() : is_done_(false) {}

  // Returns the first packet in the queue if there is any, otherwise returns an
  // empty packet.
  const Packet& Value() const override {
    return !packet_queue_.empty() ? packet_queue_.front() : empty_packet_;
  }

  Packet& Value() override {
    return !packet_queue_.empty() ? packet_queue_.front() : empty_packet_;
  }

  // Returns a reference to the name string of the InputStreamManager.
  const std::string& Name() const { return *name_; }

  bool IsDone() const override { return is_done_; }

 private:
  void SetName(const std::string* name) { name_ = name; }

  int NumberOfPackets() const { return static_cast<int>(packet_queue_.size()); }

  void ClearCurrentPacket() {
    if (!packet_queue_.empty()) {
      packet_queue_.pop();
    }
  }

  void SetHeader(const Packet& header) { header_ = header; }

  void AddPacket(Packet&& value, bool is_done);

  // Packet storage for batch processing.
  std::queue<Packet> packet_queue_;
  Packet empty_packet_;

  // Pointer to the name string of the InputStreamManager.
  const std::string* name_;
  bool is_done_;

  // Accesses InputStreamShard for setting data.
  friend class InputStreamHandler;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_INPUT_STREAM_SHARD_H_
