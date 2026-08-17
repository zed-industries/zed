//
//
// Copyright 2015 gRPC authors.
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

#ifndef GRPC_SRC_CORE_TSI_SSL_TYPES_H
#define GRPC_SRC_CORE_TSI_SSL_TYPES_H

// A collection of macros to cast between various integer types that are
// used differently between BoringSSL and OpenSSL:
// TSI_INT_AS_SIZE(x):  convert 'int x' to a length parameter for an OpenSSL
//                      function
// TSI_SIZE_AS_SIZE(x): convert 'size_t x' to a length parameter for an OpenSSL
//                      function
//

#include <grpc/support/port_platform.h>
#include <openssl/ssl.h>

#ifdef OPENSSL_IS_BORINGSSL
#define TSI_INT_AS_SIZE(x) ((size_t)(x))
#define TSI_SIZE_AS_SIZE(x) (x)
#else
#define TSI_INT_AS_SIZE(x) (x)
#define TSI_SIZE_AS_SIZE(x) ((int)(x))
#endif

#endif  // GRPC_SRC_CORE_TSI_SSL_TYPES_H
