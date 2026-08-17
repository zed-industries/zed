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

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_FRAME_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_FRAME_H

#include <grpc/support/port_platform.h>

#include <cstdint>
#include <string>
#include <variant>
#include <vector>

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "absl/types/span.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/lib/slice/slice_buffer.h"

namespace grpc_core {

///////////////////////////////////////////////////////////////////////////////
// Frame types
//
// Define structs for each kind of frame that chttp2 reasons about.
//
// Each struct gets the members defined by the HTTP/2 spec for that frame type
// *that the semantic layers of chttp2 neead to reason about*.
//
// That means, for instance, that we drop padding and prioritization data from
// these structs, as they are handled by the HTTP/2 framing layer and are
// meaningless to the semantic layers above.
//
// If a frame type is associated with a stream, it has a stream_id member.
// If that frame type is only used at the channel layer it does not.
//
// Instead of carrying bitfields of flags like the wire format, we instead
// declare a bool per flag to make producing/consuming code easier to write.
//
// Equality operators are defined for use in unit tests.

// All frames begin with a fixed 9-octet header followed by a variable-length
// frame payload. The following sizes are in bits.
// HTTP Frame {
//   Length(24),
//   Type(8),
//   Flags(8),
//   Reserved(1),
//   Stream Identifier(31),
//   Frame Payload(..),
// }
// Reference : https://www.rfc-editor.org/rfc/rfc9113.html#name-frame-format
constexpr uint8_t kFrameHeaderSize = 9;

// DATA frame
struct Http2DataFrame {
  uint32_t stream_id = 0;
  bool end_stream = false;
  SliceBuffer payload;

  bool operator==(const Http2DataFrame& other) const {
    return stream_id == other.stream_id && end_stream == other.end_stream &&
           payload.JoinIntoString() == other.payload.JoinIntoString();
  }
};

// HEADER frame
struct Http2HeaderFrame {
  uint32_t stream_id = 0;
  bool end_headers = false;
  bool end_stream = false;
  SliceBuffer payload;

  bool operator==(const Http2HeaderFrame& other) const {
    return stream_id == other.stream_id && end_headers == other.end_headers &&
           end_stream == other.end_stream &&
           payload.JoinIntoString() == other.payload.JoinIntoString();
  }
};

// CONTINUATION frame
struct Http2ContinuationFrame {
  uint32_t stream_id = 0;
  bool end_headers = false;
  SliceBuffer payload;

  bool operator==(const Http2ContinuationFrame& other) const {
    return stream_id == other.stream_id && end_headers == other.end_headers &&
           payload.JoinIntoString() == other.payload.JoinIntoString();
  }
};

// RST_STREAM frame
struct Http2RstStreamFrame {
  uint32_t stream_id = 0;
  uint32_t error_code = 0;

  bool operator==(const Http2RstStreamFrame& other) const {
    return stream_id == other.stream_id && error_code == other.error_code;
  }
};

// SETTINGS frame
struct Http2SettingsFrame {
  struct Setting {
    Setting(uint16_t id, uint32_t value) : id(id), value(value) {}

    uint16_t id;
    uint32_t value;

    bool operator==(const Setting& other) const {
      return id == other.id && value == other.value;
    }
  };
  bool ack = false;
  std::vector<Setting> settings;

  bool operator==(const Http2SettingsFrame& other) const {
    return ack == other.ack && settings == other.settings;
  }
};

// PING frame
struct Http2PingFrame {
  bool ack = false;
  uint64_t opaque = 0;

  bool operator==(const Http2PingFrame& other) const {
    return ack == other.ack && opaque == other.opaque;
  }
};

// GOAWAY frame
struct Http2GoawayFrame {
  uint32_t last_stream_id = 0;
  uint32_t error_code = 0;
  Slice debug_data;

  bool operator==(const Http2GoawayFrame& other) const {
    return last_stream_id == other.last_stream_id &&
           error_code == other.error_code &&
           debug_data.as_string_view() == other.debug_data.as_string_view();
  }
};

// WINDOW_UPDATE frame
struct Http2WindowUpdateFrame {
  uint32_t stream_id = 0;
  uint32_t increment = 0;

  bool operator==(const Http2WindowUpdateFrame& other) const {
    return stream_id == other.stream_id && increment == other.increment;
  }
};

// Security-related frame
struct Http2SecurityFrame {
  SliceBuffer payload;

  bool operator==(const Http2SecurityFrame& other) const {
    return payload.JoinIntoString() == other.payload.JoinIntoString();
  }
};

// Type of frame was unknown (and should be ignored)
struct Http2UnknownFrame {
  bool operator==(const Http2UnknownFrame&) const { return true; }
};

// This is used as a fake frame to trigger events in the HTTP2 transport.
struct Http2EmptyFrame {
  bool operator==(const Http2EmptyFrame&) const { return true; }
};

///////////////////////////////////////////////////////////////////////////////
// Frame variant
//
// A union of all the frame types above, so that we may pass around an
// arbitrary frame between layers as appropriate.
using Http2Frame =
    std::variant<Http2DataFrame, Http2HeaderFrame, Http2ContinuationFrame,
                 Http2RstStreamFrame, Http2SettingsFrame, Http2PingFrame,
                 Http2GoawayFrame, Http2WindowUpdateFrame, Http2SecurityFrame,
                 Http2UnknownFrame, Http2EmptyFrame>;

///////////////////////////////////////////////////////////////////////////////
// Frame header
//
// Define a struct for the frame header.
// Parsing this type is the first step in parsing a frame.
// No validation on the header is done during parsing - the fields should be
// instead interpreted by the frame type parser.
struct Http2FrameHeader {
  uint32_t length;
  uint8_t type;
  uint8_t flags;
  uint32_t stream_id;
  // Serialize header to 9 byte long buffer output
  // Crashes if length > 16777215 (as this is unencodable)
  void Serialize(uint8_t* output) const;
  // Parse header from 9 byte long buffer input
  static Http2FrameHeader Parse(const uint8_t* input);
  std::string ToString() const;

  bool operator==(const Http2FrameHeader& other) const {
    return length == other.length && type == other.type &&
           flags == other.flags && stream_id == other.stream_id;
  }
};

///////////////////////////////////////////////////////////////////////////////
// Parsing & serialization

// Given a frame header and a payload, parse the payload into a frame and
// return it.
// If this function returns an error, that should be considered a connection
// error.
// If a frame should simply be ignored, this function returns a
// Http2UnknownFrame.
// It is expected that hdr.length == payload.Length().
absl::StatusOr<Http2Frame> ParseFramePayload(const Http2FrameHeader& hdr,
                                             SliceBuffer payload);

// Serialize frame and append to out, leaves frames in an unknown state (may
// move things out of frames)
void Serialize(absl::Span<Http2Frame> frames, SliceBuffer& out);

///////////////////////////////////////////////////////////////////////////////
// GRPC Header

constexpr uint8_t kGrpcHeaderSizeInBytes = 5;

struct GrpcMessageHeader {
  uint8_t flags = 0;
  uint32_t length = 0;
};

// If the payload SliceBuffer is too small to hold a gRPC header, this function
// will crash. The calling function MUST ensure that the payload SliceBuffer
// has length greater than or equal to the gRPC header.
GrpcMessageHeader ExtractGrpcHeader(SliceBuffer& payload);

void AppendGrpcHeaderToSliceBuffer(SliceBuffer& payload, const uint8_t flags,
                                   const uint32_t length);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_FRAME_H
