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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_TCP_SERVER_H
#define GRPC_SRC_CORE_LIB_IOMGR_TCP_SERVER_H

#include <grpc/event_engine/endpoint_config.h>
#include <grpc/grpc.h>
#include <grpc/impl/grpc_types.h>
#include <grpc/support/port_platform.h>

#include <vector>

#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/endpoint.h"
#include "src/core/lib/iomgr/resolve_address.h"

// Forward decl of grpc_tcp_server
typedef struct grpc_tcp_server grpc_tcp_server;

typedef struct grpc_tcp_server_acceptor {
  // grpc_tcp_server_cb functions share a ref on from_server that is valid
  // until the function returns.
  grpc_tcp_server* from_server;
  // Indices that may be passed to grpc_tcp_server_port_fd().
  unsigned port_index;
  unsigned fd_index;
  // Data when the connection is passed to tcp_server from external.
  bool external_connection;
  int listener_fd;
  grpc_byte_buffer* pending_data;
} grpc_tcp_server_acceptor;

// Called for newly connected TCP connections.
// Takes ownership of acceptor.
typedef void (*grpc_tcp_server_cb)(void* arg, grpc_endpoint* ep,
                                   grpc_pollset* accepting_pollset,
                                   grpc_tcp_server_acceptor* acceptor);
namespace grpc_core {
// An interface for a handler to take a externally connected fd as a internal
// connection.
class TcpServerFdHandler {
 public:
  virtual ~TcpServerFdHandler() = default;
  virtual void Handle(int listener_fd, int fd,
                      grpc_byte_buffer* pending_read) = 0;
};
}  // namespace grpc_core

typedef struct grpc_tcp_server_vtable {
  grpc_error_handle (*create)(
      grpc_closure* shutdown_complete,
      const grpc_event_engine::experimental::EndpointConfig& config,
      grpc_tcp_server_cb on_accept_cb, void* cb_arg, grpc_tcp_server** server);
  void (*start)(grpc_tcp_server* server,
                const std::vector<grpc_pollset*>* pollsets);
  grpc_error_handle (*add_port)(grpc_tcp_server* s,
                                const grpc_resolved_address* addr,
                                int* out_port);
  grpc_core::TcpServerFdHandler* (*create_fd_handler)(grpc_tcp_server* s);
  unsigned (*port_fd_count)(grpc_tcp_server* s, unsigned port_index);
  int (*port_fd)(grpc_tcp_server* s, unsigned port_index, unsigned fd_index);
  grpc_tcp_server* (*ref)(grpc_tcp_server* s);
  void (*shutdown_starting_add)(grpc_tcp_server* s,
                                grpc_closure* shutdown_starting);
  void (*unref)(grpc_tcp_server* s);
  void (*shutdown_listeners)(grpc_tcp_server* s);
  int (*pre_allocated_fd)(grpc_tcp_server* s);
  void (*set_pre_allocated_fd)(grpc_tcp_server* s, int fd);
} grpc_tcp_server_vtable;

// Create a server, initially not bound to any ports. The caller owns one ref.
// If shutdown_complete is not NULL, it will be used by
// grpc_tcp_server_unref() when the ref count reaches zero.
// Takes ownership of the slice_allocator_factory.
grpc_error_handle grpc_tcp_server_create(
    grpc_closure* shutdown_complete,
    const grpc_event_engine::experimental::EndpointConfig& config,
    grpc_tcp_server_cb on_accept_cb, void* cb_arg, grpc_tcp_server** server);

// Start listening to bound ports
void grpc_tcp_server_start(grpc_tcp_server* server,
                           const std::vector<grpc_pollset*>* pollsets);

// Add a port to the server, returning the newly allocated port on success, or
// -1 on failure.

// The :: and 0.0.0.0 wildcard addresses are treated identically, accepting
// both IPv4 and IPv6 connections, but :: is the preferred style.  This usually
// creates one socket, but possibly two on systems which support IPv6,
// but not dualstack sockets.
// TODO(ctiller): deprecate this, and make grpc_tcp_server_add_ports to handle
//                all of the multiple socket port matching logic in one place
grpc_error_handle grpc_tcp_server_add_port(grpc_tcp_server* s,
                                           const grpc_resolved_address* addr,
                                           int* out_port);

// Create and return a TcpServerFdHandler so that it can be used by upper layer
// to hand over an externally connected fd to the grpc server.
grpc_core::TcpServerFdHandler* grpc_tcp_server_create_fd_handler(
    grpc_tcp_server* s);

// Number of fds at the given port_index, or 0 if port_index is out of
// bounds.
unsigned grpc_tcp_server_port_fd_count(grpc_tcp_server* s, unsigned port_index);

// Returns the file descriptor of the Mth (fd_index) listening socket of the Nth
// (port_index) call to add_port() on this server, or -1 if the indices are out
// of bounds. The file descriptor remains owned by the server, and will be
// cleaned up when the ref count reaches zero.
int grpc_tcp_server_port_fd(grpc_tcp_server* s, unsigned port_index,
                            unsigned fd_index);

// Ref s and return s.
grpc_tcp_server* grpc_tcp_server_ref(grpc_tcp_server* s);

// shutdown_starting is called when ref count has reached zero and the server is
// about to be destroyed. The server will be deleted after it returns. Calling
// grpc_tcp_server_ref() from it has no effect.
void grpc_tcp_server_shutdown_starting_add(grpc_tcp_server* s,
                                           grpc_closure* shutdown_starting);

// If the refcount drops to zero, enqueue calls on exec_ctx to
// shutdown_listeners and delete s.
void grpc_tcp_server_unref(grpc_tcp_server* s);

// Shutdown the fds of listeners.
void grpc_tcp_server_shutdown_listeners(grpc_tcp_server* s);

/* Get pre-allocated FD for server. -1 if none is set */
int grpc_tcp_server_pre_allocated_fd(grpc_tcp_server* s);

/* Set pre-allocated FD for server */
void grpc_tcp_server_set_pre_allocated_fd(grpc_tcp_server* s, int fd);

void grpc_tcp_server_global_init();

void grpc_set_tcp_server_impl(grpc_tcp_server_vtable* impl);

#endif  // GRPC_SRC_CORE_LIB_IOMGR_TCP_SERVER_H
