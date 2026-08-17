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

#ifndef GRPC_SRC_CORE_TSI_ALTS_FRAME_PROTECTOR_ALTS_RECORD_PROTOCOL_CRYPTER_COMMON_H
#define GRPC_SRC_CORE_TSI_ALTS_FRAME_PROTECTOR_ALTS_RECORD_PROTOCOL_CRYPTER_COMMON_H

#include <grpc/grpc.h>
#include <grpc/support/port_platform.h>

#include "src/core/tsi/alts/frame_protector/alts_counter.h"
#include "src/core/tsi/alts/frame_protector/alts_crypter.h"

///
/// This file contains common implementation that will be used in both seal and
/// unseal operations.
///

///
/// Main struct for alts_record_protocol_crypter that will be used in both
/// seal and unseal operations.
///
typedef struct alts_record_protocol_crypter {
  alts_crypter base;
  gsec_aead_crypter* crypter;
  alts_counter* ctr;
} alts_record_protocol_crypter;

///
/// This method performs input sanity checks on a subset of inputs to
/// alts_crypter_process_in_place() for both seal and unseal operations.
///
///- rp_crypter: an alts_record_protocol_crypter instance.
///- data: it represents raw data that needs to be sealed in a seal operation or
///  protected data that needs to be unsealed in an unseal operation.
///- output_size: size of data written to the data buffer after a seal or
///  unseal operation.
///- error_details: a buffer containing an error message if any of checked
///  inputs is nullptr. It is legal to pass nullptr into error_details and
///  otherwise, the parameter should be freed with gpr_free.
///
/// On success, the method returns GRPC_STATUS_OK. Otherwise,
/// it returns an error status code along with its details specified in
/// error_details (if error_details is not nullptr).
///
grpc_status_code input_sanity_check(
    const alts_record_protocol_crypter* rp_crypter, const unsigned char* data,
    size_t* output_size, char** error_details);

///
/// This method increments the counter within an alts_record_protocol_crypter
/// instance.
///
///- rp_crypter: an alts_record_protocol_crypter instance.
///- error_details: a buffer containing an error message if the method does not
///  function correctly or the counter is wrapped. It is legal to pass nullptr
///  into error_details and otherwise, the parameter should be freed with
///  gpr_free.
///
/// On success, the method returns GRPC_STATUS_OK. Otherwise,
/// it returns an error status code along with its details specified in
/// error_details (if error_details is not nullptr).
///
grpc_status_code increment_counter(alts_record_protocol_crypter* rp_crypter,
                                   char** error_details);

///
/// This method creates an alts_crypter instance, and populates the fields
/// that are common to both seal and unseal operations.
///
///- crypter: a gsec_aead_crypter instance used to perform AEAD decryption. The
///  function does not take ownership of crypter.
///- is_client: a flag indicating if the alts_crypter instance will be
///  used at the client (is_client = true) or server (is_client =
///  false) side.
///- overflow_size: overflow size of counter in bytes.
///- error_details: a buffer containing an error message if the method does
///  not function correctly. It is legal to pass nullptr into error_details, and
///  otherwise, the parameter should be freed with gpr_free.
///
/// On success of creation, the method returns alts_record_protocol_crypter
/// instance. Otherwise, it returns nullptr with its details specified in
/// error_details (if error_details is not nullptr).
///
///
alts_record_protocol_crypter* alts_crypter_create_common(
    gsec_aead_crypter* crypter, bool is_client, size_t overflow_size,
    char** error_details);

///
/// For the following two methods, please refer to the corresponding API in
/// alts_crypter.h for detailed specifications.
///
size_t alts_record_protocol_crypter_num_overhead_bytes(const alts_crypter* c);

void alts_record_protocol_crypter_destruct(alts_crypter* c);

#endif  // GRPC_SRC_CORE_TSI_ALTS_FRAME_PROTECTOR_ALTS_RECORD_PROTOCOL_CRYPTER_COMMON_H
