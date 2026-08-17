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

#ifndef GRPC_SRC_CORE_TSI_ALTS_FRAME_PROTECTOR_ALTS_COUNTER_H
#define GRPC_SRC_CORE_TSI_ALTS_FRAME_PROTECTOR_ALTS_COUNTER_H

#include <grpc/grpc.h>
#include <grpc/support/port_platform.h>
#include <stdbool.h>
#include <stdlib.h>

// Main struct for a crypter counter managed within seal/unseal operations.
typedef struct alts_counter {
  size_t size;
  size_t overflow_size;
  unsigned char* counter;
} alts_counter;

///
/// This method creates and initializes an alts_counter instance.
///
///- is_client: a flag indicating if the alts_counter instance will be used
///  at client (is_client = true) or server (is_client = false) side.
///- counter_size: size of buffer holding the counter value.
///- overflow_size: overflow size in bytes. The counter instance can be used
///  to produce at most 2^(overflow_size*8) frames.
///- crypter_counter: an alts_counter instance to be returned from the method.
///- error_details: a buffer containing an error message if the method does not
///  function correctly. It is legal to pass nullptr into error_details and
///  otherwise, the parameter should be freed with gpr_free.
///
/// On success, the method returns GRPC_STATUS_OK. Otherwise,
/// it returns an error status code along with its details specified in
/// error_details (if error_details is not nullptr).
///
grpc_status_code alts_counter_create(bool is_client, size_t counter_size,
                                     size_t overflow_size,
                                     alts_counter** crypter_counter,
                                     char** error_details);

///
/// This method increments the internal counter.
///
///- crypter_counter: an alts_counter instance.
///- is_overflow: after incrementing the internal counter, if an overflow
///  occurs, is_overflow is set to true, and no further calls to
///  alts_counter_increment() should be made. Otherwise, is_overflow is set to
///  false.
///- error_details: a buffer containing an error message if the method does not
///  function correctly. It is legal to pass nullptr into error_details and
///  otherwise, the parameter should be freed with gpr_free.
///
/// On success, the method returns GRPC_STATUS_OK. Otherwise,
/// it returns an error status code along with its details specified in
/// error_details (if error_details is not nullptr).
///
grpc_status_code alts_counter_increment(alts_counter* crypter_counter,
                                        bool* is_overflow,
                                        char** error_details);

///
/// This method returns the size of counter buffer.
///
///- crypter_counter: an alts_counter instance.
///
size_t alts_counter_get_size(alts_counter* crypter_counter);

///
/// This method returns the counter buffer.
///
///- crypter_counter: an alts_counter instance.
///
unsigned char* alts_counter_get_counter(alts_counter* crypter_counter);

///
/// This method de-allocates all memory allocated to an alts_counter instance.
///- crypter_counter: an alts_counter instance.
///
void alts_counter_destroy(alts_counter* crypter_counter);

#endif  // GRPC_SRC_CORE_TSI_ALTS_FRAME_PROTECTOR_ALTS_COUNTER_H
