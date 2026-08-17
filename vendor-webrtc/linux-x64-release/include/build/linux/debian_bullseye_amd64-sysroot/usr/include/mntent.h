/* Utilities for reading/writing fstab, mtab, etc.
   Copyright (C) 1995-2020 Free Software Foundation, Inc.
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

#ifndef	_MNTENT_H
#define	_MNTENT_H	1

#include <features.h>
#include <paths.h>
#include <bits/types/FILE.h>

/* File listing canonical interesting mount points.  */
#define	MNTTAB		_PATH_MNTTAB	/* Deprecated alias.  */

/* File listing currently active mount points.  */
#define	MOUNTED		_PATH_MOUNTED	/* Deprecated alias.  */


/* General filesystem types.  */
#define MNTTYPE_IGNORE	"ignore"	/* Ignore this entry.  */
#define MNTTYPE_NFS	"nfs"		/* Network file system.  */
#define MNTTYPE_SWAP	"swap"		/* Swap device.  */


/* Generic mount options.  */
#define MNTOPT_DEFAULTS	"defaults"	/* Use all default options.  */
#define MNTOPT_RO	"ro"		/* Read only.  */
#define MNTOPT_RW	"rw"		/* Read/write.  */
#define MNTOPT_SUID	"suid"		/* Set uid allowed.  */
#define MNTOPT_NOSUID	"nosuid"	/* No set uid allowed.  */
#define MNTOPT_NOAUTO	"noauto"	/* Do not auto mount.  */


__BEGIN_DECLS

/* Structure describing a mount table entry.  */
struct mntent
  {
    char *mnt_fsname;		/* Device or server for filesystem.  */
    char *mnt_dir;		/* Directory mounted on.  */
    char *mnt_type;		/* Type of filesystem: ufs, nfs, etc.  */
    char *mnt_opts;		/* Comma-separated options for fs.  */
    int mnt_freq;		/* Dump frequency (in days).  */
    int mnt_passno;		/* Pass number for `fsck'.  */
  };


/* Prepare to begin reading and/or writing mount table entries from the
   beginning of FILE.  MODE is as for `fopen'.  */
extern FILE *setmntent (const char *__file, const char *__mode) __THROW;

/* Read one mount table entry from STREAM.  Returns a pointer to storage
   reused on the next call, or null for EOF or error (use feof/ferror to
   check).  */
extern struct mntent *getmntent (FILE *__stream) __THROW;

#ifdef __USE_MISC
/* Reentrant version of the above function.  */
extern struct mntent *getmntent_r (FILE *__restrict __stream,
				   struct mntent *__restrict __result,
				   char *__restrict __buffer,
				   int __bufsize) __THROW;
#endif

/* Write the mount table entry described by MNT to STREAM.
   Return zero on success, nonzero on failure.  */
extern int addmntent (FILE *__restrict __stream,
		      const struct mntent *__restrict __mnt) __THROW;

/* Close a stream opened with `setmntent'.  */
extern int endmntent (FILE *__stream) __THROW;

/* Search MNT->mnt_opts for an option matching OPT.
   Returns the address of the substring, or null if none found.  */
extern char *hasmntopt (const struct mntent *__mnt,
			const char *__opt) __THROW;


__END_DECLS

#endif	/* mntent.h */
