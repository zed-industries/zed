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
//
// Defines OutputStream, which allows Calculator implementations to produce
// outputs, and allows clients to access the results produced by
// CalculatorGraph.

#ifndef MEDIAPIPE_FRAMEWORK_OUTPUT_STREAM_H_
#define MEDIAPIPE_FRAMEWORK_OUTPUT_STREAM_H_

#include <string>

#include "absl/base/macros.h"
#include "mediapipe/framework/packet.h"
#include "mediapipe/framework/port.h"
#include "mediapipe/framework/timestamp.h"

namespace mediapipe {

class OutputStream {
 public:
  OutputStream(const OutputStream&) = delete;
  OutputStream& operator=(const OutputStream&) = delete;

  // Returns the name of the stream.
  virtual const std::string& Name() const = 0;

  // Adds a Packet to the output stream. It is CHECKed that the Packet contains
  // the data and a timestamp. The most common way to create such a Packet is
  // "Adopt(ptr).At(timestamp)".
  //
  // The timestamp of the packet must satisfy Timestamp::IsAllowedInStream().
  // If the scheduling policy considers packet timestamps when scheduling
  // calculator nodes, packet timestamps must meet additional requirements:
  // * The timestamp of the packet must be greater than those of the previously
  //   added Packets, and not less than NextTimestampBound().
  // * If an OutputStream contains a Packet with the timestamp
  //   Timestamp::PreStream() or Timestamp::PostStream(), the packet must be
  //   the only packet in the stream.
  // Violation of any of these conditions causes a CHECK-failure.
  //
  // NOTE: AddPacket() cannot report errors via the return value. Instead of a
  // CHECK-failure, a subclass of OutputStream should use a callback function
  // to report errors.
  virtual void AddPacket(const Packet& packet) = 0;
  // Takes an rvalue reference of the packet and moves the packet to the output
  // stream. This AddPacket variant has the exact same logic as the const
  // lvalue reference version except the move behavior.
  virtual void AddPacket(Packet&& packet) = 0;

  // Syntactic sugar for a common way to add Packets.
  template <typename T>
  void Add(T* ptr, Timestamp timestamp) {
    AddPacket(Adopt(ptr).At(timestamp));
  }

  // Indicates that the next output will arrive no earlier than a certain
  // timestamp. Calling this method may save memory by not buffering more
  // packets than needed. It is recommended that the Calculator implementation
  // call this method whenever such a bound is available. The argument should
  // either satisfy Timestamp::IsAllowedInStream() or be a value returned by
  // Timestamp::NextAllowedInStream().
  //
  // Rather than calling SetNextTimestampBound(Timestamp::Max()), use
  // Close() below.
  virtual void SetNextTimestampBound(Timestamp timestamp) = 0;

  // Returns the next timestamp bound.
  virtual Timestamp NextTimestampBound() const = 0;

  // If a Calculator is guaranteed not to add any more Packets to an
  // OutputStream but may continue processing, it SHOULD call Close()
  // on a stream. An example of this is an audio/video decoder applied to
  // a file containing no audio and thus producing no Packets in the audio
  // output stream. If a downstream Calculator takes both the audio and
  // the video streams as inputs, and the decoder doesn't call Close()
  // on the audio OutputStream, then the framework will buffer all the
  // videos frames while waiting for the possibility that an audio frame
  // will still appear with an early timestamp.
  virtual void Close() = 0;

  // true iff the OutputStream is closed.
  virtual bool IsClosed() const = 0;

  // Set the offset.  If set, this means any packet which gets output
  // will have a timestamp of at least input_timestamp + offset.
  // The offset may be negative.  May be called from Calculator::Open(),
  // but not from Process() nor Close().
  virtual void SetOffset(TimestampDiff offset) = 0;

  // Returns true iff SetOffset() has been called.
  virtual bool OffsetEnabled() const = 0;

  // Returns the offset set by the Calculator in the Open() method (returns 0
  // if the offset hasn't been set).
  virtual TimestampDiff Offset() const = 0;

  // Sets the header for this OutputStream. May be called from
  // Calculator::Open(), but not from Process() nor Close(). This header will
  // be accessible by downstream Calculators via InputStream::Header(). The
  // header's timestamp must be unset.
  virtual void SetHeader(const Packet& packet) = 0;

  // Returns the header set by the Calculator in the Open() method, or an
  // empty Packet if none was set.
  virtual const Packet& Header() const = 0;

 protected:
  OutputStream() {}
  virtual ~OutputStream() {}
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_OUTPUT_STREAM_H_
