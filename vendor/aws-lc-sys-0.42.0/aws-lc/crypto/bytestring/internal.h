// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#ifndef OPENSSL_HEADER_BYTESTRING_INTERNAL_H
#define OPENSSL_HEADER_BYTESTRING_INTERNAL_H

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
OPENSSL_EXPORT int CBB_finish_i2d(CBB *cbb, uint8_t **outp);


// Unicode utilities.

// The following functions read one Unicode code point from |cbs| with the
// corresponding encoding and store it in |*out|. They return one on success and
// zero on error.
OPENSSL_EXPORT int cbs_get_utf8(CBS *cbs, uint32_t *out);
OPENSSL_EXPORT int cbs_get_latin1(CBS *cbs, uint32_t *out);
OPENSSL_EXPORT int cbs_get_ucs2_be(CBS *cbs, uint32_t *out);
OPENSSL_EXPORT int cbs_get_utf32_be(CBS *cbs, uint32_t *out);

// cbb_get_utf8_len returns the number of bytes needed to represent |u| in
// UTF-8.
OPENSSL_EXPORT size_t cbb_get_utf8_len(uint32_t u);

// The following functions encode |u| to |cbb| with the corresponding
// encoding. They return one on success and zero on error.
OPENSSL_EXPORT int cbb_add_utf8(CBB *cbb, uint32_t u);
OPENSSL_EXPORT int cbb_add_latin1(CBB *cbb, uint32_t u);
OPENSSL_EXPORT int cbb_add_ucs2_be(CBB *cbb, uint32_t u);
OPENSSL_EXPORT int cbb_add_utf32_be(CBB *cbb, uint32_t u);

// cbs_get_any_asn1_element parses an ASN.1 element from |cbs|. |*out_indefinite|
// is set to one if the length was indefinite and zero otherwise. On success,
// if the length is indefinite |out| will only contain the ASN.1 header,
// otherwise is will contain both the header and the content. If |out_tag| is
// not NULL, |*out_tag| is set to the element's tag number. If |out_header_len|
// is not NULL, |*out_header_len| is set to the length of the header.
//
// If |ber_ok| is one, BER encoding is permitted. In this case, if
// |out_ber_found| is not NULL and BER-specific encoding was found,
// |*out_ber_found| is set to one. If |out_indefinite| is not NULL and the
// element has indefinite-length, |*out_indefinite| is set to one.
// If |ber_ok| is zero, both |out_ber_found| and |out_indefinite| must be NULL.
//
// If |universal_tag_ok| is 1, universal tags are permitted. Otherwise, only
// context-specific tags are accepted.
//
// It returns one on success and zero on failure.
int cbs_get_any_asn1_element(CBS *cbs, CBS *out, CBS_ASN1_TAG *out_tag,
                                      size_t *out_header_len, int *out_ber_found,
                                      int *out_indefinite, int ber_ok, int universal_tag_ok);

#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_BYTESTRING_INTERNAL_H
