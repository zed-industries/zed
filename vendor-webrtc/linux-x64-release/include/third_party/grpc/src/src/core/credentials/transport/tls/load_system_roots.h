//
//
// Copyright 2018 gRPC authors.
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

#ifndef GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_LOAD_SYSTEM_ROOTS_H
#define GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_LOAD_SYSTEM_ROOTS_H

#include <grpc/slice.h>
#include <grpc/support/port_platform.h>

namespace grpc_core {

// TODO(matthewstevenson88): Update LoadSystemRootCerts to use Slice
// instead of grpc_slice.

// Returns a slice containing roots from the OS trust store
grpc_slice LoadSystemRootCerts();

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_LOAD_SYSTEM_ROOTS_H
