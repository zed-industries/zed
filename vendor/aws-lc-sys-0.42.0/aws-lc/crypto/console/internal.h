// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/base.h>

#define MIN_LENGTH 4
#define MAX_PASSWORD_LENGTH 1024

#if defined(__cplusplus)
extern "C" {
#endif

// Console Management API
//
// This API provides functions for secure console I/O operations, supporting both Unix/Linux
// (termios) and Windows systems. It handles terminal operations like reading input with
// optional echo suppression (for password entry) and signal handling.
//
// Usage requires proper lock management:
// 1. Acquire the console mutex with openssl_console_acquire_mutex()
// 2. Initialize console with openssl_console_open()
// 3. Perform console operations (read/write)
// 4. Clean up with openssl_console_close()
// 5. Release the mutex with openssl_console_release_mutex()
//
// The global mutex must be held during all console operations and released after closing the console.
OPENSSL_EXPORT void openssl_console_acquire_mutex(void);
OPENSSL_EXPORT void openssl_console_release_mutex(void);
OPENSSL_EXPORT int openssl_console_open(void);
OPENSSL_EXPORT int openssl_console_close(void);
OPENSSL_EXPORT int openssl_console_write(const char *str);
OPENSSL_EXPORT int openssl_console_read(char *buf, int minsize, int maxsize, int echo);


#if defined(__cplusplus)
}  // extern C
#endif
