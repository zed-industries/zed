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

#ifndef GRPC_TEST_CORE_TSI_ALTS_CRYPT_GSEC_TEST_UTIL_H
#define GRPC_TEST_CORE_TSI_ALTS_CRYPT_GSEC_TEST_UTIL_H

#include <grpc/grpc.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

///
/// This method returns random bytes of certain length.
///
///- bytes: buffer to hold random bytes.
///- length: length of buffer to be populated.
///
void gsec_test_random_bytes(uint8_t* bytes, size_t length);

///
/// This method returns an array of random bytes.
///
///- bytes: array to hold random bytes.
///- length: length of array to be populated.
///
void gsec_test_random_array(uint8_t** bytes, size_t length);

///
/// This method returns a uint32 that's not quite uniformly random, but good
/// enough for tests.
///
///- max_length: a max value the returned random number can choose.
///
uint32_t gsec_test_bias_random_uint32(uint32_t max_length);

///
/// This method copies data from a source to a destination buffer.
///
///- src: a source buffer.
///- des: a destination buffer.
///- source_len: the length of source buffer to be copied from its beginning.
///
void gsec_test_copy(const uint8_t* src, uint8_t** des, size_t source_len);

///
/// This method copies data from a source to a destination buffer, and flips one
/// byte in the destination buffer randomly.
///
///- src: a source buffer.
///- des: a destination buffer.
///- length: the length of source buffer to be copied from its beginning.
///
void gsec_test_copy_and_alter_random_byte(const uint8_t* src, uint8_t** des,
                                          size_t source_len);

///
/// This method compares two grpc_status_code values, and verifies if one string
/// is a substring of the other.
///
///- status1: the first grpc_status_code to be compared.
///- status2: the second grpc_status_code to be compared.
///- msg1: a string to be scanned.
///- msg2: a small string to be searched within msg1.
///
/// If both checks succeed, the method returns 1 and otherwise, it returns 0.
///
int gsec_test_expect_compare_code_and_substr(grpc_status_code status1,
                                             grpc_status_code status2,
                                             const char* msg1,
                                             const char* msg2);

#endif  // GRPC_TEST_CORE_TSI_ALTS_CRYPT_GSEC_TEST_UTIL_H */
