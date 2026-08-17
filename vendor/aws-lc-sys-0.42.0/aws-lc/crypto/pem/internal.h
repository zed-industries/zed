// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/pem.h>

#if defined(__cplusplus)
extern "C" {
#endif

// PEM utility functions
// PEM_proc_type appends a Proc-Type header to |buf|, determined by |type|.
void PEM_proc_type(char buf[PEM_BUFSIZE], int type);

// PEM_dek_info appends a DEK-Info header to |buf|, with an algorithm of |type|
// and a single parameter, specified by hex-encoding |len| bytes from |str|.
void PEM_dek_info(char buf[PEM_BUFSIZE], const char *type, size_t len,
                  char *str);

#if defined(__cplusplus)
}  // extern C
#endif
