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

#ifndef GRPC_TEST_CORE_IOMGR_ENDPOINT_TESTS_H
#define GRPC_TEST_CORE_IOMGR_ENDPOINT_TESTS_H

#include <sys/types.h>

#include "src/core/lib/iomgr/endpoint.h"

typedef struct grpc_endpoint_test_config grpc_endpoint_test_config;
typedef struct grpc_endpoint_test_fixture grpc_endpoint_test_fixture;

struct grpc_endpoint_test_fixture {
  grpc_endpoint* client_ep;
  grpc_endpoint* server_ep;
};

struct grpc_endpoint_test_config {
  const char* name;
  grpc_endpoint_test_fixture (*create_fixture)(size_t slice_size);
  void (*clean_up)();
};

void grpc_endpoint_tests(grpc_endpoint_test_config config,
                         grpc_pollset* pollset, gpr_mu* mu);

#endif  // GRPC_TEST_CORE_IOMGR_ENDPOINT_TESTS_H
