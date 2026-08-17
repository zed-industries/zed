//
//
// Copyright 2017 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_SURFACE_COMPLETION_QUEUE_FACTORY_H
#define GRPC_SRC_CORE_LIB_SURFACE_COMPLETION_QUEUE_FACTORY_H

#include <grpc/grpc.h>
#include <grpc/support/port_platform.h>

typedef struct grpc_completion_queue_factory_vtable {
  grpc_completion_queue* (*create)(const grpc_completion_queue_factory*,
                                   const grpc_completion_queue_attributes*);
} grpc_completion_queue_factory_vtable;

struct grpc_completion_queue_factory {
  const char* name;
  void* data;  // Factory specific data
  grpc_completion_queue_factory_vtable* vtable;
};

#endif  // GRPC_SRC_CORE_LIB_SURFACE_COMPLETION_QUEUE_FACTORY_H
