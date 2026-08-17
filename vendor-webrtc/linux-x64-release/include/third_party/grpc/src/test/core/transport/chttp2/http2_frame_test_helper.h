// Copyright 2025 gRPC authors.
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

#ifndef GRPC_TEST_CORE_TRANSPORT_CHTTP2_HTTP2_FRAME_TEST_HELPER_H
#define GRPC_TEST_CORE_TRANSPORT_CHTTP2_HTTP2_FRAME_TEST_HELPER_H

#include <grpc/slice.h>

#include <cstdint>

#include "absl/strings/string_view.h"
#include "absl/types/span.h"
#include "src/core/ext/transport/chttp2/transport/frame.h"
#include "src/core/ext/transport/chttp2/transport/http2_status.h"
#include "src/core/lib/event_engine/default_event_engine.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/lib/slice/slice_buffer.h"

namespace grpc_core {
namespace transport {
namespace testing {

using EventEngineSlice = grpc_event_engine::experimental::Slice;

class Http2FrameTestHelper {
 public:
  EventEngineSlice EventEngineSliceFromHttp2DataFrame(
      std::string_view payload, const uint32_t stream_id = 1,
      const bool end_stream = false) const {
    return EventEngineSliceFromHttp2Frame(
        Http2DataFrame{stream_id, end_stream, SliceBufferFromString(payload)});
  }

  EventEngineSlice EventEngineSliceFromHttp2HeaderFrame(
      std::string_view payload, const uint32_t stream_id = 1,
      const bool end_headers = true, const bool end_stream = false) const {
    return EventEngineSliceFromHttp2Frame(Http2HeaderFrame{
        stream_id, end_headers, end_stream, SliceBufferFromString(payload)});
  }

  EventEngineSlice EventEngineSliceFromHttp2RstStreamFrame(
      const uint32_t stream_id = 1,
      const uint32_t error_code =
          static_cast<uint32_t>(http2::Http2ErrorCode::kConnectError)) const {
    return EventEngineSliceFromHttp2Frame(
        Http2RstStreamFrame{stream_id, error_code});
  }

  EventEngineSlice EventEngineSliceFromHttp2SettingsFrame(
      const bool ack = false) const {
    return EventEngineSliceFromHttp2Frame(Http2SettingsFrame{ack, {}});
  }

  EventEngineSlice EventEngineSliceFromHttp2PingFrame(
      bool ack = false, uint64_t opaque = 0x123456789abcdef0) const {
    return EventEngineSliceFromHttp2Frame(Http2PingFrame{ack, opaque});
  }

  EventEngineSlice EventEngineSliceFromHttp2GoawayFrame(
      std::string_view debug_data, const uint32_t last_stream_id = 0,
      const uint32_t error_code = 0) const {
    return EventEngineSliceFromHttp2Frame(Http2GoawayFrame{
        last_stream_id, error_code, Slice::FromCopiedString(debug_data)});
  }

  EventEngineSlice EventEngineSliceFromHttp2WindowUpdateFrame(
      const uint32_t stream_id = 1,
      const uint32_t increment = 0x12345678) const {
    return EventEngineSliceFromHttp2Frame(
        Http2WindowUpdateFrame{stream_id, increment});
  }

  EventEngineSlice EventEngineSliceFromHttp2ContinuationFrame(
      std::string_view payload, const uint32_t stream_id = 1,
      const bool end_headers = true) const {
    return EventEngineSliceFromHttp2Frame(Http2ContinuationFrame{
        stream_id, end_headers, SliceBufferFromString(payload)});
  }

  EventEngineSlice EventEngineSliceFromHttp2SecurityFrame(
      std::string_view payload) const {
    return EventEngineSliceFromHttp2Frame(
        Http2SecurityFrame{SliceBufferFromString(payload)});
  }

 private:
  EventEngineSlice EventEngineSliceFromHttp2Frame(Http2Frame frame) const {
    SliceBuffer buffer;
    Serialize(absl::Span<Http2Frame>(&frame, 1), buffer);
    return EventEngineSlice(buffer.JoinIntoSlice().TakeCSlice());
  }

  SliceBuffer SliceBufferFromString(absl::string_view s) const {
    SliceBuffer temp;
    temp.Append(Slice::FromCopiedString(s));
    return temp;
  }
};

}  // namespace testing
}  // namespace transport
}  // namespace grpc_core

#endif  // GRPC_TEST_CORE_TRANSPORT_CHTTP2_HTTP2_FRAME_TEST_HELPER_H
