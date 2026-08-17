/* Functions to access FILE structure internals.
   Copyright (C) 2000-2020 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <https://www.gnu.org/licenses/>.  */

/* This header contains the same definitions as the header of the same name
   on Sun's Solaris OS.  */

#ifndef _STDIO_EXT_H
#define _STDIO_EXT_H	1

#include <stdio.h>

enum
{
  /* Query current state of the locking status.  */
  FSETLOCKING_QUERY = 0,
#define FSETLOCKING_QUERY	FSETLOCKING_QUERY
  /* The library protects all uses of the stream functions, except for
     uses of the *_unlocked functions, by calls equivalent to flockfile().  */
  FSETLOCKING_INTERNAL,
#define FSETLOCKING_INTERNAL	FSETLOCKING_INTERNAL
  /* The user will take care of locking.  */
  FSETLOCKING_BYCALLER
#define FSETLOCKING_BYCALLER	FSETLOCKING_BYCALLER
};


__BEGIN_DECLS

/* Return the size of the buffer of FP in bytes currently in use by
   the given stream.  */
extern size_t __fbufsize (FILE *__fp) __THROW;


/* Return non-zero value iff the stream FP is opened readonly, or if the
   last operation on the stream was a read operation.  */
extern int __freading (FILE *__fp) __THROW;

/* Return non-zero value iff the stream FP is opened write-only or
   append-only, or if the last operation on the stream was a write
   operation.  */
extern int __fwriting (FILE *__fp) __THROW;


/* Return non-zero value iff stream FP is not opened write-only or
   append-only.  */
extern int __freadable (FILE *__fp) __THROW;

/* Return non-zero value iff stream FP is not opened read-only.  */
extern int __fwritable (FILE *__fp) __THROW;


/* Return non-zero value iff the stream FP is line-buffered.  */
extern int __flbf (FILE *__fp) __THROW;


/* Discard all pending buffered I/O on the stream FP.  */
extern void __fpurge (FILE *__fp) __THROW;

/* Return amount of output in bytes pending on a stream FP.  */
extern size_t __fpending (FILE *__fp) __THROW;

/* Flush all line-buffered files.  */
extern void _flushlbf (void);


/* Set locking status of stream FP to TYPE.  */
extern int __fsetlocking (FILE *__fp, int __type) __THROW;

__END_DECLS

#endif	/* stdio_ext.h */
