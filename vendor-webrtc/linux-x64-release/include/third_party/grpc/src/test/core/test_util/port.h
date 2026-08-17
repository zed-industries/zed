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

#ifndef GRPC_TEST_CORE_TEST_UTIL_PORT_H
#define GRPC_TEST_CORE_TEST_UTIL_PORT_H

typedef struct grpc_pick_port_functions {
  int (*pick_unused_port_or_die_fn)(void);
  void (*recycle_unused_port_fn)(int port);
} grpc_pick_port_functions;

// pick a port number that is currently unused by either tcp or udp. abort
// on failure.
int grpc_pick_unused_port_or_die(void);

// Return a port which was previously returned by grpc_pick_unused_port().
// Implementations of grpc_pick_unused_port() backed by a portserver may limit
// the total number of ports available; this lets a binary return its allocated
// ports back to the server if it is going to allocate a large number.
void grpc_recycle_unused_port(int port);

/// Request the family of pick_port functions in \a functions be used.
/// Returns the current set so they can be restored later.
grpc_pick_port_functions grpc_set_pick_port_functions(
    grpc_pick_port_functions functions);

#endif  // GRPC_TEST_CORE_TEST_UTIL_PORT_H
