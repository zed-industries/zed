/* Copyright (C) 1991-2020 Free Software Foundation, Inc.
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

/*
 *	POSIX Standard: 9.2.2 User Database Access	<pwd.h>
 */

#ifndef	_PWD_H
#define	_PWD_H	1

#include <features.h>

__BEGIN_DECLS

#include <bits/types.h>

#define __need_size_t
#include <stddef.h>

#if defined __USE_XOPEN || defined __USE_XOPEN2K
/* The Single Unix specification says that some more types are
   available here.  */
# ifndef __gid_t_defined
typedef __gid_t gid_t;
#  define __gid_t_defined
# endif

# ifndef __uid_t_defined
typedef __uid_t uid_t;
#  define __uid_t_defined
# endif
#endif

/* A record in the user database.  */
struct passwd
{
  char *pw_name;		/* Username.  */
  char *pw_passwd;		/* Hashed passphrase, if shadow database
                                   not in use (see shadow.h).  */
  __uid_t pw_uid;		/* User ID.  */
  __gid_t pw_gid;		/* Group ID.  */
  char *pw_gecos;		/* Real name.  */
  char *pw_dir;			/* Home directory.  */
  char *pw_shell;		/* Shell program.  */
};


#ifdef __USE_MISC
# include <bits/types/FILE.h>
#endif


#if defined __USE_MISC || defined __USE_XOPEN_EXTENDED
/* Rewind the user database stream.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
extern void setpwent (void);

/* Close the user database stream.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
extern void endpwent (void);

/* Read an entry from the user database stream, opening it if necessary.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
extern struct passwd *getpwent (void);
#endif

#ifdef	__USE_MISC
/* Read a user database entry from STREAM.

   This function is not part of POSIX and therefore no official
   cancellation point.  But due to similarity with an POSIX interface
   or due to the implementation it is a cancellation point and
   therefore not marked with __THROW.  */
extern struct passwd *fgetpwent (FILE *__stream) __nonnull ((1));

/* Write a given user database entry onto the given stream.

   This function is not part of POSIX and therefore no official
   cancellation point.  But due to similarity with an POSIX interface
   or due to the implementation it is a cancellation point and
   therefore not marked with __THROW.  */
extern int putpwent (const struct passwd *__restrict __p,
		     FILE *__restrict __f);
#endif

/* Retrieve the user database entry for the given user ID.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
extern struct passwd *getpwuid (__uid_t __uid);

/* Retrieve the user database entry for the given username.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
extern struct passwd *getpwnam (const char *__name) __nonnull ((1));

#ifdef __USE_POSIX

# ifdef __USE_MISC
/* Reasonable value for the buffer sized used in the reentrant
   functions below.  But better use `sysconf'.  */
#  define NSS_BUFLEN_PASSWD	1024
# endif

/* Reentrant versions of some of the functions above.

   PLEASE NOTE: the `getpwent_r' function is not (yet) standardized.
   The interface may change in later versions of this library.  But
   the interface is designed following the principals used for the
   other reentrant functions so the chances are good this is what the
   POSIX people would choose.  */

# ifdef __USE_MISC
/* This function is not part of POSIX and therefore no official
   cancellation point.  But due to similarity with an POSIX interface
   or due to the implementation it is a cancellation point and
   therefore not marked with __THROW.  */
extern int getpwent_r (struct passwd *__restrict __resultbuf,
		       char *__restrict __buffer, size_t __buflen,
		       struct passwd **__restrict __result)
		       __nonnull ((1, 2, 4));
# endif

extern int getpwuid_r (__uid_t __uid,
		       struct passwd *__restrict __resultbuf,
		       char *__restrict __buffer, size_t __buflen,
		       struct passwd **__restrict __result)
		       __nonnull ((2, 3, 5));

extern int getpwnam_r (const char *__restrict __name,
		       struct passwd *__restrict __resultbuf,
		       char *__restrict __buffer, size_t __buflen,
		       struct passwd **__restrict __result)
		       __nonnull ((1, 2, 3, 5));


# ifdef	__USE_MISC
/* Read a user database entry from STREAM.  This function is not
   standardized and probably never will.

   This function is not part of POSIX and therefore no official
   cancellation point.  But due to similarity with an POSIX interface
   or due to the implementation it is a cancellation point and
   therefore not marked with __THROW.  */
extern int fgetpwent_r (FILE *__restrict __stream,
			struct passwd *__restrict __resultbuf,
			char *__restrict __buffer, size_t __buflen,
			struct passwd **__restrict __result)
			__nonnull ((1, 2, 3, 5));
# endif

#endif	/* POSIX or reentrant */

#ifdef __USE_GNU
/* Write a traditional /etc/passwd line, based on the user database
   entry for the given UID, to BUFFER; space for BUFFER must be
   allocated by the caller.

   This function is not part of POSIX and therefore no official
   cancellation point.  But due to similarity with an POSIX interface
   or due to the implementation it is a cancellation point and
   therefore not marked with __THROW.  */
extern int getpw (__uid_t __uid, char *__buffer);
#endif

__END_DECLS

#endif /* pwd.h  */
