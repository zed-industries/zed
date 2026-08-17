/* buffer.h

   A bare-bones string stream.

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
 
#ifndef NETTLE_BUFFER_H_INCLUDED
#define NETTLE_BUFFER_H_INCLUDED

#include "realloc.h"

#ifdef __cplusplus
extern "C" {
#endif

struct nettle_buffer
{
  uint8_t *contents;
  /* Allocated size */
  size_t alloc;

  void *realloc_ctx;
  nettle_realloc_func *realloc;

  /* Current size */
  size_t size;
};

/* Initializes a buffer that uses plain realloc */
void
nettle_buffer_init(struct nettle_buffer *buffer);

void
nettle_buffer_init_realloc(struct nettle_buffer *buffer,
			   void *realloc_ctx,
			   nettle_realloc_func *realloc);

/* Initializes a buffer of fix size */
void
nettle_buffer_init_size(struct nettle_buffer *buffer,
			size_t length, uint8_t *space);

void
nettle_buffer_clear(struct nettle_buffer *buffer);

/* Resets the buffer, without freeing the buffer space. */
void
nettle_buffer_reset(struct nettle_buffer *buffer);

int
nettle_buffer_grow(struct nettle_buffer *buffer,
		   size_t length);

#define NETTLE_BUFFER_PUTC(buffer, c) \
( (((buffer)->size < (buffer)->alloc) || nettle_buffer_grow((buffer), 1)) \
  && ((buffer)->contents[(buffer)->size++] = (c), 1) )

int
nettle_buffer_write(struct nettle_buffer *buffer,
		    size_t length, const uint8_t *data);

/* Like nettle_buffer_write, but instead of copying data to the
 * buffer, it returns a pointer to the area where the caller can copy
 * the data. The pointer is valid only until the next call that can
 * reallocate the buffer. */
uint8_t *
nettle_buffer_space(struct nettle_buffer *buffer,
		    size_t length);

/* Copy the contents of SRC to the end of DST. */
int
nettle_buffer_copy(struct nettle_buffer *dst,
		   const struct nettle_buffer *src);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_BUFFER_H_INCLUDED */
