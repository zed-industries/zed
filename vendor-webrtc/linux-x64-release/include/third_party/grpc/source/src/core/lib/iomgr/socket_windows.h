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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_SOCKET_WINDOWS_H
#define GRPC_SRC_CORE_LIB_IOMGR_SOCKET_WINDOWS_H

#include <grpc/support/port_platform.h>

#include "src/core/lib/iomgr/port.h"

#ifdef GRPC_WINSOCK_SOCKET
#include <grpc/support/atm.h>
#include <grpc/support/sync.h>
#include <winsock2.h>

#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/iomgr_internal.h"

#ifndef WSA_FLAG_NO_HANDLE_INHERIT
#define WSA_FLAG_NO_HANDLE_INHERIT 0x80
#endif

// This holds the data for an outstanding read or write on a socket.
// The mutex to protect the concurrent access to that data is the one
// inside the winsocket wrapper.
typedef struct grpc_winsocket_callback_info {
  // This is supposed to be a WSAOVERLAPPED, but in order to get that
  // definition, we need to include ws2tcpip.h, which needs to be included
  // from the top, otherwise it'll clash with a previous inclusion of
  // windows.h that in turns includes winsock.h. If anyone knows a way
  // to do it properly, feel free to send a patch.
  OVERLAPPED overlapped;
  // The callback information for the pending operation. May be empty if the
  // caller hasn't registered a callback yet.
  grpc_closure* closure;
  // A boolean to describe if the IO Completion Port got a notification for
  // that operation. This will happen if the operation completed before the
  // called had time to register a callback. We could avoid that behavior
  // altogether by forcing the caller to always register its callback before
  // proceeding queue an operation, but it is frequent for an IO Completion
  // Port to trigger quickly. This way we avoid a context switch for calling
  // the callback. We also simplify the read / write operations to avoid having
  // to hold a mutex for a long amount of time.
  int has_pending_iocp;
  // The results of the overlapped operation.
  DWORD bytes_transferred;
  int wsa_error;
  // Tracks whether the final closure has already been run when the socket is
  // shut down. This allows closures to be run immediately upon socket shutdown.
  bool closure_already_executed_at_shutdown = false;
} grpc_winsocket_callback_info;

// This is a wrapper to a Windows socket. A socket can have one outstanding
// read, and one outstanding write. Doing an asynchronous accept means waiting
// for a read operation. Doing an asynchronous connect means waiting for a
// write operation. These are completely arbitrary ties between the operation
// and the kind of event, because we can have one overlapped per pending
// operation, whichever its nature is. So we could have more dedicated pending
// operation callbacks for connect and listen. But given the scope of listen
// and accept, we don't need to go to that extent and waste memory. Also, this
// is closer to what happens in posix world.
typedef struct grpc_winsocket {
  SOCKET socket;
  bool destroy_called;

  grpc_winsocket_callback_info write_info;
  grpc_winsocket_callback_info read_info;

  gpr_mu state_mu;
  bool shutdown_called;
  bool shutdown_registered;

  // You can't add the same socket twice to the same IO Completion Port.
  // This prevents that.
  int added_to_iocp;

  // A label for iomgr to track outstanding objects
  grpc_iomgr_object iomgr_object;
} grpc_winsocket;

// Create a wrapped windows handle. This takes ownership of it, meaning that
// it will be responsible for closing it.
grpc_winsocket* grpc_winsocket_create(SOCKET socket, const char* name);

SOCKET grpc_winsocket_wrapped_socket(grpc_winsocket* socket);

// Initiate an asynchronous shutdown of the socket. Will call off any pending
// operation to cancel them.
void grpc_winsocket_shutdown(grpc_winsocket* socket);

// Destroy a socket. Should only be called if there's no pending operation.
void grpc_winsocket_destroy(grpc_winsocket* socket);

void grpc_socket_notify_on_write(grpc_winsocket* winsocket,
                                 grpc_closure* closure);

void grpc_socket_notify_on_read(grpc_winsocket* winsocket,
                                grpc_closure* closure);

bool grpc_socket_become_ready(grpc_winsocket* socket,
                              grpc_winsocket_callback_info* info);

// Returns true if this system can create AF_INET6 sockets bound to ::1.
// The value is probed once, and cached for the life of the process.
int grpc_ipv6_loopback_available(void);

void grpc_wsa_socket_flags_init();

DWORD grpc_get_default_wsa_socket_flags();

// Final cleanup operations on the socket prior to deletion.
void grpc_winsocket_finish(grpc_winsocket*);

#endif

#endif  // GRPC_SRC_CORE_LIB_IOMGR_SOCKET_WINDOWS_H
