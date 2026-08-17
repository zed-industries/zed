// Copyright (c) 2019 Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/ssl.h>

int BORINGSSL_enum_c_type_test(void);

int BORINGSSL_enum_c_type_test(void) {
#if defined(__cplusplus)
#error "This is testing the behaviour of the C compiler."
#error "It's pointless to build it in C++ mode."
#endif

  // In C++, the enums in ssl.h are explicitly typed as ints to allow them to
  // be predeclared. This function confirms that the C compiler believes them
  // to be the same size as ints. They may differ in signedness, however.
  return sizeof(enum ssl_private_key_result_t) == sizeof(int);
}
