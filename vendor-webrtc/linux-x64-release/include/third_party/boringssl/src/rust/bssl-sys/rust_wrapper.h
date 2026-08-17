// Copyright 2022 The BoringSSL Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef OPENSSL_HEADER_RUST_WRAPPER_H
#define OPENSSL_HEADER_RUST_WRAPPER_H

#include <openssl/err.h>
#include <openssl/bytestring.h>

#if defined(__cplusplus)
extern "C" {
#endif


// The following functions are wrappers over inline functions and macros in
// BoringSSL. These are not necessary, as bindgen has long supported
// --wrap-static-fns, however Android is still missing support for this. (See
// b/290347127.) These manual wrappers are, temporarily, retained for Android,
// but this codepath is no longer tested or supported by BoringSSL.
int ERR_GET_LIB_RUST(uint32_t packed_error);
int ERR_GET_REASON_RUST(uint32_t packed_error);
int ERR_GET_FUNC_RUST(uint32_t packed_error);
void CBS_init_RUST(CBS *cbs, const uint8_t *data, size_t len);
size_t CBS_len_RUST(const CBS *cbs);

#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_RUST_WRAPPER_H
