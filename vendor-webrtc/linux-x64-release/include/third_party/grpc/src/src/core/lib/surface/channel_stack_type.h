//
//
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
//
//

#ifndef GRPC_SRC_CORE_LIB_SURFACE_CHANNEL_STACK_TYPE_H
#define GRPC_SRC_CORE_LIB_SURFACE_CHANNEL_STACK_TYPE_H

#include <grpc/support/port_platform.h>

typedef enum {
  // normal top-half client channel with load-balancing, connection management
  GRPC_CLIENT_CHANNEL,
  // bottom-half of a client channel: everything that happens post-load
  // balancing (bound to a specific transport)
  GRPC_CLIENT_SUBCHANNEL,
  // dynamic part of a client channel
  GRPC_CLIENT_DYNAMIC,
  // a permanently broken client channel
  GRPC_CLIENT_LAME_CHANNEL,
  // a directly connected client channel (without load-balancing, directly talks
  // to a transport)
  GRPC_CLIENT_DIRECT_CHANNEL,
  // server side channel
  GRPC_SERVER_CHANNEL,
  // must be last
  GRPC_NUM_CHANNEL_STACK_TYPES
} grpc_channel_stack_type;

bool grpc_channel_stack_type_is_client(grpc_channel_stack_type type);

const char* grpc_channel_stack_type_string(grpc_channel_stack_type type);

#endif  // GRPC_SRC_CORE_LIB_SURFACE_CHANNEL_STACK_TYPE_H
