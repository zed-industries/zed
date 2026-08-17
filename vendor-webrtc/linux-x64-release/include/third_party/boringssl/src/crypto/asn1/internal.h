// Copyright 2005-2016 The OpenSSL Project Authors. All Rights Reserved.
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

#ifndef OPENSSL_HEADER_CRYPTO_ASN1_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_ASN1_INTERNAL_H

#include <time.h>

#include <openssl/asn1.h>
#include <openssl/asn1t.h>

#if defined(__cplusplus)
extern "C" {
#endif


// Wrapper functions for time functions.

// OPENSSL_gmtime converts a time_t value in |time| which must be in the range
// of year 0000 to 9999 to a broken out time value in |tm|. On success |tm| is
// returned. On failure NULL is returned.
OPENSSL_EXPORT struct tm *OPENSSL_gmtime(const time_t *time, struct tm *result);

// OPENSSL_gmtime_adj returns one on success, and updates |tm| by adding
// |offset_day| days and |offset_sec| seconds. It returns zero on failure. |tm|
// must be in the range of year 0000 to 9999 both before and after the update or
// a failure will be returned.
OPENSSL_EXPORT int OPENSSL_gmtime_adj(struct tm *tm, int offset_day,
                                      int64_t offset_sec);

// OPENSSL_gmtime_diff calculates the difference between |from| and |to|. It
// returns one, and outputs the difference as a number of days and seconds in
// |*out_days| and |*out_secs| on success. It returns zero on failure.  Both
// |from| and |to| must be in the range of year 0000 to 9999 or a failure will
// be returned.
OPENSSL_EXPORT int OPENSSL_gmtime_diff(int *out_days, int *out_secs,
                                       const struct tm *from,
                                       const struct tm *to);

// Internal ASN1 structures and functions: not for application use

// These are used internally in the ASN1_OBJECT to keep track of
// whether the names and data need to be free()ed
#define ASN1_OBJECT_FLAG_DYNAMIC 0x01          // internal use
#define ASN1_OBJECT_FLAG_DYNAMIC_STRINGS 0x04  // internal use
#define ASN1_OBJECT_FLAG_DYNAMIC_DATA 0x08     // internal use

// An asn1_object_st (aka |ASN1_OBJECT|) represents an ASN.1 OBJECT IDENTIFIER.
// Note: Mutating an |ASN1_OBJECT| is only permitted when initializing it. The
// library maintains a table of static |ASN1_OBJECT|s, which may be referenced
// by non-const |ASN1_OBJECT| pointers. Code which receives an |ASN1_OBJECT|
// pointer externally must assume it is immutable, even if the pointer is not
// const.
struct asn1_object_st {
  const char *sn, *ln;
  int nid;
  int length;
  const unsigned char *data;  // data remains const after init
  int flags;                  // Should we free this one
};

ASN1_OBJECT *ASN1_OBJECT_new(void);

// ASN1_ENCODING is used to save the received encoding of an ASN.1 type. This
// avoids problems with invalid encodings that break signatures.
typedef struct ASN1_ENCODING_st {
  // enc is the saved DER encoding. Its ownership is determined by |buf|.
  uint8_t *enc;
  // len is the length of |enc|. If zero, there is no saved encoding.
  size_t len;
  // buf, if non-NULL, is the |CRYPTO_BUFFER| that |enc| points into. If NULL,
  // |enc| must be released with |OPENSSL_free|.
  CRYPTO_BUFFER *buf;
} ASN1_ENCODING;

OPENSSL_EXPORT int asn1_utctime_to_tm(struct tm *tm, const ASN1_UTCTIME *d,
                                      int allow_timezone_offset);
OPENSSL_EXPORT int asn1_generalizedtime_to_tm(struct tm *tm,
                                              const ASN1_GENERALIZEDTIME *d);

int ASN1_item_ex_new(ASN1_VALUE **pval, const ASN1_ITEM *it);
void ASN1_item_ex_free(ASN1_VALUE **pval, const ASN1_ITEM *it);

void ASN1_template_free(ASN1_VALUE **pval, const ASN1_TEMPLATE *tt);

// ASN1_item_ex_d2i parses |len| bytes from |*in| as a structure of type |it|
// and writes the result to |*pval|. If |tag| is non-negative, |it| is
// implicitly tagged with the tag specified by |tag| and |aclass|. If |opt| is
// non-zero, the value is optional. If |buf| is non-NULL, |*in| must point into
// |buf|.
//
// This function returns one and advances |*in| if an object was successfully
// parsed, -1 if an optional value was successfully skipped, and zero on error.
int ASN1_item_ex_d2i(ASN1_VALUE **pval, const unsigned char **in, long len,
                     const ASN1_ITEM *it, int tag, int aclass, char opt,
                     CRYPTO_BUFFER *buf);

// ASN1_item_ex_i2d encodes |*pval| as a value of type |it| to |out| under the
// i2d output convention. It returns a non-zero length on success and -1 on
// error. If |tag| is -1. the tag and class come from |it|. Otherwise, the tag
// number is |tag| and the class is |aclass|. This is used for implicit tagging.
// This function treats a missing value as an error, not an optional field.
int ASN1_item_ex_i2d(ASN1_VALUE **pval, unsigned char **out,
                     const ASN1_ITEM *it, int tag, int aclass);

void ASN1_primitive_free(ASN1_VALUE **pval, const ASN1_ITEM *it);

// asn1_get_choice_selector returns the CHOICE selector value for |*pval|, which
// must of type |it|.
int asn1_get_choice_selector(ASN1_VALUE **pval, const ASN1_ITEM *it);

int asn1_set_choice_selector(ASN1_VALUE **pval, int value, const ASN1_ITEM *it);

// asn1_get_field_ptr returns a pointer to the field in |*pval| corresponding to
// |tt|.
ASN1_VALUE **asn1_get_field_ptr(ASN1_VALUE **pval, const ASN1_TEMPLATE *tt);

// asn1_do_adb returns the |ASN1_TEMPLATE| for the ANY DEFINED BY field |tt|,
// based on the selector INTEGER or OID in |*pval|. If |tt| is not an ADB field,
// it returns |tt|. If the selector does not match any value, it returns NULL.
// If |nullerr| is non-zero, it will additionally push an error to the error
// queue when there is no match.
const ASN1_TEMPLATE *asn1_do_adb(ASN1_VALUE **pval, const ASN1_TEMPLATE *tt,
                                 int nullerr);

void asn1_refcount_set_one(ASN1_VALUE **pval, const ASN1_ITEM *it);
int asn1_refcount_dec_and_test_zero(ASN1_VALUE **pval, const ASN1_ITEM *it);

void asn1_enc_init(ASN1_VALUE **pval, const ASN1_ITEM *it);
void asn1_enc_free(ASN1_VALUE **pval, const ASN1_ITEM *it);

// asn1_enc_restore, if |*pval| has a saved encoding, writes it to |out| under
// the i2d output convention, sets |*len| to the length, and returns one. If it
// has no saved encoding, it returns zero.
int asn1_enc_restore(int *len, unsigned char **out, ASN1_VALUE **pval,
                     const ASN1_ITEM *it);

// asn1_enc_save saves |inlen| bytes from |in| as |*pval|'s saved encoding. It
// returns one on success and zero on error. If |buf| is non-NULL, |in| must
// point into |buf|.
int asn1_enc_save(ASN1_VALUE **pval, const uint8_t *in, size_t inlen,
                  const ASN1_ITEM *it, CRYPTO_BUFFER *buf);

// asn1_encoding_clear clears the cached encoding in |enc|.
void asn1_encoding_clear(ASN1_ENCODING *enc);

// asn1_type_value_as_pointer returns |a|'s value in pointer form. This is
// usually the value object but, for BOOLEAN values, is 0 or 0xff cast to
// a pointer.
const void *asn1_type_value_as_pointer(const ASN1_TYPE *a);

// asn1_type_set0_string sets |a|'s value to the object represented by |str| and
// takes ownership of |str|.
void asn1_type_set0_string(ASN1_TYPE *a, ASN1_STRING *str);

// asn1_type_cleanup releases memory associated with |a|'s value, without
// freeing |a| itself.
void asn1_type_cleanup(ASN1_TYPE *a);

// asn1_is_printable returns one if |value| is a valid Unicode codepoint for an
// ASN.1 PrintableString, and zero otherwise.
int asn1_is_printable(uint32_t value);

// asn1_bit_string_length returns the number of bytes in |str| and sets
// |*out_padding_bits| to the number of padding bits.
//
// This function should be used instead of |ASN1_STRING_length| to correctly
// handle the non-|ASN1_STRING_FLAG_BITS_LEFT| case.
int asn1_bit_string_length(const ASN1_BIT_STRING *str,
                           uint8_t *out_padding_bits);

// asn1_marshal_bit_string marshals |in| as a DER-encoded, ASN.1 BIT STRING and
// writes the result to |out|. It returns one on success and zero on error. If
// |tag| is non-zero, the tag is replaced with |tag|.
int asn1_marshal_bit_string(CBB *out, const ASN1_BIT_STRING *in,
                            CBS_ASN1_TAG tag);

// asn1_marshal_integer marshals |in| as a DER-encoded, ASN.1 INTEGER and writes
// the result to |out|. It returns one on success and zero on error. If |tag| is
// non-zero, the tag is replaced with |tag|.
int asn1_marshal_integer(CBB *out, const ASN1_INTEGER *in, CBS_ASN1_TAG tag);

typedef struct {
  int nid;
  long minsize;
  long maxsize;
  unsigned long mask;
  unsigned long flags;
} ASN1_STRING_TABLE;

// asn1_get_string_table_for_testing sets |*out_ptr| and |*out_len| to the table
// of built-in |ASN1_STRING_TABLE| values. It is exported for testing.
OPENSSL_EXPORT void asn1_get_string_table_for_testing(
    const ASN1_STRING_TABLE **out_ptr, size_t *out_len);

typedef ASN1_VALUE *ASN1_new_func(void);
typedef void ASN1_free_func(ASN1_VALUE *a);
typedef ASN1_VALUE *ASN1_d2i_func(ASN1_VALUE **a, const unsigned char **in,
                                  long length);
typedef int ASN1_i2d_func(ASN1_VALUE *a, unsigned char **in);

typedef int ASN1_ex_d2i(ASN1_VALUE **pval, const unsigned char **in, long len,
                        const ASN1_ITEM *it, int opt, ASN1_TLC *ctx);

typedef int ASN1_ex_i2d(ASN1_VALUE **pval, unsigned char **out,
                        const ASN1_ITEM *it);
typedef int ASN1_ex_new_func(ASN1_VALUE **pval, const ASN1_ITEM *it);
typedef void ASN1_ex_free_func(ASN1_VALUE **pval, const ASN1_ITEM *it);

typedef struct ASN1_EXTERN_FUNCS_st {
  ASN1_ex_new_func *asn1_ex_new;
  ASN1_ex_free_func *asn1_ex_free;
  ASN1_ex_d2i *asn1_ex_d2i;
  ASN1_ex_i2d *asn1_ex_i2d;
} ASN1_EXTERN_FUNCS;

// ASN1_ANY_AS_STRING is an |ASN1_ITEM| with ASN.1 type ANY and C type
// |ASN1_STRING*|. Types which are not represented with |ASN1_STRING|, such as
// |ASN1_OBJECT|, are represented with type |V_ASN1_OTHER|.
DECLARE_ASN1_ITEM(ASN1_ANY_AS_STRING)


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_ASN1_INTERNAL_H
