/* sexp.h

   Parsing s-expressions.
   Copyright (C) 2002 Niels Möller

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
 
#ifndef NETTLE_SEXP_H_INCLUDED
#define NETTLE_SEXP_H_INCLUDED

#include <stdarg.h>
#include "nettle-types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define sexp_iterator_first nettle_sexp_iterator_first
#define sexp_transport_iterator_first nettle_sexp_transport_iterator_first
#define sexp_iterator_next nettle_sexp_iterator_next
#define sexp_iterator_enter_list nettle_sexp_iterator_enter_list
#define sexp_iterator_exit_list nettle_sexp_iterator_exit_list
#define sexp_iterator_subexpr nettle_sexp_iterator_subexpr
#define sexp_iterator_get_uint32 nettle_sexp_iterator_get_uint32
#define sexp_iterator_check_type nettle_sexp_iterator_check_type
#define sexp_iterator_check_types nettle_sexp_iterator_check_types
#define sexp_iterator_assoc nettle_sexp_iterator_assoc
#define sexp_format nettle_sexp_format
#define sexp_vformat nettle_sexp_vformat
#define sexp_transport_format nettle_sexp_transport_format
#define sexp_transport_vformat nettle_sexp_transport_vformat
#define sexp_token_chars nettle_sexp_token_chars

enum sexp_type
  { SEXP_ATOM, SEXP_LIST, SEXP_END };

struct sexp_iterator
{
  size_t length;
  const uint8_t *buffer;

  /* Points at the start of the current sub expression. */
  size_t start;
  /* If type is SEXP_LIST, pos points at the start of the current
   * element. Otherwise, it points at the end. */
  size_t pos;
  unsigned level;

  enum sexp_type type;
  
  size_t display_length;
  const uint8_t *display;

  size_t atom_length;
  const uint8_t *atom;
};


/* All these functions return 1 on success, 0 on failure */

/* Initializes the iterator. */
int
sexp_iterator_first(struct sexp_iterator *iterator,
		    size_t length, const uint8_t *input);

/* NOTE: Decodes the input string in place */
int
sexp_transport_iterator_first(struct sexp_iterator *iterator,
			      size_t length, uint8_t *input);

int
sexp_iterator_next(struct sexp_iterator *iterator);

/* Current element must be a list. */
int
sexp_iterator_enter_list(struct sexp_iterator *iterator);

/* Skips the rest of the current list */
int
sexp_iterator_exit_list(struct sexp_iterator *iterator);

#if 0
/* Skips out of as many lists as necessary to get back to the given
 * level. */
int
sexp_iterator_exit_lists(struct sexp_iterator *iterator,
			 unsigned level);
#endif

/* Gets start and length of the current subexpression. Implies
 * sexp_iterator_next. */
const uint8_t *
sexp_iterator_subexpr(struct sexp_iterator *iterator,
		      size_t *length);

int
sexp_iterator_get_uint32(struct sexp_iterator *iterator,
			 uint32_t *x);


/* Checks the type of the current expression, which should be a list
 *
 *  (<type> ...)
 */
int
sexp_iterator_check_type(struct sexp_iterator *iterator,
			 const char *type);

const char *
sexp_iterator_check_types(struct sexp_iterator *iterator,
			  unsigned ntypes,
			  const char * const *types);

/* Current element must be a list. Looks up element of type
 *
 *   (key rest...)
 *
 * For a matching key, the corresponding iterator is initialized
 * pointing at the start of REST.
 *
 * On success, exits the current list.
 */
int
sexp_iterator_assoc(struct sexp_iterator *iterator,
		    unsigned nkeys,
		    const char * const *keys,
		    struct sexp_iterator *values);


/* Output functions. What is a reasonable API for this? It seems
 * ugly to have to reimplement string streams. */

/* Declared for real in buffer.h */
struct nettle_buffer;

/* Returns the number of output characters, or 0 on out of memory. If
 * buffer == NULL, just compute length.
 *
 * Format strings can contained matched parentheses, tokens ("foo" in
 * the format string is formatted as "3:foo"), whitespace (which
 * separates tokens but is otherwise ignored) and the following
 * formatting specifiers:
 *
 *   %s   String represented as size_t length, const uint8_t *data.
 *
 *   %t   Optional display type, represented as
 *        size_t display_length, const uint8_t *display,
 *        display == NULL means no display type.
 *
 *   %i   Non-negative small integer, uint32_t.
 *
 *   %b   Non-negative bignum, mpz_t.
 *
 *   %l   Literal string (no length added), typically a balanced
 *        subexpression. Represented as size_t length, const uint8_t
 *        *data.
 *
 *   %(, %)  Allows insertion of unbalanced parenthesis.
 *
 * Modifiers:
 *
 *   %0   For %s, %t and %l, says that there's no length argument,
 *        instead the string is NUL-terminated, and there's only one
 *        const uint8_t * argument.
 */
 
size_t
sexp_format(struct nettle_buffer *buffer,
	    const char *format, ...);

size_t
sexp_vformat(struct nettle_buffer *buffer,
	     const char *format, va_list args);

size_t
sexp_transport_format(struct nettle_buffer *buffer,
		      const char *format, ...);

size_t
sexp_transport_vformat(struct nettle_buffer *buffer,
		       const char *format, va_list args);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_SEXP_H_INCLUDED */
