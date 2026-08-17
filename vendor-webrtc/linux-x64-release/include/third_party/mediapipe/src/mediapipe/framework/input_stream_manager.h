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

#ifndef MEDIAPIPE_FRAMEWORK_INPUT_STREAM_MANAGER_H_
#define MEDIAPIPE_FRAMEWORK_INPUT_STREAM_MANAGER_H_

#include <cstdint>
#include <deque>
#include <functional>
#include <list>
#include <string>

#include "absl/base/thread_annotations.h"
#include "absl/status/status.h"
#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/packet.h"
#include "mediapipe/framework/packet_type.h"
#include "mediapipe/framework/timestamp.h"

namespace mediapipe {

// An OutputStreamManager will add packets to InputStreamManager through
// InputStreamHandler as they are output.  A CalculatorNode prepares the input
// packets for a particular invocation by calling InputStreamManager's
// PopPacketAtTimestamp() or PopQueueHead() function through InputStreamHandler.
//
// The InputStreamManager may be closed by the consumer. When the
// InputStreamManager is closed, any further modifications to it by the producer
// are silently ignored.
//
// An input stream is written to by exactly one output stream and is read by a
// single node. None of its methods should hold a lock when they invoke a
// callback in the scheduler.
class InputStreamManager {
 public:
  // Function type for becomes_full_callback and becomes_not_full_callback.
  // The arguments are the input stream manager and its
  // last_reported_stream_full_.  The value of last_reported_stream_full_ is
  // maintained by the callback.
  typedef std::function<void(InputStreamManager*, bool*)> QueueSizeCallback;

  InputStreamManager(const InputStreamManager&) = delete;
  InputStreamManager& operator=(const InputStreamManager&) = delete;

  InputStreamManager() = default;

  // Initializes the InputStreamManager.
  absl::Status Initialize(const std::string& name,
                          const PacketType* packet_type, bool back_edge);

  // Returns the stream name.
  const std::string& Name() const;

  // Returns true if the input stream is a back edge.
  bool BackEdge() const { return back_edge_; }

  // Sets the header Packet.
  absl::Status SetHeader(const Packet& header);

  Packet Header() const {
    absl::MutexLock stream_lock(&stream_mutex_);
    return header_;
  }

  // Reset the input stream for another run of the graph (i.e. another
  // image/video/audio).
  void PrepareForRun() ABSL_LOCKS_EXCLUDED(stream_mutex_);

  // Adds a list of timestamped packets. Sets "notify" to true if the queue
  // becomes non-empty. Does nothing if the input stream is closed.
  //
  // The timestamp of each packet must satisfy Timestamp::IsAllowedInStream().
  // Unless DisableTimestamps() is called, packet timestamps must meet
  // additional requirements:
  // * The timestamp of each packet must be greater than those of the
  //   previously added Packets, and not less than the next timestamp bound.
  // * If a packet has the timestamp Timestamp::PreStream() or
  //   Timestamp::PostStream(), the packet must be the only packet in the
  //   stream.
  // Violation of any of these conditions causes an error status.
  absl::Status AddPackets(const std::list<Packet>& container, bool* notify)
      ABSL_LOCKS_EXCLUDED(stream_mutex_);

  // Move a list of timestamped packets. Sets "notify" to true if the queue
  // becomes non-empty. Does nothing if the input stream is closed. After the
  // move, all packets in the container must be empty.
  absl::Status MovePackets(std::list<Packet>* container, bool* notify)
      ABSL_LOCKS_EXCLUDED(stream_mutex_);

  // Closes the input stream.  This function can be called multiple times.
  void Close() ABSL_LOCKS_EXCLUDED(stream_mutex_);

  // Sets the bound on the next timestamp to be added to the input stream.
  // Sets "notify" to true if the bound is advanced while the packet queue is
  // empty. Returns an error status if this decreases the bound, unless
  // DisableTimestamps() is called. Does nothing if the input stream is
  // closed.
  absl::Status SetNextTimestampBound(Timestamp bound, bool* notify)
      ABSL_LOCKS_EXCLUDED(stream_mutex_);

  // Returns the smallest timestamp at which we might see an input in
  // this input stream. This is the timestamp of the first item in the queue if
  // the queue is non-empty, or the next timestamp bound if it is empty.
  // Sets is_empty to queue_.empty() if it is not nullptr.
  Timestamp MinTimestampOrBound(bool* is_empty) const
      ABSL_LOCKS_EXCLUDED(stream_mutex_);

  // Turns off the use of packet timestamps.
  void DisableTimestamps();

  // Returns true iff the queue is empty.
  bool IsEmpty() const ABSL_LOCKS_EXCLUDED(stream_mutex_);

  // If the queue is not empty, returns the packet at the front of the queue.
  // Otherwise, returns an empty packet.
  Packet QueueHead() const ABSL_LOCKS_EXCLUDED(stream_mutex_);

  // Advances time to timestamp.  Pops and returns the packet in the queue
  // with a matching timestamp, if it exists.  Time can be advanced to any
  // timestamp, however, packets will be lost if they are skipped over.
  // Use MinTimestampOrBound() to determine what the next timestamp that
  // should be processed at should be.  Each call to PopPacketAtTimestamp()
  // must have a timestamp greater than or equal to the last.  Sets
  // "num_packets_dropped" to the total number of packets that were dropped
  // (skipped over). Sets "stream_is_done" if  the next timestamp bound reaches
  // Timestamp::Done() after the pop.
  Packet PopPacketAtTimestamp(Timestamp timestamp, int* num_packets_dropped,
                              bool* stream_is_done)
      ABSL_LOCKS_EXCLUDED(stream_mutex_);

  // Pops and returns the packet at the head of the queue if the queue is
  // non-empty. Sets "stream_is_done" if  the next timestamp bound reaches
  // Timestamp::Done() after the pop.
  Packet PopQueueHead(bool* stream_is_done) ABSL_LOCKS_EXCLUDED(stream_mutex_);

  // Returns the number of packets in the queue.
  int NumPacketsAdded() const ABSL_LOCKS_EXCLUDED(stream_mutex_);

  // Returns the number of packets in the queue.
  int QueueSize() const ABSL_LOCKS_EXCLUDED(stream_mutex_);

  // Returns true iff the queue is full.
  bool IsFull() const ABSL_LOCKS_EXCLUDED(stream_mutex_);

  // Returns the max queue size. -1 indicates that there is no maximum.
  int MaxQueueSize() const ABSL_LOCKS_EXCLUDED(stream_mutex_);

  // Sets the maximum queue size for the stream. Used to determine when the
  // callbacks for becomes_full and becomes_not_full should be invoked. A value
  // of -1 means that there is no maximum queue size.
  void SetMaxQueueSize(int max_queue_size) ABSL_LOCKS_EXCLUDED(stream_mutex_);

  // If there are equal to or more than n packets in the queue, this function
  // returns the min timestamp of among the latest n packets of the queue.  If
  // there are fewer than n packets in the queue, this function returns
  // Timestamp::Unset().
  // NOTE: This is a public API intended for FixedSizeInputStreamHandler only.
  Timestamp GetMinTimestampAmongNLatest(int n) const
      ABSL_LOCKS_EXCLUDED(stream_mutex_);

  // pop_front()s packets that are earlier than the given timestamp.
  // NOTE: This is a public API intended for FixedSizeInputStreamHandler only.
  void ErasePacketsEarlierThan(Timestamp timestamp)
      ABSL_LOCKS_EXCLUDED(stream_mutex_);

  // If a maximum queue size is specified (!= -1), these callbacks that are
  // invoked when the input queue becomes full (>= max_queue_size_) or when it
  // becomes non-full (< max_queue_size_).
  void SetQueueSizeCallbacks(QueueSizeCallback becomes_full_callback,
                             QueueSizeCallback becomes_not_full_callback);

 private:
  // Adds or moves a list of timestamped packets. Sets "notify" to true if the
  // queue becomes non-empty. Returns an error if the packets have errors. Does
  // nothing if the input stream is closed.
  // If the caller is AddPackets(), Container must be const reference.
  // Otherwise, the caller must be MovePackets() and Container should be
  // non-const reference.
  template <typename Container>
  absl::Status AddOrMovePacketsInternal(Container container, bool* notify)
      ABSL_LOCKS_EXCLUDED(stream_mutex_);

  // Returns true if the next timestamp bound reaches Timestamp::Done().
  bool IsDone() const ABSL_EXCLUSIVE_LOCKS_REQUIRED(stream_mutex_);

  // Returns the smallest timestamp at which this stream might see an input.
  Timestamp MinTimestampOrBoundHelper() const;

  mutable absl::Mutex stream_mutex_;
  std::deque<Packet> queue_ ABSL_GUARDED_BY(stream_mutex_);
  // The number of packets added to queue_.  Used to verify a packet at
  // Timestamp::PostStream() is the only Packet in the stream.
  int64_t num_packets_added_ ABSL_GUARDED_BY(stream_mutex_);
  Timestamp next_timestamp_bound_ ABSL_GUARDED_BY(stream_mutex_);
  // The |timestamp| argument passed to the last SelectAtTimestamp() call.
  // Ignored if enable_timestamps_ is false.
  Timestamp last_select_timestamp_ ABSL_GUARDED_BY(stream_mutex_);
  bool closed_ ABSL_GUARDED_BY(stream_mutex_);
  // True if packet timestamps are used.
  bool enable_timestamps_ = true;
  std::string name_;
  const PacketType* packet_type_;
  bool back_edge_;
  // The header packet of the input stream.
  Packet header_ ABSL_GUARDED_BY(stream_mutex_);

  // The maximum queue size for this stream if set.
  int max_queue_size_ ABSL_GUARDED_BY(stream_mutex_) = -1;

  // Callback to notify the framework that we have hit the maximum queue size.
  QueueSizeCallback becomes_full_callback_;

  // Callback to notify the framework that the queue size has becomes less than
  // the maximum specified.
  QueueSizeCallback becomes_not_full_callback_;

  // This variable is used by the QueueSizeCallback to record the queue
  // fullness reported in the last completed QueueSizeCallback.
  // This variable is only accessed during the QueueSizeCallback.
  bool last_reported_stream_full_ = false;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_INPUT_STREAM_MANAGER_H_
