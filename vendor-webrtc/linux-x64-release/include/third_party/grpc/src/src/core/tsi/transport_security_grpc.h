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

#ifndef GRPC_SRC_CORE_TSI_TRANSPORT_SECURITY_GRPC_H
#define GRPC_SRC_CORE_TSI_TRANSPORT_SECURITY_GRPC_H

#include <grpc/slice_buffer.h>
#include <grpc/support/port_platform.h>

#include "src/core/tsi/transport_security.h"

// This method creates a tsi_zero_copy_grpc_protector object. It return TSI_OK
// assuming there is no fatal error.
// The caller is responsible for destroying the protector.
tsi_result tsi_handshaker_result_create_zero_copy_grpc_protector(
    const tsi_handshaker_result* self, size_t* max_output_protected_frame_size,
    tsi_zero_copy_grpc_protector** protector);

// -- tsi_zero_copy_grpc_protector object --

// Outputs protected frames.
// - unprotected_slices is the unprotected data to be protected.
// - protected_slices is the protected output frames. One or more frames
//   may be produced in this protect function.
// - Returns TSI_OK in case of success or a specific error code in
//   case of failure.
// - Can be called concurrently with tsi_zero_copy_grpc_protector_unprotect.
tsi_result tsi_zero_copy_grpc_protector_protect(
    tsi_zero_copy_grpc_protector* self, grpc_slice_buffer* unprotected_slices,
    grpc_slice_buffer* protected_slices);

// Outputs unprotected bytes.
// - protected_slices is the bytes of protected frames.
// - unprotected_slices is the unprotected output data.
// - If min_progress_size is not null, it returns the size of the last
//   incomplete frame which could not be fully unprotected.
// - Returns TSI_OK in case of success. Success includes cases where
//   there is not enough data to output in which case unprotected_slices has 0
//   bytes.
// - Can be called concurrently with tsi_zero_copy_grpc_protector_protect.
tsi_result tsi_zero_copy_grpc_protector_unprotect(
    tsi_zero_copy_grpc_protector* self, grpc_slice_buffer* protected_slices,
    grpc_slice_buffer* unprotected_slices, int* min_progress_size);

// Destroys the tsi_zero_copy_grpc_protector object.
void tsi_zero_copy_grpc_protector_destroy(tsi_zero_copy_grpc_protector* self);

// Returns value of max protected frame size. Useful for testing.
tsi_result tsi_zero_copy_grpc_protector_max_frame_size(
    tsi_zero_copy_grpc_protector* self, size_t* max_frame_size);

// Base for tsi_zero_copy_grpc_protector implementations.
// Implementations must guarantee that protect and unprotect can be called
// concurrently.
struct tsi_zero_copy_grpc_protector_vtable {
  tsi_result (*protect)(tsi_zero_copy_grpc_protector* self,
                        grpc_slice_buffer* unprotected_slices,
                        grpc_slice_buffer* protected_slices);
  tsi_result (*unprotect)(tsi_zero_copy_grpc_protector* self,
                          grpc_slice_buffer* protected_slices,
                          grpc_slice_buffer* unprotected_slices,
                          int* min_progress_size);
  void (*destroy)(tsi_zero_copy_grpc_protector* self);
  tsi_result (*max_frame_size)(tsi_zero_copy_grpc_protector* self,
                               size_t* max_frame_size);
};
struct tsi_zero_copy_grpc_protector {
  const tsi_zero_copy_grpc_protector_vtable* vtable;
};

#endif  // GRPC_SRC_CORE_TSI_TRANSPORT_SECURITY_GRPC_H
