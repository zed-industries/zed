//
//
// Copyright 2022 gRPC authors.
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
//
//

#ifndef GRPC_SRC_CORE_EXT_FILTERS_LOGGING_LOGGING_SINK_H
#define GRPC_SRC_CORE_EXT_FILTERS_LOGGING_LOGGING_SINK_H

#include <grpc/support/port_platform.h>
#include <stdint.h>

#include <map>
#include <string>

#include "absl/numeric/int128.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "src/core/util/time.h"

namespace grpc_core {

// Interface for a logging sink that will be used by the logging filter.
class LoggingSink {
 public:
  class Config {
   public:
    // Constructs a default config which has logging disabled
    Config() {}
    Config(uint32_t max_metadata_bytes, uint32_t max_message_bytes)
        : enabled_(true),
          max_metadata_bytes_(max_metadata_bytes),
          max_message_bytes_(max_message_bytes) {}
    bool ShouldLog() { return enabled_; }

    uint32_t max_metadata_bytes() const { return max_metadata_bytes_; }

    uint32_t max_message_bytes() const { return max_message_bytes_; }

    bool operator==(const Config& other) const {
      return max_metadata_bytes_ == other.max_metadata_bytes_ &&
             max_message_bytes_ == other.max_message_bytes_;
    }

   private:
    bool enabled_ = false;
    uint32_t max_metadata_bytes_ = 0;
    uint32_t max_message_bytes_ = 0;
  };

  struct Entry {
    enum class EventType {
      kUnknown = 0,
      kClientHeader,
      kServerHeader,
      kClientMessage,
      kServerMessage,
      kClientHalfClose,
      kServerTrailer,
      kCancel
    };

    static std::string EventTypeString(EventType type) {
      switch (type) {
        case EventType::kUnknown:
          return "UNKNOWN";
        case EventType::kClientHeader:
          return "CLIENT_HEADER";
        case EventType::kServerHeader:
          return "SERVER_HEADER";
        case EventType::kClientMessage:
          return "CLIENT_MESSAGE";
        case EventType::kServerMessage:
          return "SERVER_MESSAGE";
        case EventType::kClientHalfClose:
          return "CLIENT_HALF_CLOSE";
        case EventType::kServerTrailer:
          return "SERVER_TRAILER";
        case EventType::kCancel:
          return "CANCEL";
      }
      return absl::StrCat("INVALID(", static_cast<int>(type), ")");
    }

    enum class Logger { kUnknown = 0, kClient, kServer };

    static std::string LoggerString(Logger logger) {
      switch (logger) {
        case Logger::kUnknown:
          return "UNKNOWN";
        case Logger::kClient:
          return "CLIENT";
        case Logger::kServer:
          return "SERVER";
      }
      return absl::StrCat("INVALID(", static_cast<int>(logger), ")");
    }

    struct Payload {
      std::map<std::string, std::string> metadata;
      Duration timeout;
      uint32_t status_code = 0;
      std::string status_message;
      std::string status_details;
      uint32_t message_length = 0;
      std::string message;
    };

    struct Address {
      enum class Type { kUnknown = 0, kIpv4, kIpv6, kUnix };
      Type type = LoggingSink::Entry::Address::Type::kUnknown;
      std::string address;
      uint32_t ip_port = 0;
    };

    absl::uint128 call_id = 0;
    uint64_t sequence_id = 0;
    EventType type = LoggingSink::Entry::EventType::kUnknown;
    Logger logger = LoggingSink::Entry::Logger::kUnknown;
    Payload payload;
    bool payload_truncated = false;
    Address peer;
    std::string authority;
    std::string service_name;
    std::string method_name;
    Timestamp timestamp;
    // Optional tracing details
    std::string trace_id;
    std::string span_id;
    bool is_sampled = false;
    bool is_trailer_only = false;
  };

  virtual ~LoggingSink() = default;

  virtual Config FindMatch(bool is_client, absl::string_view service,
                           absl::string_view method) = 0;

  virtual void LogEntry(Entry entry) = 0;
};

inline std::ostream& operator<<(std::ostream& out,
                                const LoggingSink::Entry::EventType& type) {
  return out << LoggingSink::Entry::EventTypeString(type);
}

inline std::ostream& operator<<(std::ostream& out,
                                const LoggingSink::Entry::Logger& logger) {
  return out << LoggingSink::Entry::LoggerString(logger);
}

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_FILTERS_LOGGING_LOGGING_SINK_H
