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

#ifndef MEDIAPIPE_FRAMEWORK_OUTPUT_STREAM_MANAGER_H_
#define MEDIAPIPE_FRAMEWORK_OUTPUT_STREAM_MANAGER_H_

#include <functional>
#include <string>
#include <vector>

#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/output_stream_shard.h"
#include "mediapipe/framework/packet.h"
#include "mediapipe/framework/packet_type.h"
#include "mediapipe/framework/port.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/framework/timestamp.h"

namespace mediapipe {

class InputStreamHandler;

// Each output stream has an OutputStreamManager object, which manages the input
// stream mirrors, the error callback, and some other metadata of the output
// stream. It's also responsible for computing output timestamp bound and
// propagating updates to the downstream.
class OutputStreamManager {
 public:
  OutputStreamManager() = default;

  // Initializes the OutputStreamManager.
  absl::Status Initialize(const std::string& name,
                          const PacketType* packet_type);

  // Prepares this for processing. If an error occurs in a user called function
  // (such as AddPacket()) then error_callback will be called before returning
  // control to the user.
  void PrepareForRun(std::function<void(absl::Status)> error_callback);

  // Gets the stream name.
  const std::string& Name() const { return output_stream_spec_.name; }

  // Closes the stream and sets the output bound to Timestamp::Done(). Note that
  // OutputStreamHandler should trigger PropagateUpdatesToMirrors() for all
  // those completed tasks that haven't been propagated and then calls this
  // function to close the stream.
  void Close();
  // Returns whether the stream is closed.
  bool IsClosed() const;

  bool OffsetEnabled() const { return output_stream_spec_.offset_enabled; }
  TimestampDiff Offset() const { return output_stream_spec_.offset; }
  // Returns a const reference to the header.
  const Packet& Header() const { return output_stream_spec_.header; }
  // Propagates the header packet to the mirrors.
  void PropagateHeader();

  // Locks the data in the OutputStreamManager that can be set only from
  // Calculator::Open(). This is currently only the offset (set via SetOffset)
  // and the header data (set via SetHeader). The OutputStreamHandler calls
  // this after Calculator::Open().
  void LockIntroData() { output_stream_spec_.locked_intro_data = true; }

  // Adds an InputStreamImpl, which is represented as a pointer to an
  // InputStreamHandler and a CollectionItemId, to mirrors_.
  // The caller retains the ownership of the InputStreamHandler.
  void AddMirror(InputStreamHandler* input_stream_handler, CollectionItemId id);

  // Sets the maximum queue size on all mirrors.
  void SetMaxQueueSize(int max_queue_size);

  // Returns the next timetstamp bound of the output stream.
  Timestamp NextTimestampBound() const;

  // Computes the output timestamp bound based on the input timestamp, the
  // timestamp of the last added packet, and the next timestamp bound from
  // the OutputStreamShard.
  // The function is invoked by OutputStreamHandler after the calculator node
  // finishes a call to Calculator::Process().
  Timestamp ComputeOutputTimestampBound(
      const OutputStreamShard& output_stream_shard,
      Timestamp input_timestamp) const;

  // Propagates the updates to the mirrors and clears the packet queue in
  // the OutputStreamShard afterwards.
  void PropagateUpdatesToMirrors(Timestamp next_timestamp_bound,
                                 OutputStreamShard* output_stream_shard);

  void ResetShard(OutputStreamShard* output_stream_shard);

  OutputStreamSpec* Spec() { return &output_stream_spec_; }
  const OutputStreamSpec* Spec() const { return &output_stream_spec_; }

 private:
  // The necessary information to locate an InputStreamImpl.
  struct Mirror {
    Mirror(InputStreamHandler* input_stream_handler, const CollectionItemId& id)
        : input_stream_handler(input_stream_handler), id(id) {}

    InputStreamHandler* const input_stream_handler;
    const CollectionItemId id;
  };

  // The output stream spec shared across all output stream shards and the
  // output stream manager.
  OutputStreamSpec output_stream_spec_;
  std::vector<Mirror> mirrors_;

  mutable absl::Mutex stream_mutex_;
  Timestamp next_timestamp_bound_ ABSL_GUARDED_BY(stream_mutex_);
  bool closed_ ABSL_GUARDED_BY(stream_mutex_);
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_OUTPUT_STREAM_MANAGER_H_
