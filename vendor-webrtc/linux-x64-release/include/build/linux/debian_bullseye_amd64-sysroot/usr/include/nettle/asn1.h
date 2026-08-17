/* asn1.h

   Limited support for ASN.1 DER decoding.

   Copyright (C) 2005 Niels Möller

   This file is part of GNU Nettle.

   GNU Nettle is free software: you can redistribute it and/or
   modify it under the terms of either:

     * the GNU Lesser General Public License as published by the Free
       Software Foundation; either version 3 of the License, or (at your
       option) any later version.

   or

     * the GNU General Public License as published by the Free
       Software Foundation; either version 2 of the License, or (at your
       option) any later version.

   or both in parallel, as here.

   GNU Nettle is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   General Public License for more details.

   You should have received copies of the GNU General Public License and
   the GNU Lesser General Public License along with this program.  If
   not, see http://www.gnu.org/licenses/.
*/

#ifndef NETTLE_ASN1_H_INCLUDED
#define NETTLE_ASN1_H_INCLUDED

#include "nettle-types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define asn1_der_iterator_first nettle_asn1_der_iterator_first
#define asn1_der_iterator_next nettle_asn1_der_iterator_next
#define asn1_der_decode_constructed nettle_asn1_der_decode_constructed
#define asn1_der_decode_constructed_last nettle_asn1_der_decode_constructed_last
#define asn1_der_decode_bitstring nettle_asn1_der_decode_bitstring
#define asn1_der_decode_bitstring_last nettle_asn1_der_decode_bitstring_last
#define asn1_der_get_uint32 nettle_asn1_der_get_uint32
#define asn1_der_get_bignum nettle_asn1_der_get_bignum


/* enum asn1_type keeps the class number and the constructive in bits
   13-14, and the constructive flag in bit 12. The remaining 14 bits
   are the tag (although currently, only tags in the range 0-30 are
   supported). */

enum
  {
    ASN1_TYPE_CONSTRUCTED = 1 << 12,

    ASN1_CLASS_UNIVERSAL = 0,
    ASN1_CLASS_APPLICATION = 1 << 13,
    ASN1_CLASS_CONTEXT_SPECIFIC = 2 << 13,
    ASN1_CLASS_PRIVATE = 3 << 13,

    ASN1_CLASS_MASK = 3 << 13,
    ASN1_CLASS_SHIFT = 13,
  };

enum asn1_type
  {
    ASN1_BOOLEAN = 1,
    ASN1_INTEGER = 2,
    ASN1_BITSTRING = 3,
    ASN1_OCTETSTRING = 4,
    ASN1_NULL = 5,
    ASN1_IDENTIFIER = 6,
    ASN1_REAL = 9,
    ASN1_ENUMERATED = 10,
    ASN1_UTF8STRING = 12,
    ASN1_SEQUENCE = 16 | ASN1_TYPE_CONSTRUCTED,
    ASN1_SET = 17 | ASN1_TYPE_CONSTRUCTED,
    ASN1_PRINTABLESTRING = 19,
    ASN1_TELETEXSTRING = 20,
    ASN1_IA5STRING = 22,
    ASN1_UTC = 23,
    ASN1_UNIVERSALSTRING = 28,
    ASN1_BMPSTRING = 30,
  };

enum asn1_iterator_result
  {
    ASN1_ITERATOR_ERROR,
    ASN1_ITERATOR_PRIMITIVE,
    ASN1_ITERATOR_CONSTRUCTED,
    ASN1_ITERATOR_END,
  };

/* Parsing DER objects. */
struct asn1_der_iterator
{
  size_t buffer_length;
  const uint8_t *buffer;

  /* Next object to parse. */
  size_t pos;

  enum asn1_type type;

  /* Pointer to the current object */
  size_t length;
  const uint8_t *data;
};

/* Initializes the iterator. */
enum asn1_iterator_result
asn1_der_iterator_first(struct asn1_der_iterator *iterator,
			size_t length, const uint8_t *input);

enum asn1_iterator_result
asn1_der_iterator_next(struct asn1_der_iterator *iterator);

/* Starts parsing of a constructed object. */
enum asn1_iterator_result
asn1_der_decode_constructed(struct asn1_der_iterator *i,
			    struct asn1_der_iterator *contents);

/* For the common case that we have a sequence at the end of the
   object. Checks that the current object is the final one, and then
   reinitializes the iterator to parse its ontents. */
enum asn1_iterator_result
asn1_der_decode_constructed_last(struct asn1_der_iterator *i);

enum asn1_iterator_result
asn1_der_decode_bitstring(struct asn1_der_iterator *i,
			  struct asn1_der_iterator *contents);

enum asn1_iterator_result
asn1_der_decode_bitstring_last(struct asn1_der_iterator *i);

/* All these functions return 1 on success, 0 on failure */
int
asn1_der_get_uint32(struct asn1_der_iterator *i,
		    uint32_t *x);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_ASN1_H_INCLUDED */
