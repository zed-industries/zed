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

//
// wakeup_fd abstracts the concept of a file descriptor for the purpose of
// waking up a thread in select()/poll()/epoll_wait()/etc.

// The poll() family of system calls provide a way for a thread to block until
// there is activity on one (or more) of a set of file descriptors. An
// application may wish to wake up this thread to do non file related work. The
// typical way to do this is to add a pipe to the set of file descriptors, then
// write to the pipe to wake up the thread in poll().
//
// Linux has a lighter weight eventfd specifically designed for this purpose.
// wakeup_fd abstracts the difference between the two.
//
// Setup:
// 1. Before calling anything, call global_init() at least once.
// 1. Call grpc_wakeup_fd_init() to set up a wakeup_fd.
// 2. Add the result of GRPC_WAKEUP_FD_FD to the set of monitored file
//    descriptors for the poll() style API you are using. Monitor the file
//    descriptor for readability.
// 3. To tear down, call grpc_wakeup_fd_destroy(). This closes the underlying
//    file descriptor.
//
// Usage:
// 1. To wake up a polling thread, call grpc_wakeup_fd_wakeup() on a wakeup_fd
//    it is monitoring.
// 2. If the polling thread was awakened by a wakeup_fd event, call
//    grpc_wakeup_fd_consume_wakeup() on it.
//
#ifndef GRPC_SRC_CORE_LIB_IOMGR_WAKEUP_FD_POSIX_H
#define GRPC_SRC_CORE_LIB_IOMGR_WAKEUP_FD_POSIX_H

#include <grpc/support/port_platform.h>

#include "src/core/lib/iomgr/error.h"

void grpc_wakeup_fd_global_init(void);
void grpc_wakeup_fd_global_destroy(void);

// Force using the fallback implementation. This is intended for testing
// purposes only.
void grpc_wakeup_fd_global_init_force_fallback(void);

int grpc_has_wakeup_fd(void);

typedef struct grpc_wakeup_fd grpc_wakeup_fd;

typedef struct grpc_wakeup_fd_vtable {
  grpc_error_handle (*init)(grpc_wakeup_fd* fd_info);
  grpc_error_handle (*consume)(grpc_wakeup_fd* fd_info);
  grpc_error_handle (*wakeup)(grpc_wakeup_fd* fd_info);
  void (*destroy)(grpc_wakeup_fd* fd_info);
  // Must be called before calling any other functions
  int (*check_availability)(void);
} grpc_wakeup_fd_vtable;

struct grpc_wakeup_fd {
  int read_fd;
  int write_fd;
};

extern int grpc_allow_specialized_wakeup_fd;
extern int grpc_allow_pipe_wakeup_fd;

#define GRPC_WAKEUP_FD_GET_READ_FD(fd_info) ((fd_info)->read_fd)

grpc_error_handle grpc_wakeup_fd_init(grpc_wakeup_fd* fd_info);
grpc_error_handle grpc_wakeup_fd_consume_wakeup(grpc_wakeup_fd* fd_info);
grpc_error_handle grpc_wakeup_fd_wakeup(grpc_wakeup_fd* fd_info);
void grpc_wakeup_fd_destroy(grpc_wakeup_fd* fd_info);

// Defined in some specialized implementation's .c file, or by
// wakeup_fd_nospecial.c if no such implementation exists.
extern const grpc_wakeup_fd_vtable grpc_specialized_wakeup_fd_vtable;

#endif  // GRPC_SRC_CORE_LIB_IOMGR_WAKEUP_FD_POSIX_H
