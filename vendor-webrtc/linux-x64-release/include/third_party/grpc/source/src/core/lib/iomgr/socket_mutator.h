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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_SOCKET_MUTATOR_H
#define GRPC_SRC_CORE_LIB_IOMGR_SOCKET_MUTATOR_H

#include <grpc/impl/grpc_types.h>
#include <grpc/support/port_platform.h>
#include <grpc/support/sync.h>
#include <stdbool.h>

/// How is an fd to be used?
typedef enum {
  /// Used for client connection
  GRPC_FD_CLIENT_CONNECTION_USAGE,
  /// Used for server listening
  GRPC_FD_SERVER_LISTENER_USAGE,
  /// Used for server connection
  GRPC_FD_SERVER_CONNECTION_USAGE,
} grpc_fd_usage;

/// Information about an fd to mutate
typedef struct {
  /// File descriptor to mutate
  int fd;
  /// How the fd will be used
  grpc_fd_usage usage;
} grpc_mutate_socket_info;

/// The virtual table of grpc_socket_mutator
struct grpc_socket_mutator_vtable {
  /// Mutates the socket options of \a fd -- deprecated, prefer mutate_fd_2
  bool (*mutate_fd)(int fd, grpc_socket_mutator* mutator);
  /// Compare socket mutator \a a and \a b
  int (*compare)(grpc_socket_mutator* a, grpc_socket_mutator* b);
  /// Destroys the socket mutator instance
  void (*destroy)(grpc_socket_mutator* mutator);
  /// Mutates the socket options of the fd in \a info - if set takes preference
  /// to mutate_fd
  bool (*mutate_fd_2)(const grpc_mutate_socket_info* info,
                      grpc_socket_mutator* mutator);
};

/// The Socket Mutator interface allows changes on socket options
struct grpc_socket_mutator {
  const grpc_socket_mutator_vtable* vtable;
  gpr_refcount refcount;
};

/// called by concrete implementations to initialize the base struct
void grpc_socket_mutator_init(grpc_socket_mutator* mutator,
                              const grpc_socket_mutator_vtable* vtable);

/// Wrap \a mutator as a grpc_arg
grpc_arg grpc_socket_mutator_to_arg(grpc_socket_mutator* mutator);

/// Perform the file descriptor mutation operation of \a mutator on \a fd
bool grpc_socket_mutator_mutate_fd(grpc_socket_mutator* mutator, int fd,
                                   grpc_fd_usage usage);

/// Compare if \a a and \a b are the same mutator or have same settings
int grpc_socket_mutator_compare(grpc_socket_mutator* a, grpc_socket_mutator* b);

grpc_socket_mutator* grpc_socket_mutator_ref(grpc_socket_mutator* mutator);
void grpc_socket_mutator_unref(grpc_socket_mutator* mutator);

#endif  // GRPC_SRC_CORE_LIB_IOMGR_SOCKET_MUTATOR_H
