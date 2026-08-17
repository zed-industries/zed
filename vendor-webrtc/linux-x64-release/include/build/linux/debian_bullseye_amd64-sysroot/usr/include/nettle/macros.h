/* macros.h

   Copyright (C) 2001, 2010 Niels Möller

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

#ifndef NETTLE_MACROS_H_INCLUDED
#define NETTLE_MACROS_H_INCLUDED

/* Reads a 64-bit integer, in network, big-endian, byte order */
#define READ_UINT64(p)				\
(  (((uint64_t) (p)[0]) << 56)			\
 | (((uint64_t) (p)[1]) << 48)			\
 | (((uint64_t) (p)[2]) << 40)			\
 | (((uint64_t) (p)[3]) << 32)			\
 | (((uint64_t) (p)[4]) << 24)			\
 | (((uint64_t) (p)[5]) << 16)			\
 | (((uint64_t) (p)[6]) << 8)			\
 |  ((uint64_t) (p)[7]))

#define WRITE_UINT64(p, i)			\
do {						\
  (p)[0] = ((i) >> 56) & 0xff;			\
  (p)[1] = ((i) >> 48) & 0xff;			\
  (p)[2] = ((i) >> 40) & 0xff;			\
  (p)[3] = ((i) >> 32) & 0xff;			\
  (p)[4] = ((i) >> 24) & 0xff;			\
  (p)[5] = ((i) >> 16) & 0xff;			\
  (p)[6] = ((i) >> 8) & 0xff;			\
  (p)[7] = (i) & 0xff;				\
} while(0)

/* Reads a 32-bit integer, in network, big-endian, byte order */
#define READ_UINT32(p)				\
(  (((uint32_t) (p)[0]) << 24)			\
 | (((uint32_t) (p)[1]) << 16)			\
 | (((uint32_t) (p)[2]) << 8)			\
 |  ((uint32_t) (p)[3]))

#define WRITE_UINT32(p, i)			\
do {						\
  (p)[0] = ((i) >> 24) & 0xff;			\
  (p)[1] = ((i) >> 16) & 0xff;			\
  (p)[2] = ((i) >> 8) & 0xff;			\
  (p)[3] = (i) & 0xff;				\
} while(0)

/* Analogous macros, for 24 and 16 bit numbers */
#define READ_UINT24(p)				\
(  (((uint32_t) (p)[0]) << 16)			\
 | (((uint32_t) (p)[1]) << 8)			\
 |  ((uint32_t) (p)[2]))

#define WRITE_UINT24(p, i)			\
do {						\
  (p)[0] = ((i) >> 16) & 0xff;			\
  (p)[1] = ((i) >> 8) & 0xff;			\
  (p)[2] = (i) & 0xff;				\
} while(0)

#define READ_UINT16(p)				\
(  (((uint32_t) (p)[0]) << 8)			\
 |  ((uint32_t) (p)[1]))

#define WRITE_UINT16(p, i)			\
do {						\
  (p)[0] = ((i) >> 8) & 0xff;			\
  (p)[1] = (i) & 0xff;				\
} while(0)

/* And the other, little-endian, byteorder */
#define LE_READ_UINT64(p)			\
(  (((uint64_t) (p)[7]) << 56)			\
 | (((uint64_t) (p)[6]) << 48)			\
 | (((uint64_t) (p)[5]) << 40)			\
 | (((uint64_t) (p)[4]) << 32)			\
 | (((uint64_t) (p)[3]) << 24)			\
 | (((uint64_t) (p)[2]) << 16)			\
 | (((uint64_t) (p)[1]) << 8)			\
 |  ((uint64_t) (p)[0]))

#define LE_WRITE_UINT64(p, i)			\
do {						\
  (p)[7] = ((i) >> 56) & 0xff;			\
  (p)[6] = ((i) >> 48) & 0xff;			\
  (p)[5] = ((i) >> 40) & 0xff;			\
  (p)[4] = ((i) >> 32) & 0xff;			\
  (p)[3] = ((i) >> 24) & 0xff;			\
  (p)[2] = ((i) >> 16) & 0xff;			\
  (p)[1] = ((i) >> 8) & 0xff;			\
  (p)[0] = (i) & 0xff;				\
} while (0)

#define LE_READ_UINT32(p)			\
(  (((uint32_t) (p)[3]) << 24)			\
 | (((uint32_t) (p)[2]) << 16)			\
 | (((uint32_t) (p)[1]) << 8)			\
 |  ((uint32_t) (p)[0]))

#define LE_WRITE_UINT32(p, i)			\
do {						\
  (p)[3] = ((i) >> 24) & 0xff;			\
  (p)[2] = ((i) >> 16) & 0xff;			\
  (p)[1] = ((i) >> 8) & 0xff;			\
  (p)[0] = (i) & 0xff;				\
} while(0)

/* Analogous macros, for 16 bit numbers */
#define LE_READ_UINT16(p)			\
  (  (((uint32_t) (p)[1]) << 8)			\
     |  ((uint32_t) (p)[0]))

#define LE_WRITE_UINT16(p, i)			\
  do {						\
    (p)[1] = ((i) >> 8) & 0xff;			\
    (p)[0] = (i) & 0xff;			\
  } while(0)

/* Macro to make it easier to loop over several blocks. */
#define FOR_BLOCKS(length, dst, src, blocksize)	\
  assert( !((length) % (blocksize)));           \
  for (; (length); ((length) -= (blocksize),	\
		  (dst) += (blocksize),		\
		  (src) += (blocksize)) )

/* The masking of the right shift is needed to allow n == 0 (using
   just 32 - n and 64 - n results in undefined behaviour). Most uses
   of these macros use a constant and non-zero rotation count. */
#define ROTL32(n,x) (((x)<<(n)) | ((x)>>((-(n)&31))))

#define ROTL64(n,x) (((x)<<(n)) | ((x)>>((-(n))&63)))

/* Requires that size > 0 */
#define INCREMENT(size, ctr)			\
  do {						\
    unsigned increment_i = (size) - 1;		\
    if (++(ctr)[increment_i] == 0)		\
      while (increment_i > 0			\
	     && ++(ctr)[--increment_i] == 0 )	\
	;					\
  } while (0)


/* Helper macro for Merkle-Damgård hash functions. Assumes the context
   structs includes the following fields:

     uint8_t block[...];		// Buffer holding one block
     unsigned int index;		// Index into block
*/

/* Currently used by sha512 (and sha384) only. */
#define MD_INCR(ctx) ((ctx)->count_high += !++(ctx)->count_low)

/* Takes the compression function f as argument. NOTE: also clobbers
   length and data. */
#define MD_UPDATE(ctx, length, data, f, incr)				\
  do {									\
    if ((ctx)->index)							\
      {									\
	/* Try to fill partial block */					\
	unsigned __md_left = sizeof((ctx)->block) - (ctx)->index;	\
	if ((length) < __md_left)					\
	  {								\
	    memcpy((ctx)->block + (ctx)->index, (data), (length));	\
	    (ctx)->index += (length);					\
	    goto __md_done; /* Finished */				\
	  }								\
	else								\
	  {								\
	    memcpy((ctx)->block + (ctx)->index, (data), __md_left);	\
									\
	    f((ctx), (ctx)->block);					\
	    (incr);							\
									\
	    (data) += __md_left;					\
	    (length) -= __md_left;					\
	  }								\
      }									\
    while ((length) >= sizeof((ctx)->block))				\
      {									\
	f((ctx), (data));						\
	(incr);								\
									\
	(data) += sizeof((ctx)->block);					\
	(length) -= sizeof((ctx)->block);				\
      }									\
    memcpy ((ctx)->block, (data), (length));				\
    (ctx)->index = (length);						\
  __md_done:								\
    ;									\
  } while (0)

/* Pads the block to a block boundary with the bit pattern 1 0*,
   leaving size octets for the length field at the end. If needed,
   compresses the block and starts a new one. */
#define MD_PAD(ctx, size, f)						\
  do {									\
    unsigned __md_i;							\
    __md_i = (ctx)->index;						\
									\
    /* Set the first char of padding to 0x80. This is safe since there	\
       is always at least one byte free */				\
									\
    assert(__md_i < sizeof((ctx)->block));				\
    (ctx)->block[__md_i++] = 0x80;					\
									\
    if (__md_i > (sizeof((ctx)->block) - (size)))			\
      { /* No room for length in this block. Process it and		\
	   pad with another one */					\
	memset((ctx)->block + __md_i, 0, sizeof((ctx)->block) - __md_i); \
									\
	f((ctx), (ctx)->block);						\
	__md_i = 0;							\
      }									\
    memset((ctx)->block + __md_i, 0,					\
	   sizeof((ctx)->block) - (size) - __md_i);			\
									\
  } while (0)

#endif /* NETTLE_MACROS_H_INCLUDED */
