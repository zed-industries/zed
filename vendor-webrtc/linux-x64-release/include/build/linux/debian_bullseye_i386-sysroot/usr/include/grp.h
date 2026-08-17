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
 *	POSIX Standard: 9.2.1 Group Database Access	<grp.h>
 */

#ifndef	_GRP_H
#define	_GRP_H	1

#include <features.h>

__BEGIN_DECLS

#include <bits/types.h>

#define __need_size_t
#include <stddef.h>


/* For the Single Unix specification we must define this type here.  */
#if (defined __USE_XOPEN || defined __USE_XOPEN2K) && !defined __gid_t_defined
typedef __gid_t gid_t;
# define __gid_t_defined
#endif

/* The group structure.	 */
struct group
  {
    char *gr_name;		/* Group name.	*/
    char *gr_passwd;		/* Password.	*/
    __gid_t gr_gid;		/* Group ID.	*/
    char **gr_mem;		/* Member list.	*/
  };


#ifdef __USE_MISC
# include <bits/types/FILE.h>
#endif


#if defined __USE_MISC || defined __USE_XOPEN_EXTENDED
/* Rewind the group-file stream.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
extern void setgrent (void);

/* Close the group-file stream.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
extern void endgrent (void);

/* Read an entry from the group-file stream, opening it if necessary.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
extern struct group *getgrent (void);
#endif

#ifdef	__USE_MISC
/* Read a group entry from STREAM.

   This function is not part of POSIX and therefore no official
   cancellation point.  But due to similarity with an POSIX interface
   or due to the implementation it is a cancellation point and
   therefore not marked with __THROW.  */
extern struct group *fgetgrent (FILE *__stream);
#endif

#ifdef __USE_GNU
/* Write the given entry onto the given stream.

   This function is not part of POSIX and therefore no official
   cancellation point.  But due to similarity with an POSIX interface
   or due to the implementation it is a cancellation point and
   therefore not marked with __THROW.  */
extern int putgrent (const struct group *__restrict __p,
		     FILE *__restrict __f);
#endif

/* Search for an entry with a matching group ID.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
extern struct group *getgrgid (__gid_t __gid);

/* Search for an entry with a matching group name.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
extern struct group *getgrnam (const char *__name);

#ifdef __USE_POSIX

# ifdef __USE_MISC
/* Reasonable value for the buffer sized used in the reentrant
   functions below.  But better use `sysconf'.  */
#  define NSS_BUFLEN_GROUP	1024
# endif

/* Reentrant versions of some of the functions above.

   PLEASE NOTE: the `getgrent_r' function is not (yet) standardized.
   The interface may change in later versions of this library.  But
   the interface is designed following the principals used for the
   other reentrant functions so the chances are good this is what the
   POSIX people would choose.

   This function is not part of POSIX and therefore no official
   cancellation point.  But due to similarity with an POSIX interface
   or due to the implementation it is a cancellation point and
   therefore not marked with __THROW.  */

# ifdef __USE_GNU
extern int getgrent_r (struct group *__restrict __resultbuf,
		       char *__restrict __buffer, size_t __buflen,
		       struct group **__restrict __result);
# endif

/* Search for an entry with a matching group ID.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
extern int getgrgid_r (__gid_t __gid, struct group *__restrict __resultbuf,
		       char *__restrict __buffer, size_t __buflen,
		       struct group **__restrict __result);

/* Search for an entry with a matching group name.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
extern int getgrnam_r (const char *__restrict __name,
		       struct group *__restrict __resultbuf,
		       char *__restrict __buffer, size_t __buflen,
		       struct group **__restrict __result);

# ifdef	__USE_MISC
/* Read a group entry from STREAM.  This function is not standardized
   an probably never will.

   This function is not part of POSIX and therefore no official
   cancellation point.  But due to similarity with an POSIX interface
   or due to the implementation it is a cancellation point and
   therefore not marked with __THROW.  */
extern int fgetgrent_r (FILE *__restrict __stream,
			struct group *__restrict __resultbuf,
			char *__restrict __buffer, size_t __buflen,
			struct group **__restrict __result);
# endif

#endif	/* POSIX or reentrant */


#ifdef	__USE_MISC

# define __need_size_t
# include <stddef.h>

/* Set the group set for the current user to GROUPS (N of them).  */
extern int setgroups (size_t __n, const __gid_t *__groups) __THROW;

/* Store at most *NGROUPS members of the group set for USER into
   *GROUPS.  Also include GROUP.  The actual number of groups found is
   returned in *NGROUPS.  Return -1 if the if *NGROUPS is too small.

   This function is not part of POSIX and therefore no official
   cancellation point.  But due to similarity with an POSIX interface
   or due to the implementation it is a cancellation point and
   therefore not marked with __THROW.  */
extern int getgrouplist (const char *__user, __gid_t __group,
			 __gid_t *__groups, int *__ngroups);

/* Initialize the group set for the current user
   by reading the group database and using all groups
   of which USER is a member.  Also include GROUP.

   This function is not part of POSIX and therefore no official
   cancellation point.  But due to similarity with an POSIX interface
   or due to the implementation it is a cancellation point and
   therefore not marked with __THROW.  */
extern int initgroups (const char *__user, __gid_t __group);

#endif /* Use misc.  */

__END_DECLS

#endif /* grp.h  */
