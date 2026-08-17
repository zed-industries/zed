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

#ifndef GRPC_TEST_CORE_TEST_UTIL_TEST_TCP_SERVER_H
#define GRPC_TEST_CORE_TEST_UTIL_TEST_TCP_SERVER_H

#include <grpc/support/sync.h>

#include <vector>

#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/iomgr_fwd.h"
#include "src/core/lib/iomgr/tcp_server.h"

// test_tcp_server should be stack-allocated or new'ed, never gpr_malloc'ed
// since it contains C++ objects.
struct test_tcp_server {
  grpc_tcp_server* tcp_server = nullptr;
  grpc_closure shutdown_complete;
  bool shutdown = false;
  // mu is filled in by grpc_pollset_init and controls the pollset.
  // TODO(unknown): Switch this to a Mutex once pollset_init can provide a Mutex
  gpr_mu* mu;
  std::vector<grpc_pollset*> pollset;
  grpc_tcp_server_cb on_connect;
  void* cb_data;
};

void test_tcp_server_init(test_tcp_server* server,
                          grpc_tcp_server_cb on_connect, void* user_data);
void test_tcp_server_start(test_tcp_server* server, int port);
void test_tcp_server_poll(test_tcp_server* server, int milliseconds);
void test_tcp_server_destroy(test_tcp_server* server);

#endif  // GRPC_TEST_CORE_TEST_UTIL_TEST_TCP_SERVER_H
