// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#ifndef OPENSSL_HEADER_CRYPTO_UBE_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_UBE_INTERNAL_H

#if defined(__cplusplus)
extern "C" {
#endif

#include <openssl/base.h>

// get_ube_generation_number returns the uniqueness-breaking event (UBE)
// generation number for the address space in |current_generation_number|. The
// UBE generation number is a non-zero, strictly-monotonic counter with the
// following property: if queried in an address space and then subsequently
// queried, after an UBE occurred, a greater value will be observed.
//
// This function should be used to protect unique memory. First cache a UBE
// generation number associating it to the unique memory at
// creation/initialization time. Before using the unique memory check whether
// the generation number has changed. Note, however, that detection methods rely
// on technology that is unique to a platform. Hence, support for UBE detection
// also depends on the platform AWS-LC is executed on.
//
// The parameter |current_generation_number| must be synchronized by the caller.
//
// Returns 1 on success and 0 if not supported. 0 means that UBE detection is
// not supported and any unique state must randomize before usage.
// In case of an error or if UBE detection is unavailable, all subsequent
// entries will immediately return.
OPENSSL_EXPORT int CRYPTO_get_ube_generation_number(uint64_t *current_generation_number);

// set_fork_ube_generation_number_FOR_TESTING sets the fork generation number to
// the value |fork_gn|. This value will be the fork generation value used by the
// UBE logic, overriding the generation number from the real fork detection.
// |allow_mocked_ube_detection_FOR_TESTING| must have been invoked
// (once per-process) to allow mocking the fork generation number.
OPENSSL_EXPORT void set_fork_ube_generation_number_FOR_TESTING(uint64_t fork_gn);

// set_vm_ube_generation_number_FOR_TESTING sets the vm_ube generation
// number to the value |vm_ube_gn|. This value will be the vm_ube generation
// value used by the UBE logic, overriding the generation number from the real
// vm_ube detection.
// |allow_mocked_ube_detection_FOR_TESTING| must have been invoked (once
// per-process) to allow mocking the vm_ube generation number.
OPENSSL_EXPORT void set_vm_ube_generation_number_FOR_TESTING(uint32_t vm_ube_gn);

// allow_mocked_ube_detection_FOR_TESTING allows mocking UBE detection even
// though real detection is not available. This function must be called in
// test code when mocking the generation numbers, once per-process. Mocking is
// process-global i.e. all threads will read the same mocked value.
OPENSSL_EXPORT void allow_mocked_ube_detection_FOR_TESTING(void);

// disable_mocked_ube_detection_FOR_TESTING disables mocking UBE detection. It
// also resets any mocked values to default values (0). This function should be
// invoked when exiting testing.
OPENSSL_EXPORT void disable_mocked_ube_detection_FOR_TESTING(void);

#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_UBE_INTERNAL_H
