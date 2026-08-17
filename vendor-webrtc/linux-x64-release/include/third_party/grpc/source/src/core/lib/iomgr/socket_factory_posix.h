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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_SOCKET_FACTORY_POSIX_H
#define GRPC_SRC_CORE_LIB_IOMGR_SOCKET_FACTORY_POSIX_H

#include <grpc/impl/grpc_types.h>
#include <grpc/support/port_platform.h>
#include <grpc/support/sync.h>

#include "src/core/lib/iomgr/resolve_address.h"

/// The virtual table of grpc_socket_factory
struct grpc_socket_factory_vtable {
  /// Replacement for socket(2)
  int (*socket)(grpc_socket_factory* factory, int domain, int type,
                int protocol);
  /// Replacement for bind(2)
  int (*bind)(grpc_socket_factory* factory, int sockfd,
              const grpc_resolved_address* addr);
  /// Compare socket factory \a a and \a b
  int (*compare)(grpc_socket_factory* a, grpc_socket_factory* b);
  /// Destroys the socket factory instance
  void (*destroy)(grpc_socket_factory* factory);
};
/// The Socket Factory interface allows changes on socket options
struct grpc_socket_factory {
  const grpc_socket_factory_vtable* vtable;
  gpr_refcount refcount;
};

/// called by concrete implementations to initialize the base struct
void grpc_socket_factory_init(grpc_socket_factory* factory,
                              const grpc_socket_factory_vtable* vtable);

/// Wrap \a factory as a grpc_arg
grpc_arg grpc_socket_factory_to_arg(grpc_socket_factory* factory);

/// Perform the equivalent of a socket(2) operation using \a factory
int grpc_socket_factory_socket(grpc_socket_factory* factory, int domain,
                               int type, int protocol);

/// Perform the equivalent of a bind(2) operation using \a factory
int grpc_socket_factory_bind(grpc_socket_factory* factory, int sockfd,
                             const grpc_resolved_address* addr);

/// Compare if \a a and \a b are the same factory or have same settings
int grpc_socket_factory_compare(grpc_socket_factory* a, grpc_socket_factory* b);

grpc_socket_factory* grpc_socket_factory_ref(grpc_socket_factory* factory);
void grpc_socket_factory_unref(grpc_socket_factory* factory);

#endif  // GRPC_SRC_CORE_LIB_IOMGR_SOCKET_FACTORY_POSIX_H
