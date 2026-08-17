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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_ENDPOINT_H
#define GRPC_SRC_CORE_LIB_IOMGR_ENDPOINT_H

#include <grpc/slice.h>
#include <grpc/slice_buffer.h>
#include <grpc/support/port_platform.h>
#include <grpc/support/time.h>

#include "absl/strings/string_view.h"
#include "src/core/lib/iomgr/pollset.h"
#include "src/core/lib/iomgr/pollset_set.h"

// An endpoint caps a streaming channel between two communicating processes.
// Examples may be: a tcp socket, <stdin+stdout>, or some shared memory.

typedef struct grpc_endpoint grpc_endpoint;
typedef struct grpc_endpoint_vtable grpc_endpoint_vtable;

struct grpc_endpoint_vtable {
  void (*read)(grpc_endpoint* ep, grpc_slice_buffer* slices, grpc_closure* cb,
               bool urgent, int min_progress_size);
  void (*write)(grpc_endpoint* ep, grpc_slice_buffer* slices, grpc_closure* cb,
                void* arg, int max_frame_size);
  void (*add_to_pollset)(grpc_endpoint* ep, grpc_pollset* pollset);
  void (*add_to_pollset_set)(grpc_endpoint* ep, grpc_pollset_set* pollset);
  void (*delete_from_pollset_set)(grpc_endpoint* ep, grpc_pollset_set* pollset);
  void (*destroy)(grpc_endpoint* ep);
  absl::string_view (*get_peer)(grpc_endpoint* ep);
  absl::string_view (*get_local_address)(grpc_endpoint* ep);
  int (*get_fd)(grpc_endpoint* ep);
  bool (*can_track_err)(grpc_endpoint* ep);
};

// When data is available on the connection, calls the callback with slices.
// Callback success indicates that the endpoint can accept more reads, failure
// indicates the endpoint is closed.
// Valid slices may be placed into \a slices even when the callback is
// invoked with !error.ok().
void grpc_endpoint_read(grpc_endpoint* ep, grpc_slice_buffer* slices,
                        grpc_closure* cb, bool urgent, int min_progress_size);

absl::string_view grpc_endpoint_get_peer(grpc_endpoint* ep);

absl::string_view grpc_endpoint_get_local_address(grpc_endpoint* ep);

// Get the file descriptor used by \a ep. Return -1 if \a ep is not using an fd.
//
int grpc_endpoint_get_fd(grpc_endpoint* ep);

// Write slices out to the socket.

// If the connection is ready for more data after the end of the call, it
// returns GRPC_ENDPOINT_DONE.
// Otherwise it returns GRPC_ENDPOINT_PENDING and calls cb when the
// connection is ready for more data.
// \a slices may be mutated at will by the endpoint until cb is called.
// No guarantee is made to the content of slices after a write EXCEPT that
// it is a valid slice buffer.
// \a arg is platform specific. It is currently only used by TCP on linux
// platforms as an argument that would be forwarded to the timestamps callback.
// \a max_frame_size. A hint to the endpoint implementation to construct
// frames which do not exceed the specified size.
//
void grpc_endpoint_write(grpc_endpoint* ep, grpc_slice_buffer* slices,
                         grpc_closure* cb, void* arg, int max_frame_size);

// Causes any pending and future read/write callbacks to run immediately with
// success==0
void grpc_endpoint_destroy(grpc_endpoint* ep);

// Add an endpoint to a pollset or pollset_set, so that when the pollset is
// polled, events from this endpoint are considered
void grpc_endpoint_add_to_pollset(grpc_endpoint* ep, grpc_pollset* pollset);
void grpc_endpoint_add_to_pollset_set(grpc_endpoint* ep,
                                      grpc_pollset_set* pollset_set);

// Delete an endpoint from a pollset_set
void grpc_endpoint_delete_from_pollset_set(grpc_endpoint* ep,
                                           grpc_pollset_set* pollset_set);

bool grpc_endpoint_can_track_err(grpc_endpoint* ep);

struct grpc_endpoint {
  const grpc_endpoint_vtable* vtable;

  void Orphan() { grpc_endpoint_destroy(this); }
};

#endif  // GRPC_SRC_CORE_LIB_IOMGR_ENDPOINT_H
