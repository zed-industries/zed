// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

// Simple program that should also be able to compiler, udeful to test different
// compiler flags

#include <stdlib.h>
#include <stdatomic.h>

// Some platforms define ATOMIC_LONG_LOCK_FREE as an expression like:
//  (__atomic_always_lock_free (sizeof (atomic_long), (void *) 0) ? 2 :
//     (__atomic_is_lock_free (sizeof (atomic_long), (void *) 0) ? 1 : 0))
// which relies on __atomic_is_lock_free which is a runtime function and not a
// compile time constant. This means ATOMIC_LONG_LOCK_FREE can not be used in a
// preprocessor check which AWS-LC needs to do to swap implementations of
// CRYPTO_refcount_inc at compile time. If that is the case the following #if
// won't compile, the following message is helpful to debug any future issues.
#if ATOMIC_LONG_LOCK_FREE < 0
#error "Should not get here, the above line should be false or invalid"
#endif

int main(int argc, char **argv) {
    return EXIT_SUCCESS;
}
