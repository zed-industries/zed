/* Copyright (C) 2009-2020 Free Software Foundation, Inc.
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

/* Declaration of types and functions for shadow group suite.  */

#ifndef _GSHADOW_H
#define _GSHADOW_H	1

#include <features.h>
#include <paths.h>
#include <bits/types/FILE.h>

#define __need_size_t
#include <stddef.h>

/* Path to the user database files.  */
#define	GSHADOW _PATH_GSHADOW


__BEGIN_DECLS

/* Structure of the group file.  */
struct sgrp
  {
    char *sg_namp;		/* Group name.  */
    char *sg_passwd;		/* Encrypted password.  */
    char **sg_adm;		/* Group administrator list.  */
    char **sg_mem;		/* Group member list.  */
  };


/* Open database for reading.

   This function is not part of POSIX and therefore no official
   cancellation point.  But due to similarity with an POSIX interface
   or due to the implementation it is a cancellation point and
   therefore not marked with __THROW.  */
extern void setsgent (void);

/* Close database.

   This function is not part of POSIX and therefore no official
   cancellation point.  But due to similarity with an POSIX interface
   or due to the implementation it is a cancellation point and
   therefore not marked with __THROW.  */
extern void endsgent (void);

/* Get next entry from database, perhaps after opening the file.

   This function is not part of POSIX and therefore no official
   cancellation point.  But due to similarity with an POSIX interface
   or due to the implementation it is a cancellation point and
   therefore not marked with __THROW.  */
extern struct sgrp *getsgent (void);

/* Get shadow entry matching NAME.

   This function is not part of POSIX and therefore no official
   cancellation point.  But due to similarity with an POSIX interface
   or due to the implementation it is a cancellation point and
   therefore not marked with __THROW.  */
extern struct sgrp *getsgnam (const char *__name);

/* Read shadow entry from STRING.

   This function is not part of POSIX and therefore no official
   cancellation point.  But due to similarity with an POSIX interface
   or due to the implementation it is a cancellation point and
   therefore not marked with __THROW.  */
extern struct sgrp *sgetsgent (const char *__string);

/* Read next shadow entry from STREAM.

   This function is not part of POSIX and therefore no official
   cancellation point.  But due to similarity with an POSIX interface
   or due to the implementation it is a cancellation point and
   therefore not marked with __THROW.  */
extern struct sgrp *fgetsgent (FILE *__stream);

/* Write line containing shadow password entry to stream.

   This function is not part of POSIX and therefore no official
   cancellation point.  But due to similarity with an POSIX interface
   or due to the implementation it is a cancellation point and
   therefore not marked with __THROW.  */
extern int putsgent (const struct sgrp *__g, FILE *__stream);


#ifdef __USE_MISC
/* Reentrant versions of some of the functions above.

   These functions are not part of POSIX and therefore no official
   cancellation point.  But due to similarity with an POSIX interface
   or due to the implementation they are cancellation points and
   therefore not marked with __THROW.  */
extern int getsgent_r (struct sgrp *__result_buf, char *__buffer,
		       size_t __buflen, struct sgrp **__result);

extern int getsgnam_r (const char *__name, struct sgrp *__result_buf,
		       char *__buffer, size_t __buflen,
		       struct sgrp **__result);

extern int sgetsgent_r (const char *__string, struct sgrp *__result_buf,
			char *__buffer, size_t __buflen,
			struct sgrp **__result);

extern int fgetsgent_r (FILE *__stream, struct sgrp *__result_buf,
			char *__buffer, size_t __buflen,
			struct sgrp **__result);
#endif	/* misc */

__END_DECLS

#endif /* gshadow.h */
