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

#ifndef GRPC_SRC_CORE_TSI_ALTS_FRAME_PROTECTOR_ALTS_CRYPTER_H
#define GRPC_SRC_CORE_TSI_ALTS_FRAME_PROTECTOR_ALTS_CRYPTER_H

#include <grpc/grpc.h>
#include <grpc/support/port_platform.h>
#include <stdbool.h>
#include <string.h>

#include "src/core/tsi/alts/crypt/gsec.h"

///
/// An alts_crypter interface for an ALTS record protocol providing
/// seal/unseal functionality. The interface is thread-compatible.
///

typedef struct alts_crypter alts_crypter;

///
/// A typical usage of the interface would be
///------------------------------------------------------------------------------
///// Perform a seal operation. We assume the gsec_aead_crypter instance -
///// client_aead_crypter is created beforehand with a 16-byte key and 12-byte
///// nonce length.
///
/// alts_crypter* client = nullptr;
/// char* client_error_in_creation = nullptr;
/// unsigned char* data = nullptr;
/// grpc_status_code client_status =
///                alts_seal_crypter_create(client_aead_crypter, 1, 5, &client,
///                                         &client_error_in_creation);
/// if (client_status == GRPC_STATUS_OK) {
///  size_t data_size = 100;
///  size_t num_overhead_bytes = alts_crypter_num_overhead_bytes(client);
///  size_t data_allocated_size = data_size + num_overhead_bytes;
///  data = gpr_malloc(data_allocated_size);
///  char* client_error_in_seal = nullptr;
///  // Client performs a seal operation.
///  client_status = alts_crypter_process_in_place(client, data,
///                                                data_allocated_size,
///                                                &data_size,
///                                                &client_error_in_seal);
///  if (client_status != GRPC_STATUS_OK) {
///    fprintf(stderr, "seal operation failed with error code:"
///                    "%d, message: %s\n", client_status,
///                     client_error_in_seal);
///   }
///   gpr_free(client_error_in_seal);
///} else {
///    fprintf(stderr, "alts_crypter instance creation failed with error"
///                    "code: %d, message: %s\n", client_status,
///                     client_error_in_creation);
///}
///
///...
///
/// gpr_free(client_error_in_creation);
/// alts_crypter_destroy(client);
///
///...
///
///// Perform an unseal operation. We assume the gsec_aead_crypter instance -
///// server_aead_crypter is created beforehand with a 16-byte key and 12-byte
///// nonce length. The key used in the creation of gsec_aead_crypter instances
///// at server and client sides should be identical.
///
/// alts_crypter* server = nullptr;
/// char* server_error_in_creation = nullptr;
/// grpc_status_code server_status =
///              alts_unseal_crypter_create(server_aead_crypter, 0, 5, &server,
///                                         &server_error_in_creation);
/// if (server_status == GRPC_STATUS_OK) {
///  size_t num_overhead_bytes = alts_crypter_num_overhead_bytes(server);
///  size_t data_size = 100 + num_overhead_bytes;
///  size_t data_allocated_size = data_size;
///  char* server_error_in_unseal = nullptr;
///  // Server performs an unseal operation.
///  server_status = alts_crypter_process_in_place(server, data,
///                                                data_allocated_size,
///                                                &data_size,
///                                                &server_error_in_unseal);
///  if (server_status != GRPC_STATUS_OK) {
///    fprintf(stderr, "unseal operation failed with error code:"
///                    "%d, message: %s\n", server_status,
///                     server_error_in_unseal);
///  }
///  gpr_free(server_error_in_unseal);
///} else {
///    fprintf(stderr, "alts_crypter instance creation failed with error"
///                    "code: %d, message: %s\n", server_status,
///                     server_error_in_creation);
///}
///
///...
///
/// gpr_free(data);
/// gpr_free(server_error_in_creation);
/// alts_crypter_destroy(server);
///
///...
///------------------------------------------------------------------------------
///

// V-table for alts_crypter operations
typedef struct alts_crypter_vtable {
  size_t (*num_overhead_bytes)(const alts_crypter* crypter);
  grpc_status_code (*process_in_place)(alts_crypter* crypter,
                                       unsigned char* data,
                                       size_t data_allocated_size,
                                       size_t data_size, size_t* output_size,
                                       char** error_details);
  void (*destruct)(alts_crypter* crypter);
} alts_crypter_vtable;

// Main struct for alts_crypter interface
struct alts_crypter {
  const alts_crypter_vtable* vtable;
};

///
/// This method gets the number of overhead bytes needed for sealing data that
/// is the difference in size between the protected and raw data. The counter
/// value used in a seal or unseal operation is locally maintained (not sent or
/// received from the other peer) and therefore, will not be counted as part of
/// overhead bytes.
///
///- crypter: an alts_crypter instance.
///
/// On success, the method returns the number of overhead bytes. Otherwise, it
/// returns zero.
///
///
size_t alts_crypter_num_overhead_bytes(const alts_crypter* crypter);

///
/// This method performs either a seal or an unseal operation depending on the
/// alts_crypter instance - crypter passed to the method. If the crypter is
/// an instance implementing a seal operation, the method will perform a seal
/// operation. That is, it seals raw data and stores the result in-place, and
/// the memory allocated for data must be at least data_length +
/// alts_crypter_num_overhead_bytes(). If the crypter is an instance
/// implementing an unseal operation, the method will perform an unseal
/// operation. That is, it unseals protected data and stores the result
/// in-place. The size of unsealed data will be data_length -
/// alts_crypter_num_overhead_bytes(). Integrity tag will be verified during
/// the unseal operation, and if verification fails, the data will be wiped.
/// The counters used in both seal and unseal operations are managed internally.
///
///- crypter: an alts_crypter instance.
///- data: if the method performs a seal operation, the data represents raw data
///  that needs to be sealed. It also plays the role of buffer to hold the
///  protected data as a result of seal. If the method performs an unseal
///  operation, the data represents protected data that needs to be unsealed. It
///  also plays the role of buffer to hold raw data as a result of unseal.
///- data_allocated_size: the size of data buffer. The parameter is used to
///  check whether the result of either seal or unseal can be safely written to
///  the data buffer.
///- data_size: if the method performs a seal operation, data_size
///  represents the size of raw data that needs to be sealed, and if the method
///  performs an unseal operation, data_size represents the size of protected
///  data that needs to be unsealed.
///- output_size: size of data written to the data buffer after a seal or an
///  unseal operation.
///- error_details: a buffer containing an error message if the method does not
///  function correctly. It is legal to pass nullptr into error_details and
///  otherwise, the parameter should be freed with gpr_free.
///
/// On success, the method returns GRPC_STATUS_OK. Otherwise,
/// it returns an error status code along with its details specified in
/// error_details (if error_details is not nullptr).
///
grpc_status_code alts_crypter_process_in_place(
    alts_crypter* crypter, unsigned char* data, size_t data_allocated_size,
    size_t data_size, size_t* output_size, char** error_details);

///
/// This method creates an alts_crypter instance to be used to perform a seal
/// operation, given a gsec_aead_crypter instance and a flag indicating if the
/// created instance will be used at the client or server side. It takes
/// ownership of gsec_aead_crypter instance.
///
///- gc: a gsec_aead_crypter instance used to perform AEAD encryption.
///- is_client: a flag indicating if the alts_crypter instance will be
///  used at the client (is_client = true) or server (is_client =
///  false) side.
///- overflow_size: overflow size of counter in bytes.
///- crypter: an alts_crypter instance to be returned from the method.
///- error_details: a buffer containing an error message if the method does
///  not function correctly. It is legal to pass nullptr into error_details, and
///  otherwise, the parameter should be freed with gpr_free.
///
/// On success of creation, the method returns GRPC_STATUS_OK.
/// Otherwise, it returns an error status code along with its details specified
/// in error_details (if error_details is not nullptr).
///
grpc_status_code alts_seal_crypter_create(gsec_aead_crypter* gc, bool is_client,
                                          size_t overflow_size,
                                          alts_crypter** crypter,
                                          char** error_details);

///
/// This method creates an alts_crypter instance used to perform an unseal
/// operation, given a gsec_aead_crypter instance and a flag indicating if the
/// created instance will be used at the client or server side. It takes
/// ownership of gsec_aead_crypter instance.
///
///- gc: a gsec_aead_crypter instance used to perform AEAD decryption.
///- is_client: a flag indicating if the alts_crypter instance will be
///  used at the client (is_client = true) or server (is_client =
///  false) side.
///- overflow_size: overflow size of counter in bytes.
///- crypter: an alts_crypter instance to be returned from the method.
///- error_details: a buffer containing an error message if the method does
///  not function correctly. It is legal to pass nullptr into error_details, and
///  otherwise, the parameter should be freed with gpr_free.
///
/// On success of creation, the method returns GRPC_STATUS_OK.
/// Otherwise, it returns an error status code along with its details specified
/// in error_details (if error_details is not nullptr).
///
grpc_status_code alts_unseal_crypter_create(gsec_aead_crypter* gc,
                                            bool is_client,
                                            size_t overflow_size,
                                            alts_crypter** crypter,
                                            char** error_details);

///
/// This method destroys an alts_crypter instance by de-allocating all of its
/// occupied memory. A gsec_aead_crypter instance passed in at alts_crypter
/// instance creation time will be destroyed in this method.
///
///- crypter: an alts_crypter instance.
///
void alts_crypter_destroy(alts_crypter* crypter);

#endif  // GRPC_SRC_CORE_TSI_ALTS_FRAME_PROTECTOR_ALTS_CRYPTER_H
