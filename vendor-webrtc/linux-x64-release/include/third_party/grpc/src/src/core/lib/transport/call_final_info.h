// Copyright 2024 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_TRANSPORT_CALL_FINAL_INFO_H
#define GRPC_SRC_CORE_LIB_TRANSPORT_CALL_FINAL_INFO_H

#include <grpc/status.h>
#include <grpc/support/port_platform.h>
#include <grpc/support/time.h>

#include <cstdint>

struct grpc_transport_one_way_stats {
  uint64_t framing_bytes = 0;
  uint64_t data_bytes = 0;
  uint64_t header_bytes = 0;
};

struct grpc_transport_stream_stats {
  grpc_transport_one_way_stats incoming;
  grpc_transport_one_way_stats outgoing;
  gpr_timespec latency = gpr_inf_future(GPR_TIMESPAN);
};

void grpc_transport_move_one_way_stats(grpc_transport_one_way_stats* from,
                                       grpc_transport_one_way_stats* to);

void grpc_transport_move_stats(grpc_transport_stream_stats* from,
                               grpc_transport_stream_stats* to);

struct grpc_call_stats {
  grpc_transport_stream_stats transport_stream_stats;
  gpr_timespec latency;  // From call creating to enqueuing of received status
};
/// Information about the call upon completion.
struct grpc_call_final_info {
  grpc_call_stats stats;
  grpc_status_code final_status = GRPC_STATUS_OK;
  const char* error_string = nullptr;
};

#endif  // GRPC_SRC_CORE_LIB_TRANSPORT_CALL_FINAL_INFO_H
