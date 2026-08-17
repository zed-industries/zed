//
//
// Copyright 2018 gRPC authors.
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

#ifndef GRPC_SRC_CORE_TSI_ALTS_ZERO_COPY_FRAME_PROTECTOR_ALTS_GRPC_RECORD_PROTOCOL_H
#define GRPC_SRC_CORE_TSI_ALTS_ZERO_COPY_FRAME_PROTECTOR_ALTS_GRPC_RECORD_PROTOCOL_H

#include <grpc/slice_buffer.h>
#include <grpc/support/port_platform.h>

#include "src/core/tsi/transport_security_interface.h"

///
/// This alts_grpc_record_protocol object protects and unprotects a single frame
/// stored in grpc slice buffer with zero or minimized memory copy.
/// Implementations of this object must be thread compatible.
///
typedef struct alts_grpc_record_protocol alts_grpc_record_protocol;

///
/// This methods performs protect operation on unprotected data and appends the
/// protected frame to protected_slices. The caller needs to ensure the length
/// of unprotected data plus the frame overhead is less than or equal to the
/// maximum frame length. The input unprotected data slice buffer will be
/// cleared, although the actual unprotected data bytes are not modified.
///
///- self: an alts_grpc_record_protocol instance.
///- unprotected_slices: the unprotected data to be protected.
///- protected_slices: slice buffer where the protected frame is appended.
///
/// This method returns TSI_OK in case of success or a specific error code in
/// case of failure.
///
tsi_result alts_grpc_record_protocol_protect(
    alts_grpc_record_protocol* self, grpc_slice_buffer* unprotected_slices,
    grpc_slice_buffer* protected_slices);

///
/// This methods performs unprotect operation on a full frame of protected data
/// and appends unprotected data to unprotected_slices. It is the caller's
/// responsibility to prepare a full frame of data before calling this method.
/// The input protected frame slice buffer will be cleared, although the actual
/// protected data bytes are not modified.
///
///- self: an alts_grpc_record_protocol instance.
///- protected_slices: a full frame of protected data in grpc slices.
///- unprotected_slices: slice buffer where unprotected data is appended.
///
/// This method returns TSI_OK in case of success or a specific error code in
/// case of failure.
///
tsi_result alts_grpc_record_protocol_unprotect(
    alts_grpc_record_protocol* self, grpc_slice_buffer* protected_slices,
    grpc_slice_buffer* unprotected_slices);

///
/// This method returns maximum allowed unprotected data size, given maximum
/// protected frame size.
///
///- self: an alts_grpc_record_protocol instance.
///- max_protected_frame_size: maximum protected frame size.
///
/// On success, the method returns the maximum allowed unprotected data size.
/// Otherwise, it returns zero.
///
size_t alts_grpc_record_protocol_max_unprotected_data_size(
    const alts_grpc_record_protocol* self, size_t max_protected_frame_size);

///
/// This method destroys an alts_grpc_record_protocol instance by de-allocating
/// all of its occupied memory.
///
void alts_grpc_record_protocol_destroy(alts_grpc_record_protocol* self);

#endif  // GRPC_SRC_CORE_TSI_ALTS_ZERO_COPY_FRAME_PROTECTOR_ALTS_GRPC_RECORD_PROTOCOL_H
