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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_EV_POSIX_H
#define GRPC_SRC_CORE_LIB_IOMGR_EV_POSIX_H

#include "src/core/lib/iomgr/port.h"

#ifdef GRPC_POSIX_SOCKET_EV

#include <grpc/support/port_platform.h>
#include <poll.h>

#include "src/core/lib/debug/trace.h"
#include "src/core/lib/iomgr/exec_ctx.h"
#include "src/core/lib/iomgr/pollset.h"
#include "src/core/lib/iomgr/pollset_set.h"
#include "src/core/lib/iomgr/wakeup_fd_posix.h"

typedef struct grpc_fd grpc_fd;

typedef struct grpc_event_engine_vtable {
  size_t pollset_size;
  bool can_track_err;
  bool run_in_background;

  grpc_fd* (*fd_create)(int fd, const char* name, bool track_err);
  int (*fd_wrapped_fd)(grpc_fd* fd);
  void (*fd_orphan)(grpc_fd* fd, grpc_closure* on_done, int* release_fd,
                    const char* reason);
  void (*fd_shutdown)(grpc_fd* fd, grpc_error_handle why);
  void (*fd_notify_on_read)(grpc_fd* fd, grpc_closure* closure);
  void (*fd_notify_on_write)(grpc_fd* fd, grpc_closure* closure);
  void (*fd_notify_on_error)(grpc_fd* fd, grpc_closure* closure);
  void (*fd_set_readable)(grpc_fd* fd);
  void (*fd_set_writable)(grpc_fd* fd);
  void (*fd_set_error)(grpc_fd* fd);
  bool (*fd_is_shutdown)(grpc_fd* fd);

  void (*pollset_init)(grpc_pollset* pollset, gpr_mu** mu);
  void (*pollset_shutdown)(grpc_pollset* pollset, grpc_closure* closure);
  void (*pollset_destroy)(grpc_pollset* pollset);
  grpc_error_handle (*pollset_work)(grpc_pollset* pollset,
                                    grpc_pollset_worker** worker,
                                    grpc_core::Timestamp deadline);
  grpc_error_handle (*pollset_kick)(grpc_pollset* pollset,
                                    grpc_pollset_worker* specific_worker);
  void (*pollset_add_fd)(grpc_pollset* pollset, struct grpc_fd* fd);

  grpc_pollset_set* (*pollset_set_create)(void);
  void (*pollset_set_destroy)(grpc_pollset_set* pollset_set);
  void (*pollset_set_add_pollset)(grpc_pollset_set* pollset_set,
                                  grpc_pollset* pollset);
  void (*pollset_set_del_pollset)(grpc_pollset_set* pollset_set,
                                  grpc_pollset* pollset);
  void (*pollset_set_add_pollset_set)(grpc_pollset_set* bag,
                                      grpc_pollset_set* item);
  void (*pollset_set_del_pollset_set)(grpc_pollset_set* bag,
                                      grpc_pollset_set* item);
  void (*pollset_set_add_fd)(grpc_pollset_set* pollset_set, grpc_fd* fd);
  void (*pollset_set_del_fd)(grpc_pollset_set* pollset_set, grpc_fd* fd);

  bool (*is_any_background_poller_thread)(void);
  const char* name;
  bool (*check_engine_available)(bool explicit_request);
  void (*init_engine)();
  void (*shutdown_background_closure)(void);
  void (*shutdown_engine)(void);
  bool (*add_closure_to_background_poller)(grpc_closure* closure,
                                           grpc_error_handle error);

  void (*fd_set_pre_allocated)(grpc_fd* fd);
} grpc_event_engine_vtable;

// register a new event engine factory
void grpc_register_event_engine_factory(const grpc_event_engine_vtable* vtable,
                                        bool add_at_head);

void grpc_event_engine_init(void);
void grpc_event_engine_shutdown(void);

// Returns true if polling engine can track errors separately, false otherwise.
// If this is true, fd can be created with track_err set. After this, error
// events will be reported using fd_notify_on_error. If it is not set, errors
// will continue to be reported through fd_notify_on_read and
// fd_notify_on_write.
//
bool grpc_event_engine_can_track_errors();

// Returns true if polling engine runs in the background, false otherwise.
// Currently only 'epollbg' runs in the background.
//
bool grpc_event_engine_run_in_background();

// Create a wrapped file descriptor.
// Requires fd is a non-blocking file descriptor.
// \a track_err if true means that error events would be tracked separately
// using grpc_fd_notify_on_error. Currently, valid only for linux systems.
// This takes ownership of closing fd.
grpc_fd* grpc_fd_create(int fd, const char* name, bool track_err);

// Return the wrapped fd, or -1 if it has been released or closed.
int grpc_fd_wrapped_fd(grpc_fd* fd);

// Releases fd to be asynchronously destroyed.
// on_done is called when the underlying file descriptor is definitely close()d.
// If on_done is NULL, no callback will be made.
// If release_fd is not NULL, it's set to fd and fd will not be closed.
// Requires: *fd initialized; no outstanding notify_on_read or
// notify_on_write.
// MUST NOT be called with a pollset lock taken
void grpc_fd_orphan(grpc_fd* fd, grpc_closure* on_done, int* release_fd,
                    const char* reason);

// Has grpc_fd_shutdown been called on an fd?
bool grpc_fd_is_shutdown(grpc_fd* fd);

// Cause any current and future callbacks to fail.
void grpc_fd_shutdown(grpc_fd* fd, grpc_error_handle why);

// Register read interest, causing read_cb to be called once when fd becomes
// readable, on deadline specified by deadline, or on shutdown triggered by
// grpc_fd_shutdown.
// read_cb will be called with read_cb_arg when *fd becomes readable.
// read_cb is Called with status of GRPC_CALLBACK_SUCCESS if readable,
// GRPC_CALLBACK_TIMED_OUT if the call timed out,
// and CANCELLED if the call was cancelled.

// Requires:This method must not be called before the read_cb for any previous
// call runs. Edge triggered events are used whenever they are supported by the
// underlying platform. This means that users must drain fd in read_cb before
// calling notify_on_read again. Users are also expected to handle spurious
// events, i.e read_cb is called while nothing can be readable from fd
void grpc_fd_notify_on_read(grpc_fd* fd, grpc_closure* closure);

// Exactly the same semantics as above, except based on writable events.
void grpc_fd_notify_on_write(grpc_fd* fd, grpc_closure* closure);

// Exactly the same semantics as above, except based on error events. track_err
// needs to have been set on grpc_fd_create
void grpc_fd_notify_on_error(grpc_fd* fd, grpc_closure* closure);

// Forcibly set the fd to be readable, resulting in the closure registered with
// grpc_fd_notify_on_read being invoked.
//
void grpc_fd_set_readable(grpc_fd* fd);

// Forcibly set the fd to be writable, resulting in the closure registered with
// grpc_fd_notify_on_write being invoked.
//
void grpc_fd_set_writable(grpc_fd* fd);

// Forcibly set the fd to have errored, resulting in the closure registered with
// grpc_fd_notify_on_error being invoked.
//
void grpc_fd_set_error(grpc_fd* fd);

// Set the fd to be preallocated
void grpc_fd_set_pre_allocated(grpc_fd* fd);

// pollset_posix functions

// Add an fd to a pollset
void grpc_pollset_add_fd(grpc_pollset* pollset, struct grpc_fd* fd);

// pollset_set_posix functions

void grpc_pollset_set_add_fd(grpc_pollset_set* pollset_set, grpc_fd* fd);
void grpc_pollset_set_del_fd(grpc_pollset_set* pollset_set, grpc_fd* fd);

// Returns true if the caller is a worker thread for any background poller.
bool grpc_is_any_background_poller_thread();

// Returns true if the closure is registered into the background poller. Note
// that the closure may or may not run yet when this function returns, and the
// closure should not be blocking or long-running.
bool grpc_add_closure_to_background_poller(grpc_closure* closure,
                                           grpc_error_handle error);

// Shut down all the closures registered in the background poller.
void grpc_shutdown_background_closure();

// override to allow tests to hook poll() usage
typedef int (*grpc_poll_function_type)(struct pollfd*, nfds_t, int);
extern grpc_poll_function_type grpc_poll_function;

#endif  // GRPC_POSIX_SOCKET_EV

// Return the name of the poll strategy
const char* grpc_get_poll_strategy_name();

#endif  // GRPC_SRC_CORE_LIB_IOMGR_EV_POSIX_H
