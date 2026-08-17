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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_TCP_SERVER_UTILS_POSIX_H
#define GRPC_SRC_CORE_LIB_IOMGR_TCP_SERVER_UTILS_POSIX_H

#include <grpc/support/port_platform.h>

#include <memory>

#include "absl/container/flat_hash_map.h"
#include "src/core/lib/event_engine/posix.h"
#include "src/core/lib/iomgr/ev_posix.h"
#include "src/core/lib/iomgr/resolve_address.h"
#include "src/core/lib/iomgr/socket_utils_posix.h"
#include "src/core/lib/iomgr/tcp_server.h"
#include "src/core/lib/iomgr/timer.h"
#include "src/core/lib/resource_quota/memory_quota.h"

// one listening port
typedef struct grpc_tcp_listener {
  int fd;
  grpc_fd* emfd;
  grpc_tcp_server* server;
  grpc_resolved_address addr;
  int port;
  unsigned port_index;
  unsigned fd_index;
  grpc_closure read_closure;
  grpc_closure destroyed_closure;
  struct grpc_tcp_listener* next;
  // sibling is a linked list of all listeners for a given port. add_port and
  // clone_port place all new listeners in the same sibling list. A member of
  // the 'sibling' list is also a member of the 'next' list. The head of each
  // sibling list has is_sibling==0, and subsequent members of sibling lists
  // have is_sibling==1. is_sibling allows separate sibling lists to be
  // identified while iterating through 'next'.
  struct grpc_tcp_listener* sibling;
  int is_sibling;
  // If an accept4() call fails, a timer is started to drain the accept queue in
  // case no further connection attempts reach the gRPC server.
  grpc_closure retry_closure;
  grpc_timer retry_timer;
  gpr_atm retry_timer_armed;
} grpc_tcp_listener;

// the overall server
struct grpc_tcp_server {
  gpr_refcount refs;
  // Called whenever accept() succeeds on a server port.
  grpc_tcp_server_cb on_accept_cb = nullptr;
  void* on_accept_cb_arg = nullptr;

  gpr_mu mu;

  // active port count: how many ports are actually still listening
  size_t active_ports = 0;
  // destroyed port count: how many ports are completely destroyed
  size_t destroyed_ports = 0;

  // is this server shutting down?
  bool shutdown = false;
  // have listeners been shutdown?
  bool shutdown_listeners = false;
  // use SO_REUSEPORT
  bool so_reuseport = false;
  // expand wildcard addresses to a list of all local addresses
  bool expand_wildcard_addrs = false;

  // linked list of server ports
  grpc_tcp_listener* head = nullptr;
  grpc_tcp_listener* tail = nullptr;
  unsigned nports = 0;

  // List of closures passed to shutdown_starting_add().
  grpc_closure_list shutdown_starting{nullptr, nullptr};

  // shutdown callback
  grpc_closure* shutdown_complete = nullptr;

  // all pollsets interested in new connections. The object pointed at is not
  // owned by this struct
  const std::vector<grpc_pollset*>* pollsets = nullptr;

  // next pollset to assign a channel to
  gpr_atm next_pollset_to_assign = 0;

  // Contains config extracted from channel args for this server
  grpc_core::PosixTcpOptions options;

  // a handler for external connections, owned
  grpc_core::TcpServerFdHandler* fd_handler = nullptr;

  // used to create slice allocators for endpoints, owned
  grpc_core::MemoryQuotaRefPtr memory_quota;

  /* used when event engine based servers are enabled */
  int n_bind_ports = 0;
  absl::flat_hash_map<int, std::tuple<int, int>> listen_fd_to_index_map;
  std::unique_ptr<grpc_event_engine::experimental::EventEngine::Listener>
      ee_listener = nullptr;
  /* used to store a pre-allocated FD assigned to a socket */
  int pre_allocated_fd;
};

// If successful, add a listener to \a s for \a addr, set \a dsmode for the
// socket, and return the \a listener.
grpc_error_handle grpc_tcp_server_add_addr(grpc_tcp_server* s,
                                           const grpc_resolved_address* addr,
                                           unsigned port_index,
                                           unsigned fd_index,
                                           grpc_dualstack_mode* dsmode,
                                           grpc_tcp_listener** listener);

// Get all addresses assigned to network interfaces on the machine and create a
// listener for each. requested_port is the port to use for every listener, or 0
// to select one random port that will be used for every listener. Set *out_port
// to the port selected. Return absl::OkStatus() only if all listeners were
// added.
grpc_error_handle grpc_tcp_server_add_all_local_addrs(grpc_tcp_server* s,
                                                      unsigned port_index,
                                                      int requested_port,
                                                      int* out_port);

// Prepare a recently-created socket for listening.
grpc_error_handle grpc_tcp_server_prepare_socket(
    grpc_tcp_server*, int fd, const grpc_resolved_address* addr,
    bool so_reuseport, int* port);
// Ruturn true if the platform supports ifaddrs
bool grpc_tcp_server_have_ifaddrs(void);

// Initialize (but don't start) the timer and callback to retry accept4() on a
// listening socket after file descriptors have been exhausted. This must be
// called when creating a new listener.
void grpc_tcp_server_listener_initialize_retry_timer(
    grpc_tcp_listener* listener);

#endif  // GRPC_SRC_CORE_LIB_IOMGR_TCP_SERVER_UTILS_POSIX_H
