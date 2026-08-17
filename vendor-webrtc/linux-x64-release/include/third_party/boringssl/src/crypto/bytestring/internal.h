// Copyright 2014 The BoringSSL Authors
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

#ifndef OPENSSL_HEADER_CRYPTO_BYTESTRING_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_BYTESTRING_INTERNAL_H

#include <openssl/base.h>

#if defined(__cplusplus)
extern "C" {
#endif


// CBS_asn1_ber_to_der reads a BER element from |in|. If it finds
// indefinite-length elements or constructed strings then it converts the BER
// data to DER, sets |out| to the converted contents and |*out_storage| to a
// buffer which the caller must release with |OPENSSL_free|. Otherwise, it sets
// |out| to the original BER element in |in| and |*out_storage| to NULL.
// Additionally, |*in| will be advanced over the BER element.
//
// This function should successfully process any valid BER input, however it
// will not convert all of BER's deviations from DER. BER is ambiguous between
// implicitly-tagged SEQUENCEs of strings and implicitly-tagged constructed
// strings. Implicitly-tagged strings must be parsed with
// |CBS_get_ber_implicitly_tagged_string| instead of |CBS_get_asn1|. The caller
// must also account for BER variations in the contents of a primitive.
//
// It returns one on success and zero otherwise.
OPENSSL_EXPORT int CBS_asn1_ber_to_der(CBS *in, CBS *out,
                                       uint8_t **out_storage);

// CBS_get_asn1_implicit_string parses a BER string of primitive type
// |inner_tag| implicitly-tagged with |outer_tag|. It sets |out| to the
// contents. If concatenation was needed, it sets |*out_storage| to a buffer
// which the caller must release with |OPENSSL_free|. Otherwise, it sets
// |*out_storage| to NULL.
//
// This function does not parse all of BER. It requires the string be
// definite-length. Constructed strings are allowed, but all children of the
// outermost element must be primitive. The caller should use
// |CBS_asn1_ber_to_der| before running this function.
//
// It returns one on success and zero otherwise.
OPENSSL_EXPORT int CBS_get_asn1_implicit_string(CBS *in, CBS *out,
                                                uint8_t **out_storage,
                                                CBS_ASN1_TAG outer_tag,
                                                CBS_ASN1_TAG inner_tag);

// CBB_finish_i2d calls |CBB_finish| on |cbb| which must have been initialized
// with |CBB_init|. If |outp| is not NULL then the result is written to |*outp|
// and |*outp| is advanced just past the output. It returns the number of bytes
// in the result, whether written or not, or a negative value on error. On
// error, it calls |CBB_cleanup| on |cbb|.
//
// This function may be used to help implement legacy i2d ASN.1 functions.
int CBB_finish_i2d(CBB *cbb, uint8_t **outp);


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_BYTESTRING_INTERNAL_H
