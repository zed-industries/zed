// Copyright 1995-2016 The OpenSSL Project Authors. All Rights Reserved.
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

#ifndef OPENSSL_HEADER_CRYPTO_PEM_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_PEM_INTERNAL_H

#include <openssl/pem.h>

#ifdef __cplusplus
extern "C" {
#endif


// PEM_get_EVP_CIPHER_INFO decodes |header| as a PEM header block and writes the
// specified cipher and IV to |cipher|. It returns one on success and zero on
// error. |header| must be a NUL-terminated string. If |header| does not
// specify encryption, this function will return success and set
// |cipher->cipher| to NULL.
int PEM_get_EVP_CIPHER_INFO(const char *header, EVP_CIPHER_INFO *cipher);

// PEM_do_header decrypts |*len| bytes from |data| in-place according to the
// information in |cipher|. On success, it returns one and sets |*len| to the
// length of the plaintext. Otherwise, it returns zero. If |cipher| specifies
// encryption, the key is derived from a password returned from |callback|.
int PEM_do_header(const EVP_CIPHER_INFO *cipher, uint8_t *data, long *len,
                  pem_password_cb *callback, void *u);


#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // OPENSSL_HEADER_CRYPTO_PEM_INTERNAL_H
