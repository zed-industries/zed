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

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHAOTIC_GOOD_TCP_ZTRACE_COLLECTOR_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHAOTIC_GOOD_TCP_ZTRACE_COLLECTOR_H

#include <cstdint>
#include <map>
#include <string>

#include "src/core/channelz/ztrace_collector.h"
#include "src/core/ext/transport/chaotic_good/tcp_frame_header.h"

namespace grpc_core::chaotic_good {
namespace tcp_ztrace_collector_detail {

class Config {
 public:
  explicit Config(std::map<std::string, std::string>) {}

  template <typename T>
  bool Finishes(const T&) {
    return false;
  }
};

void TcpFrameHeaderToJsonObject(const TcpFrameHeader& header,
                                Json::Object& object);
void TcpDataFrameHeaderToJsonObject(const TcpDataFrameHeader& header,
                                    Json::Object& object);
void MarkRead(bool read, Json::Object& object);

}  // namespace tcp_ztrace_collector_detail

struct ReadFrameHeaderTrace {
  TcpFrameHeader header;

  size_t MemoryUsage() const { return sizeof(*this); }

  void RenderJson(Json::Object& object) const {
    tcp_ztrace_collector_detail::MarkRead(true, object);
    tcp_ztrace_collector_detail::TcpFrameHeaderToJsonObject(header, object);
  }
};

struct ReadDataHeaderTrace {
  TcpDataFrameHeader header;

  size_t MemoryUsage() const { return sizeof(*this); }

  void RenderJson(Json::Object& object) const {
    tcp_ztrace_collector_detail::MarkRead(true, object);
    tcp_ztrace_collector_detail::TcpDataFrameHeaderToJsonObject(header, object);
  }
};

struct WriteFrameHeaderTrace {
  TcpFrameHeader header;

  size_t MemoryUsage() const { return sizeof(*this); }

  void RenderJson(Json::Object& object) const {
    tcp_ztrace_collector_detail::MarkRead(false, object);
    tcp_ztrace_collector_detail::TcpFrameHeaderToJsonObject(header, object);
  }
};

struct EndpointWriteMetricsTrace {
  absl::Time timestamp;
  grpc_event_engine::experimental::EventEngine::Endpoint::WriteEvent
      write_event;
  std::vector<std::pair<absl::string_view, int64_t>> metrics;

  size_t MemoryUsage() const {
    return sizeof(*this) + sizeof(metrics[0]) * metrics.size();
  }

  void RenderJson(Json::Object& object) const {
    std::string name;
    switch (write_event) {
      case grpc_event_engine::experimental::EventEngine::Endpoint::WriteEvent::
          kSendMsg:
        name = "SEND_MSG_METRICS";
        break;
      default:
        name = absl::StrCat("ENDPOINT_WRITE_METRICS_TYPE_",
                            static_cast<int>(write_event));
    }
    object["metadata_type"] = Json::FromString(name);
    object["fathom_timestamp"] = Json::FromString(absl::StrCat(timestamp));
    for (const auto& [name, value] : metrics) {
      object.emplace(name, Json::FromNumber(value));
    }
  }
};

struct WriteLargeFrameHeaderTrace {
  TcpDataFrameHeader data_header;
  std::vector<double> lb_decisions;

  size_t MemoryUsage() const {
    return sizeof(*this) + sizeof(double) * lb_decisions.size();
  }

  void RenderJson(Json::Object& object) const {
    tcp_ztrace_collector_detail::MarkRead(false, object);
    tcp_ztrace_collector_detail::TcpDataFrameHeaderToJsonObject(data_header,
                                                                object);
    Json::Array lb;
    for (double d : lb_decisions) {
      lb.emplace_back(Json::FromNumber(d));
    }
    object["lb_decisions"] = Json::FromArray(std::move(lb));
  }
};

using TcpZTraceCollector =
    channelz::ZTraceCollector<tcp_ztrace_collector_detail::Config,
                              ReadFrameHeaderTrace, ReadDataHeaderTrace,
                              WriteFrameHeaderTrace, WriteLargeFrameHeaderTrace,
                              EndpointWriteMetricsTrace>;

}  // namespace grpc_core::chaotic_good

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHAOTIC_GOOD_TCP_ZTRACE_COLLECTOR_H
