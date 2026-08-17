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

#ifndef GRPC_SRC_CORE_TSI_ALTS_ZERO_COPY_FRAME_PROTECTOR_ALTS_IOVEC_RECORD_PROTOCOL_H
#define GRPC_SRC_CORE_TSI_ALTS_ZERO_COPY_FRAME_PROTECTOR_ALTS_IOVEC_RECORD_PROTOCOL_H

#include <grpc/support/port_platform.h>
#include <stdbool.h>

#include "src/core/tsi/alts/crypt/gsec.h"

constexpr size_t kZeroCopyFrameMessageType = 0x06;
constexpr size_t kZeroCopyFrameLengthFieldSize = 4;
constexpr size_t kZeroCopyFrameMessageTypeFieldSize = 4;
constexpr size_t kZeroCopyFrameHeaderSize =
    kZeroCopyFrameLengthFieldSize + kZeroCopyFrameMessageTypeFieldSize;

// Limit k on number of frames such that at most 2^(8 * k) frames can be sent.
constexpr size_t kAltsRecordProtocolRekeyFrameLimit = 8;
constexpr size_t kAltsRecordProtocolFrameLimit = 5;

// An implementation of alts record protocol. The API is thread-compatible.

typedef struct iovec iovec_t;

typedef struct alts_iovec_record_protocol alts_iovec_record_protocol;

///
/// This method gets the length of record protocol frame header.
///
size_t alts_iovec_record_protocol_get_header_length();

///
/// This method gets the length of record protocol frame tag.
///
///- rp: an alts_iovec_record_protocol instance.
///
/// On success, the method returns the length of record protocol frame tag.
/// Otherwise, it returns zero.
///
size_t alts_iovec_record_protocol_get_tag_length(
    const alts_iovec_record_protocol* rp);

///
/// This method returns maximum allowed unprotected data size, given maximum
/// protected frame size.
///
///- rp: an alts_iovec_record_protocol instance.
///- max_protected_frame_size: maximum protected frame size.
///
/// On success, the method returns the maximum allowed unprotected data size.
/// Otherwise, it returns zero.
///
size_t alts_iovec_record_protocol_max_unprotected_data_size(
    const alts_iovec_record_protocol* rp, size_t max_protected_frame_size);

///
/// This method performs integrity-only protect operation on a
/// alts_iovec_record_protocol instance, i.e., compute frame header and tag. The
/// caller needs to allocate the memory for header and tag prior to calling this
/// method.
///
///- rp: an alts_iovec_record_protocol instance.
///- unprotected_vec: an iovec array containing unprotected data.
///- unprotected_vec_length: the array length of unprotected_vec.
///- header: an iovec containing the output frame header.
///- tag: an iovec containing the output frame tag.
///- error_details: a buffer containing an error message if the method does not
///  function correctly. It is OK to pass nullptr into error_details.
///
/// On success, the method returns GRPC_STATUS_OK. Otherwise, it returns an
/// error status code along with its details specified in error_details (if
/// error_details is not nullptr).
///
grpc_status_code alts_iovec_record_protocol_integrity_only_protect(
    alts_iovec_record_protocol* rp, const iovec_t* unprotected_vec,
    size_t unprotected_vec_length, iovec_t header, iovec_t tag,
    char** error_details);

///
/// This method performs integrity-only unprotect operation on a
/// alts_iovec_record_protocol instance, i.e., verify frame header and tag.
///
///- rp: an alts_iovec_record_protocol instance.
///- protected_vec: an iovec array containing protected data.
///- protected_vec_length: the array length of protected_vec.
///- header: an iovec containing the frame header.
///- tag: an iovec containing the frame tag.
///- error_details: a buffer containing an error message if the method does not
///  function correctly. It is OK to pass nullptr into error_details.
///
/// On success, the method returns GRPC_STATUS_OK. Otherwise, it returns an
/// error status code along with its details specified in error_details (if
/// error_details is not nullptr).
///
grpc_status_code alts_iovec_record_protocol_integrity_only_unprotect(
    alts_iovec_record_protocol* rp, const iovec_t* protected_vec,
    size_t protected_vec_length, iovec_t header, iovec_t tag,
    char** error_details);

///
/// This method performs privacy-integrity protect operation on a
/// alts_iovec_record_protocol instance, i.e., compute a protected frame. The
/// caller needs to allocate the memory for the protected frame prior to calling
/// this method.
///
///- rp: an alts_iovec_record_protocol instance.
///- unprotected_vec: an iovec array containing unprotected data.
///- unprotected_vec_length: the array length of unprotected_vec.
///- protected_frame: an iovec containing the output protected frame.
///- error_details: a buffer containing an error message if the method does not
///  function correctly. It is OK to pass nullptr into error_details.
///
/// On success, the method returns GRPC_STATUS_OK. Otherwise, it returns an
/// error status code along with its details specified in error_details (if
/// error_details is not nullptr).
///
grpc_status_code alts_iovec_record_protocol_privacy_integrity_protect(
    alts_iovec_record_protocol* rp, const iovec_t* unprotected_vec,
    size_t unprotected_vec_length, iovec_t protected_frame,
    char** error_details);

///
/// This method performs privacy-integrity unprotect operation on a
/// alts_iovec_record_protocol instance given a full protected frame, i.e.,
/// compute the unprotected data. The caller needs to allocated the memory for
/// the unprotected data prior to calling this method.
///
///- rp: an alts_iovec_record_protocol instance.
///- header: an iovec containing the frame header.
///- protected_vec: an iovec array containing protected data including the tag.
///- protected_vec_length: the array length of protected_vec.
///- unprotected_data: an iovec containing the output unprotected data.
///- error_details: a buffer containing an error message if the method does not
///  function correctly. It is OK to pass nullptr into error_details.
///
/// On success, the method returns GRPC_STATUS_OK. Otherwise, it returns an
/// error status code along with its details specified in error_details (if
/// error_details is not nullptr).
///
grpc_status_code alts_iovec_record_protocol_privacy_integrity_unprotect(
    alts_iovec_record_protocol* rp, iovec_t header,
    const iovec_t* protected_vec, size_t protected_vec_length,
    iovec_t unprotected_data, char** error_details);

///
/// This method creates an alts_iovec_record_protocol instance, given a
/// gsec_aead_crypter instance, a flag indicating if the created instance will
/// be used at the client or server side, and a flag indicating if the created
/// instance will be used for integrity-only mode or privacy-integrity mode. The
/// ownership of gsec_aead_crypter instance is transferred to this new object.
///
///- crypter: a gsec_aead_crypter instance used to perform AEAD decryption.
///- overflow_size: overflow size of counter in bytes.
///- is_client: a flag indicating if the alts_iovec_record_protocol instance
///  will be used at the client or server side.
///- is_integrity_only: a flag indicating if the alts_iovec_record_protocol
///  instance will be used for integrity-only or privacy-integrity mode.
///- is_protect: a flag indicating if the alts_grpc_record_protocol instance
///  will be used for protect or unprotect.
///- rp: an alts_iovec_record_protocol instance to be returned from
///  the method.
///- error_details: a buffer containing an error message if the method does not
///  function correctly. It is OK to pass nullptr into error_details.
///
/// On success, the method returns GRPC_STATUS_OK. Otherwise, it returns an
/// error status code along with its details specified in error_details (if
/// error_details is not nullptr).
///
grpc_status_code alts_iovec_record_protocol_create(
    gsec_aead_crypter* crypter, size_t overflow_size, bool is_client,
    bool is_integrity_only, bool is_protect, alts_iovec_record_protocol** rp,
    char** error_details);

///
/// This method destroys an alts_iovec_record_protocol instance by de-allocating
/// all of its occupied memory. A gsec_aead_crypter instance passed in at
/// gsec_alts_crypter instance creation time will be destroyed in this method.
///
void alts_iovec_record_protocol_destroy(alts_iovec_record_protocol* rp);

#endif  // GRPC_SRC_CORE_TSI_ALTS_ZERO_COPY_FRAME_PROTECTOR_ALTS_IOVEC_RECORD_PROTOCOL_H
