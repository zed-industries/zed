// Copyright 2025 The gRPC Authors
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

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_HTTP2_ZTRACE_COLLECTOR_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_HTTP2_ZTRACE_COLLECTOR_H

#include <cstdint>
#include <map>
#include <string>

#include "src/core/channelz/ztrace_collector.h"
#include "src/core/ext/transport/chttp2/transport/frame.h"

namespace grpc_core {
namespace http2_ztrace_collector_detail {

class Config {
 public:
  explicit Config(std::map<std::string, std::string>) {}

  template <typename T>
  bool Finishes(const T&) {
    return false;
  }
};

}  // namespace http2_ztrace_collector_detail

template <bool kRead>
struct H2DataTrace {
  uint32_t stream_id;
  bool end_stream;
  uint32_t payload_length;

  size_t MemoryUsage() const { return sizeof(*this); }

  void RenderJson(Json::Object& json) const {
    json["read"] = Json::FromBool(kRead);
    json["frame_type"] = Json::FromString("DATA");
    json["stream_id"] = Json::FromNumber(stream_id);
    json["end_stream"] = Json::FromBool(end_stream);
    json["payload_length"] = Json::FromNumber(payload_length);
  }
};

template <bool kRead>
struct H2HeaderTrace {
  uint32_t stream_id;
  bool end_headers;
  bool end_stream;
  bool continuation;
  uint32_t payload_length;

  size_t MemoryUsage() const { return sizeof(*this); }

  void RenderJson(Json::Object& json) const {
    json["read"] = Json::FromBool(kRead);
    json["frame_type"] = continuation ? Json::FromString("CONTINUATION")
                                      : Json::FromString("HEADERS");
    json["stream_id"] = Json::FromNumber(stream_id);
    json["end_headers"] = Json::FromBool(end_headers);
    json["end_stream"] = Json::FromBool(end_stream);
    json["payload_length"] = Json::FromNumber(payload_length);
  }
};

template <bool kRead>
struct H2RstStreamTrace {
  uint32_t stream_id;
  uint32_t error_code;

  size_t MemoryUsage() const { return sizeof(*this); }

  void RenderJson(Json::Object& json) const {
    json["read"] = Json::FromBool(kRead);
    json["frame_type"] = Json::FromString("RST_STREAM");
    json["stream_id"] = Json::FromNumber(stream_id);
    json["error_code"] = Json::FromNumber(error_code);
  }
};

template <bool kRead>
struct H2SettingsTrace {
  bool ack;
  std::vector<Http2SettingsFrame::Setting> settings;

  size_t MemoryUsage() const {
    return sizeof(*this) +
           sizeof(Http2SettingsFrame::Setting) * settings.size();
  }

  void RenderJson(Json::Object& json) const {
    json["read"] = Json::FromBool(kRead);
    json["frame_type"] = Json::FromString("SETTINGS");
    json["ack"] = Json::FromBool(ack);
    Json::Array settings_array;
    for (const auto& setting : settings) {
      Json::Object setting_object;
      setting_object["id"] = Json::FromNumber(setting.id);
      setting_object["value"] = Json::FromNumber(setting.value);
      settings_array.push_back(Json::FromObject(std::move(setting_object)));
    }
    json["settings"] = Json::FromArray(std::move(settings_array));
  }
};

template <bool kRead>
struct H2PingTrace {
  bool ack;
  uint64_t opaque;

  size_t MemoryUsage() const { return sizeof(*this); }

  void RenderJson(Json::Object& json) const {
    json["read"] = Json::FromBool(kRead);
    json["frame_type"] = Json::FromString("PING");
    json["ack"] = Json::FromBool(ack);
    json["opaque"] = Json::FromNumber(opaque);
  }
};

template <bool kRead>
struct H2GoAwayTrace {
  uint32_t last_stream_id;
  uint32_t error_code;
  std::string debug_data;

  size_t MemoryUsage() const { return sizeof(*this) + debug_data.size(); }

  void RenderJson(Json::Object& json) const {
    json["read"] = Json::FromBool(kRead);
    json["frame_type"] = Json::FromString("GOAWAY");
    json["last_stream_id"] = Json::FromNumber(last_stream_id);
    json["error_code"] = Json::FromNumber(error_code);
    json["debug_data"] = Json::FromString(debug_data);
  }
};

template <bool kRead>
struct H2WindowUpdateTrace {
  uint32_t stream_id;
  uint32_t window_size_increment;

  size_t MemoryUsage() const { return sizeof(*this); }

  void RenderJson(Json::Object& json) const {
    json["read"] = Json::FromBool(kRead);
    json["frame_type"] = Json::FromString("WINDOW_UPDATE");
    json["stream_id"] = Json::FromNumber(stream_id);
    json["window_size_increment"] = Json::FromNumber(window_size_increment);
  }
};

template <bool kRead>
struct H2SecurityTrace {
  uint32_t payload_length;

  size_t MemoryUsage() const { return sizeof(*this); }

  void RenderJson(Json::Object& json) const {
    json["read"] = Json::FromBool(kRead);
    json["frame_type"] = Json::FromString("SECURITY");
    json["payload_length"] = Json::FromNumber(payload_length);
  }
};

struct H2UnknownFrameTrace {
  uint8_t type;
  uint8_t flags;
  uint32_t stream_id;
  uint32_t payload_length;

  size_t MemoryUsage() const { return sizeof(*this); }

  void RenderJson(Json::Object& json) const {
    json["frame_type"] = Json::FromString("UNKNOWN");
    json["type"] = Json::FromNumber(type);
    json["flags"] = Json::FromNumber(flags);
    json["stream_id"] = Json::FromNumber(stream_id);
    json["payload_length"] = Json::FromNumber(payload_length);
  }
};

struct H2FlowControlStall {
  int64_t transport_window;
  int64_t stream_window;
  uint32_t stream_id;

  size_t MemoryUsage() const { return sizeof(*this); }

  void RenderJson(Json::Object& json) const {
    json["metadata_type"] = Json::FromString("FLOW_CONTROL_STALL");
    json["transport_window"] = Json::FromNumber(transport_window);
    json["stream_window"] = Json::FromNumber(stream_window);
    json["stream_id"] = Json::FromNumber(stream_id);
  }
};

struct H2BeginWriteCycle {
  uint32_t target_size;

  size_t MemoryUsage() const { return sizeof(*this); }

  void RenderJson(Json::Object& json) const {
    json["metadata_type"] = Json::FromString("BEGIN_WRITE_CYCLE");
    json["target_size"] = Json::FromNumber(target_size);
  }
};

struct H2BeginEndpointWrite {
  uint32_t write_size;

  size_t MemoryUsage() const { return sizeof(*this); }

  void RenderJson(Json::Object& json) const {
    json["metadata_type"] = Json::FromString("BEGIN_ENDPOINT_WRITE");
    json["write_size"] = Json::FromNumber(write_size);
  }
};

struct H2EndWriteCycle {
  size_t MemoryUsage() const { return sizeof(*this); }

  void RenderJson(Json::Object& json) const {
    json["metadata_type"] = Json::FromString("END_WRITE_CYCLE");
  }
};

using Http2ZTraceCollector = channelz::ZTraceCollector<
    http2_ztrace_collector_detail::Config, H2DataTrace<false>,
    H2HeaderTrace<false>, H2RstStreamTrace<false>, H2SettingsTrace<false>,
    H2PingTrace<false>, H2GoAwayTrace<false>, H2WindowUpdateTrace<false>,
    H2SecurityTrace<false>, H2DataTrace<true>, H2HeaderTrace<true>,
    H2RstStreamTrace<true>, H2SettingsTrace<true>, H2PingTrace<true>,
    H2GoAwayTrace<true>, H2WindowUpdateTrace<true>, H2SecurityTrace<true>,
    H2UnknownFrameTrace, H2FlowControlStall, H2BeginWriteCycle, H2EndWriteCycle,
    H2BeginEndpointWrite>;

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_HTTP2_ZTRACE_COLLECTOR_H
