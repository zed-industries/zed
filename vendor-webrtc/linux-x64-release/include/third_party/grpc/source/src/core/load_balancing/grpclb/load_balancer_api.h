//
//
// Copyright 2016 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LOAD_BALANCING_GRPCLB_LOAD_BALANCER_API_H
#define GRPC_SRC_CORE_LOAD_BALANCING_GRPCLB_LOAD_BALANCER_API_H
#include <grpc/slice.h>
#include <grpc/support/port_platform.h>
#include <stdint.h>

#include <vector>

#include "absl/strings/string_view.h"
#include "src/core/load_balancing/grpclb/grpclb_client_stats.h"
#include "src/core/util/time.h"
#include "upb/mem/arena.h"

#define GRPC_GRPCLB_SERVICE_NAME_MAX_LENGTH 128
#define GRPC_GRPCLB_SERVER_IP_ADDRESS_MAX_SIZE 16
#define GRPC_GRPCLB_SERVER_LOAD_BALANCE_TOKEN_MAX_SIZE 50

namespace grpc_core {

// Contains server information. When the drop field is not true, use the other
// fields.
struct GrpcLbServer {
  int32_t ip_size;
  char ip_addr[GRPC_GRPCLB_SERVER_IP_ADDRESS_MAX_SIZE];
  int32_t port;
  char load_balance_token[GRPC_GRPCLB_SERVER_LOAD_BALANCE_TOKEN_MAX_SIZE];
  bool drop;

  bool operator==(const GrpcLbServer& other) const;
};

struct GrpcLbResponse {
  enum { INITIAL, SERVERLIST, FALLBACK } type;
  Duration client_stats_report_interval;
  std::vector<GrpcLbServer> serverlist;
};

// Creates a serialized grpclb request.
grpc_slice GrpcLbRequestCreate(absl::string_view lb_service_name,
                               upb_Arena* arena);

// Creates a serialized grpclb load report request.
grpc_slice GrpcLbLoadReportRequestCreate(
    int64_t num_calls_started, int64_t num_calls_finished,
    int64_t num_calls_finished_with_client_failed_to_send,
    int64_t num_calls_finished_known_received,
    const GrpcLbClientStats::DroppedCallCounts* drop_token_counts,
    upb_Arena* arena);

// Deserialize a grpclb response.
bool GrpcLbResponseParse(const grpc_slice& serialized_response,
                         upb_Arena* arena, GrpcLbResponse* result);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LOAD_BALANCING_GRPCLB_LOAD_BALANCER_API_H
