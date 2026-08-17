// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

/* This header is provided in order to make compiling against code that expects
   OpenSSL easier. */

#include <openssl/crypto.h>

// MySQL does regex parsing on the opensslv.h file directly.
// https://github.com/mysql/mysql-server/blob/8.0/cmake/ssl.cmake#L208-L227
// |OPENSSL_VERSION_NUMBER| is defined here again to comply to this. MySQL
// only parses this to define version numbers in their CMake script.
// It does not require it to be active.
#if 0
#define OPENSSL_VERSION_NUMBER 0x1010107f
#endif
